use oasbi::common::Tac as SbiTac;
use ngap_models::Tac as NgapTac;

use super::Element;

impl From<Element<&SbiTac>> for Element<NgapTac> {
    fn from(sbi_tac: Element<&SbiTac>) -> Self {
        let ngap_tac = convert_sbi_tac_to_ngap_tac(&sbi_tac.0);
        Element(ngap_tac)
    }
}


impl From<Element<&NgapTac>> for Element<SbiTac> {
    fn from(ngap_tac: Element<&NgapTac>) -> Self {
        let sbi_tac = convert_ngap_tac_to_sbi_tac(&ngap_tac.0);
        Element(sbi_tac)
    }
}


fn convert_sbi_tac_to_ngap_tac(sbi_tac: &SbiTac) -> NgapTac {
    let bytes = sbi_tac.as_bytes();
    let mut dst = [0; 3];
    // SAFETY: The bytes are guaranteed to be valid for NgapTac because:
    // 1. They are constructed from exactly 3 bytes from SbiTac
    // 2. This results in a 6-character hex string which is a valid NgapTac format
    faster_hex::hex_decode(bytes, &mut dst).unwrap();
    NgapTac(dst)
}

fn convert_ngap_tac_to_sbi_tac(ngap_tac: &NgapTac) -> SbiTac {
    let hex_string = faster_hex::hex_string(&ngap_tac.0);

    // SAFETY: The hex_string is guaranteed to be valid for SbiTac because:
    // 1. It's constructed from exactly 3 bytes from NgapTac
    // 2. Each byte is formatted as a 2-digit hex string using {:02x}
    // 3. This results in a 6-character hex string which is a valid SbiTac format
    unsafe { SbiTac::new_unchecked(hex_string) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_interconversion_sbi_tac_to_ngap_tac(sbi_tac: &str, ngap_tac: [u8; 3]) {
        let ngap_tac = NgapTac(ngap_tac);
        let sbi_tac = unsafe { SbiTac::new_unchecked(sbi_tac.to_string()) };
        let decoded_sbi_tac = convert_ngap_tac_to_sbi_tac(&ngap_tac);
        let decoded_ngap_tac = convert_sbi_tac_to_ngap_tac(&decoded_sbi_tac);
        assert_eq!(sbi_tac.as_str(), decoded_sbi_tac.as_str());
        assert_eq!(ngap_tac.0, decoded_ngap_tac.0);
    }
    
    
    #[test]
    fn test_ngap_to_sbi_tac_conversion() {
        test_interconversion_sbi_tac_to_ngap_tac("123456", [0x12, 0x34, 0x56]);
        test_interconversion_sbi_tac_to_ngap_tac("000000", [0x00, 0x00, 0x00]);
        test_interconversion_sbi_tac_to_ngap_tac("ffffff", [0xFF, 0xFF, 0xFF]);
    }
}

