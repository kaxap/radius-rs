/// Definitions for vendor Symbol, vendor value 388
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymbolAdminRole(pub u32);
 
#[allow(non_upper_case_globals)]
impl SymbolAdminRole {
	pub const Monitor: SymbolAdminRole = SymbolAdminRole(1);
	pub const Helpdesk: SymbolAdminRole = SymbolAdminRole(2);
	pub const Networkadmin: SymbolAdminRole = SymbolAdminRole(4);
	pub const Sysadmin: SymbolAdminRole = SymbolAdminRole(8);
	pub const Webadmin: SymbolAdminRole = SymbolAdminRole(16);
	pub const Security: SymbolAdminRole = SymbolAdminRole(32);
	pub const Deviceprovisioningadmin: SymbolAdminRole = SymbolAdminRole(64);
	pub const Superuser: SymbolAdminRole = SymbolAdminRole(32768);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymbolQosProfile(pub u32);
 
#[allow(non_upper_case_globals)]
impl SymbolQosProfile {
	pub const Besteffort: SymbolQosProfile = SymbolQosProfile(1);
	pub const Background: SymbolQosProfile = SymbolQosProfile(2);
	pub const Video: SymbolQosProfile = SymbolQosProfile(3);
	pub const Voice: SymbolQosProfile = SymbolQosProfile(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymbolLoginSource(pub u32);
 
#[allow(non_upper_case_globals)]
impl SymbolLoginSource {
	pub const Http: SymbolLoginSource = SymbolLoginSource(16);
	pub const Ssh: SymbolLoginSource = SymbolLoginSource(32);
	pub const Telnet: SymbolLoginSource = SymbolLoginSource(64);
	pub const Console: SymbolLoginSource = SymbolLoginSource(128);
	pub const All: SymbolLoginSource = SymbolLoginSource(240);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaSymbolAdminRole(SymbolAdminRole(v))},
		2 => value!(i, Attribute::VsaSymbolCurrentEssid(i)),
		3 => value!(i, Attribute::VsaSymbolAllowedEssid(i)),
		4 => map!{i, be_u32, |v| Attribute::VsaSymbolWlanIndex(v)},
		5 => map! {i, be_u32, |v| Attribute::VsaSymbolQosProfile(SymbolQosProfile(v))},
		6 => value!(i, Attribute::VsaSymbolAllowedRadio(i)),
		7 => value!(i, Attribute::VsaSymbolExpiryDateTime(i)),
		8 => value!(i, Attribute::VsaSymbolStartDateTime(i)),
		9 => value!(i, Attribute::VsaSymbolPostureStatus(i)),
		10 => map!{i, be_u32, |v| Attribute::VsaSymbolDownlinkLimit(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaSymbolUplinkLimit(v)},
		12 => value!(i, Attribute::VsaSymbolUserGroup(i)),
		17 => value!(i, Attribute::VsaSymbolApName(i)),
		18 => value!(i, Attribute::VsaSymbolApIpAddress(i)),
		19 => value!(i, Attribute::VsaSymbolApMacAddress(i)),
		22 => value!(i, Attribute::VsaSymbolVlanName(i)),
		31 => value!(i, Attribute::VsaSymbolAppPolicy(i)),
		32 => value!(i, Attribute::VsaSymbolApRfDomain(i)),
		33 => value!(i, Attribute::VsaSymbolNsightAllowedLocation(i)),
		100 => map! {i, be_u32, |v| Attribute::VsaSymbolLoginSource(SymbolLoginSource(v))},
        _ => value!(i, Attribute::VsaUnknown(388, typ, i)),
    }
}
