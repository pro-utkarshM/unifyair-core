use ascii::AsciiString;
use ngap_models::{
	AmfPointer as NgapAmfPointer,
	AmfSetId as NgapAmfSetId,
	FiveGSTmsi as NgapFiveGSTmsi,
	FiveGTmsi as NgapFiveGTmsi,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FiveGSTmsi(AsciiString);

impl From<NgapFiveGSTmsi> for FiveGSTmsi {
	fn from(value: NgapFiveGSTmsi) -> Self {
		let NgapFiveGSTmsi {
			amf_set_id: NgapAmfSetId(set_id),
			amf_pointer: NgapAmfPointer(pointer),
			five_g_tmsi: NgapFiveGTmsi(tmsi),
		} = value;
		let amf_set_id = set_id.into_inner().into_vec();
		let amf_pointer = pointer.into_inner().into_vec();
		let tmsi: [u8; 6] = [
			amf_set_id[0],
			amf_set_id[1] | amf_pointer[0],
			tmsi[0],
			tmsi[1],
			tmsi[2],
			tmsi[3],
		];
		let tmsi_str = faster_hex::hex_string(&tmsi);
		let tmsi_str_bytes = tmsi_str.into_bytes();

		// Safety: The hex string check is done here, ensuring that the conversion to
		// AsciiString is valid.
		Self(unsafe { AsciiString::from_ascii_unchecked(tmsi_str_bytes) })
	}
}


#[cfg(test)]
mod tests {
	use bitvec::prelude::*;
	use ngap_models::{AmfPointer, AmfSetId, FiveGSTmsi as NgapFiveGSTmsi, FiveGTmsi};

	use super::*;

	#[test]
	fn test_ngap_five_g_tmsi_conversion() {
		// Create a mock NgapFiveGSTmsi
		let amf_set_id =
			AmfSetId(BitVec::from_bitslice(bits![u8, Msb0; 0, 1, 0, 1, 1, 0, 1, 0, 1, 0]).into());
		let amf_pointer =
			AmfPointer(BitVec::from_bitslice(bits![u8, Msb0; 1, 0, 1, 0, 1, 0]).into());
		let five_g_tmsi = FiveGTmsi([0x12, 0x34, 0x56, 0x78]);
		// Merged amf_set_id and amf_pointer value is 0b0101_1010_1010_1010

		let ngap_five_g_tmsi = NgapFiveGSTmsi {
			amf_set_id,
			amf_pointer,
			five_g_tmsi,
		};

		// Perform the conversion
		let converted: FiveGSTmsi = ngap_five_g_tmsi.into();

		// Expected TMSI string
		let expected_tmsi_str = "5aaa12345678"; // This should match the expected hex string

		// Assert the conversion result
		assert_eq!(converted.0.as_str(), expected_tmsi_str);
	}
}