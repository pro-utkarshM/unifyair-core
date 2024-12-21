#![feature(error_generic_member_access)]
#![feature(adt_const_params)]
#![feature(async_closure)]

use std::{backtrace::Backtrace, error::Error, fmt::Debug};
use http::{
	Request as HttpRequest,
	Version,
	request::Builder as HttpReqBuilder,
};

use reqwest::Body; 

use oasbi::{ReqError, common::ProblemDetails};
use reqwest::{Client, Method, Request, Url};
use serde::Serialize;
use thiserror::Error;

mod header_map_serializer;
pub mod nf_client;
pub mod nrf_client;
pub use header_map_serializer::{HeaderSerDeError, to_headers};

pub struct NFConfig {}

pub trait NFType {}

pub struct AmfClient {
	client: Client,
}

pub struct NFProxy {}

#[derive(Debug, Error)]
pub enum ServiceError<E, I>
where
	E: Error + Debug,
	I: Error + Debug,
{
	#[error("Service Error: {0}")]
	Service(
		#[source]
		#[backtrace]
		E,
	),
	#[error("The Inner Error: {0}")]
	Inner(
		#[from]
		#[backtrace]
		I,
	),
}

#[derive(Debug, Error)]
pub enum GenericClientError {
	#[error("Request Execution Failed")]
	ClientRequestError(#[from] reqwest::Error),

	#[error("Response Parsing Failed")]
	ResponseParseError(
		#[from]
		#[backtrace]
		ReqError,
	),

	#[error("Response Failed with status: {0}")]
	InvalidResponse(u16, Option<ProblemDetails>, #[backtrace] Backtrace),

	#[error("Invalid Header: {0}")]
	HeaderSerDeError(
		#[from] header_map_serializer::HeaderSerDeError,
		#[backtrace] Backtrace,
	),

	#[error("Invalid Query: {0}")]
	QuerySerDeError(#[from] serde_urlencoded::ser::Error),

	#[error("Error while serializing body: {0}")]
	SerializationError(#[from] serde_json::Error),

	#[error("Error while preparing path: {0}")]
	PathCreationError(#[from] formatx::Error),

	#[error("Error while building the request: {0}")]
	BuilderError(#[from] http::Error),

	#[error("Error while building url: {0}")]
	UriBuilderError(#[from] url::ParseError),

	#[error("Error While making tower request: {0}")]
	TowerHttpError(#[from] tower_reqwest::Error),
}

pub fn prepare_request<H, Q, B>(
	url: Url,
	path: &str,
	method: Method,
	header: Option<&H>,
	query: Option<&Q>,
	body: Option<&B>,
) -> Result<Request, GenericClientError>
where
	Q: Serialize,
	H: Serialize,
	B: Serialize,
{
	let mut url = url;
	url.set_path(path);
	let mut request = Request::new(method, url);
	let mut pairs = request.url_mut().query_pairs_mut();
	if query.is_some() {
		let serializer = serde_urlencoded::Serializer::new(&mut pairs);
		query.serialize(serializer)?;
	}
	drop(pairs);
	header
		.map(|h| to_headers(&h))
		.transpose()?
		.map(|h| request.headers_mut().extend(h));
	*request.body_mut() = body
		.map(|t| serde_json::to_vec(t).map(|t| t.into()))
		.transpose()?;
	Ok(request)
}

pub fn prepare_http_request<H, Q, B>(
	base_url: &str,
	path: &str,
	method: Method,
	header: Option<&H>,
	query: Option<&Q>,
	body: Option<&B>,
) -> Result<HttpRequest<Body>, GenericClientError>
where
	Q: Serialize,
	H: Serialize,
	B: Serialize,
{
	let mut url = Url::parse(base_url)?;
	url.join(path)?;

	query
		.map(|q| serde_urlencoded::to_string(q))
		.transpose()?
		.map(|q| url.set_query(Some(&q)));

	let request = HttpReqBuilder::new()
		.version(Version::HTTP_2)
		.uri(url.to_string())
		.method(method);

	let body: Body = body.map_or(Ok(Body::default()), |t| {
		serde_json::to_vec(t).map(|t| t.into())
	})?;
	let mut req = request.body(body)?;
	header
		.map(|h| to_headers(h))
		.transpose()?
		.map(|h| req.headers_mut().extend(h));

	Ok(req)
}
