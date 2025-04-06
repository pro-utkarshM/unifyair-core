use ngap_models::{GlobalRanNodeId, PagingDrx};
use oasbi::common::{PlmnId, Snssai, Tai};
use tokio_util::sync::CancellationToken;
use std::sync::Arc;
use crate::ngap::manager::{ContextManager, Identifiable};
use crate::ngap::network::TnlaAssociation;
use super::UeContext;
use nonempty::NonEmpty;

use derive_new::new;

#[derive(Debug, new)]
pub struct GnbContext {
	pub tnla_association: Arc<TnlaAssociation>,
	#[new(default)]
	pub global_ran_node_id: GlobalRanNodeId,
	#[new(value = "ContextManager::new()")]
	pub ue_context_manager: ContextManager<UeContext>,
	#[new(default)]
	pub name: String,
	#[new(default)]
	pub default_paging_drx: PagingDrx,
	pub sctp_loop_cancellation: CancellationToken,
}


#[derive(Debug)]
pub struct SupportedTai {
	pub tai: Tai,
	pub snssais: NonEmpty<Snssai>,
}


impl Identifiable for GnbContext {
	type ID = GlobalRanNodeId;

	fn id(&self) -> &Self::ID {
		&self.global_ran_node_id
	}
}

