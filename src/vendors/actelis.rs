/// Definitions for vendor Actelis, vendor value 5468
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaActelisPrivilege(i)),
        _ => value!(i, Attribute::VsaUnknown(5468, typ, i)),
    }
}
