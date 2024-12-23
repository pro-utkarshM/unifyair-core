#![feature(error_generic_member_access)]

pub mod config;
pub(crate) mod context;
pub mod models;
pub mod ngap;

use std::{rc::Rc, sync::Arc};

use client::nrf_client::{NrfClient, NrfManagementError};
use config::OmniPathConfig;
use nf_base::NfInstance;
use openapi_nrf::models::RegisterNfInstanceHeaderParams;
use reqwest::{Client, Url};
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::{
	config::SerdeValidated,
	context::app_context::{AppContext, Configuration},
	models::sbi::ModelBuildError,
};

#[derive(Error, Debug)]
pub enum OmniPathError {
	#[error("ConfigError: Configuration Error")]
	ConfigError(
		#[from]
		#[backtrace]
		OmniPathConfigError,
	),

	#[error("NrfError: Nrf Error")]
	NrfError(
		#[from]
		#[backtrace]
		NrfError,
	),
}

#[derive(Error, Debug)]
pub enum OmniPathConfigError {
	#[error("InvalidNrfUriError: The Nrf Uri is Invalid: {0} {1}")]
	InvalidNrfUriError(#[source] url::ParseError, String),

	#[error("InvalidConfig: Invalid Configuration")]
	InvalidConfig(#[from] serde_valid::validation::Errors),
}

#[derive(Error, Debug)]
pub enum NrfError {
	#[error("ModelBuildError: Unable to build the model")]
	ModelBuildError(#[from] ModelBuildError),

	#[error("NrfManagementError: Unable to register")]
	NrfManagementError(
		#[from]
		#[backtrace]
		NrfManagementError,
	),
}

pub struct OmniPathApp {
	nrf_url: Url,
	nrf_client: Arc<NrfClient>,
	config: Rc<SerdeValidated<OmniPathConfig>>,
	shutdown: CancellationToken,
	app_context: AppContext,
}

pub fn create_nrf_client() -> Client {
	Client::builder()
		.connection_verbose(true)
		// .https_only(true)
		.build()
		.unwrap()
}

impl NfInstance for OmniPathApp {
	type Config = OmniPathConfig;
	type Error = OmniPathError;

	fn initialize(
		config: Self::Config,
		shutdown: CancellationToken,
	) -> Result<Self, Self::Error> {
		let nrf_uri = &config.configuration.nrf_uri.to_string();
		let nrf_url = Url::parse(nrf_uri)
			.map_err(|e| OmniPathConfigError::InvalidNrfUriError(e, nrf_uri.to_owned()))?;
		let nrf_client = Arc::new(NrfClient::new(create_nrf_client()));
		let valid_config =
			SerdeValidated::new(config).map_err(OmniPathConfigError::InvalidConfig)?;
		let app_context = AppContext::initialize(&valid_config);
		Ok(Self {
			nrf_url,
			nrf_client,
			config: Rc::new(valid_config),
			shutdown,
			app_context,
		})
	}

	async fn start(&self) -> Result<(), Self::Error> {
		Ok(())
	}

	async fn register_nf(&self) -> Result<(), Self::Error> {
		let nf_profile = self
			.app_context
			.build_nf_profile()
			.map_err(NrfError::from)?;
		let nf_instance_id = self.app_context.get_nf_id().0;
		let (nf_profile, instance_id) = self
			.nrf_client
			.register_nf_instance(
				self.nrf_url.clone(),
				nf_instance_id,
				&RegisterNfInstanceHeaderParams::default(),
				&nf_profile,
			)
			.await
			.map_err(NrfError::from)?;
		info!("Nrf Nf Id: {:#?}", instance_id);
		info!("Nrf Profile Response: {:#?}", nf_profile);
		if let Some(nf_id) = instance_id {
			if (nf_id == self.app_context.get_nf_id()) {
				let update_config_fn = move |config: &mut Configuration| {
					config.nf_id = nf_id;
				};
				self.app_context.commit_config(update_config_fn);
			}
		}
		Ok(())
	}

	async fn deregister_nf(&self) -> Result<(), Self::Error> {
		let nf_instance_id = self.app_context.get_nf_id().0;
		self.nrf_client
			.deregister_nf_instance(self.nrf_url.clone(), nf_instance_id)
			.await
			.map_err(NrfError::from)?;
		Ok(())
	}
}
