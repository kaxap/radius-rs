/// Definitions for vendor NetworkPhysics, vendor value 7119
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		33 => value!(i, Attribute::VsaNetsensoryPrivilege(i)),
        _ => value!(i, Attribute::VsaUnknown(7119, typ, i)),
    }
}
