use std::ops::Deref;

use oasbi::{
	common::NfType,
	nrf::types::{AmfInfo, NfProfile, NfProfile1Unchecked, NfStatus},
};
use openapi_nrf::models::NfProfile1;
use tracing::{info, trace};

use crate::{context::app_context::AppContext, models::sbi::ModelBuildError};

impl AppContext {
	pub fn build_nf_profile(&self) -> Result<NfProfile1, ModelBuildError> {
		let config = self.get_config();
		let sbi = self.get_sbi_config();
		let amf_id = config.served_guami_list[0].amf_id;
		let amf_info = AmfInfo {
			amf_region_id: amf_id.region_id,
			amf_set_id: amf_id.set_id,
			guami_list: config.served_guami_list.clone(),
			tai_list: config.support_tai_list.clone(),
			..Default::default()
		};
		let plmn_list = config
			.plmn_support_list
			.iter()
			.map(|e| e.plmn_id)
			.collect::<Vec<_>>();
		let mut nf_profile = NfProfile1Unchecked {
			nf_instance_id: config.nf_id,
			nf_type: NfType::Amf,
			nf_status: NfStatus::Registered,
			amf_info: Some(amf_info),
			plmn_list,
			ipv4_addresses: vec![sbi.register_ipv4.into()],
			nf_services: config.nf_services.clone(),
			..Default::default()
		};
		trace!("NfProfile 1: {:#?}", nf_profile);
		Ok(nf_profile.try_into()?)
	}
}
