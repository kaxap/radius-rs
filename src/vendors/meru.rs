/// Definitions for vendor Meru, vendor value 15983
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaMeruAccessPointId(v)},
		2 => value!(i, Attribute::VsaMeruAccessPointName(i)),
        _ => value!(i, Attribute::VsaUnknown(15983, typ, i)),
    }
}
