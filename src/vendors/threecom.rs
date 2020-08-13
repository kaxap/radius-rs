/// Definitions for vendor 3com, vendor value 43
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThreeComUserAccessLevel(pub u32);
 
#[allow(non_upper_case_globals)]
impl ThreeComUserAccessLevel {
	pub const ThreeComVisitor: ThreeComUserAccessLevel = ThreeComUserAccessLevel(0);
	pub const ThreeComMonitor: ThreeComUserAccessLevel = ThreeComUserAccessLevel(1);
	pub const ThreeComManager: ThreeComUserAccessLevel = ThreeComUserAccessLevel(2);
	pub const ThreeComAdministrator: ThreeComUserAccessLevel = ThreeComUserAccessLevel(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaThreeComUserAccessLevel(ThreeComUserAccessLevel(v))},
		2 => value!(i, Attribute::VsaThreeComVlanName(i)),
		3 => value!(i, Attribute::VsaThreeComMobilityProfile(i)),
		4 => value!(i, Attribute::VsaThreeComEncryptionType(i)),
		5 => value!(i, Attribute::VsaThreeComTimeOfDay(i)),
		6 => value!(i, Attribute::VsaThreeComSsid(i)),
		7 => value!(i, Attribute::VsaThreeComEndDate(i)),
		8 => value!(i, Attribute::VsaThreeComUrl(i)),
		26 => map!{i, be_u32, |v| Attribute::VsaThreeComConnectId(v)},
		59 => map!{i, be_u32, |v| Attribute::VsaThreeComNasStartupTimestamp(v)},
		60 => value!(i, Attribute::VsaThreeComIpHostAddr(i)),
		255 => value!(i, Attribute::VsaThreeComProductId(i)),
        _ => value!(i, Attribute::VsaUnknown(43, typ, i)),
    }
}
