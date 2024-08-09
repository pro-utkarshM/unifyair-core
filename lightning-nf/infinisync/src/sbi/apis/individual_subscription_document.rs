use super::SbiServer;
use async_trait::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use openapi_smf::apis::individual_subscription_document::{
	DeleteIndividualSubcriptionResponse, GetIndividualSubcriptionResponse,
	IndividualSubscriptionDocument, ReplaceIndividualSubcriptionResponse,
};
use openapi_smf::models;

/// IndividualSubscriptionDocument
#[async_trait]
#[allow(clippy::ptr_arg)]
impl IndividualSubscriptionDocument for SbiServer {
	/// Delete an individual subscription for event notifications from the SMF.
	///
	/// DeleteIndividualSubcription - DELETE /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions/{subId}
	async fn delete_individual_subcription(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::DeleteIndividualSubcriptionPathParams,
	) -> Result<DeleteIndividualSubcriptionResponse, String> {
		todo!();
	}

	/// Read an individual subscription for event notifications from the SMF.
	///
	/// GetIndividualSubcription - GET /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions/{subId}
	async fn get_individual_subcription(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::GetIndividualSubcriptionPathParams,
	) -> Result<GetIndividualSubcriptionResponse, String> {
		todo!();
	}

	/// Replace an individual subscription for event notifications from the SMF.
	///
	/// ReplaceIndividualSubcription - PUT /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions/{subId}
	async fn replace_individual_subcription(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ReplaceIndividualSubcriptionPathParams,
		body: models::NsmfEventExposure,
	) -> Result<ReplaceIndividualSubcriptionResponse, String> {
		todo!();
	}
}
