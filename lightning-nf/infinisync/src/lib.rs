use std::net::SocketAddrV4;

use crate::config::InfiniSyncConfig;
use crate::sbi::start_server;

pub async fn run(config: &InfiniSyncConfig) {
	let ip_addr = config.configuration.sbi.binding_ip_v4.parse().unwrap();
	let port = config.configuration.sbi.port;
	let server_addr = SocketAddrV4::new(ip_addr, port);
	start_server(&server_addr.to_string()).await;
}

pub mod config;
mod net_gateways;
pub mod sbi;
pub mod pfcp;
