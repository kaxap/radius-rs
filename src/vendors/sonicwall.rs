/// Definitions for vendor SonicWall, vendor value 8741
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SonicwallUserPrivilege(pub u32);
 
#[allow(non_upper_case_globals)]
impl SonicwallUserPrivilege {
	pub const RemoteAccess: SonicwallUserPrivilege = SonicwallUserPrivilege(1);
	pub const BypassFilters: SonicwallUserPrivilege = SonicwallUserPrivilege(2);
	pub const VpnClientAccess: SonicwallUserPrivilege = SonicwallUserPrivilege(3);
	pub const AccessToVpn: SonicwallUserPrivilege = SonicwallUserPrivilege(4);
	pub const LimitedManagement: SonicwallUserPrivilege = SonicwallUserPrivilege(5);
	pub const L2TpClientAccess: SonicwallUserPrivilege = SonicwallUserPrivilege(6);
	pub const WirelessGuest: SonicwallUserPrivilege = SonicwallUserPrivilege(7);
	pub const WirelessAddAcl: SonicwallUserPrivilege = SonicwallUserPrivilege(8);
	pub const InternetAccess: SonicwallUserPrivilege = SonicwallUserPrivilege(9);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaSs3FirewallUserPrivilege(v)},
		3 => value!(i, Attribute::VsaSonicwallUserGroup(i)),
        _ => value!(i, Attribute::VsaUnknown(8741, typ, i)),
    }
}
