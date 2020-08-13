/// Definitions for vendor Meinberg, vendor value 5597
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MbgManagementPrivilegeLevel(pub u32);
 
#[allow(non_upper_case_globals)]
impl MbgManagementPrivilegeLevel {
	pub const MbgPrivSuperuser: MbgManagementPrivilegeLevel = MbgManagementPrivilegeLevel(100);
	pub const MbgPrivAdmin: MbgManagementPrivilegeLevel = MbgManagementPrivilegeLevel(200);
	pub const MbgPrivInfo: MbgManagementPrivilegeLevel = MbgManagementPrivilegeLevel(300);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaMbgManagementPrivilegeLevel(MbgManagementPrivilegeLevel(v))},
        _ => value!(i, Attribute::VsaUnknown(5597, typ, i)),
    }
}
