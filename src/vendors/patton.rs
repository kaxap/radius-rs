/// Definitions for vendor Patton, vendor value 1768
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PattonDisconnectCause(pub u32);
 
#[allow(non_upper_case_globals)]
impl PattonDisconnectCause {
	pub const ValidCauseCodeNotYetReceived: PattonDisconnectCause = PattonDisconnectCause(0x00);
	pub const UnallocatedRunassignedNumber: PattonDisconnectCause = PattonDisconnectCause(0x01);
	pub const NoRouteToSpecifiedTransitNetworkWan: PattonDisconnectCause = PattonDisconnectCause(0x02);
	pub const NoRouteToDestination: PattonDisconnectCause = PattonDisconnectCause(0x03);
	pub const SendSpecialInformationTone: PattonDisconnectCause = PattonDisconnectCause(0x04);
	pub const MisdialledTrunkPrefix: PattonDisconnectCause = PattonDisconnectCause(0x05);
	pub const ChannelUnacceptable: PattonDisconnectCause = PattonDisconnectCause(0x06);
	pub const CallAwardedAndBeingDeliveredInAnEstablishedChannel: PattonDisconnectCause = PattonDisconnectCause(0x07);
	pub const Prefix0DialedButNotAllowed: PattonDisconnectCause = PattonDisconnectCause(0x08);
	pub const Prefix1DialedButNotAllowed: PattonDisconnectCause = PattonDisconnectCause(0x09);
	pub const Prefix1DialedButNotRequired: PattonDisconnectCause = PattonDisconnectCause(0x0A);
	pub const MoreDigitsReceivedThanAllowedCallIsProceeding: PattonDisconnectCause = PattonDisconnectCause(0x0B);
	pub const NormalCallClearing: PattonDisconnectCause = PattonDisconnectCause(0x10);
	pub const UserBusy: PattonDisconnectCause = PattonDisconnectCause(0x11);
	pub const NoUserResponding: PattonDisconnectCause = PattonDisconnectCause(0x12);
	pub const NoAnswerFromUser: PattonDisconnectCause = PattonDisconnectCause(0x13);
	pub const CallRejected: PattonDisconnectCause = PattonDisconnectCause(0x15);
	pub const NumberChanged: PattonDisconnectCause = PattonDisconnectCause(0x16);
	pub const ReverseChargingRejected: PattonDisconnectCause = PattonDisconnectCause(0x17);
	pub const CallSuspended: PattonDisconnectCause = PattonDisconnectCause(0x18);
	pub const CallResumed: PattonDisconnectCause = PattonDisconnectCause(0x19);
	pub const NonSelectedUserClearing: PattonDisconnectCause = PattonDisconnectCause(0x1A);
	pub const DestinationOutOfOrder: PattonDisconnectCause = PattonDisconnectCause(0x1B);
	pub const InvalidNumberFormatIncompleteNumber: PattonDisconnectCause = PattonDisconnectCause(0x1C);
	pub const FacilityRejected: PattonDisconnectCause = PattonDisconnectCause(0x1D);
	pub const ResponseToStatusEnquiry: PattonDisconnectCause = PattonDisconnectCause(0x1E);
	pub const NormalUnspecified: PattonDisconnectCause = PattonDisconnectCause(0x1F);
	pub const CircuitOutOfOrder: PattonDisconnectCause = PattonDisconnectCause(0x21);
	pub const NoCircuitOrChannelAvailable: PattonDisconnectCause = PattonDisconnectCause(0x22);
	pub const DestinationUnattainable: PattonDisconnectCause = PattonDisconnectCause(0x23);
	pub const DegradedService: PattonDisconnectCause = PattonDisconnectCause(0x25);
	pub const NetworkWanOutOfOrder: PattonDisconnectCause = PattonDisconnectCause(0x26);
	pub const TransitDelayRangeCannotBeAchieved: PattonDisconnectCause = PattonDisconnectCause(0x27);
	pub const ThroughputRangeCannotBeAchieved: PattonDisconnectCause = PattonDisconnectCause(0x28);
	pub const TemporaryFailure: PattonDisconnectCause = PattonDisconnectCause(0x29);
	pub const SwitchingEquipmentCongestion: PattonDisconnectCause = PattonDisconnectCause(0x2A);
	pub const AccessInformationDiscarded: PattonDisconnectCause = PattonDisconnectCause(0x2B);
	pub const RequestedCircuitChannelNotAvailable: PattonDisconnectCause = PattonDisconnectCause(0x2C);
	pub const PreEmpted: PattonDisconnectCause = PattonDisconnectCause(0x2D);
	pub const PrecedenceCallBlocked: PattonDisconnectCause = PattonDisconnectCause(0x2E);
	pub const ResourceUnavailableUnspecified: PattonDisconnectCause = PattonDisconnectCause(0x2F);
	pub const QualityOfServiceUnavailable: PattonDisconnectCause = PattonDisconnectCause(0x31);
	pub const RequestedFacilityNotSubscribed: PattonDisconnectCause = PattonDisconnectCause(0x32);
	pub const ReverseChargingNotAllowed: PattonDisconnectCause = PattonDisconnectCause(0x33);
	pub const OutgoingCallsBarred: PattonDisconnectCause = PattonDisconnectCause(0x34);
	pub const OutgoingCallsBarredWithinCug: PattonDisconnectCause = PattonDisconnectCause(0x35);
	pub const IncomingCallsBarred: PattonDisconnectCause = PattonDisconnectCause(0x36);
	pub const IncomingCallsBarredWithinCug: PattonDisconnectCause = PattonDisconnectCause(0x37);
	pub const CallWaitingNotSubscribed: PattonDisconnectCause = PattonDisconnectCause(0x38);
	pub const BearerCapabilityNotAuthorized: PattonDisconnectCause = PattonDisconnectCause(0x39);
	pub const BearerCapabilityNotPresentlyAvailable: PattonDisconnectCause = PattonDisconnectCause(0x3A);
	pub const ServiceOrOptionNotAvailableUnspecified: PattonDisconnectCause = PattonDisconnectCause(0x3F);
	pub const BearerServiceNotImplemented: PattonDisconnectCause = PattonDisconnectCause(0x41);
	pub const ChannelTypeNotImplemented: PattonDisconnectCause = PattonDisconnectCause(0x42);
	pub const TransitNetworkSelectionNotImplemented: PattonDisconnectCause = PattonDisconnectCause(0x43);
	pub const MessageNotImplemented: PattonDisconnectCause = PattonDisconnectCause(0x44);
	pub const RequestedFacilityNotImplemented: PattonDisconnectCause = PattonDisconnectCause(0x45);
	pub const OnlyRestrictedDigitalInformationBearerCapabilityIsAvail: PattonDisconnectCause = PattonDisconnectCause(0x46);
	pub const ServiceOrOptionNotImplementedUnspecified: PattonDisconnectCause = PattonDisconnectCause(0x4F);
	pub const InvalidCallReferenceValue: PattonDisconnectCause = PattonDisconnectCause(0x51);
	pub const IdentifiedChannelDoesNotExist: PattonDisconnectCause = PattonDisconnectCause(0x52);
	pub const ASuspendedCallExistsButThisCallIdentityDoesNot: PattonDisconnectCause = PattonDisconnectCause(0x53);
	pub const CallIdentityInUse: PattonDisconnectCause = PattonDisconnectCause(0x54);
	pub const NoCallSuspended: PattonDisconnectCause = PattonDisconnectCause(0x55);
	pub const CallHavingTheRequestedCallIdentityHasBeenCleared: PattonDisconnectCause = PattonDisconnectCause(0x56);
	pub const CalledUserNotMemberOfCug: PattonDisconnectCause = PattonDisconnectCause(0x57);
	pub const IncompatibleDestination: PattonDisconnectCause = PattonDisconnectCause(0x58);
	pub const NonExistentAbbreviatedAddressEntry: PattonDisconnectCause = PattonDisconnectCause(0x59);
	pub const DestinationAddressMissingAndDirectCallNotSubscribed: PattonDisconnectCause = PattonDisconnectCause(0x5A);
	pub const InvalidTransitNetworkSelectionNationalUse: PattonDisconnectCause = PattonDisconnectCause(0x5B);
	pub const InvalidFacilityParameter: PattonDisconnectCause = PattonDisconnectCause(0x5C);
	pub const MandatoryInformationElementIsMissing: PattonDisconnectCause = PattonDisconnectCause(0x5D);
	pub const InvalidMessageUnspecified: PattonDisconnectCause = PattonDisconnectCause(0x5F);
	pub const MandatoryInformationElementIsMissing2: PattonDisconnectCause = PattonDisconnectCause(0x60);
	pub const MessageTypeNonExistentOrNotImplemented: PattonDisconnectCause = PattonDisconnectCause(0x61);
	pub const MessageNotCompatibleWithCallState: PattonDisconnectCause = PattonDisconnectCause(0x62);
	pub const InformationElementNonexistantOrNotImplemented: PattonDisconnectCause = PattonDisconnectCause(0x63);
	pub const InvalidInformationElementContents: PattonDisconnectCause = PattonDisconnectCause(0x64);
	pub const MessageNotCompatibleWithCallState2: PattonDisconnectCause = PattonDisconnectCause(0x65);
	pub const RecoveryOnTimerExpiry: PattonDisconnectCause = PattonDisconnectCause(0x66);
	pub const ParameterNonExistentOrNotImplementedPassedOn: PattonDisconnectCause = PattonDisconnectCause(0x67);
	pub const ProtocolErrorUnspecified: PattonDisconnectCause = PattonDisconnectCause(0x6F);
	pub const InternetworkingUnspecified: PattonDisconnectCause = PattonDisconnectCause(0x7F);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		16 => value!(i, Attribute::VsaPattonProtocol(i)),
		32 => value!(i, Attribute::VsaPattonSetupTime(i)),
		33 => value!(i, Attribute::VsaPattonConnectTime(i)),
		34 => value!(i, Attribute::VsaPattonDisconnectTime(i)),
		35 => map! {i, be_u32, |v| Attribute::VsaPattonDisconnectCause(PattonDisconnectCause(v))},
		36 => value!(i, Attribute::VsaPattonDisconnectSource(i)),
		48 => value!(i, Attribute::VsaPattonCalledUniqueId(i)),
		49 => map!{i, take!(4), |v:&[u8]| Attribute::VsaPattonCalledIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		50 => value!(i, Attribute::VsaPattonCalledNumberingPlan(i)),
		51 => value!(i, Attribute::VsaPattonCalledTypeOfNumber(i)),
		52 => value!(i, Attribute::VsaPattonCalledName(i)),
		53 => value!(i, Attribute::VsaPattonCalledStationId(i)),
		64 => map!{i, be_u32, |v| Attribute::VsaPattonCalledRxOctets(v)},
		65 => map!{i, be_u32, |v| Attribute::VsaPattonCalledTxOctets(v)},
		66 => map!{i, be_u32, |v| Attribute::VsaPattonCalledRxPackets(v)},
		67 => map!{i, be_u32, |v| Attribute::VsaPattonCalledTxPackets(v)},
		68 => map!{i, be_u32, |v| Attribute::VsaPattonCalledRxLostPackets(v)},
		69 => map!{i, be_u32, |v| Attribute::VsaPattonCalledTxLostPackets(v)},
		70 => map!{i, be_u32, |v| Attribute::VsaPattonCalledRxJitter(v)},
		71 => map!{i, be_u32, |v| Attribute::VsaPattonCalledTxJitter(v)},
		72 => value!(i, Attribute::VsaPattonCalledCodec(i)),
		73 => map!{i, be_u32, |v| Attribute::VsaPattonCalledRemoteIp(v)},
		74 => map!{i, be_u32, |v| Attribute::VsaPattonCalledRemoteUdpPort(v)},
		75 => map!{i, be_u32, |v| Attribute::VsaPattonCalledLocalUdpPort(v)},
		76 => map!{i, be_u32, |v| Attribute::VsaPattonCalledQos(v)},
		77 => map!{i, be_u32, |v| Attribute::VsaPattonCalledMos(v)},
		78 => map!{i, be_u32, |v| Attribute::VsaPattonCalledRoundTripTime(v)},
		80 => value!(i, Attribute::VsaPattonCallingUniqueId(i)),
		81 => map!{i, take!(4), |v:&[u8]| Attribute::VsaPattonCallingIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		82 => value!(i, Attribute::VsaPattonCallingNumberingPlan(i)),
		83 => value!(i, Attribute::VsaPattonCallingTypeOfNumber(i)),
		88 => value!(i, Attribute::VsaPattonCallingPresentationIndicator(i)),
		89 => value!(i, Attribute::VsaPattonCallingScreeningIndicator(i)),
		84 => value!(i, Attribute::VsaPattonCallingName(i)),
		85 => value!(i, Attribute::VsaPattonCallingStationId(i)),
		96 => map!{i, be_u32, |v| Attribute::VsaPattonCallingRxOctets(v)},
		97 => map!{i, be_u32, |v| Attribute::VsaPattonCallingTxOctets(v)},
		98 => map!{i, be_u32, |v| Attribute::VsaPattonCallingRxPackets(v)},
		99 => map!{i, be_u32, |v| Attribute::VsaPattonCallingTxPackets(v)},
		100 => map!{i, be_u32, |v| Attribute::VsaPattonCallingLostTxPackets(v)},
		101 => map!{i, be_u32, |v| Attribute::VsaPattonCallingLostRxPackets(v)},
		102 => map!{i, be_u32, |v| Attribute::VsaPattonCallingRxJitter(v)},
		103 => map!{i, be_u32, |v| Attribute::VsaPattonCallingTxJitter(v)},
		104 => value!(i, Attribute::VsaPattonCallingCodec(i)),
		105 => map!{i, be_u32, |v| Attribute::VsaPattonCallingRemoteIp(v)},
		106 => map!{i, be_u32, |v| Attribute::VsaPattonCallingRemoteUdpPort(v)},
		107 => map!{i, be_u32, |v| Attribute::VsaPattonCallingLocalUdpPort(v)},
		108 => map!{i, be_u32, |v| Attribute::VsaPattonCallingQos(v)},
		109 => map!{i, be_u32, |v| Attribute::VsaPattonCallingMos(v)},
		110 => map!{i, be_u32, |v| Attribute::VsaPattonCallingRoundTripTime(v)},
        _ => value!(i, Attribute::VsaUnknown(1768, typ, i)),
    }
}
