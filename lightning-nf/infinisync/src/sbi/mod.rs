mod apis;
use apis::SbiServer;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;

pub async fn start_server(addr: &str) {
	// initialize tracing
	tracing_subscriber::fmt::init();

	// Init Axum router
	let app = openapi_smf::server::new(Arc::new(SbiServer {}));

	// Run the server with graceful shutdown
	let listener = TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, app)
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap();
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
