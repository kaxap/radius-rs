/// Definitions for vendor NetBorder, vendor value 35987
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetborderHangupcause(pub u32);
 
#[allow(non_upper_case_globals)]
impl NetborderHangupcause {
	pub const None: NetborderHangupcause = NetborderHangupcause(0);
	pub const UnallocatedNumber: NetborderHangupcause = NetborderHangupcause(1);
	pub const NoRouteTransitNet: NetborderHangupcause = NetborderHangupcause(2);
	pub const NoRouteDestination: NetborderHangupcause = NetborderHangupcause(3);
	pub const ChannelUnacceptable: NetborderHangupcause = NetborderHangupcause(6);
	pub const CallAwardedDelivery: NetborderHangupcause = NetborderHangupcause(7);
	pub const NormalClearing: NetborderHangupcause = NetborderHangupcause(16);
	pub const UserBusy: NetborderHangupcause = NetborderHangupcause(17);
	pub const NoUserResponse: NetborderHangupcause = NetborderHangupcause(18);
	pub const NoAnswer: NetborderHangupcause = NetborderHangupcause(19);
	pub const SubscriberAbsent: NetborderHangupcause = NetborderHangupcause(20);
	pub const CallRejected: NetborderHangupcause = NetborderHangupcause(21);
	pub const NumberChanged: NetborderHangupcause = NetborderHangupcause(22);
	pub const RedirectoToNewDestination: NetborderHangupcause = NetborderHangupcause(23);
	pub const ExchangeRoutingError: NetborderHangupcause = NetborderHangupcause(25);
	pub const DestinationOutOfOrder: NetborderHangupcause = NetborderHangupcause(27);
	pub const InvalidNumberFormat: NetborderHangupcause = NetborderHangupcause(28);
	pub const FacilityRejected: NetborderHangupcause = NetborderHangupcause(29);
	pub const ResponseToStatusEnquiry: NetborderHangupcause = NetborderHangupcause(30);
	pub const NormalUnspecified: NetborderHangupcause = NetborderHangupcause(31);
	pub const NormalCircuitCongestion: NetborderHangupcause = NetborderHangupcause(34);
	pub const NetworkOutOfOrder: NetborderHangupcause = NetborderHangupcause(38);
	pub const NormalTemporaryFailure: NetborderHangupcause = NetborderHangupcause(41);
	pub const SwitchCongestion: NetborderHangupcause = NetborderHangupcause(42);
	pub const AccessInfoDiscarded: NetborderHangupcause = NetborderHangupcause(43);
	pub const RequestedChanUnavail: NetborderHangupcause = NetborderHangupcause(44);
	pub const PreEmpted: NetborderHangupcause = NetborderHangupcause(45);
	pub const FacilityNotSubscribed: NetborderHangupcause = NetborderHangupcause(50);
	pub const OutgoingCallBarred: NetborderHangupcause = NetborderHangupcause(52);
	pub const IncomingCallBarred: NetborderHangupcause = NetborderHangupcause(54);
	pub const BearercapabilityNotauth: NetborderHangupcause = NetborderHangupcause(57);
	pub const BearercapabilityNotavail: NetborderHangupcause = NetborderHangupcause(58);
	pub const ServiceUnavailable: NetborderHangupcause = NetborderHangupcause(63);
	pub const BearercapabilityNotimpl: NetborderHangupcause = NetborderHangupcause(65);
	pub const ChanNotImplemented: NetborderHangupcause = NetborderHangupcause(66);
	pub const FacilityNotImplemented: NetborderHangupcause = NetborderHangupcause(69);
	pub const ServiceNotImplemented: NetborderHangupcause = NetborderHangupcause(79);
	pub const InvalidCallReference: NetborderHangupcause = NetborderHangupcause(81);
	pub const IncompatibleDestination: NetborderHangupcause = NetborderHangupcause(88);
	pub const InvalidMsgUnspecified: NetborderHangupcause = NetborderHangupcause(95);
	pub const MandatoryIeMissing: NetborderHangupcause = NetborderHangupcause(96);
	pub const MessageTypeNonexist: NetborderHangupcause = NetborderHangupcause(97);
	pub const WrongMessage: NetborderHangupcause = NetborderHangupcause(98);
	pub const IeNonexist: NetborderHangupcause = NetborderHangupcause(99);
	pub const InvalidIeContents: NetborderHangupcause = NetborderHangupcause(100);
	pub const WrongCallState: NetborderHangupcause = NetborderHangupcause(101);
	pub const RecoveryOnTimerExpire: NetborderHangupcause = NetborderHangupcause(102);
	pub const MandatoryIeLengthError: NetborderHangupcause = NetborderHangupcause(103);
	pub const ProtocolError: NetborderHangupcause = NetborderHangupcause(111);
	pub const Interworking: NetborderHangupcause = NetborderHangupcause(127);
	pub const Success: NetborderHangupcause = NetborderHangupcause(142);
	pub const OriginatorCancel: NetborderHangupcause = NetborderHangupcause(487);
	pub const Crash: NetborderHangupcause = NetborderHangupcause(500);
	pub const SystemShutdown: NetborderHangupcause = NetborderHangupcause(501);
	pub const LoseRace: NetborderHangupcause = NetborderHangupcause(502);
	pub const ManagerRequest: NetborderHangupcause = NetborderHangupcause(503);
	pub const BlindTransfer: NetborderHangupcause = NetborderHangupcause(600);
	pub const AttendedTransfer: NetborderHangupcause = NetborderHangupcause(601);
	pub const AllottedTimeout: NetborderHangupcause = NetborderHangupcause(602);
	pub const UserChallenge: NetborderHangupcause = NetborderHangupcause(603);
	pub const MediaTimeout: NetborderHangupcause = NetborderHangupcause(604);
	pub const PickedOff: NetborderHangupcause = NetborderHangupcause(605);
	pub const UserNotRegistered: NetborderHangupcause = NetborderHangupcause(606);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaNetborderAvpair(i)),
		2 => value!(i, Attribute::VsaNetborderClid(i)),
		3 => value!(i, Attribute::VsaNetborderDialplan(i)),
		4 => value!(i, Attribute::VsaNetborderSrc(i)),
		5 => value!(i, Attribute::VsaNetborderDst(i)),
		6 => value!(i, Attribute::VsaNetborderSrcChannel(i)),
		7 => value!(i, Attribute::VsaNetborderDstChannel(i)),
		8 => value!(i, Attribute::VsaNetborderAni(i)),
		9 => value!(i, Attribute::VsaNetborderAniii(i)),
		10 => value!(i, Attribute::VsaNetborderLastapp(i)),
		11 => value!(i, Attribute::VsaNetborderLastdata(i)),
		12 => value!(i, Attribute::VsaNetborderDisposition(i)),
		13 => map! {i, be_u32, |v| Attribute::VsaNetborderHangupcause(NetborderHangupcause(v))},
		15 => map!{i, be_u32, |v| Attribute::VsaNetborderBillusec(v)},
		16 => map!{i, be_u32, |v| Attribute::VsaNetborderAmaflags(v)},
		17 => value!(i, Attribute::VsaNetborderRdnis(i)),
		18 => value!(i, Attribute::VsaNetborderContext(i)),
		19 => value!(i, Attribute::VsaNetborderSource(i)),
		20 => value!(i, Attribute::VsaNetborderCallstartdate(i)),
		21 => value!(i, Attribute::VsaNetborderCallanswerdate(i)),
		22 => value!(i, Attribute::VsaNetborderCalltransferdate(i)),
		23 => value!(i, Attribute::VsaNetborderCallenddate(i)),
		24 => value!(i, Attribute::VsaNetborderSignalbond(i)),
        _ => value!(i, Attribute::VsaUnknown(35987, typ, i)),
    }
}
