/// Definitions for vendor RedCreek, vendor value 1958
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		5 => map!{i, take!(4), |v:&[u8]| Attribute::VsaRedcreekTunneledIpAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		6 => map!{i, take!(4), |v:&[u8]| Attribute::VsaRedcreekTunneledIpNetmask(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		7 => map!{i, take!(4), |v:&[u8]| Attribute::VsaRedcreekTunneledGateway(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		8 => value!(i, Attribute::VsaRedcreekTunneledDnsServer(i)),
		9 => value!(i, Attribute::VsaRedcreekTunneledWinsServer1(i)),
		10 => value!(i, Attribute::VsaRedcreekTunneledWinsServer2(i)),
		11 => value!(i, Attribute::VsaRedcreekTunneledHostname(i)),
		12 => value!(i, Attribute::VsaRedcreekTunneledDomainname(i)),
		13 => value!(i, Attribute::VsaRedcreekTunneledSearchList(i)),
        _ => value!(i, Attribute::VsaUnknown(1958, typ, i)),
    }
}
