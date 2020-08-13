/// Definitions for vendor Walabi, vendor value 2004
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaWbAuthTimeLeft(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaWbAuthAccumBw(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaWbAuthBwQuota(v)},
		4 => map!{i, be_u32, |v| Attribute::VsaWbAuthBwCount(v)},
		5 => map!{i, be_u32, |v| Attribute::VsaWbAuthUploadLimit(v)},
		6 => map!{i, be_u32, |v| Attribute::VsaWbAuthDownloadLimit(v)},
		7 => map!{i, be_u32, |v| Attribute::VsaWbAuthLoginTime(v)},
		8 => map!{i, be_u32, |v| Attribute::VsaWbAuthLogoutTime(v)},
		9 => map!{i, be_u32, |v| Attribute::VsaWbAuthTimeDiff(v)},
		10 => map!{i, be_u32, |v| Attribute::VsaWbAuthBwUsage(v)},
        _ => value!(i, Attribute::VsaUnknown(2004, typ, i)),
    }
}
