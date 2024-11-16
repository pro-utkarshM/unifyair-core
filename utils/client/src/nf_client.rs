use std::sync::Arc;

use oasbi::{common::NfType, nrf::types::NfProfile};
use openapi_nrf::models::{
	SearchNfInstancesHeaderParams,
	SearchNfInstancesQueryParams,
	SearchResult,
};
use reqwest::Url;

use crate::nrf_client::{NrfClient, NrfDiscoveryError};

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

pub struct NFClient<T: NfClientController + ApiBaseUrl, const APP_TYPE: NfType> {
	nrf_client: Arc<NrfClient>,
	controller: T,
	nf_profile: NfProfile,
	req_client: Client,
}

impl<T: NfClientController, const APP_TYPE: NfType> NFClient<T, APP_TYPE> {
	async fn new(
		nrf_client: Arc<NrfClient>,
		controller: T,
	) -> Result<Self, NrfDiscoveryError> {
		let url = controller.base_url();
		let search_params = controller.get_search_params(APP_TYPE);
		let header_params = SearchNfInstancesHeaderParams {
			..Default::default()
		};
		let search_result = nrf_client
			.search_nf_instance(url, search_params, header_params)
			.await?;
		let nf_profile = controller.profile_selection(search_result);
		Ok(NFClient {
			nrf_client,
			controller,
			nf_profile,
		})
	}
}
