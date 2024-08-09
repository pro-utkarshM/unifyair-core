use super::SbiServer;
use async_trait::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use openapi_smf::apis::sm_contexts_collection::{PostSmContextsResponse, SmContextsCollection};
use openapi_smf::models;

/// SmContextsCollection
#[async_trait]
#[allow(clippy::ptr_arg)]
impl SmContextsCollection for SbiServer {
	/// Create SM Context.
	///
	/// PostSmContexts - POST /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts
	async fn post_sm_contexts(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: axum::body::Body,
	) -> Result<PostSmContextsResponse, String> {
		todo!();
	}
}
