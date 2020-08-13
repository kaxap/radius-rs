/// Definitions for vendor WISPr, vendor value 14122
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaWisprLocationId(i)),
		2 => value!(i, Attribute::VsaWisprLocationName(i)),
		3 => value!(i, Attribute::VsaWisprLogoffUrl(i)),
		4 => value!(i, Attribute::VsaWisprRedirectionUrl(i)),
		5 => map!{i, be_u32, |v| Attribute::VsaWisprBandwidthMinUp(v)},
		6 => map!{i, be_u32, |v| Attribute::VsaWisprBandwidthMinDown(v)},
		7 => map!{i, be_u32, |v| Attribute::VsaWisprBandwidthMaxUp(v)},
		8 => map!{i, be_u32, |v| Attribute::VsaWisprBandwidthMaxDown(v)},
		9 => value!(i, Attribute::VsaWisprSessionTerminateTime(i)),
		10 => value!(i, Attribute::VsaWisprSessionTerminateEndOfDay(i)),
		11 => value!(i, Attribute::VsaWisprBillingClassOfService(i)),
        _ => value!(i, Attribute::VsaUnknown(14122, typ, i)),
    }
}
