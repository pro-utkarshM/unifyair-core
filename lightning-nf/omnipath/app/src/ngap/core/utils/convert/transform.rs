use bitvec::prelude::*;
use ngap_models::{
	AmfPointer as NgapAmfPointer,
	AmfRegionId as NgapAmfRegionId,
	AmfSetId as NgapAmfSetId,
	Guami as NgapGuami,
	PlmnSupportItem as NgapPlmnSupportItem,
	PlmnSupportList as NgapPlmnSupportList,
	ServedGuamiItem as NgapServedGuamiItem,
	ServedGuamiList as NgapServedGuamiList,
	SliceSupportItem as NgapSliceSupportItem,
	SliceSupportList as NgapSliceSupportList,
};
use nonempty::NonEmpty;
use oasbi::common::{AmfId as SbiAmfId, Guami as SbiGuami, Snssai as SbiSnssai};
use tracing::info;

use super::{Element, convert, transform_nonempty};
use crate::config::PlmnSupportItem as ConfigPlmnSupportItem;

impl From<Element<&NonEmpty<ConfigPlmnSupportItem>>> for Element<NgapPlmnSupportList> {
	fn from(value: Element<&NonEmpty<ConfigPlmnSupportItem>>) -> Self {
		let plmn_support_list = transform_nonempty(&value.0, |item| NgapPlmnSupportItem {
			plmn_identity: convert(&item.plmn_id),
			slice_support_list: convert(&item.snssai_list),
			..Default::default()
		});
		Element(NgapPlmnSupportList(plmn_support_list))
	}
}

impl From<Element<&NonEmpty<SbiSnssai>>> for Element<NgapSliceSupportList> {
	fn from(value: Element<&NonEmpty<SbiSnssai>>) -> Self {
		let slice_support_list = transform_nonempty(&value.0, |item| NgapSliceSupportItem {
			snssai: convert(item),
		});
		Element(NgapSliceSupportList(slice_support_list))
	}
}

impl From<Element<&NonEmpty<SbiGuami>>> for Element<NgapServedGuamiList> {
	fn from(value: Element<&NonEmpty<SbiGuami>>) -> Self {
		let guami_list = transform_nonempty(&value.0, |item| NgapServedGuamiItem {
			guami: convert(item),
			..Default::default()
		});
		Element(NgapServedGuamiList(guami_list))
	}
}

impl From<Element<&SbiGuami>> for Element<NgapGuami> {
	fn from(value: Element<&SbiGuami>) -> Self {
		let (amf_region_id, amf_set_id, amf_pointer) = convert(&value.0.amf_id);

		Element(NgapGuami {
			plmn_identity: convert(&value.0.plmn_id),
			amf_region_id,
			amf_set_id,
			amf_pointer,
		})
	}
}

impl From<Element<&SbiAmfId>> for Element<(NgapAmfRegionId, NgapAmfSetId, NgapAmfPointer)> {
	fn from(value: Element<&SbiAmfId>) -> Self {
		let SbiAmfId {
			region_id,
			set_id,
			pointer_id,
		} = value.0;
		let mut ngap_amf_region_id = bitvec![u8,Msb0; 0; 8];
		let mut ngap_amf_set_id = bitvec![u8,Msb0; 0; 10];
		let mut ngap_amf_pointer = bitvec![u8,Msb0; 0; 6];

		ngap_amf_pointer.store_be::<u8>(*pointer_id);
		ngap_amf_region_id.store_be::<u8>(region_id.inner());
		ngap_amf_set_id.store_be::<u16>(set_id.inner());

		Element((
			NgapAmfRegionId(ngap_amf_region_id.into()),
			NgapAmfSetId(ngap_amf_set_id.into()),
			NgapAmfPointer(ngap_amf_pointer.into()),
		))
	}
}
