/// Definitions for vendor Surfnet, vendor value 1076
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaSurfnetAvpair(i)),
		2 => value!(i, Attribute::VsaSurfnetServiceIdentifier(i)),
		3 => value!(i, Attribute::VsaSurfnetServiceProvider(i)),
        _ => value!(i, Attribute::VsaUnknown(1076, typ, i)),
    }
}
