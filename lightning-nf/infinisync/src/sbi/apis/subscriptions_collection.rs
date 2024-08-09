use super::SbiServer;
use async_trait::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use openapi_smf::apis::subscriptions_collection::{
	CreateIndividualSubcriptionResponse, SubscriptionsCollection,
};
use openapi_smf::models;

/// SubscriptionsCollection
#[async_trait]
#[allow(clippy::ptr_arg)]
impl SubscriptionsCollection for SbiServer {
	/// Create an individual subscription for event notifications from the SMF.
	///
	/// CreateIndividualSubcription - POST /nsmf-pdusession/v1/nsmf-event-exposure/v1/subscriptions
	async fn create_individual_subcription(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		body: models::NsmfEventExposure,
	) -> Result<CreateIndividualSubcriptionResponse, String> {
		todo!();
	}
}
