/// Definitions for vendor Foundry, vendor value 1991
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FoundryInmPrivilege(pub u32);
 
#[allow(non_upper_case_globals)]
impl FoundryInmPrivilege {
	pub const AaaPri0: FoundryInmPrivilege = FoundryInmPrivilege(0);
	pub const AaaPri1: FoundryInmPrivilege = FoundryInmPrivilege(1);
	pub const AaaPri2: FoundryInmPrivilege = FoundryInmPrivilege(2);
	pub const AaaPri3: FoundryInmPrivilege = FoundryInmPrivilege(3);
	pub const AaaPri4: FoundryInmPrivilege = FoundryInmPrivilege(4);
	pub const AaaPri5: FoundryInmPrivilege = FoundryInmPrivilege(5);
	pub const AaaPri6: FoundryInmPrivilege = FoundryInmPrivilege(6);
	pub const AaaPri7: FoundryInmPrivilege = FoundryInmPrivilege(7);
	pub const AaaPri8: FoundryInmPrivilege = FoundryInmPrivilege(8);
	pub const AaaPri9: FoundryInmPrivilege = FoundryInmPrivilege(9);
	pub const AaaPri10: FoundryInmPrivilege = FoundryInmPrivilege(10);
	pub const AaaPri11: FoundryInmPrivilege = FoundryInmPrivilege(11);
	pub const AaaPri12: FoundryInmPrivilege = FoundryInmPrivilege(12);
	pub const AaaPri13: FoundryInmPrivilege = FoundryInmPrivilege(13);
	pub const AaaPri14: FoundryInmPrivilege = FoundryInmPrivilege(14);
	pub const AaaPri15: FoundryInmPrivilege = FoundryInmPrivilege(15);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FoundryMacBasedVlanQos(pub u32);
 
#[allow(non_upper_case_globals)]
impl FoundryMacBasedVlanQos {
	pub const QosPriority0: FoundryMacBasedVlanQos = FoundryMacBasedVlanQos(0);
	pub const QosPriority1: FoundryMacBasedVlanQos = FoundryMacBasedVlanQos(1);
	pub const QosPriority2: FoundryMacBasedVlanQos = FoundryMacBasedVlanQos(2);
	pub const QosPriority3: FoundryMacBasedVlanQos = FoundryMacBasedVlanQos(3);
	pub const QosPriority4: FoundryMacBasedVlanQos = FoundryMacBasedVlanQos(4);
	pub const QosPriority5: FoundryMacBasedVlanQos = FoundryMacBasedVlanQos(5);
	pub const QosPriority6: FoundryMacBasedVlanQos = FoundryMacBasedVlanQos(6);
	pub const QosPriority7: FoundryMacBasedVlanQos = FoundryMacBasedVlanQos(7);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaFoundryPrivilegeLevel(v)},
		2 => value!(i, Attribute::VsaFoundryCommandString(i)),
		3 => map!{i, be_u32, |v| Attribute::VsaFoundryCommandExceptionFlag(v)},
		4 => map! {i, be_u32, |v| Attribute::VsaFoundryInmPrivilege(FoundryInmPrivilege(v))},
		5 => value!(i, Attribute::VsaFoundryAccessList(i)),
		6 => map!{i, be_u32, |v| Attribute::VsaFoundryMacAuthentNeeds802dot1X(v)},
		7 => map!{i, be_u32, |v| Attribute::VsaFoundry802dot1XValidLookup(v)},
		8 => map! {i, be_u32, |v| Attribute::VsaFoundryMacBasedVlanQos(FoundryMacBasedVlanQos(v))},
		9 => value!(i, Attribute::VsaFoundryInmRoleAorList(i)),
		10 => value!(i, Attribute::VsaFoundrySiContextRole(i)),
		11 => value!(i, Attribute::VsaFoundrySiRoleTemplate(i)),
        _ => value!(i, Attribute::VsaUnknown(1991, typ, i)),
    }
}
