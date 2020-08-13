/// Definitions for vendor Microsemi, vendor value 40676
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaMicrosemiUserFullName(i)),
		2 => value!(i, Attribute::VsaMicrosemiUserName(i)),
		3 => value!(i, Attribute::VsaMicrosemiUserInitials(i)),
		4 => value!(i, Attribute::VsaMicrosemiUserEmail(i)),
		5 => value!(i, Attribute::VsaMicrosemiUserGroup(i)),
		6 => value!(i, Attribute::VsaMicrosemiFallbackUserGroup(i)),
		7 => value!(i, Attribute::VsaMicrosemiNetworkElementGroup(i)),
        _ => value!(i, Attribute::VsaUnknown(40676, typ, i)),
    }
}
