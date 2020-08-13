/// Definitions for vendor Yubico, vendor value 41482
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaYubikeyKey(i)),
		2 => value!(i, Attribute::VsaYubikeyPublicId(i)),
		3 => value!(i, Attribute::VsaYubikeyPrivateId(i)),
		4 => map!{i, be_u32, |v| Attribute::VsaYubikeyCounter(v)},
		5 => map!{i, be_u32, |v| Attribute::VsaYubikeyTimestamp(v)},
		6 => map!{i, be_u32, |v| Attribute::VsaYubikeyRandom(v)},
		7 => value!(i, Attribute::VsaYubikeyOtp(i)),
        _ => value!(i, Attribute::VsaUnknown(41482, typ, i)),
    }
}
