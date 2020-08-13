/// Definitions for vendor fdXtended, vendor value 34536
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaFdxtendedBandwidthUp(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaFdxtendedBandwidthDown(v)},
		3 => value!(i, Attribute::VsaFdxtendedPostauthurl(i)),
		4 => value!(i, Attribute::VsaFdxtendedOne2OnenatIp(i)),
		5 => map!{i, be_u32, |v| Attribute::VsaFdxtendedContentfilter(v)},
		6 => map!{i, be_u32, |v| Attribute::VsaFdxtendedNetworkpolicy(v)},
		7 => map!{i, be_u32, |v| Attribute::VsaFdxtendedBytesdown(v)},
		8 => map!{i, be_u32, |v| Attribute::VsaFdxtendedBytesup(v)},
		9 => value!(i, Attribute::VsaFdxtendedExpiration(i)),
		10 => map!{i, be_u32, |v| Attribute::VsaFdxtendedSessiontimeout(v)},
		11 => value!(i, Attribute::VsaFdxtendedWanInterface(i)),
        _ => value!(i, Attribute::VsaUnknown(34536, typ, i)),
    }
}
