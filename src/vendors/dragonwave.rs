/// Definitions for vendor DragonWave, vendor value 7262
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DragonwavePrivilegeLevel(pub u32);
 
#[allow(non_upper_case_globals)]
impl DragonwavePrivilegeLevel {
	pub const DragonwaveAdminUser: DragonwavePrivilegeLevel = DragonwavePrivilegeLevel(1);
	pub const DragonwaveNocUser: DragonwavePrivilegeLevel = DragonwavePrivilegeLevel(2);
	pub const DragonwaveSuperUser: DragonwavePrivilegeLevel = DragonwavePrivilegeLevel(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaDragonwavePrivilegeLevel(DragonwavePrivilegeLevel(v))},
        _ => value!(i, Attribute::VsaUnknown(7262, typ, i)),
    }
}
