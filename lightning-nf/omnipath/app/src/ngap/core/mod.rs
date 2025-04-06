use std::sync::Arc;

use asn1_per::{PerCodec, SerDes, ThreeGppAsn1PerError};
use bytes::Bytes;

use ngap_models::NgapPdu;
use thiserror::Error;
use tracing::error;

use super::context::{GnbContext, decode_ngap_pdu};
use crate::ngap::context::NgapContext;
pub(crate) mod ng_setup;
mod utils;

impl NgapContext {
	pub fn ngap_route(
		&self,
		gnb_context: Arc<GnbContext>,
		request: NgapPdu,
	) -> Option<NgapPdu> {
		match request {
			_ => todo!(),
		};
	}
}

#[derive(Debug, Error)]
pub enum NgapError {
	#[error("Invalid request")]
	InvalidRequest,

	#[error("Cause Error")]
	CauseError,

	#[error("Encoding Error")]
	EncodingError,
}
