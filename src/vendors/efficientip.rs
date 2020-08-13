/// Definitions for vendor EfficientIP, vendor value 2440
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaEfficientipVersion(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaEfficientipServiceClass(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaEfficientipIdentityType(v)},
		16 => value!(i, Attribute::VsaEfficientipFirstName(i)),
		17 => value!(i, Attribute::VsaEfficientipLastName(i)),
		18 => value!(i, Attribute::VsaEfficientipPseudonym(i)),
		19 => value!(i, Attribute::VsaEfficientipIpHost(i)),
		20 => value!(i, Attribute::VsaEfficientipEmail(i)),
		32 => value!(i, Attribute::VsaEfficientipFirstLoginPath(i)),
		33 => value!(i, Attribute::VsaEfficientipMaintainerGroup(i)),
		34 => value!(i, Attribute::VsaEfficientipGroups(i)),
		35 => value!(i, Attribute::VsaEfficientipAdminGroup(i)),
		64 => value!(i, Attribute::VsaEfficientipExtraBlob(i)),
        _ => value!(i, Attribute::VsaUnknown(2440, typ, i)),
    }
}
