/// Definitions for vendor Gemtek, vendor value 10529
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		21 => map!{i, be_u32, |v| Attribute::VsaAcctSessionInputOctets(v)},
		22 => map!{i, be_u32, |v| Attribute::VsaAcctSessionInputGigawords(v)},
		23 => map!{i, be_u32, |v| Attribute::VsaAcctSessionOutputOctets(v)},
		24 => map!{i, be_u32, |v| Attribute::VsaAcctSessionOutputGigawords(v)},
		25 => map!{i, be_u32, |v| Attribute::VsaAcctSessionOctets(v)},
		26 => map!{i, be_u32, |v| Attribute::VsaAcctSessionGigawords(v)},
        _ => value!(i, Attribute::VsaUnknown(10529, typ, i)),
    }
}
