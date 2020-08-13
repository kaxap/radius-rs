/// Definitions for vendor Nokia, vendor value 94
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaNokiaAvpair(i)),
		2 => value!(i, Attribute::VsaNokiaUserProfile(i)),
		3 => value!(i, Attribute::VsaNokiaServiceName(i)),
		4 => value!(i, Attribute::VsaNokiaServiceId(i)),
		5 => value!(i, Attribute::VsaNokiaServiceUsername(i)),
		6 => value!(i, Attribute::VsaNokiaServicePassword(i)),
		7 => value!(i, Attribute::VsaNokiaServicePrimaryIndicator(i)),
		8 => value!(i, Attribute::VsaNokiaServiceChargingType(i)),
		9 => value!(i, Attribute::VsaNokiaServiceEncryptedPassword(i)),
		10 => value!(i, Attribute::VsaNokiaSessionAccessMethod(i)),
		11 => value!(i, Attribute::VsaNokiaSessionChargingType(i)),
		12 => map!{i, be_u32, |v| Attribute::VsaNokiaOcsId1(v)},
		13 => map!{i, be_u32, |v| Attribute::VsaNokiaOcsId2(v)},
		14 => map!{i, be_u32, |v| Attribute::VsaNokiaTrecIndex(v)},
		15 => value!(i, Attribute::VsaNokiaRequestedApn(i)),
        _ => value!(i, Attribute::VsaUnknown(94, typ, i)),
    }
}
