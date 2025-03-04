use std::{
	collections::{HashMap, HashSet},
	hash::BuildHasherDefault,
	net::{IpAddr, SocketAddr},
	sync::Arc,
};

use rustc_hash::FxBuildHasher;
use socket2::Domain;
use solana_nohash_hasher::NoHashHasher;
use tokio::sync::RwLock;
use tokio_sctp::{SctpListener, SctpSocket, SctpStream};
use tokio_util::sync::CancellationToken;
use tracing::log::{error, info, trace};

use crate::{NetworkError, TnlaAssociation};

const MAX_TNLA_ASSOCIATIONS: usize = 32;
const DEFAULT_NGAP_PORT: u16 = 38412;

type UnitHasher<T> = BuildHasherDefault<NoHashHasher<T>>;

pub struct Network {
	listener: SctpListener,
	associations: RwLock<Associations>,
}

pub struct Associations {
	associations: HashMap<usize, Arc<TnlaAssociation>, UnitHasher<usize>>,
	associations_set: HashSet<(SocketAddr, SocketAddr), FxBuildHasher>,
}

impl Associations {
	pub fn new() -> Self {
		Self {
			associations: HashMap::with_hasher(UnitHasher::default()),
			associations_set: HashSet::with_hasher(FxBuildHasher::default()),
		}
	}

	/// Adds a new TNLA association to the network.
	///
	/// This function takes an `SctpStream`, creates a new `TnlaAssociation`,
	/// and adds it to the appropriate data structures within the `Associations`
	/// struct. It checks for duplicate associations based on local and remote
	/// addresses.
	///
	/// # Arguments
	///
	/// * `stream`: The `SctpStream` representing the SCTP connection.
	///
	/// # Returns
	///
	/// * `Result<TnlaAssociation, NetworkError>`: The new `TnlaAssociation` if
	///   successful, or a `NetworkError` if an error occurred.
	pub fn add_tnla_association(
		&mut self,
		stream: SctpStream,
	) -> Result<Arc<TnlaAssociation>, NetworkError> {
		let association = Arc::new(TnlaAssociation::new(stream)?);

		// Check if the association already exists using the associations_set.
		if !self
			.associations_set
			.insert((association.local_addr, association.remote_addr))
		{
			// Return an error if a duplicate association is detected.
			return Err(NetworkError::AssociationAlreadyExists(
				association.local_addr,
				association.remote_addr,
			));
		}

		// Insert the association into the `associations` HashMap, keyed by the
		// association ID.
		self.associations
			.insert(association.id, association.clone());

		Ok(association)
	}

	/// Removes a TNLA association from the network based on its ID.
	///
	/// # Arguments
	///
	/// * `id`: The ID of the association to remove.
	///
	/// # Returns
	///
	/// * `Option<Arc<TnlaAssociation>>`: The removed `TnlaAssociation` if
	///   found, or `None` if no association with the given ID exists.
	pub fn remove_tnla_association(
		&mut self,
		id: usize,
	) -> Option<Arc<TnlaAssociation>> {
		// Remove the association from the `associations` HashMap.
		if let Some(association) = self.associations.remove(&id) {
			// Remove the association from the `associations_set`.
			self.associations_set
				.remove(&(association.local_addr, association.remote_addr));

			Some(association)
		} else {
			None
		}
	}
}

impl Network {
	pub fn new(
		ip_addr: IpAddr,
		port: u16,
		sctp_config: &config::SCTP,
	) -> Result<Self, NetworkError> {
		let domain = match ip_addr {
			IpAddr::V4(_) => Domain::IPV4,
			IpAddr::V6(_) => Domain::IPV6,
		};
		let init_msg = sctp_config.into();
		let socket = SctpSocket::new(domain).map_err(NetworkError::SocketCreationError)?;

		// TODO: Setting initialization parameters is effective only on an unconnected
		// socket (for one-to-many style sockets, only future associations are
		// affected by the change).
		socket
			.set_sctp_initmsg(&init_msg)
			.map_err(NetworkError::SctpSocketConfigurationError)?;

		socket.set_nodelay(true).map_err(NetworkError::SctpSocketConfigurationError)?;

		let addr = SocketAddr::new(ip_addr, port);
		let listener =
			SctpListener::bind_from(socket, addr).map_err(NetworkError::ListenerBindingError)?;

		Ok(Self {
			listener,
			associations: RwLock::new(Associations::new()),
		})
	}

	/// Accepts a new connection and creates a TNLA association. Starts
	/// listening for messages on the new association.
	///
	/// # Arguments
	///
	/// * `cancel`: A `CancellationToken` that can be used to cancel the
	///   operation.
	///
	/// # Returns
	async fn accept_and_create_tnla(
		&self,
		cancel: CancellationToken,
	) -> Result<(), NetworkError> {
		let (stream, addr) = self
			.listener
			.accept()
			.await
			.map_err(NetworkError::ConnectionAcceptError)?;
		info!("Accepted connection from: {:?}", addr);

		let mut associations = self.associations.write().await; // Acquire write lock
		let tnla = associations.add_tnla_association(stream)?; // Insert using the new insert method

		let tnla_cancel = cancel.child_token();
		tokio::spawn(async move {
			// if let Err(e) = tnla.run(tnla_cancel).await {
			// 	let net_err = NetworkError::TnlaError(format!("{:?}", e));
			// 	println!("TNLA association error: {:?}", net_err);
			// }
		});

		Ok(())
	}

	/// Starts the main loop of the network, accepting connections,
	/// creating TNLA associations, and spawning futures.  Runs until
	/// cancelled or an error occurs.
	pub async fn run(
		&self,
		cancel: CancellationToken,
	) -> Result<(), NetworkError> {
		loop {
			tokio::select! {
			biased;

			res = self.accept_and_create_tnla(cancel.clone()) => {
				if let Err(e) = res {
					error!("Error accepting connection: {:?}", e);
				}
			}

			_ = cancel.cancelled() => {
				info!("Cancellation requested, exiting loop");
				break;
			}}
		}

		self.graceful_shutdown().await?;
		Ok(())
	}

	// TODO: Implement graceful shutdown for the network
	pub async fn graceful_shutdown(&self) -> Result<(), NetworkError> {
		Ok(())
	}
}
