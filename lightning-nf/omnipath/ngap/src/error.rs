use std::{io::Error as IoError, net::SocketAddr};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
	#[error("SocketCreationError: Failed to create SCTP socket")]
	SocketCreationError(#[source] IoError),
	#[error("ListenerBindingError: Failed to bind SCTP listener")]
	ListenerBindingError(#[source] IoError),
	#[error("SctpSocketConfigurationError: Failed to set SCTP socket parameters")]
	SctpSocketConfigurationError(#[source] IoError),
	#[error("LocalAddressError: Failed to get local address for SCTP association")]
	LocalAddressError(#[source] IoError),
	#[error("RemoteAddressError: Failed to get remote address for SCTP association")]
	RemoteAddressError(#[source] IoError),
	#[error("ConnectionAcceptError: Failed to accept sctp connection")]
	ConnectionAcceptError(#[source] IoError),
	#[error("AssociationAlreadyExists: TNLA association between {0} and {1} already exists")]
	AssociationAlreadyExists(SocketAddr, SocketAddr),
	#[error("ReadError: Failed to read from SCTP stream")]
	ReadError(#[source] IoError),
	#[error("WriteError: Failed to write to SCTP stream")]
	WriteError(#[source] IoError),
}
