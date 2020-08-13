/// Definitions for vendor Alteon, vendor value 1872
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlteonServiceType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AlteonServiceType {
	pub const AlteonL4Admin: AlteonServiceType = AlteonServiceType(250);
	pub const AlteonSlbadmin: AlteonServiceType = AlteonServiceType(251);
	pub const AlteonOper: AlteonServiceType = AlteonServiceType(252);
	pub const AlteonL4Oper: AlteonServiceType = AlteonServiceType(253);
	pub const AlteonSlboper: AlteonServiceType = AlteonServiceType(254);
	pub const AlteonUser: AlteonServiceType = AlteonServiceType(255);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaAlteonGroupMapping(i)),
		3 => map!{i, be_u32, |v| Attribute::VsaAlteonVpnId(v)},
		4 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAlteonClientIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		5 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAlteonClientNetmask(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		6 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAlteonPrimaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		7 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAlteonSecondaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		8 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAlteonPrimaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		9 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAlteonSecondaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		10 => value!(i, Attribute::VsaAlteonDomainName(i)),
		26 => map! {i, be_u32, |v| Attribute::VsaAlteonServiceType(AlteonServiceType(v))},
        _ => value!(i, Attribute::VsaUnknown(1872, typ, i)),
    }
}
