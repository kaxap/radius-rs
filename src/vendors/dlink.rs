/// Definitions for vendor Dlink, vendor value 171
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DlinkUserLevel(pub u32);
 
#[allow(non_upper_case_globals)]
impl DlinkUserLevel {
	pub const UserLegacy: DlinkUserLevel = DlinkUserLevel(1);
	pub const User: DlinkUserLevel = DlinkUserLevel(3);
	pub const Operator: DlinkUserLevel = DlinkUserLevel(4);
	pub const Admin: DlinkUserLevel = DlinkUserLevel(5);
	pub const PowerUser: DlinkUserLevel = DlinkUserLevel(6);
	pub const AdminLegacy: DlinkUserLevel = DlinkUserLevel(15);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaDlinkUserLevel(DlinkUserLevel(v))},
		2 => map!{i, be_u32, |v| Attribute::VsaDlinkIngressBandwidthAssignment(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaDlinkEgressBandwidthAssignment(v)},
		4 => map!{i, be_u32, |v| Attribute::VsaDlink1PPriority(v)},
		10 => value!(i, Attribute::VsaDlinkVlanName(i)),
		11 => value!(i, Attribute::VsaDlinkVlanId(i)),
		12 => value!(i, Attribute::VsaDlinkAclProfile(i)),
		13 => value!(i, Attribute::VsaDlinkAclRule(i)),
		14 => value!(i, Attribute::VsaDlinkAclScript(i)),
        _ => value!(i, Attribute::VsaUnknown(171, typ, i)),
    }
}
