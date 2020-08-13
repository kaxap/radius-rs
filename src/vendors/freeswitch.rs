/// Definitions for vendor Freeswitch, vendor value 27880
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FreeswitchHangupcause(pub u32);
 
#[allow(non_upper_case_globals)]
impl FreeswitchHangupcause {
	pub const None: FreeswitchHangupcause = FreeswitchHangupcause(0);
	pub const UnallocatedNumber: FreeswitchHangupcause = FreeswitchHangupcause(1);
	pub const NoRouteTransitNet: FreeswitchHangupcause = FreeswitchHangupcause(2);
	pub const NoRouteDestination: FreeswitchHangupcause = FreeswitchHangupcause(3);
	pub const ChannelUnacceptable: FreeswitchHangupcause = FreeswitchHangupcause(6);
	pub const CallAwardedDelivery: FreeswitchHangupcause = FreeswitchHangupcause(7);
	pub const NormalClearing: FreeswitchHangupcause = FreeswitchHangupcause(16);
	pub const UserBusy: FreeswitchHangupcause = FreeswitchHangupcause(17);
	pub const NoUserResponse: FreeswitchHangupcause = FreeswitchHangupcause(18);
	pub const NoAnswer: FreeswitchHangupcause = FreeswitchHangupcause(19);
	pub const SubscriberAbsent: FreeswitchHangupcause = FreeswitchHangupcause(20);
	pub const CallRejected: FreeswitchHangupcause = FreeswitchHangupcause(21);
	pub const NumberChanged: FreeswitchHangupcause = FreeswitchHangupcause(22);
	pub const RedirectoToNewDestination: FreeswitchHangupcause = FreeswitchHangupcause(23);
	pub const ExchangeRoutingError: FreeswitchHangupcause = FreeswitchHangupcause(25);
	pub const DestinationOutOfOrder: FreeswitchHangupcause = FreeswitchHangupcause(27);
	pub const InvalidNumberFormat: FreeswitchHangupcause = FreeswitchHangupcause(28);
	pub const FacilityRejected: FreeswitchHangupcause = FreeswitchHangupcause(29);
	pub const ResponseToStatusEnquiry: FreeswitchHangupcause = FreeswitchHangupcause(30);
	pub const NormalUnspecified: FreeswitchHangupcause = FreeswitchHangupcause(31);
	pub const NormalCircuitCongestion: FreeswitchHangupcause = FreeswitchHangupcause(34);
	pub const NetworkOutOfOrder: FreeswitchHangupcause = FreeswitchHangupcause(38);
	pub const NormalTemporaryFailure: FreeswitchHangupcause = FreeswitchHangupcause(41);
	pub const SwitchCongestion: FreeswitchHangupcause = FreeswitchHangupcause(42);
	pub const AccessInfoDiscarded: FreeswitchHangupcause = FreeswitchHangupcause(43);
	pub const RequestedChanUnavail: FreeswitchHangupcause = FreeswitchHangupcause(44);
	pub const PreEmpted: FreeswitchHangupcause = FreeswitchHangupcause(45);
	pub const FacilityNotSubscribed: FreeswitchHangupcause = FreeswitchHangupcause(50);
	pub const OutgoingCallBarred: FreeswitchHangupcause = FreeswitchHangupcause(52);
	pub const IncomingCallBarred: FreeswitchHangupcause = FreeswitchHangupcause(54);
	pub const BearercapabilityNotauth: FreeswitchHangupcause = FreeswitchHangupcause(57);
	pub const BearercapabilityNotavail: FreeswitchHangupcause = FreeswitchHangupcause(58);
	pub const ServiceUnavailable: FreeswitchHangupcause = FreeswitchHangupcause(63);
	pub const BearercapabilityNotimpl: FreeswitchHangupcause = FreeswitchHangupcause(65);
	pub const ChanNotImplemented: FreeswitchHangupcause = FreeswitchHangupcause(66);
	pub const FacilityNotImplemented: FreeswitchHangupcause = FreeswitchHangupcause(69);
	pub const ServiceNotImplemented: FreeswitchHangupcause = FreeswitchHangupcause(79);
	pub const InvalidCallReference: FreeswitchHangupcause = FreeswitchHangupcause(81);
	pub const IncompatibleDestination: FreeswitchHangupcause = FreeswitchHangupcause(88);
	pub const InvalidMsgUnspecified: FreeswitchHangupcause = FreeswitchHangupcause(95);
	pub const MandatoryIeMissing: FreeswitchHangupcause = FreeswitchHangupcause(96);
	pub const MessageTypeNonexist: FreeswitchHangupcause = FreeswitchHangupcause(97);
	pub const WrongMessage: FreeswitchHangupcause = FreeswitchHangupcause(98);
	pub const IeNonexist: FreeswitchHangupcause = FreeswitchHangupcause(99);
	pub const InvalidIeContents: FreeswitchHangupcause = FreeswitchHangupcause(100);
	pub const WrongCallState: FreeswitchHangupcause = FreeswitchHangupcause(101);
	pub const RecoveryOnTimerExpire: FreeswitchHangupcause = FreeswitchHangupcause(102);
	pub const MandatoryIeLengthError: FreeswitchHangupcause = FreeswitchHangupcause(103);
	pub const ProtocolError: FreeswitchHangupcause = FreeswitchHangupcause(111);
	pub const Interworking: FreeswitchHangupcause = FreeswitchHangupcause(127);
	pub const Success: FreeswitchHangupcause = FreeswitchHangupcause(142);
	pub const OriginatorCancel: FreeswitchHangupcause = FreeswitchHangupcause(487);
	pub const Crash: FreeswitchHangupcause = FreeswitchHangupcause(500);
	pub const SystemShutdown: FreeswitchHangupcause = FreeswitchHangupcause(501);
	pub const LoseRace: FreeswitchHangupcause = FreeswitchHangupcause(502);
	pub const ManagerRequest: FreeswitchHangupcause = FreeswitchHangupcause(503);
	pub const BlindTransfer: FreeswitchHangupcause = FreeswitchHangupcause(600);
	pub const AttendedTransfer: FreeswitchHangupcause = FreeswitchHangupcause(601);
	pub const AllottedTimeout: FreeswitchHangupcause = FreeswitchHangupcause(602);
	pub const UserChallenge: FreeswitchHangupcause = FreeswitchHangupcause(603);
	pub const MediaTimeout: FreeswitchHangupcause = FreeswitchHangupcause(604);
	pub const PickedOff: FreeswitchHangupcause = FreeswitchHangupcause(605);
	pub const UserNotRegistered: FreeswitchHangupcause = FreeswitchHangupcause(606);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaFreeswitchAvpair(i)),
		2 => value!(i, Attribute::VsaFreeswitchClid(i)),
		3 => value!(i, Attribute::VsaFreeswitchDialplan(i)),
		4 => value!(i, Attribute::VsaFreeswitchSrc(i)),
		5 => value!(i, Attribute::VsaFreeswitchDst(i)),
		6 => value!(i, Attribute::VsaFreeswitchSrcChannel(i)),
		7 => value!(i, Attribute::VsaFreeswitchDstChannel(i)),
		8 => value!(i, Attribute::VsaFreeswitchAni(i)),
		9 => value!(i, Attribute::VsaFreeswitchAniii(i)),
		10 => value!(i, Attribute::VsaFreeswitchLastapp(i)),
		11 => value!(i, Attribute::VsaFreeswitchLastdata(i)),
		12 => value!(i, Attribute::VsaFreeswitchDisposition(i)),
		13 => map! {i, be_u32, |v| Attribute::VsaFreeswitchHangupcause(FreeswitchHangupcause(v))},
		15 => map!{i, be_u32, |v| Attribute::VsaFreeswitchBillusec(v)},
		16 => map!{i, be_u32, |v| Attribute::VsaFreeswitchAmaflags(v)},
		17 => value!(i, Attribute::VsaFreeswitchRdnis(i)),
		18 => value!(i, Attribute::VsaFreeswitchContext(i)),
		19 => value!(i, Attribute::VsaFreeswitchSource(i)),
		20 => value!(i, Attribute::VsaFreeswitchCallstartdate(i)),
		21 => value!(i, Attribute::VsaFreeswitchCallanswerdate(i)),
		22 => value!(i, Attribute::VsaFreeswitchCalltransferdate(i)),
		23 => value!(i, Attribute::VsaFreeswitchCallenddate(i)),
		24 => value!(i, Attribute::VsaFreeswitchSignalbond(i)),
        _ => value!(i, Attribute::VsaUnknown(27880, typ, i)),
    }
}
