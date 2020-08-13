/// Definitions for vendor Fortinet, vendor value 12356
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaFortinetGroupName(i)),
		2 => map!{i, take!(4), |v:&[u8]| Attribute::VsaFortinetClientIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		3 => value!(i, Attribute::VsaFortinetVdomName(i)),
		4 => value!(i, Attribute::VsaFortinetClientIpv6Address(i)),
		5 => value!(i, Attribute::VsaFortinetInterfaceName(i)),
		6 => value!(i, Attribute::VsaFortinetAccessProfile(i)),
        _ => value!(i, Attribute::VsaUnknown(12356, typ, i)),
    }
}
