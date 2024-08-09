use super::SbiServer;
use async_trait::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use openapi_smf::apis::pdu_sessions_collection::{PduSessionsCollection, PostPduSessionsResponse};
use openapi_smf::models;

/// PduSessionsCollection
#[async_trait]
#[allow(clippy::ptr_arg)]
impl PduSessionsCollection for SbiServer {
	/// Create.
	///
	/// PostPduSessions - POST /nsmf-pdusession/v1/nsmf-pdusession/v1/pdu-sessions
	async fn post_pdu_sessions(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: axum::body::Body,
	) -> Result<PostPduSessionsResponse, String> {
		todo!();
	}
}
