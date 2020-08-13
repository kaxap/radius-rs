/// Definitions for vendor Roaring-Penguin, vendor value 10055
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaRpUpstreamSpeedLimit(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaRpDownstreamSpeedLimit(v)},
		3 => value!(i, Attribute::VsaRpHurl(i)),
		4 => value!(i, Attribute::VsaRpMotm(i)),
		5 => map!{i, be_u32, |v| Attribute::VsaRpMaxSessionsPerUser(v)},
        _ => value!(i, Attribute::VsaUnknown(10055, typ, i)),
    }
}
