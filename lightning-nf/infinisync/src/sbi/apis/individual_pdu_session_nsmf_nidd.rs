use super::SbiServer;
use async_trait::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use openapi_smf::apis::individual_pdu_session_nsmf_nidd::{
	DeliverResponse, IndividualPduSessionNsmfNidd,
};
use openapi_smf::models;

/// IndividualPduSessionNsmfNidd
#[async_trait]
#[allow(clippy::ptr_arg)]
impl IndividualPduSessionNsmfNidd for SbiServer {
	/// Delivery Service Operation.
	///
	/// Deliver - POST /nsmf-pdusession/v1/nsmf-nidd/v1/pdu-sessions/{pduSessionRef}/deliver
	async fn deliver(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::DeliverPathParams,
		body: axum::body::Body,
	) -> Result<DeliverResponse, String> {
		todo!();
	}
}
