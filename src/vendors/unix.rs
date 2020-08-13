/// Definitions for vendor Unix, vendor value 4
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		10 => map!{i, be_u32, |v| Attribute::VsaUnixFtpUid(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaUnixFtpGid(v)},
		12 => value!(i, Attribute::VsaUnixFtpHome(i)),
		13 => value!(i, Attribute::VsaUnixFtpShell(i)),
		14 => value!(i, Attribute::VsaUnixFtpGroupNames(i)),
		15 => value!(i, Attribute::VsaUnixFtpGroupIds(i)),
        _ => value!(i, Attribute::VsaUnknown(4, typ, i)),
    }
}
