use std::net::{IpAddr, Ipv4Addr};

use nf_base::{LoggingConfig, NfConfig, RuntimeConfig};
use oasbi::{
	common::{Guami, PlmnId, Snssai, Tai, Uri, UriScheme},
	nrf::types::ServiceName,
};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OmniPathConfig {
	pub info: Info,
	#[validate]
	pub configuration: Configuration,
	pub logger: LoggingConfig,
	pub runtime: RuntimeConfig,
	#[validate]
	pub sbi: Sbi,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Info {
	#[serde_as(as = "DisplayFromStr")]
	pub version: versions::SemVer,
	pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
	pub amf_name: String,
	pub ngap_ip_list: Vec<IpAddr>,
	pub ngap_port: u16,
	#[validate(min_items = 1)]
	pub served_guami_list: Vec<Guami>,
	#[validate(min_items = 1)]
	pub support_tai_list: Vec<Tai>,
	#[validate(min_items = 1)]
	pub plmn_support_list: Vec<PlmnSupportItem>,
	#[validate(min_items = 1)]
	pub support_dnn_list: Vec<String>,
	pub nrf_uri: Uri,
	// 	pub security: NasSecurity,
	// 	pub network_name: NetworkName,
	// 	pub t3502_value: u16,
	// 	pub t3512_value: u16,
	// 	pub non3gpp_dereg_timer_value: u16,
	// 	pub t3513: Timer,
	// 	pub t3522: Timer,
	// 	pub t3550: Timer,
	// 	pub t3555: Timer,
	// 	pub t3560: Timer,
	// 	pub t3565: Timer,
	// 	pub t3570: Timer,
	// 	pub locality: String,
	// 	pub sctp: SCTP,
	// 	pub default_ue_ctx_req: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, smart_default::SmartDefault, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Sbi {
	#[serde(default = "UriScheme::default")]
	pub scheme: UriScheme,
	#[default(_code = "std::net::Ipv4Addr::LOCALHOST")]
	pub register_ipv4: Ipv4Addr,
	#[default(_code = "std::net::Ipv4Addr::LOCALHOST")]
	pub binding_ipv4: Ipv4Addr,
	pub port: u16,
	pub tls: Tls,
	#[validate(min_items = 1)]
	#[validate(custom = enum_list([ServiceName::NamfComm, ServiceName::NamfEvts, ServiceName::NamfMt, ServiceName::NamfLoc]))]
	pub service_name_list: Vec<ServiceName>,
}

#[allow(dead_code)]
pub fn enum_list<const N: usize>(
	enumerate: [ServiceName; N]
) -> impl FnOnce(&Vec<ServiceName>) -> Result<(), serde_valid::validation::Error> {
	move |val: &Vec<ServiceName>| {
		if val.iter().all(|item| enumerate.contains(item)) {
			Ok(())
		} else {
			Err(serde_valid::validation::Error::Custom(format!(
				"Not all elements in {} are present in {}.",
				display_slice(&val),
				display_slice(&enumerate)
			)))
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlmnSupportItem {
	pub plmn_id: PlmnId,
	pub snssai_list: Vec<Snssai>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tls {
	pub pem: String,
	pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NasSecurity {
	pub integrity_order: Vec<String>,
	pub ciphering_order: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkName {
	pub full: String,
	pub short: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct MobilityRestrictionList {
	enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct MaskedIMEISV {
	enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct RedirectVoiceFallback {
	enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct NetworkFeatureSupport5GS {
	enable: bool,
	length: u8,
	ims_vo_ps: u8,
	emc: u8,
	emf: u8,
	iwk_n26: u8,
	mpsi: u8,
	emc_n3: u8,
	mcsi: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Timer {
	enable: bool,
	expire_time: i32,
	max_retry_times: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct SCTP {
	num_ostreams: u8,
	max_instreams: u8,
	max_attempts: u8,
	max_init_timeout: u8,
}

impl NfConfig for OmniPathConfig {
	fn get_log_config(&self) -> &LoggingConfig {
		&self.logger
	}
	fn get_runtime_config(&self) -> &RuntimeConfig {
		&self.runtime
	}
}

impl Sbi {
	pub(crate) fn get_ipv4_uri(&self) -> String {
		format!(
			"{}://{}:{}",
			self.scheme.to_string(),
			self.register_ipv4,
			self.port
		)
	}
}

fn display_slice<T: ToString>(input: &[T]) -> String {
	input
		.iter()
		.map(|item| item.to_string())
		.collect::<Vec<_>>()
		.join(", ")
}

pub struct SerdeValidated<T>(T);
impl<T: Validate> SerdeValidated<T> {
//	pub fn new(value: &T) -> Result<&Self, serde_valid::validation::Errors> {
//		value.validate()?;
//		Ok(&SerdeValidated(value))
//	}

	pub fn new(value: T) -> Result<Self, serde_valid::validation::Errors> {
		value.validate()?;
		Ok(SerdeValidated(value))
	}

	pub fn inner(&self) -> &T {
		&self.0
	}

	pub fn into_inner(self) -> T {
		self.0
	}
}
