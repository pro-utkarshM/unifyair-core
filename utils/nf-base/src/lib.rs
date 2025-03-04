use std::{error, fmt};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio_util::sync::CancellationToken;

pub trait NfInstance: Sized {
	type Config: DeserializeOwned + fmt::Debug + NfConfig;
	type Error: error::Error + Send + Sync + 'static;
	fn initialize(
		cfg: Self::Config,
		shutdown: CancellationToken,
	) -> Result<Self, Self::Error>;
	async fn start(&self) -> Result<(), Self::Error>;
	async fn register_nf(&self) -> Result<(), Self::Error>;
	async fn deregister_nf(&self) -> Result<(), Self::Error>;
}

pub trait NfConfig {
	fn get_log_config(&self) -> &LoggingConfig;
	fn get_runtime_config(&self) -> &RuntimeConfig;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoggingConfig {
	pub enable: bool,
	pub level: String,
	pub report_caller: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeConfig {
	#[serde(rename = "type")]
	pub rt_type: RuntimeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RuntimeType {
	#[serde(rename = "single")]
	Single,
	#[serde(rename = "multi")]
	Multi,
}
