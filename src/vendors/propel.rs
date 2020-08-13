/// Definitions for vendor Propel, vendor value 14895
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaPropelAccelerate(v)},
		2 => value!(i, Attribute::VsaPropelDialedDigits(i)),
		3 => map!{i, take!(4), |v:&[u8]| Attribute::VsaPropelClientIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		4 => map!{i, take!(4), |v:&[u8]| Attribute::VsaPropelClientNasIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		5 => map!{i, be_u32, |v| Attribute::VsaPropelClientSourceId(v)},
		6 => map!{i, be_u32, |v| Attribute::VsaPropelContentFilterId(v)},
        _ => value!(i, Attribute::VsaUnknown(14895, typ, i)),
    }
}
