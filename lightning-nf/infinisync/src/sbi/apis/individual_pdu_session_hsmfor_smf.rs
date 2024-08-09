use super::SbiServer;
use async_trait::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use openapi_smf::apis::individual_pdu_session_hsmfor_smf::IndividualPduSessionHsmforSmf;
use openapi_smf::apis::individual_pdu_session_hsmfor_smf::{
	ReleasePduSessionResponse, RetrievePduSessionResponse, TransferMoDataResponse,
	UpdatePduSessionResponse,
};
use openapi_smf::models;

#[async_trait]
#[allow(clippy::ptr_arg)]
impl IndividualPduSessionHsmforSmf for SbiServer {
	/// Release.
	///
	/// ReleasePduSession - POST /nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/release
	async fn release_pdu_session(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::ReleasePduSessionPathParams,
		body: axum::body::Body,
	) -> Result<ReleasePduSessionResponse, String> {
		todo!();
	}

	/// Retrieve.
	///
	/// RetrievePduSession - POST /nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/retrieve
	async fn retrieve_pdu_session(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::RetrievePduSessionPathParams,
		body: models::RetrieveData,
	) -> Result<RetrievePduSessionResponse, String> {
		todo!();
	}

	/// Transfer MO Data.
	///
	/// TransferMoData - POST /nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/transfer-mo-data
	async fn transfer_mo_data(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::TransferMoDataPathParams,
		body: axum::body::Body,
	) -> Result<TransferMoDataResponse, String> {
		todo!();
	}

	/// Update (initiated by V-SMF or I-SMF).
	///
	/// UpdatePduSession - POST /nsmf-pdusession/v1/pdu-sessions/{pduSessionRef}/modify
	async fn update_pdu_session(
		&self,
		method: Method,
		host: Host,
		cookies: CookieJar,
		path_params: models::UpdatePduSessionPathParams,
		body: axum::body::Body,
	) -> Result<UpdatePduSessionResponse, String> {
		todo!();
	}
}
