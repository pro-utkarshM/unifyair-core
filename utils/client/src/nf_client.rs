use std::{iter::once, sync::Arc};

use http::{
	Request as HttpRequest,
	Response as HttpResponse,
	StatusCode,
	header::AUTHORIZATION,
}; use http_body_util::BodyExt;
use oasbi::{DeserResponse, common::NfType, nrf::types::NfProfile};
use openapi_nrf::models::{
	SearchNfInstancesHeaderParams,
	SearchNfInstancesQueryParams,
	SearchResult,
};
use reqwest::{Body, Client, ClientBuilder, Response};
use serde::Serialize;
use thiserror::Error;
use tower::{
	Service,
	ServiceBuilder,
	ServiceExt,
};
use tower_http::{
	classify::{ServerErrorsAsFailures, SharedClassifier},
	sensitive_headers::{SetSensitiveRequestHeaders, SetSensitiveRequestHeadersLayer},
	trace::{Trace, TraceLayer},
};
use tower_reqwest::{HttpClientLayer, HttpClientService};
use url::Url;

use crate::{
	GenericClientError,
	nrf_client::{NrfClient, NrfDiscoveryError},
};

pub trait ApiBaseUrl {
	fn base_url(&self) -> Url;
}

pub trait NfClientController {
	const CLIENT_TYPE: NfType;
	fn profile_selection(
		&self,
		search_result: SearchResult,
	) -> NfProfile;
	fn get_search_params(
		&self,
		requester_nf_type: NfType,
	) -> SearchNfInstancesQueryParams {
		SearchNfInstancesQueryParams {
			requester_nf_type,
			target_nf_type: Self::CLIENT_TYPE,
			..Default::default()
		}
	}
}

type TowerReqwestClient = SetSensitiveRequestHeaders<
	Trace<HttpClientService<Client>, SharedClassifier<ServerErrorsAsFailures>>,
>;

pub struct NFClient<T, const APP_TYPE: NfType> {
	nrf_client: Arc<NrfClient>,
	controller: T,
	nf_profile: NfProfile,
	req_client: TowerReqwestClient,
}

impl<T, const APP_TYPE: NfType> NFClient<T, APP_TYPE>
where
	T: NfClientController + ApiBaseUrl,
{
	pub async fn new(
		nrf_client: Arc<NrfClient>,
		controller: T,
	) -> Result<Self, NfClientError> {
		let url = controller.base_url();
		let search_params = controller.get_search_params(APP_TYPE);
		let header_params = SearchNfInstancesHeaderParams {
			..Default::default()
		};
		let search_result = nrf_client
			.search_nf_instance(url, search_params, header_params)
			.await?;
		let nf_profile = controller.profile_selection(search_result);
		let builder = ClientBuilder::new();
		let client = builder.build()?;

		let service = ServiceBuilder::new()
			// Mark the `Authorization` request header as sensitive so it doesn't show in logs
			.layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
			// High level logging of requests and responses
			.layer(TraceLayer::new_for_http())
			.layer(HttpClientLayer)
			.service(client);

		Ok(NFClient {
			nrf_client,
			controller,
			nf_profile,
			req_client: service,
		})
	}


	pub async fn request<H, Q, B, Resp>(
		&self,
	    req: HttpRequest<Body>
	) -> Result<(StatusCode, Resp), GenericClientError>
	where
		Q: Serialize,
		H: Serialize,
		B: Serialize,
		Resp: DeserResponse,
	{
		let mut service = self.req_client.clone();
		let resp = service.ready().await?.call(req).await?;
		let (parts, body) = resp.into_parts();
		let body_stream = body.into_data_stream();
		let resp_body = Body::wrap_stream(body_stream);
		let resp = HttpResponse::from_parts(parts, resp_body);
		let req_resp = Response::from(resp);
		Ok(Resp::deserialize(req_resp).await?)
	}
}

#[derive(Error, Debug)]
pub enum NfClientError {
	#[error("Error while creating client")]
	ClientCreationError(
		#[from]
		#[backtrace]
		reqwest::Error,
	),
	#[error("Nrf Search Error")]
	NrfDiscoveryError(
		#[from]
		#[backtrace]
		NrfDiscoveryError,
	),
}
