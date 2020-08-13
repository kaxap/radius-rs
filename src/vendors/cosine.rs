/// Definitions for vendor Cosine, vendor value 3085
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaCosineConnectionProfileName(i)),
		2 => value!(i, Attribute::VsaCosineEnterpriseId(i)),
		3 => value!(i, Attribute::VsaCosineAddressPoolName(i)),
		4 => map!{i, be_u32, |v| Attribute::VsaCosineDsByte(v)},
		5 => value!(i, Attribute::VsaCosineVpiVci(i)),
		6 => map!{i, be_u32, |v| Attribute::VsaCosineDlci(v)},
		7 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCosineLnsIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		8 => value!(i, Attribute::VsaCosineCliUserPermissionId(i)),
        _ => value!(i, Attribute::VsaUnknown(3085, typ, i)),
    }
}
