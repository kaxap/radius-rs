/// Definitions for vendor Alcatel, vendor value 3041
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AatMcastClient(pub u32);
 
#[allow(non_upper_case_globals)]
impl AatMcastClient {
	pub const MulticastNo: AatMcastClient = AatMcastClient(0);
	pub const MulticastYes: AatMcastClient = AatMcastClient(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AatRequireAuth(pub u32);
 
#[allow(non_upper_case_globals)]
impl AatRequireAuth {
	pub const NotRequireAuth: AatRequireAuth = AatRequireAuth(0);
	pub const RequireAuth: AatRequireAuth = AatRequireAuth(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AatFrDirect(pub u32);
 
#[allow(non_upper_case_globals)]
impl AatFrDirect {
	pub const No: AatFrDirect = AatFrDirect(0);
	pub const Yes: AatFrDirect = AatFrDirect(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AatSourceIpCheck(pub u32);
 
#[allow(non_upper_case_globals)]
impl AatSourceIpCheck {
	pub const SourceIpCheckNo: AatSourceIpCheck = AatSourceIpCheck(0);
	pub const SourceIpCheckYes: AatSourceIpCheck = AatSourceIpCheck(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AatIpTos(pub u32);
 
#[allow(non_upper_case_globals)]
impl AatIpTos {
	pub const IpTosNormal: AatIpTos = AatIpTos(0);
	pub const IpTosDisabled: AatIpTos = AatIpTos(1);
	pub const IpTosCost: AatIpTos = AatIpTos(2);
	pub const IpTosReliability: AatIpTos = AatIpTos(4);
	pub const IpTosThroughput: AatIpTos = AatIpTos(8);
	pub const IpTosLatency: AatIpTos = AatIpTos(16);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AatIpTosApplyTo(pub u32);
 
#[allow(non_upper_case_globals)]
impl AatIpTosApplyTo {
	pub const IpTosApplyToIncoming: AatIpTosApplyTo = AatIpTosApplyTo(1024);
	pub const IpTosApplyToBoth: AatIpTosApplyTo = AatIpTosApplyTo(3072);
	pub const IpTosApplyToOutgoing: AatIpTosApplyTo = AatIpTosApplyTo(2048);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AatIpTosPrecedence(pub u32);
 
#[allow(non_upper_case_globals)]
impl AatIpTosPrecedence {
	pub const IpTosPrecedencePriNormal: AatIpTosPrecedence = AatIpTosPrecedence(0);
	pub const IpTosPrecedencePriOne: AatIpTosPrecedence = AatIpTosPrecedence(32);
	pub const IpTosPrecedencePriTwo: AatIpTosPrecedence = AatIpTosPrecedence(64);
	pub const IpTosPrecedencePriThree: AatIpTosPrecedence = AatIpTosPrecedence(96);
	pub const IpTosPrecedencePriFour: AatIpTosPrecedence = AatIpTosPrecedence(128);
	pub const IpTosPrecedencePriFive: AatIpTosPrecedence = AatIpTosPrecedence(160);
	pub const IpTosPrecedencePriSix: AatIpTosPrecedence = AatIpTosPrecedence(192);
	pub const IpTosPrecedencePriSeven: AatIpTosPrecedence = AatIpTosPrecedence(224);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AatAuthType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AatAuthType {
	pub const AatAuthNone: AatAuthType = AatAuthType(0);
	pub const AatAuthDefault: AatAuthType = AatAuthType(1);
	pub const AatAuthAny: AatAuthType = AatAuthType(2);
	pub const AatAuthPap: AatAuthType = AatAuthType(3);
	pub const AatAuthChap: AatAuthType = AatAuthType(4);
	pub const AatAuthMsChap: AatAuthType = AatAuthType(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AatClientAssignDns(pub u32);
 
#[allow(non_upper_case_globals)]
impl AatClientAssignDns {
	pub const DnsAssignNo: AatClientAssignDns = AatClientAssignDns(0);
	pub const DnsAssignYes: AatClientAssignDns = AatClientAssignDns(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		5 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAatClientPrimaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		6 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAatClientPrimaryWinsNbns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		7 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAatClientSecondaryWinsNbns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		8 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAatClientSecondaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		9 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAatPppAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		10 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAatPppNetmask(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		12 => value!(i, Attribute::VsaAatPrimaryHomeAgent(i)),
		13 => value!(i, Attribute::VsaAatSecondaryHomeAgent(i)),
		14 => value!(i, Attribute::VsaAatHomeAgentPassword(i)),
		15 => value!(i, Attribute::VsaAatHomeNetworkName(i)),
		16 => map!{i, be_u32, |v| Attribute::VsaAatHomeAgentUdpPort(v)},
		17 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAatIpDirect(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		18 => map! {i, be_u32, |v| Attribute::VsaAatFrDirect(AatFrDirect(v))},
		19 => value!(i, Attribute::VsaAatFrDirectProfile(i)),
		20 => map!{i, be_u32, |v| Attribute::VsaAatFrDirectDlci(v)},
		21 => value!(i, Attribute::VsaAatAtmDirect(i)),
		22 => map! {i, be_u32, |v| Attribute::VsaAatIpTos(AatIpTos(v))},
		23 => map! {i, be_u32, |v| Attribute::VsaAatIpTosPrecedence(AatIpTosPrecedence(v))},
		24 => map! {i, be_u32, |v| Attribute::VsaAatIpTosApplyTo(AatIpTosApplyTo(v))},
		27 => map! {i, be_u32, |v| Attribute::VsaAatMcastClient(AatMcastClient(v))},
		28 => map!{i, be_u32, |v| Attribute::VsaAatModemPortNo(v)},
		29 => map!{i, be_u32, |v| Attribute::VsaAatModemSlotNo(v)},
		30 => map!{i, be_u32, |v| Attribute::VsaAatModemShelfNo(v)},
		60 => value!(i, Attribute::VsaAatFilter(i)),
		61 => value!(i, Attribute::VsaAatVrouterName(i)),
		62 => map! {i, be_u32, |v| Attribute::VsaAatRequireAuth(AatRequireAuth(v))},
		63 => value!(i, Attribute::VsaAatIpPoolDefinition(i)),
		64 => map!{i, be_u32, |v| Attribute::VsaAatAssignIpPool(v)},
		65 => value!(i, Attribute::VsaAatDataFilter(i)),
		66 => map! {i, be_u32, |v| Attribute::VsaAatSourceIpCheck(AatSourceIpCheck(v))},
		67 => value!(i, Attribute::VsaAatModemAnswerString(i)),
		68 => map! {i, be_u32, |v| Attribute::VsaAatAuthType(AatAuthType(v))},
		70 => map!{i, be_u32, |v| Attribute::VsaAatQos(v)},
		71 => map!{i, be_u32, |v| Attribute::VsaAatQoa(v)},
		72 => map! {i, be_u32, |v| Attribute::VsaAatClientAssignDns(AatClientAssignDns(v))},
		128 => map!{i, be_u32, |v| Attribute::VsaAatAtmVpi(v)},
		129 => map!{i, be_u32, |v| Attribute::VsaAatAtmVci(v)},
		130 => map!{i, be_u32, |v| Attribute::VsaAatInputOctetsDiff(v)},
		131 => map!{i, be_u32, |v| Attribute::VsaAatOutputOctetsDiff(v)},
		132 => value!(i, Attribute::VsaAatUserMacAddress(i)),
		133 => value!(i, Attribute::VsaAatAtmTrafficProfile(i)),
        _ => value!(i, Attribute::VsaUnknown(3041, typ, i)),
    }
}
