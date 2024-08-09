use super::SbiServer;
use async_trait::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use openapi_smf::apis::individual_sm_context::{
	IndividualSmContext, ReleaseSmContextResponse, RetrieveSmContextResponse, SendMoDataResponse,
	UpdateSmContextResponse,
};
use openapi_smf::models;
/// IndividualSmContext
#[async_trait]
#[allow(clippy::ptr_arg)]
impl IndividualSmContext for SbiServer {
	/// Release SM Context.
	///
	/// ReleaseSmContext - POST /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/release
	async fn release_sm_context(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ReleaseSmContextPathParams,
		body: axum::body::Body,
	) -> Result<ReleaseSmContextResponse, String> {
		todo!();
	}

	/// Retrieve SM Context.
	///
	/// RetrieveSmContext - POST /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/retrieve
	async fn retrieve_sm_context(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::RetrieveSmContextPathParams,
		body: Option<models::SmContextRetrieveData>,
	) -> Result<RetrieveSmContextResponse, String> {
		todo!();
	}

	/// Send MO Data.
	///
	/// SendMoData - POST /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/send-mo-data
	async fn send_mo_data(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::SendMoDataPathParams,
		body: axum::body::Body,
	) -> Result<SendMoDataResponse, String> {
		todo!();
	}

	/// Update SM Context.
	///
	/// UpdateSmContext - POST /nsmf-pdusession/v1/nsmf-pdusession/v1/sm-contexts/{smContextRef}/modify
	async fn update_sm_context(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UpdateSmContextPathParams,
		body: axum::body::Body,
	) -> Result<UpdateSmContextResponse, String> {
		todo!();
	}
}
