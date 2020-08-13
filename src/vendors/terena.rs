/// Definitions for vendor TERENA, vendor value 25178
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		10 => value!(i, Attribute::VsaEduroamSpCountry(i)),
		11 => value!(i, Attribute::VsaEduroamMonitoringInflate(i)),
        _ => value!(i, Attribute::VsaUnknown(25178, typ, i)),
    }
}
