/// Definitions for vendor Gandalf, vendor value 64
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfOperationalModes(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfOperationalModes {
	pub const Disabled: GandalfOperationalModes = GandalfOperationalModes(1);
	pub const CalledOnly: GandalfOperationalModes = GandalfOperationalModes(2);
	pub const CallingCalled: GandalfOperationalModes = GandalfOperationalModes(3);
	pub const CallingOnly: GandalfOperationalModes = GandalfOperationalModes(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfCompressionStatus(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfCompressionStatus {
	pub const Disabled: GandalfCompressionStatus = GandalfCompressionStatus(1);
	pub const Enabled: GandalfCompressionStatus = GandalfCompressionStatus(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfMinOutgoingBearer(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfMinOutgoingBearer {
	pub const Unrestricted64K: GandalfMinOutgoingBearer = GandalfMinOutgoingBearer(1);
	pub const Digital56K: GandalfMinOutgoingBearer = GandalfMinOutgoingBearer(2);
	pub const Three100HzAudio: GandalfMinOutgoingBearer = GandalfMinOutgoingBearer(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfPppAuthentication(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfPppAuthentication {
	pub const Chap: GandalfPppAuthentication = GandalfPppAuthentication(1);
	pub const Pap: GandalfPppAuthentication = GandalfPppAuthentication(2);
	pub const PapSendingOnIncomingCalls: GandalfPppAuthentication = GandalfPppAuthentication(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfPppNcpType(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfPppNcpType {
	pub const Bcp: GandalfPppNcpType = GandalfPppNcpType(2);
	pub const Ipcp: GandalfPppNcpType = GandalfPppNcpType(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfFwdMulticastIn(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfFwdMulticastIn {
	pub const Disabled: GandalfFwdMulticastIn = GandalfFwdMulticastIn(1);
	pub const Enabled: GandalfFwdMulticastIn = GandalfFwdMulticastIn(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfFwdBroadcastIn(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfFwdBroadcastIn {
	pub const Disabled: GandalfFwdBroadcastIn = GandalfFwdBroadcastIn(1);
	pub const Enabled: GandalfFwdBroadcastIn = GandalfFwdBroadcastIn(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfFwdUnicastIn(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfFwdUnicastIn {
	pub const Disabled: GandalfFwdUnicastIn = GandalfFwdUnicastIn(1);
	pub const Enabled: GandalfFwdUnicastIn = GandalfFwdUnicastIn(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfFwdMulticastOut(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfFwdMulticastOut {
	pub const Disabled: GandalfFwdMulticastOut = GandalfFwdMulticastOut(1);
	pub const Enabled: GandalfFwdMulticastOut = GandalfFwdMulticastOut(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfFwdBroadcastOut(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfFwdBroadcastOut {
	pub const Disabled: GandalfFwdBroadcastOut = GandalfFwdBroadcastOut(1);
	pub const Enabled: GandalfFwdBroadcastOut = GandalfFwdBroadcastOut(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfFwdUnicastOut(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfFwdUnicastOut {
	pub const Disabled: GandalfFwdUnicastOut = GandalfFwdUnicastOut(1);
	pub const Enabled: GandalfFwdUnicastOut = GandalfFwdUnicastOut(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfIpxSpoofingState(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfIpxSpoofingState {
	pub const Forward: GandalfIpxSpoofingState = GandalfIpxSpoofingState(1);
	pub const Spoof: GandalfIpxSpoofingState = GandalfIpxSpoofingState(2);
	pub const FilterAllOutgoingRipSap: GandalfIpxSpoofingState = GandalfIpxSpoofingState(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfIpxWatchdogSpoof(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfIpxWatchdogSpoof {
	pub const Disabled: GandalfIpxWatchdogSpoof = GandalfIpxWatchdogSpoof(1);
	pub const Enabled: GandalfIpxWatchdogSpoof = GandalfIpxWatchdogSpoof(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfModemMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfModemMode {
	pub const Disabled: GandalfModemMode = GandalfModemMode(1);
	pub const Enabled: GandalfModemMode = GandalfModemMode(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfModemRequired1(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfModemRequired1 {
	pub const Disabled: GandalfModemRequired1 = GandalfModemRequired1(1);
	pub const Enabled: GandalfModemRequired1 = GandalfModemRequired1(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GandalfModemRequired2(pub u32);
 
#[allow(non_upper_case_globals)]
impl GandalfModemRequired2 {
	pub const Disabled: GandalfModemRequired2 = GandalfModemRequired2(1);
	pub const Enabled: GandalfModemRequired2 = GandalfModemRequired2(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		0 => value!(i, Attribute::VsaGandalfRemoteLanName(i)),
		1 => map! {i, be_u32, |v| Attribute::VsaGandalfOperationalModes(GandalfOperationalModes(v))},
		2 => map! {i, be_u32, |v| Attribute::VsaGandalfCompressionStatus(GandalfCompressionStatus(v))},
		3 => map! {i, be_u32, |v| Attribute::VsaGandalfMinOutgoingBearer(GandalfMinOutgoingBearer(v))},
		5 => value!(i, Attribute::VsaGandalfAuthenticationString(i)),
		6 => map! {i, be_u32, |v| Attribute::VsaGandalfPppAuthentication(GandalfPppAuthentication(v))},
		7 => map! {i, be_u32, |v| Attribute::VsaGandalfPppNcpType(GandalfPppNcpType(v))},
		8 => map! {i, be_u32, |v| Attribute::VsaGandalfFwdMulticastIn(GandalfFwdMulticastIn(v))},
		9 => map! {i, be_u32, |v| Attribute::VsaGandalfFwdBroadcastIn(GandalfFwdBroadcastIn(v))},
		10 => map! {i, be_u32, |v| Attribute::VsaGandalfFwdUnicastIn(GandalfFwdUnicastIn(v))},
		11 => map! {i, be_u32, |v| Attribute::VsaGandalfFwdMulticastOut(GandalfFwdMulticastOut(v))},
		12 => map! {i, be_u32, |v| Attribute::VsaGandalfFwdBroadcastOut(GandalfFwdBroadcastOut(v))},
		13 => map! {i, be_u32, |v| Attribute::VsaGandalfFwdUnicastOut(GandalfFwdUnicastOut(v))},
		14 => map!{i, be_u32, |v| Attribute::VsaGandalfAroundTheCorner(v)},
		15 => value!(i, Attribute::VsaGandalfChannelGroupName1(i)),
		16 => value!(i, Attribute::VsaGandalfDialPrefixName1(i)),
		17 => value!(i, Attribute::VsaGandalfPhoneNumber1(i)),
		18 => value!(i, Attribute::VsaGandalfCallingLineId1(i)),
		19 => value!(i, Attribute::VsaGandalfChannelGroupName2(i)),
		20 => value!(i, Attribute::VsaGandalfDialPrefixName2(i)),
		21 => value!(i, Attribute::VsaGandalfPhoneNumber2(i)),
		22 => value!(i, Attribute::VsaGandalfCallingLineId2(i)),
		23 => map! {i, be_u32, |v| Attribute::VsaGandalfIpxSpoofingState(GandalfIpxSpoofingState(v))},
		24 => map! {i, be_u32, |v| Attribute::VsaGandalfIpxWatchdogSpoof(GandalfIpxWatchdogSpoof(v))},
		25 => value!(i, Attribute::VsaGandalfSapGroupName1(i)),
		26 => value!(i, Attribute::VsaGandalfSapGroupName2(i)),
		27 => value!(i, Attribute::VsaGandalfSapGroupName3(i)),
		28 => value!(i, Attribute::VsaGandalfSapGroupName4(i)),
		29 => value!(i, Attribute::VsaGandalfSapGroupName5(i)),
		30 => value!(i, Attribute::VsaGandalfHuntGroup(i)),
		31 => map! {i, be_u32, |v| Attribute::VsaGandalfModemMode(GandalfModemMode(v))},
		32 => map! {i, be_u32, |v| Attribute::VsaGandalfModemRequired1(GandalfModemRequired1(v))},
		33 => map! {i, be_u32, |v| Attribute::VsaGandalfModemRequired2(GandalfModemRequired2(v))},
        _ => value!(i, Attribute::VsaUnknown(64, typ, i)),
    }
}
