use std::{backtrace::Backtrace, str::FromStr};

use formatx::formatx;
use oasbi::{
	common::NfInstanceId,
	service_properties::{
		NrfNFDiscoveryOperation,
		NrfNFManagementOperation,
		NrfService,
		ServiceProperties,
	},
	DeserResponse,
};
use openapi_nrf::{
	apis::{
		nf_instance_id_document::{DeregisterNfInstanceResponse, RegisterNfInstanceResponse},
		nf_instances_store::SearchNfInstancesResponse,
	},
	models::{
		DeregisterNfInstancePathParams,
		NfProfile1,
		RegisterNfInstanceHeaderParams,
		RegisterNfInstancePathParams,
		SearchNfInstancesHeaderParams,
		SearchNfInstancesQueryParams,
		SearchResult,
	},
};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::trace;
use uuid::Uuid;
use crate::{prepare_request, GenericClientError};

/// `TraitSatisfier` is an empty enum that exists solely to satisfy trait
/// bounds.
///
/// This enum serves as a "phantom" type, implementing traits without being used
/// directly in any operations. It provides a means to satisfy trait constraints
/// for types or generics that require `Serialize`, `Deserialize`, or `Debug`
/// implementations without having any runtime functionality.
///
/// # Example
/// ```
/// use my_crate::TraitSatisfier;
///
/// // This can be used in generic contexts where trait bounds need to be fulfilled
/// // without creating an actual instance.
/// ```
///
/// Since `TraitSatisfier` has no variants, it cannot be
/// instantiated.
#[derive(Serialize, Deserialize, Debug)]
pub enum TraitSatisfier {}

pub struct NrfClient {
	client: Client,
}

impl NrfClient {
	pub fn new(client: Client) -> Self {
		Self { client }
	}

	pub async fn search_nf_instance(
		&self,
		url: Url,
		query: SearchNfInstancesQueryParams,
		header: SearchNfInstancesHeaderParams,
	) -> Result<SearchResult, NrfDiscoveryError> {
		let nrf_service_properties =
			NrfService::NFDiscovery(NrfNFDiscoveryOperation::SearchNFInstances);
		let method = nrf_service_properties.get_http_method();
		let path = nrf_service_properties.get_path();
		let request = prepare_request(
			url,
			&path,
			method,
			Some(&header),
			Some(&query),
			Option::<&TraitSatisfier>::None,
		)?;
		let response = self
			.client
			.execute(request)
			.await
			.map_err(GenericClientError::from)?;
		let (status_code, response) =
			<SearchNfInstancesResponse as DeserResponse>::deserialize(response)
				.await
				.map_err(GenericClientError::from)?;
		match (status_code.as_u16(), response) {
			(_, SearchNfInstancesResponse::Status200 { body, .. }) => Ok(body),
			(status, SearchNfInstancesResponse::Status400(problem))
			| (status, SearchNfInstancesResponse::Status403(problem))
			| (status, SearchNfInstancesResponse::Status404(problem))
			| (status, SearchNfInstancesResponse::Status411(problem))
			| (status, SearchNfInstancesResponse::Status415(problem))
			| (status, SearchNfInstancesResponse::Status429(problem))
			| (status, SearchNfInstancesResponse::Status500(problem))
			| (status, SearchNfInstancesResponse::Status501(problem))
			| (status, SearchNfInstancesResponse::Status503(problem)) => Err(
				GenericClientError::InvalidResponse(status, Some(problem), Backtrace::capture()),
			)?,
			(status, _) => Err(GenericClientError::InvalidResponse(
				status,
				None,
				Backtrace::capture(),
			))?,
		}
	}

	pub async fn register_nf_instance(
		&self,
		url: Url,
		nf_instance_id: Uuid,
		header: &RegisterNfInstanceHeaderParams,
		body: &NfProfile1,
	) -> Result<(NfProfile1, Option<NfInstanceId>), NrfManagementError> {
		let nrf_service_properties =
			NrfService::NFManagement(NrfNFManagementOperation::RegisterNFInstance);
		let method = nrf_service_properties.get_http_method();
		let path = formatx!(&nrf_service_properties.get_path(), nf_instance_id)
			.map_err(GenericClientError::from)?;
		trace!("path: {path:?}");
		let request = prepare_request(
			url,
			&path,
			method,
			Some(header),
			Option::<&TraitSatisfier>::None,
			Some(body),
		)?;
		let response = self
			.client
			.execute(request)
			.await
			.map_err(GenericClientError::from)?;
		let (status_code, response) =
			<RegisterNfInstanceResponse as DeserResponse>::deserialize(response)
				.await
				.map_err(GenericClientError::from)?;
		match (status_code.as_u16(), response) {
			(_, RegisterNfInstanceResponse::Status200 { body, .. }) => Ok((body, None)),
			(_, RegisterNfInstanceResponse::Status201 { body, location, .. }) => {
				if let Some(index) = location.rfind('/') {
					trace!("location string: {}", &location[index..]);
					match NfInstanceId::from_str(&location[index + 1..]) {
						Ok(instance) => Ok((body, Some(instance))),
						Err(e) => {
							return Err(NrfManagementError::InvalidLocationSent(
								e,
								location,
								Backtrace::capture(),
							));
						}
					}
				} else {
					Err(NrfManagementError::LocationParseError(
						location,
						Backtrace::capture(),
					))
				}
			}
			(status, RegisterNfInstanceResponse::Status400(problem))
			| (status, RegisterNfInstanceResponse::Status401(problem))
			| (status, RegisterNfInstanceResponse::Status403(problem))
			| (status, RegisterNfInstanceResponse::Status404(problem))
			| (status, RegisterNfInstanceResponse::Status411(problem))
			| (status, RegisterNfInstanceResponse::Status415(problem))
			| (status, RegisterNfInstanceResponse::Status429(problem))
			| (status, RegisterNfInstanceResponse::Status500(problem))
			| (status, RegisterNfInstanceResponse::Status501(problem))
			| (status, RegisterNfInstanceResponse::Status503(problem)) => Err(
				GenericClientError::InvalidResponse(status, Some(problem), Backtrace::capture()),
			)?,
			(status, _) => Err(GenericClientError::InvalidResponse(
				status,
				None,
				Backtrace::capture(),
			))?,
		}
	}

	pub async fn deregister_nf_instance(
		&self,
		url: Url,
		nf_instance_id: Uuid,
	) -> Result<(), NrfManagementError> {
		let nrf_service_properties =
			NrfService::NFManagement(NrfNFManagementOperation::DeregisterNFInstance);
		let method = nrf_service_properties.get_http_method();
		let path = formatx!(&nrf_service_properties.get_path(), nf_instance_id)
			.map_err(GenericClientError::from)?;
		let request = prepare_request(
			url,
			&path,
			method,
			Option::<&TraitSatisfier>::None,
			Option::<&TraitSatisfier>::None,
			Option::<&TraitSatisfier>::None,
		)?;
		let response = self
			.client
			.execute(request)
			.await
			.map_err(GenericClientError::from)?;
		let (status_code, response) =
			<DeregisterNfInstanceResponse as DeserResponse>::deserialize(response)
				.await
				.map_err(GenericClientError::from)?;
		match (status_code.as_u16(), response) {
			(_, DeregisterNfInstanceResponse::Status204) => Ok(()),
			(status, DeregisterNfInstanceResponse::Status400(problem))
			| (status, DeregisterNfInstanceResponse::Status401(problem))
			| (status, DeregisterNfInstanceResponse::Status403(problem))
			| (status, DeregisterNfInstanceResponse::Status404(problem))
			| (status, DeregisterNfInstanceResponse::Status411(problem))
			| (status, DeregisterNfInstanceResponse::Status429(problem))
			| (status, DeregisterNfInstanceResponse::Status500(problem))
			| (status, DeregisterNfInstanceResponse::Status501(problem))
			| (status, DeregisterNfInstanceResponse::Status503(problem)) => Err(
				GenericClientError::InvalidResponse(status, Some(problem), Backtrace::capture()),
			)?,
			(status, _) => Err(GenericClientError::InvalidResponse(
				status,
				None,
				Backtrace::capture(),
			))?,
		}
	}

	pub async fn authenticaion_request(
		&self,
		source_instance_id: NfInstanceId,
		source_nf_type: NfType,
		target_nf_type: NfType,
		target_service_name: ServiceName,
	) -> Result<AccessTokenRsp, NrfAuthorizationError> {
		let mut token_req = AccessTokenReq::default();
		token_req.target_nf_type = Some(target_nf_type);
		token_req.nf_type = Some(source_nf_type);
		token_req.nf_instance_id = source_instance_id;
		token_req.scope = AccessTokenReqScope::from_str(&target_service_name.to_string())?;
		let nrf_service_properties =
			NrfService::AccessToken(NrfAccessTokenOperation::AccessTokenRequest);
		let method = nrf_service_properties.get_http_method();
		let path = nrf_service_properties.get_path();

		let request = prepare_request(
			self.init_config.url.clone(),
			&path,
			method,
			Option::<&TraitSatisfier>::None,
			Option::<&TraitSatisfier>::None,
			Some(&token_req),
			ContentType::APP_FORM,
		)?;
		let response = self
			.client
			.execute(request)
			.await
			.map_err(GenericClientError::from)?;

		let (status_code, response) =
			<AccessTokenRequestResponse as DeserResponse>::deserialize(response)
				.await
				.map_err(GenericClientError::from)?;

		match (status_code.as_u16(), response) {
			(_, AccessTokenRequestResponse::Status200 { body, .. }) => Ok(body),
			(_, AccessTokenRequestResponse::Status400 { body, .. }) => {
				Err(NrfAuthorizationError::AccessTokenError(body))
			}
			(status, AccessTokenRequestResponse::Status401(problem))
			| (status, AccessTokenRequestResponse::Status403(problem))
			| (status, AccessTokenRequestResponse::Status404(problem))
			| (status, AccessTokenRequestResponse::Status411(problem))
			| (status, AccessTokenRequestResponse::Status429(problem))
			| (status, AccessTokenRequestResponse::Status500(problem))
			| (status, AccessTokenRequestResponse::Status501(problem))
			| (status, AccessTokenRequestResponse::Status503(problem)) => Err(
				GenericClientError::InvalidResponse(status, Some(problem), Backtrace::capture()),
			)?,
			(status, _) => Err(GenericClientError::InvalidResponse(
				status,
				None,
				Backtrace::capture(),
			))?,
		}
	}
}

impl NrfClient {
	pub async fn get_token<const T: NfType>(
		&self,
		target_service_name: ServiceName,
	) -> Result<TokenEntry<AccessTokenRsp>, NrfAuthorizationError> {
		let token_entry = self.nf_token_store.get(&target_service_name).await?;
		match token_entry {
			Some(entry) => Ok(entry),
			None => {
				let resp = self
					.nf_token_store
					.set(
						target_service_name.clone(),
						self.authenticaion_request(
							self.nf_config.load().nf_instance_id,
							self.init_config.source,
							T,
							target_service_name,
						),
					)
					.await?;
				Ok(resp)
			}
		}
	}

	pub async fn set_auth_token<const T: NfType>(
		&self,
		req: &mut Request,
		service_name: ServiceName,
	) -> Result<(), NrfAuthorizationError> {
		if self.nf_config.load().oauth_enabled {
			let token_entry = self.get_token::<T>(service_name).await?;
			set_auth_token(req, token_entry)?;
		}
		Ok(())
	}
}

#[derive(Debug, Error)]
pub enum NrfError {
	#[error("Error Encountered While Discovery")]
	DiscoveryError(
		#[from]
		#[backtrace]
		NrfDiscoveryError,
	),
}

#[derive(Debug, Error)]
pub enum NrfDiscoveryError {
	#[error(transparent)]
	GenericClientError(
		#[from]
		#[backtrace]
		GenericClientError,
	),
}

#[derive(Debug, Error)]
pub enum NrfManagementError {
	#[error("Invalid Location string: {1}")]
	InvalidLocationSent(#[source] uuid::Error, String, #[backtrace] Backtrace),

	#[error("LocationParseError: Unable to get uuid from location {0}")]
	LocationParseError(String, #[backtrace] Backtrace),

	#[error(transparent)]
	GenericClientError(
		#[from]
		#[backtrace]
		GenericClientError,
	),

	#[error("NrfAuthorizationError")]
	NrfAuthorizationError(#[from] NrfAuthorizationError),
}

#[derive(Error, Debug)]
pub enum NrfAuthorizationError {
	#[error("InvalidRequestScope: Converting Services to token scope error")]
	InvalidRequestScope(#[from] ConversionError),

	#[error(transparent)]
	GenericClientError(
		#[from]
		#[backtrace]
		GenericClientError,
	),

	#[error("AccessTokenError: Nrf Token Api Error {0:?}")]
	AccessTokenError(AccessTokenErr),

	#[error("TokenStoreError: Oauth Token Store Error {0:?}")]
	TokenStoreError(#[from] StoreError),

	#[error("TokenParsingError: Invalid Auth Token ")]
	TokenParsingError(#[from] header::InvalidHeaderValue),
}

pub(crate) fn set_auth_token(
	req: &mut Request,
	token_entry: TokenEntry<AccessTokenRsp>,
) -> Result<(), header::InvalidHeaderValue> {
	let token: &str = &token_entry.get().access_token;
	let token_type = token_entry.get().token_type;
	let token: String = match token_type {
		AccessTokenRspTokenType::Bearer => {
			let mut string = "Bearer ".to_owned();
			string.push_str(token);
			string
		}
	};
	let headers_mut = req.headers_mut();
	headers_mut.insert(AUTHORIZATION, token.try_into()?);
	Ok(())
}
