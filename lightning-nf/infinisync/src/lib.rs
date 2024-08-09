use crate::config::InfiniSyncConfig;
use crate::sbi::start_server;
pub async fn run(config: &InfiniSyncConfig) {
	start_server(&config.configuration.sbi.binding_ip_v4).await;
}

pub mod config;
mod net_gateways;
pub mod sbi;
