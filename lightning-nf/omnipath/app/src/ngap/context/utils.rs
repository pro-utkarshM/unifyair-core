// use asn1_codecs::PerCodecError;
use asn1_per::{CodecDataAllocator, PerCodec, PerCodecError, SerDes, ThreeGppAsn1PerError};
use ngap_models::{
	convert_diagnostics_to_ie, Cause, CauseProtocol, CriticalityDiagnostics, ErrorIndication, InitiatingMessage, NgapPdu, ToNgapPdu
};
use tracing::error;

pub fn codec_to_bytes<T: PerCodec>(data: &T) -> Result<Vec<u8>, ThreeGppAsn1PerError> {
	let mut d = T::Allocator::new_codec_data();
	data.encode(&mut d)?;
	Ok(d.into_bytes())
}

/// Attempts to decode an NGAP PDU payload and returns either the successfully
/// decoded PDU or an error indication PDU that should be sent back to the
/// sender. Logs any decoding errors internally.
///
/// # Arguments
/// * `request` - The raw NGAP PDU bytes to decode
///
/// # Returns
/// * `Ok(NgapPdu)` - Successfully decoded PDU
/// * `Err(NgapPdu)` - Error indication PDU with diagnostics information to be
///   sent back
pub fn decode_ngap_pdu(request: &[u8]) -> Result<NgapPdu, (NgapPdu, PerCodecError)> {
	match NgapPdu::from_bytes(request) {
		Ok(pdu) => Ok(pdu),
		Err(e) => {
			error!(
				"Error decoding NGAP PDU payload: {:?} error: {:#?}",
				&request, e
			);

			let (err, codec_error) = build_criticality_diagnostics(&request, e);
			Err((err.to_pdu(), codec_error))
		}
	}
}

pub fn build_criticality_diagnostics(
	request: &[u8],
	error: ThreeGppAsn1PerError,
) -> (ErrorIndication, PerCodecError) {
	let ThreeGppAsn1PerError {
		diagnostics,
		codec_error,
	} = error;
	let i_es_criticality_diagnostics = convert_diagnostics_to_ie(diagnostics);

	let (triggering_message, procedure_code, procedure_criticality) =
		NgapPdu::get_message_info(&request);

	let criticality_diagnostics = CriticalityDiagnostics {
		procedure_code,
		triggering_message,
		procedure_criticality,
		i_es_criticality_diagnostics,
	};

	let err = ErrorIndication {
		cause: Some(Cause::Protocol(
			CauseProtocol::AbstractSyntaxErrorFalselyConstructedMessage,
		)),
		criticality_diagnostics: Some(criticality_diagnostics),
		..Default::default()
	};

	(err, codec_error)
}
