/// Definitions for vendor Shasta, vendor value 3199
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShastaUserPrivilege(pub u32);
 
#[allow(non_upper_case_globals)]
impl ShastaUserPrivilege {
	pub const User: ShastaUserPrivilege = ShastaUserPrivilege(1);
	pub const SuperUser: ShastaUserPrivilege = ShastaUserPrivilege(2);
	pub const SsuperUser: ShastaUserPrivilege = ShastaUserPrivilege(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaShastaUserPrivilege(ShastaUserPrivilege(v))},
		2 => value!(i, Attribute::VsaShastaServiceProfile(i)),
		3 => value!(i, Attribute::VsaShastaVpnName(i)),
        _ => value!(i, Attribute::VsaUnknown(3199, typ, i)),
    }
}
