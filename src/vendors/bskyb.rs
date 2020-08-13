/// Definitions for vendor BSkyB, vendor value 16924
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaSkyWifiApId(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaSkyWifiServiceId(v)},
		3 => value!(i, Attribute::VsaSkyWifiFilterProfile(i)),
		4 => value!(i, Attribute::VsaSkyWifiBillingClass(i)),
		5 => map!{i, be_u32, |v| Attribute::VsaSkyWifiProviderId(v)},
		6 => value!(i, Attribute::VsaSkyWifiCredentials(i)),
        _ => value!(i, Attribute::VsaUnknown(16924, typ, i)),
    }
}
