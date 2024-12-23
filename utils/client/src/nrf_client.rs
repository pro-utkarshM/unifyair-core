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
}
