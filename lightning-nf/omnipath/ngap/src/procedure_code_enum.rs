use std::convert::TryFrom;

use thiserror::Error;

use ngap_models::ProcedureCode;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProcedureCodeEnum {
	AMFConfigurationUpdate = 0,
	AMFStatusIndication = 1,
	CellTrafficTrace = 2,
	DeactivateTrace = 3,
	DownlinkNASTransport = 4,
	DownlinkNonUEAssociatedNRPPaTransport = 5,
	DownlinkRANConfigurationTransfer = 6,
	DownlinkRANStatusTransfer = 7,
	DownlinkUEAssociatedNRPPaTransport = 8,
	ErrorIndication = 9,
	HandoverCancel = 10,
	HandoverNotification = 11,
	HandoverPreparation = 12,
	HandoverResourceAllocation = 13,
	InitialContextSetup = 14,
	InitialUEMessage = 15,
	LocationReportingControl = 16,
	LocationReportingFailureIndication = 17,
	LocationReport = 18,
	NASNonDeliveryIndication = 19,
	NGReset = 20,
	NGSetup = 21,
	OverloadStart = 22,
	OverloadStop = 23,
	Paging = 24,
	PathSwitchRequest = 25,
	PDUSessionResourceModify = 26,
	PDUSessionResourceModifyIndication = 27,
	PDUSessionResourceRelease = 28,
	PDUSessionResourceSetup = 29,
	PDUSessionResourceNotify = 30,
	PrivateMessage = 31,
	PWSCancel = 32,
	PWSFailureIndication = 33,
	PWSRestartIndication = 34,
	RANConfigurationUpdate = 35,
	RerouteNASRequest = 36,
	RRCInactiveTransitionReport = 37,
	TraceFailureIndication = 38,
	TraceStart = 39,
	UEContextModification = 40,
	UEContextRelease = 41,
	UEContextReleaseRequest = 42,
	UERadioCapabilityCheck = 43,
	UERadioCapabilityInfoIndication = 44,
	UETNLABindingRelease = 45,
	UplinkNASTransport = 46,
	UplinkNonUEAssociatedNRPPaTransport = 47,
	UplinkRANConfigurationTransfer = 48,
	UplinkRANStatusTransfer = 49,
	UplinkUEAssociatedNRPPaTransport = 50,
	WriteReplaceWarning = 51,
	SecondaryRATDataUsageReport = 52,
}

impl TryFrom<ProcedureCode> for ProcedureCodeEnum {
	type Error = ProcedureCodeEnumError;

	fn try_from(value: ProcedureCode) -> Result<Self, ProcedureCodeEnumError> {
		ProcedureCodeEnum::try_from(value.0)
	}
}

impl TryFrom<u8> for ProcedureCodeEnum {
	type Error = ProcedureCodeEnumError;

	fn try_from(value: u8) -> Result<Self, ProcedureCodeEnumError> {
		match value {
			0 => Ok(ProcedureCodeEnum::AMFConfigurationUpdate),
			1 => Ok(ProcedureCodeEnum::AMFStatusIndication),
			2 => Ok(ProcedureCodeEnum::CellTrafficTrace),
			3 => Ok(ProcedureCodeEnum::DeactivateTrace),
			4 => Ok(ProcedureCodeEnum::DownlinkNASTransport),
			5 => Ok(ProcedureCodeEnum::DownlinkNonUEAssociatedNRPPaTransport),
			6 => Ok(ProcedureCodeEnum::DownlinkRANConfigurationTransfer),
			7 => Ok(ProcedureCodeEnum::DownlinkRANStatusTransfer),
			8 => Ok(ProcedureCodeEnum::DownlinkUEAssociatedNRPPaTransport),
			9 => Ok(ProcedureCodeEnum::ErrorIndication),
			10 => Ok(ProcedureCodeEnum::HandoverCancel),
			11 => Ok(ProcedureCodeEnum::HandoverNotification),
			12 => Ok(ProcedureCodeEnum::HandoverPreparation),
			13 => Ok(ProcedureCodeEnum::HandoverResourceAllocation),
			14 => Ok(ProcedureCodeEnum::InitialContextSetup),
			15 => Ok(ProcedureCodeEnum::InitialUEMessage),
			16 => Ok(ProcedureCodeEnum::LocationReportingControl),
			17 => Ok(ProcedureCodeEnum::LocationReportingFailureIndication),
			18 => Ok(ProcedureCodeEnum::LocationReport),
			19 => Ok(ProcedureCodeEnum::NASNonDeliveryIndication),
			20 => Ok(ProcedureCodeEnum::NGReset),
			21 => Ok(ProcedureCodeEnum::NGSetup),
			22 => Ok(ProcedureCodeEnum::OverloadStart),
			23 => Ok(ProcedureCodeEnum::OverloadStop),
			24 => Ok(ProcedureCodeEnum::Paging),
			25 => Ok(ProcedureCodeEnum::PathSwitchRequest),
			26 => Ok(ProcedureCodeEnum::PDUSessionResourceModify),
			27 => Ok(ProcedureCodeEnum::PDUSessionResourceModifyIndication),
			28 => Ok(ProcedureCodeEnum::PDUSessionResourceRelease),
			29 => Ok(ProcedureCodeEnum::PDUSessionResourceSetup),
			30 => Ok(ProcedureCodeEnum::PDUSessionResourceNotify),
			31 => Ok(ProcedureCodeEnum::PrivateMessage),
			32 => Ok(ProcedureCodeEnum::PWSCancel),
			33 => Ok(ProcedureCodeEnum::PWSFailureIndication),
			34 => Ok(ProcedureCodeEnum::PWSRestartIndication),
			35 => Ok(ProcedureCodeEnum::RANConfigurationUpdate),
			36 => Ok(ProcedureCodeEnum::RerouteNASRequest),
			37 => Ok(ProcedureCodeEnum::RRCInactiveTransitionReport),
			38 => Ok(ProcedureCodeEnum::TraceFailureIndication),
			39 => Ok(ProcedureCodeEnum::TraceStart),
			40 => Ok(ProcedureCodeEnum::UEContextModification),
			41 => Ok(ProcedureCodeEnum::UEContextRelease),
			42 => Ok(ProcedureCodeEnum::UEContextReleaseRequest),
			43 => Ok(ProcedureCodeEnum::UERadioCapabilityCheck),
			44 => Ok(ProcedureCodeEnum::UERadioCapabilityInfoIndication),
			45 => Ok(ProcedureCodeEnum::UETNLABindingRelease),
			46 => Ok(ProcedureCodeEnum::UplinkNASTransport),
			47 => Ok(ProcedureCodeEnum::UplinkNonUEAssociatedNRPPaTransport),
			48 => Ok(ProcedureCodeEnum::UplinkRANConfigurationTransfer),
			49 => Ok(ProcedureCodeEnum::UplinkRANStatusTransfer),
			50 => Ok(ProcedureCodeEnum::UplinkUEAssociatedNRPPaTransport),
			51 => Ok(ProcedureCodeEnum::WriteReplaceWarning),
			52 => Ok(ProcedureCodeEnum::SecondaryRATDataUsageReport),
			_ => Err(ProcedureCodeEnumError::UnableToPerformTryFrom),
		}
	}
}

impl From<ProcedureCodeEnum> for ProcedureCode {
	fn from(value: ProcedureCodeEnum) -> ProcedureCode {
		ProcedureCode(value as u8)
	}
}

#[derive(Error, Debug)]
pub enum ProcedureCodeEnumError {
	#[error("Unable to perform try from on procedure code")]
	UnableToPerformTryFrom,
}
