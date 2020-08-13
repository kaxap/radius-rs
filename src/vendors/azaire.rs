/// Definitions for vendor Azaire, vendor value 7751
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AzaireSelectionMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AzaireSelectionMode {
	pub const Subscribed: AzaireSelectionMode = AzaireSelectionMode(0);
	pub const SentByMs: AzaireSelectionMode = AzaireSelectionMode(1);
	pub const ChosenBySgsn: AzaireSelectionMode = AzaireSelectionMode(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AzaireApnResolutionReq(pub u32);
 
#[allow(non_upper_case_globals)]
impl AzaireApnResolutionReq {
	pub const NotRequired: AzaireApnResolutionReq = AzaireApnResolutionReq(0);
	pub const Required: AzaireApnResolutionReq = AzaireApnResolutionReq(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AzaireStatus(pub u32);
 
#[allow(non_upper_case_globals)]
impl AzaireStatus {
	pub const Success: AzaireStatus = AzaireStatus(0);
	pub const Failure: AzaireStatus = AzaireStatus(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AzaireAuthType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AzaireAuthType {
	pub const PppSim: AzaireAuthType = AzaireAuthType(1);
	pub const DummyImsi: AzaireAuthType = AzaireAuthType(2);
	pub const SoftSim: AzaireAuthType = AzaireAuthType(3);
	pub const RadiusSim: AzaireAuthType = AzaireAuthType(4);
	pub const PostPaid: AzaireAuthType = AzaireAuthType(5);
	pub const PrePaid: AzaireAuthType = AzaireAuthType(6);
	pub const LocalRadius: AzaireAuthType = AzaireAuthType(7);
	pub const ProxyRadius: AzaireAuthType = AzaireAuthType(8);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaAzaireTriplets(i)),
		2 => value!(i, Attribute::VsaAzaireImsi(i)),
		3 => value!(i, Attribute::VsaAzaireMsisdn(i)),
		4 => value!(i, Attribute::VsaAzaireApn(i)),
		5 => value!(i, Attribute::VsaAzaireQos(i)),
		6 => map! {i, be_u32, |v| Attribute::VsaAzaireSelectionMode(AzaireSelectionMode(v))},
		7 => map! {i, be_u32, |v| Attribute::VsaAzaireApnResolutionReq(AzaireApnResolutionReq(v))},
		8 => value!(i, Attribute::VsaAzaireStartTime(i)),
		9 => map!{i, be_u32, |v| Attribute::VsaAzaireNasType(v)},
		10 => map! {i, be_u32, |v| Attribute::VsaAzaireStatus(AzaireStatus(v))},
		11 => value!(i, Attribute::VsaAzaireApnOi(i)),
		12 => map! {i, be_u32, |v| Attribute::VsaAzaireAuthType(AzaireAuthType(v))},
		13 => value!(i, Attribute::VsaAzaireGnUserName(i)),
		14 => value!(i, Attribute::VsaAzaireBrandCode(i)),
		15 => value!(i, Attribute::VsaAzairePolicyName(i)),
		16 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAzaireClientLocalIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
        _ => value!(i, Attribute::VsaUnknown(7751, typ, i)),
    }
}
