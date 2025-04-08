use std::sync::Arc;

use ngap_models::{
	AmfUeNgapId,
	DownlinkNasTransport,
	NasPdu,
	RanUeNgapId,
	RrcEstablishmentCause,
	ToNgapPdu,
};
use crate::utils::models::FiveGSTmsi;
use tracing::info;

use super::{
	GnbContext,
	ngap_context::{NgapWriteError, encode_and_write_ngap_pdu},
};
use crate::ngap::manager::Identifiable;

#[derive(Debug)]
pub struct UeContext {
	pub ran_ue_ngap_id: RanUeNgapId,
	pub amf_ue_ngap_id: AmfUeNgapId,
	pub rrc_establishment_cause: RrcEstablishmentCause,
	pub gnb_context: Arc<GnbContext>,
	pub five_g_s_tmsi: Option<FiveGSTmsi>,
}

impl UeContext {
	pub async fn send_downlink_nas_transport(
		&self,
		nas_pdu: Vec<u8>,
	) -> Result<(), NgapWriteError> {
		let nas_pdu = NasPdu(nas_pdu);
		let downlink_nas_transport = DownlinkNasTransport {
			ran_ue_ngap_id: self.ran_ue_ngap_id,
			amf_ue_ngap_id: self.amf_ue_ngap_id,
			nas_pdu,
			..Default::default()
		};

		let pdu = downlink_nas_transport.to_pdu();
		encode_and_write_ngap_pdu(self.gnb_context.tnla_association.as_ref(), pdu).await
	}

    pub async fn handle_nas(&mut self, nas_pdu: Vec<u8>) {
        info!("Current UeContext: {:?}", self);
        info!("Received NAS PDU: {:?}", nas_pdu);
    }
}

impl Identifiable for UeContext {
	type ID = RanUeNgapId;

	fn id(&self) -> &Self::ID {
		&self.ran_ue_ngap_id
	}
}



