pub mod config;
pub(crate) mod context;
pub mod models;
pub mod ngap;

use std::{rc::Rc, sync::Arc};

use client::nrf_client::NrfClient;
use config::OmniPathConfig;
use nf_base::NfInstance;
use reqwest::{Client, Url};
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use crate::config::SerdeValidated;
use crate::context::app_context::AppContext;

#[derive(Error, Debug)]
pub enum OmniPathError {
	#[error(transparent)]
	ConfigError(#[from] OmniPathConfigError),
}

#[derive(Error, Debug)]
pub enum OmniPathConfigError {
	#[error("The Nrf Uri is Invalid: {0} {1}")]
	InvalidNrfUriError(#[source] url::ParseError, String),

	#[error("Invalid Configuration: {0}")]
	InvalidConfig(#[from] serde_valid::validation::Errors),
}

pub struct OmniPathApp {
	nrf_url: Url,
	nrf_client: Arc<NrfClient>,
	config: Rc<SerdeValidated<OmniPathConfig>>,
	shutdown: CancellationToken,
	app_context: AppContext,
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
		let nrf_client = Arc::new(NrfClient::new(Client::new()));
		let valid_config = SerdeValidated::new(config).map_err(OmniPathConfigError::InvalidConfig)?;
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
		let nf_profile = self.app_context.build_nf_profile();
		Ok(())
	}

	async fn deregister_nf(&self) -> Result<(), Self::Error> {
		Ok(())
	}
}
