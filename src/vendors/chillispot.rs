/// Definitions for vendor ChilliSpot, vendor value 14559
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ServiceType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ServiceType {
	pub const ChillispotAuthorizeOnly: ServiceType = ServiceType(0x38df0001);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaChillispotMaxInputOctets(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaChillispotMaxOutputOctets(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaChillispotMaxTotalOctets(v)},
		4 => map!{i, be_u32, |v| Attribute::VsaChillispotBandwidthMaxUp(v)},
		5 => map!{i, be_u32, |v| Attribute::VsaChillispotBandwidthMaxDown(v)},
		6 => value!(i, Attribute::VsaChillispotConfig(i)),
		7 => value!(i, Attribute::VsaChillispotLang(i)),
		8 => value!(i, Attribute::VsaChillispotVersion(i)),
		9 => value!(i, Attribute::VsaChillispotOriginalurl(i)),
		100 => value!(i, Attribute::VsaChillispotUamAllowed(i)),
		101 => value!(i, Attribute::VsaChillispotMacAllowed(i)),
		102 => map!{i, be_u32, |v| Attribute::VsaChillispotInterval(v)},
        _ => value!(i, Attribute::VsaUnknown(14559, typ, i)),
    }
}
