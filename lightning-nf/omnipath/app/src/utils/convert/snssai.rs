use ngap_models::{Sd as NgapSd, Snssai as NgapSnssai, Sst as NgapSst};
use oasbi::common::{Snssai as SbiSnssai, SnssaiSd};

use super::Element;

impl From<Element<&SbiSnssai>> for Element<NgapSnssai> {
	fn from(sbi_snssai: Element<&SbiSnssai>) -> Self {
		let ngap_snssai = convert_sbi_snssai_to_ngap_snssai(sbi_snssai.0);
		Element(ngap_snssai)
	}
}

impl From<Element<&NgapSnssai>> for Element<SbiSnssai> {
	fn from(ngap_snssai: Element<&NgapSnssai>) -> Self {
		let sbi_snssai = convert_ngap_snssai_to_sbi_snssai(ngap_snssai.0);
		Element(sbi_snssai)
	}
}

fn convert_sbi_snssai_to_ngap_snssai(sbi: &SbiSnssai) -> NgapSnssai {
	let sst = NgapSst([sbi.sst]);

	let sd = if let Some(sd_str) = &sbi.sd {
		// Convert hex string to [u8; 3]
		let bytes = sd_str.as_bytes();
		let mut dst = [0; 3];
		// SAFETY: The bytes are guaranteed to be valid for NgapSd because:
		// 1. They are constructed from exactly 3 bytes from SbiSd
		// 2. This results in a 6-character hex string which is a valid NgapSd format
		faster_hex::hex_decode(bytes, &mut dst).unwrap();
		Some(NgapSd(dst))
	} else {
		None
	};

	NgapSnssai { sst, sd }
}

fn convert_ngap_snssai_to_sbi_snssai(ngap: &NgapSnssai) -> SbiSnssai {
	let sst = ngap.sst.0[0];

	let sd = if let Some(sd) = &ngap.sd {
		// Convert [u8; 3] to hex string
		let hex = faster_hex::hex_string(&sd.0);
		// SAFETY: Using new_unchecked is safe here because:
		// 1. The input is exactly 3 bytes from NgapSnssai's Sd type
		// 2. faster_hex::hex_string guarantees to produce a valid hex string
		// 3. The output will always be a 6-character hex string (2 chars per byte)
		// 4. This matches the SnssaiSd's validation pattern "^[A-Fa-f0-9]{6}$"
		Some(unsafe { SnssaiSd::new_unchecked(hex) })
	} else {
		None
	};

	SbiSnssai { sst, sd }
}

#[cfg(test)]
mod tests {
	use super::*;

	fn test_interconversion_snssai(
		sst: u8,
		sd_bytes: Option<[u8; 3]>,
		sd_hex: Option<&str>,
	) {
		// Create NGAP SNSSAI
		let ngap_snssai = NgapSnssai {
			sst: NgapSst([sst]),
			sd: sd_bytes.map(NgapSd),
		};

		// Create SBI SNSSAI
		let sbi_snssai = SbiSnssai {
			sst,
			sd: sd_hex.map(|hex| SnssaiSd::try_from(hex.to_string()).unwrap()),
		};

		// Test conversions
		let decoded_sbi_snssai = convert_ngap_snssai_to_sbi_snssai(&ngap_snssai);
		let decoded_ngap_snssai = convert_sbi_snssai_to_ngap_snssai(&decoded_sbi_snssai);

		// Verify SST
		assert_eq!(sbi_snssai.sst, decoded_sbi_snssai.sst);
		assert_eq!(ngap_snssai.sst.0, decoded_ngap_snssai.sst.0);

		// Verify SD if present
		match (sd_bytes, sd_hex) {
			(Some(bytes), Some(hex)) => {
				assert_eq!(decoded_sbi_snssai.sd.unwrap().to_string(), hex);
				assert_eq!(decoded_ngap_snssai.sd.unwrap().0, bytes);
			}
			(None, None) => {
				assert!(decoded_sbi_snssai.sd.is_none());
				assert!(decoded_ngap_snssai.sd.is_none());
			}
			_ => panic!("Mismatched SD test parameters"),
		}
	}

	#[test]
	fn test_snssai_conversions() {
		// Test SST only
		test_interconversion_snssai(1, None, None);

		// Test SST and SD
		test_interconversion_snssai(2, Some([0xAB, 0xCD, 0xEF]), Some("abcdef"));

		// Test with zero values
		test_interconversion_snssai(0, Some([0x00, 0x00, 0x00]), Some("000000"));

		// Test with max values
		test_interconversion_snssai(255, Some([0xFF, 0xFF, 0xFF]), Some("ffffff"));
	}
}
