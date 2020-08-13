/// Definitions for vendor Merit, vendor value 61
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		211 => value!(i, Attribute::VsaMeritProxyAction(i)),
		222 => value!(i, Attribute::VsaMeritUserId(i)),
		223 => value!(i, Attribute::VsaMeritUserRealm(i)),
        _ => value!(i, Attribute::VsaUnknown(61, typ, i)),
    }
}
