/// Definitions for vendor T-Systems-Nova, vendor value 16787
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaTSystemsNovaLocationId(i)),
		2 => value!(i, Attribute::VsaTSystemsNovaLocationName(i)),
		3 => value!(i, Attribute::VsaTSystemsNovaLogoffUrl(i)),
		4 => value!(i, Attribute::VsaTSystemsNovaRedirectionUrl(i)),
		5 => map!{i, be_u32, |v| Attribute::VsaTSystemsNovaBandwidthMinUp(v)},
		6 => map!{i, be_u32, |v| Attribute::VsaTSystemsNovaBandwidthMinDown(v)},
		7 => map!{i, be_u32, |v| Attribute::VsaTSystemsNovaBandwidthMaxUp(v)},
		8 => map!{i, be_u32, |v| Attribute::VsaTSystemsNovaBandwidthMaxDown(v)},
		9 => map!{i, be_u32, |v| Attribute::VsaTSystemsNovaSessionTerminateTime(v)},
		10 => map!{i, be_u32, |v| Attribute::VsaTSystemsNovaSessionTerminateEod(v)},
		11 => value!(i, Attribute::VsaTSystemsNovaBillingClassOfService(i)),
		12 => value!(i, Attribute::VsaTSystemsNovaServiceName(i)),
		13 => map!{i, be_u32, |v| Attribute::VsaTSystemsNovaPriceOfService(v)},
		14 => value!(i, Attribute::VsaTSystemsNovaVisitingProviderCode(i)),
		15 => value!(i, Attribute::VsaTSystemsNovaUnknownavp(i)),
        _ => value!(i, Attribute::VsaUnknown(16787, typ, i)),
    }
}
