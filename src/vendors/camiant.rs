/// Definitions for vendor Camiant, vendor value 21274
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CamiantSuiRole(pub u32);
 
#[allow(non_upper_case_globals)]
impl CamiantSuiRole {
	pub const Camiantview: CamiantSuiRole = CamiantSuiRole(101);
	pub const Camiantuser: CamiantSuiRole = CamiantSuiRole(102);
	pub const Camiantservice: CamiantSuiRole = CamiantSuiRole(104);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaCamiantMiRole(i)),
		2 => map! {i, be_u32, |v| Attribute::VsaCamiantSuiRole(CamiantSuiRole(v))},
		3 => value!(i, Attribute::VsaCamiantMiScope(i)),
        _ => value!(i, Attribute::VsaUnknown(21274, typ, i)),
    }
}
