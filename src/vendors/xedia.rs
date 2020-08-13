/// Definitions for vendor Xedia, vendor value 838
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, take!(4), |v:&[u8]| Attribute::VsaXediaDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		2 => map!{i, take!(4), |v:&[u8]| Attribute::VsaXediaNetbiosServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		3 => value!(i, Attribute::VsaXediaAddressPool(i)),
		4 => map!{i, be_u32, |v| Attribute::VsaXediaPppEchoInterval(v)},
		5 => map!{i, be_u32, |v| Attribute::VsaXediaSshPrivileges(v)},
		6 => value!(i, Attribute::VsaXediaClientAccessNetwork(i)),
		7 => map!{i, be_u32, |v| Attribute::VsaXediaClientFirewallSetting(v)},
		8 => map!{i, be_u32, |v| Attribute::VsaXediaSavePassword(v)},
        _ => value!(i, Attribute::VsaUnknown(838, typ, i)),
    }
}
