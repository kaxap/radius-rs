/// Definitions for vendor Compatible, vendor value 255
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		0 => map!{i, be_u32, |v| Attribute::VsaCompatibleTunnelDelay(v)},
		1 => map!{i, be_u32, |v| Attribute::VsaCompatibleTunnelThroughput(v)},
		3 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCompatibleTunnelServerEndpoint(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		4 => value!(i, Attribute::VsaCompatibleTunnelGroupInfo(i)),
		5 => value!(i, Attribute::VsaCompatibleTunnelPassword(i)),
		6 => map!{i, be_u32, |v| Attribute::VsaCompatibleEcho(v)},
		7 => map!{i, be_u32, |v| Attribute::VsaCompatibleTunnelIpx(v)},
        _ => value!(i, Attribute::VsaUnknown(255, typ, i)),
    }
}
