/// Definitions for vendor Citrix, vendor value 66
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		10 => map!{i, be_u32, |v| Attribute::VsaCitrixUid(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaCitrixGid(v)},
		12 => value!(i, Attribute::VsaCitrixHome(i)),
		13 => value!(i, Attribute::VsaCitrixShell(i)),
		14 => value!(i, Attribute::VsaCitrixGroupNames(i)),
		15 => value!(i, Attribute::VsaCitrixGroupIds(i)),
		16 => value!(i, Attribute::VsaCitrixUserGroups(i)),
        _ => value!(i, Attribute::VsaUnknown(66, typ, i)),
    }
}
