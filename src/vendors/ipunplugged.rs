/// Definitions for vendor ipUnplugged, vendor value 5925
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		51 => map!{i, be_u32, |v| Attribute::VsaIpuMipSpi(v)},
		52 => value!(i, Attribute::VsaIpuMipKey(i)),
		53 => map!{i, be_u32, |v| Attribute::VsaIpuMipAlgType(v)},
		54 => map!{i, be_u32, |v| Attribute::VsaIpuMipAlgMode(v)},
		55 => map!{i, be_u32, |v| Attribute::VsaIpuMipReplayProt(v)},
		61 => map!{i, take!(4), |v:&[u8]| Attribute::VsaIpuIkeRemoteAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		62 => map!{i, take!(4), |v:&[u8]| Attribute::VsaIpuIkeLocalAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		63 => value!(i, Attribute::VsaIpuIkeAuth(i)),
		64 => value!(i, Attribute::VsaIpuIkeConfName(i)),
		65 => value!(i, Attribute::VsaIpuIkeCmd(i)),
        _ => value!(i, Attribute::VsaUnknown(5925, typ, i)),
    }
}
