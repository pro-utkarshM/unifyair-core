use bytes::Bytes;
use infinisync::pfcp::association_setup_request_handler;
use pfcp::{
	PfcpHeader,
	TlvDecode,
	message::{
		PfcpAssociationSetupRequest,
		PfcpAssociationSetupResponse,
		PfcpSessionEstablishmentRequest,
		PfcpSessionEstablishmentResponse,
		PfcpSessionModificationRequest,
		PfcpSessionModificationResponse,
	},
	mock::*,
};

pub fn decode<T: TlvDecode>(msg: Bytes) -> (PfcpHeader, T) {
	let header = PfcpHeader::decode(msg.as_ref()).unwrap();
	let msg = msg.slice(header.total_header_length()..);
	let body = TlvDecode::decode(msg.as_ref()).unwrap();
	(header, body)
}

#[test]
fn test_association_setup_request() {
	let request = get_association_setup_request();
	let (header, body) = decode::<PfcpAssociationSetupRequest>(request);
	let resp = association_setup_request_handler(header, body);
}

#[test]
fn test_session_establishment_request() {
	let request = get_session_establishment_request();
	let (encoded_bytes, body) = encode_decode_bytes::<PfcpSessionEstablishmentRequest>(request);
	assert_eq!(encoded_bytes.as_ref(), body.as_ref());
}

#[test]
fn test_session_modification_request() {
	let request = get_session_modification_request();
	let (encoded_bytes, body) = encode_decode_bytes::<PfcpSessionModificationRequest>(request);
	assert_eq!(encoded_bytes.as_ref(), body.as_ref());
}
