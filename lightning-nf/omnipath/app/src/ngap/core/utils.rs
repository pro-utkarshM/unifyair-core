use ngap_models::{AmfUeNgapId, Cause, CauseProtocol, ErrorIndication, ExtendedRanNodeName, RanNodeName, RanUeNgapId};

pub fn resolve_ran_name(
	ran_node_name: Option<RanNodeName>,
	extended_ran_node_name: Option<ExtendedRanNodeName>,
) -> String {
	match (ran_node_name, extended_ran_node_name) {
		(Some(ran_node_name), None) => ran_node_name.0,
		(Some(_), Some(extended_ran_node_name)) | (None, Some(extended_ran_node_name)) => {
			let ExtendedRanNodeName {
				ran_node_name_visible_string,
				ran_node_name_utf8_string,
			} = extended_ran_node_name;
			match (ran_node_name_visible_string, ran_node_name_utf8_string) {
				(Some(ran_node_name_visible_string), None) => ran_node_name_visible_string.0,
				(Some(_), Some(ran_node_name_utf8_string))
				| (None, Some(ran_node_name_utf8_string)) => ran_node_name_utf8_string.0,
				(None, None) => String::new(),
			}
		}
		(None, None) => String::new(),
	}
}

pub fn new_semantic_error(amf_ue_ngap_id: Option<AmfUeNgapId>, ran_ue_ngap_id: Option<RanUeNgapId>) -> ErrorIndication {
	ErrorIndication {
		cause: Some(new_semantic_error_cause()),
		amf_ue_ngap_id,
		ran_ue_ngap_id,
        ..Default::default()
	}
}

#[inline(always)]
pub fn new_semantic_error_cause() -> Cause {
	Cause::Protocol(CauseProtocol::SemanticError)
}
