/// Definitions for vendor Versanet, vendor value 2180
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VersanetTerminationCause(pub u32);
 
#[allow(non_upper_case_globals)]
impl VersanetTerminationCause {
	pub const NormalHangupNoErrorOccurred: VersanetTerminationCause = VersanetTerminationCause(0);
	pub const CallWaitingCausedDisconnect: VersanetTerminationCause = VersanetTerminationCause(3);
	pub const PhysicalCarrierLoss: VersanetTerminationCause = VersanetTerminationCause(4);
	pub const NoErrCorrectionAtOtherEnd: VersanetTerminationCause = VersanetTerminationCause(5);
	pub const NoRespToFeatureNegotiation: VersanetTerminationCause = VersanetTerminationCause(6);
	pub const OneStModemAsyncOnly2NdSync: VersanetTerminationCause = VersanetTerminationCause(7);
	pub const NoFramingTechniqueInCommon: VersanetTerminationCause = VersanetTerminationCause(8);
	pub const NoProtocolInCommon: VersanetTerminationCause = VersanetTerminationCause(9);
	pub const BadRespToFeatureNegotiation: VersanetTerminationCause = VersanetTerminationCause(10);
	pub const NoSyncInfoFromRemoteModem: VersanetTerminationCause = VersanetTerminationCause(11);
	pub const NormalHangupByRemoteModem: VersanetTerminationCause = VersanetTerminationCause(12);
	pub const RetransmissionLimitReached: VersanetTerminationCause = VersanetTerminationCause(13);
	pub const ProtocolViolationOccurred: VersanetTerminationCause = VersanetTerminationCause(14);
	pub const LostDtr: VersanetTerminationCause = VersanetTerminationCause(15);
	pub const ReceivedGstnCleardown: VersanetTerminationCause = VersanetTerminationCause(16);
	pub const InactivityTimeout: VersanetTerminationCause = VersanetTerminationCause(17);
	pub const SpeedNotSupported: VersanetTerminationCause = VersanetTerminationCause(18);
	pub const LongSpaceDisconnect: VersanetTerminationCause = VersanetTerminationCause(19);
	pub const KeyAbortDisconnect: VersanetTerminationCause = VersanetTerminationCause(20);
	pub const ClearsPreviousDiscReason: VersanetTerminationCause = VersanetTerminationCause(21);
	pub const NoConnectionEstablished: VersanetTerminationCause = VersanetTerminationCause(22);
	pub const DisconnectAfterThreeRetrains: VersanetTerminationCause = VersanetTerminationCause(23);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaVersanetTerminationCause(VersanetTerminationCause(v))},
        _ => value!(i, Attribute::VsaUnknown(2180, typ, i)),
    }
}
