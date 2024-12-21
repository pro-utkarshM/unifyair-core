mod apis;
pub(crate) mod handler;
use std::{net::SocketAddr, sync::Arc};

use apis::SbiServer;
use axum::Router;
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::InfiniSyncError;

pub async fn start_server(addr: SocketAddr) -> Result<(), InfiniSyncError> {
	// Init Axum router
	let mut app: Router = openapi_smf::server::new(Arc::new(SbiServer {}));
	app = app.layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

	// Run the server with graceful shutdown
	let listener = TcpListener::bind(addr).await?;
	axum::serve(listener, app)
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap();

	Ok(())
}

async fn shutdown_signal() {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");

	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}
}
