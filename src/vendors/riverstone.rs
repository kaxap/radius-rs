/// Definitions for vendor Riverstone, vendor value 5567
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaRiverstoneCommand(i)),
		2 => value!(i, Attribute::VsaRiverstoneSystemEvent(i)),
		3 => value!(i, Attribute::VsaRiverstoneSnmpConfigChange(i)),
		4 => map!{i, be_u32, |v| Attribute::VsaRiverstoneUserLevel(v)},
        _ => value!(i, Attribute::VsaUnknown(5567, typ, i)),
    }
}
