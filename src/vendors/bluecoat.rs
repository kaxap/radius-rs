/// Definitions for vendor BlueCoat, vendor value 14501
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlueCoatAuthorization(pub u32);
 
#[allow(non_upper_case_globals)]
impl BlueCoatAuthorization {
	pub const NoAccess: BlueCoatAuthorization = BlueCoatAuthorization(0);
	pub const ReadOnlyAccess: BlueCoatAuthorization = BlueCoatAuthorization(1);
	pub const ReadWriteAccess: BlueCoatAuthorization = BlueCoatAuthorization(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaBlueCoatGroup(i)),
		2 => map! {i, be_u32, |v| Attribute::VsaBlueCoatAuthorization(BlueCoatAuthorization(v))},
        _ => value!(i, Attribute::VsaUnknown(14501, typ, i)),
    }
}
