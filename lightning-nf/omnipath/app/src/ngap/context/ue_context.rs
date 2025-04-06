use ngap_models::{AmfUeNgapId, GlobalGnbId, GlobalRanNodeId, RanUeNgapId};

use crate::ngap::manager::Identifiable;

#[derive(Debug)]
pub struct UeContext {
    pub id: UeId,
}

impl Identifiable for UeContext {
	type ID = UeId;

	fn id(&self) -> &Self::ID {
        &self.id
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct UeId {
    pub global_ran_node_id: GlobalGnbId,
    pub ran_ue_ngap_id: RanUeNgapId,
    pub amf_ue_ngap_id: AmfUeNgapId,
}

