use ngap_models::PlmnIdentity as NgapPlmnIdentity;
use oasbi::common::{
	Mcc as SbiMcc,
	Mnc as SbiMnc,
	PlmnId as SbiPlmnId,
	PlmnIdNid as SbiPlmnIdNid,
	error::ConversionError,
};

use super::Element;

/// Converts MCC (Mobile Country Code) and MNC (Mobile Network Code) to PLMN
/// (Public Land Mobile Network) Identity and vice versa according to 3GPP
/// encoding rules specified in 38.413 9.3.3.5.
///
/// The encoding follows these rules:
/// - Each digit (0-9) is encoded in 4 bits from 0000 to 1001
/// - 1111 is used as a filler digit for 2-digit MNC
/// - Two digits are packed per octet with the following format:
///   - bits 8-5: second digit
///   - bits 4-1: first digit
///
/// The 3-byte PLMN Identity is encoded as follows:
/// ```text
/// Octet 1: MCC digit 2 (bits 8-5) | MCC digit 1 (bits 4-1)
/// Octet 2: MNC digit 1 or 1111 for 2-digit MNC (bits 8-5) | MCC digit 3 (bits 4-1)
/// Octet 3: MNC digit 3/2 (bits 8-5) | MNC digit 2/1 (bits 4-1)
/// ```
///
/// # Arguments
/// * `mcc` - Mobile Country Code (3 digits)
/// * `mnc` - Mobile Network Code (2 or 3 digits)
///
/// # Returns
/// Returns a `NgapPlmnIdentity` containing the encoded 3-byte PLMN identifier
///
/// # Example encoding:
/// For MCC = 234 and MNC = 15 (2 digits):
/// ```text
/// Octet 1: 0011 (3) | 0010 (2) = 0x32
/// Octet 2: 1111 (F) | 0100 (4) = 0xF4
/// Octet 3: 0101 (5) | 0001 (1) = 0x51
/// ```

impl From<Element<&SbiPlmnId>> for Element<NgapPlmnIdentity> {
	fn from(value: Element<&SbiPlmnId>) -> Self {
		Element(convert_mcc_mnc_to_plmn_id(&value.0.mcc, &value.0.mnc))
	}
}

impl From<Element<&SbiPlmnIdNid>> for Element<NgapPlmnIdentity> {
	fn from(value: Element<&SbiPlmnIdNid>) -> Self {
		Element(convert_mcc_mnc_to_plmn_id(&value.0.mcc, &value.0.mnc))
	}
}

impl TryFrom<Element<&NgapPlmnIdentity>> for Element<SbiPlmnId> {
	type Error = ConversionError;

	fn try_from(value: Element<&NgapPlmnIdentity>) -> Result<Self, Self::Error> {
		convert_plmn_id_to_mcc_mnc(value.0).map(Element)
	}
}

fn convert_mcc_mnc_to_plmn_id(
	mcc: &SbiMcc,
	mnc: &SbiMnc,
) -> NgapPlmnIdentity {
	// Convert MCC and MNC to strings to get individual digits
	let mcc_bytes = mcc.as_bytes();
	let mnc_bytes = mnc.as_bytes();

	// Create a 3-byte array for PLMN Identity
	let mut plmn_bytes = [0u8; 3];

	// First byte: MCC digit 2 (bits 8-5) and MCC digit 1 (bits 4-1)
	plmn_bytes[0] = (mcc_bytes[1] - b'0') << 4 | (mcc_bytes[0] - b'0');

	// Second byte: MNC digit 1 (bits 8-5) and MCC digit 3 (bits 4-1)
	plmn_bytes[1] = if mnc_bytes.len() == 2 {
		// For 2-digit MNC, use filler (1111) for MNC digit 1
		0xF0 | (mcc_bytes[2] - b'0')
	} else {
		// For 3-digit MNC, use MNC digit 1
		(mnc_bytes[0] - b'0') << 4 | (mcc_bytes[2] - b'0')
	};

	// Third byte: MNC digit 3 (bits 8-5) and MNC digit 2 (bits 4-1)
	plmn_bytes[2] = if mnc_bytes.len() == 2 {
		// For 2-digit MNC, use MNC digits 2 and 1
		(mnc_bytes[1] - b'0') << 4 | (mnc_bytes[0] - b'0')
	} else {
		// For 3-digit MNC, use MNC digits 3 and 2
		(mnc_bytes[2] - b'0') << 4 | (mnc_bytes[1] - b'0')
	};

	NgapPlmnIdentity(plmn_bytes)
}

/// Converts a PLMN (Public Land Mobile Network) Identity back to MCC (Mobile
/// Country Code) and MNC (Mobile Network Code) according to 3GPP decoding
/// rules.
fn convert_plmn_id_to_mcc_mnc(plmn_id: &NgapPlmnIdentity) -> Result<SbiPlmnId, ConversionError> {
	let [octet1, octet2, octet3] = plmn_id.0;

	// Extract MCC digits and validate they are 0-9
	let mcc_digit1 = (octet1 & 0x0F) as u8;
	let mcc_digit2 = (octet1 >> 4) as u8;
	let mcc_digit3 = (octet2 & 0x0F) as u8;

	// Extract MNC digits
	let mnc_digit1 = (octet3 >> 4) as u8;
	let mnc_digit2 = (octet3 & 0x0F) as u8;
	let mnc_digit3 = (octet2 >> 4) as u8;

	// Validate MNC digits
	if mnc_digit1 > 9 || mnc_digit2 > 9 || mcc_digit1 > 9 || mcc_digit2 > 9 || mcc_digit3 > 9 {
		return Err(format!("Invalid MCC/MNC digits in PLMN Identity: {:?}", plmn_id).into());
	}
	// Build MCC string
	let mcc_str = format!("{}{}{}", mcc_digit1, mcc_digit2, mcc_digit3);

	// Check if MNC is 2 or 3 digits
	// If mnc_digit3 is >= 0xA (value 10 or greater), it's a filler indicating a
	// 2-digit MNC and is ignored
	let mnc_str = if mnc_digit3 >= 0xA {
		format!("{}{}", mnc_digit2, mnc_digit1)
	} else {
		format!("{}{}{}", mnc_digit3, mnc_digit2, mnc_digit1)
	};

	// SAFETY: We have already validated the digits are 0-9
	let mcc = unsafe { SbiMcc::new_unchecked(mcc_str) };
	let mnc = unsafe { SbiMnc::new_unchecked(mnc_str) };
	Ok(SbiPlmnId { mcc, mnc })
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use oasbi::common::{Mcc as SbiMcc, Mnc as SbiMnc};

	use super::*;

	fn test_mcc_mnc_interconversion(
		mcc: &str,
		mnc: &str,
		plmn_id: [u8; 3],
	) {
		let encoded_mcc = SbiMcc::from_str(mcc).unwrap();
		let encoded_mnc = SbiMnc::from_str(mnc).unwrap();
		let plmn_identity = convert_mcc_mnc_to_plmn_id(&encoded_mcc, &encoded_mnc);
		assert_eq!(plmn_identity.0, plmn_id);

		let SbiPlmnId {
			mcc: decoded_mcc,
			mnc: decoded_mnc,
		} = convert_plmn_id_to_mcc_mnc(&plmn_identity).unwrap();
		assert_eq!(decoded_mcc.to_string(), mcc);
		assert_eq!(decoded_mnc.to_string(), mnc);
	}

	#[test]
	fn test_mcc_mnc_to_plmn_id_interconversion() {
		test_mcc_mnc_interconversion("208", "93", [0x02, 0xF8, 0x39]);
		test_mcc_mnc_interconversion("234", "15", [0x32, 0xF4, 0x51]);
		test_mcc_mnc_interconversion("001", "001", [0x00, 0x01, 0x10]);
	}
}
