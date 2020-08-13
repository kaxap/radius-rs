/// Definitions for vendor Prosoft, vendor value 4735
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProsoftAuthRole(pub u32);
 
#[allow(non_upper_case_globals)]
impl ProsoftAuthRole {
	pub const ReadStatus: ProsoftAuthRole = ProsoftAuthRole(0);
	pub const ReadConfig: ProsoftAuthRole = ProsoftAuthRole(1);
	pub const ReadWrite: ProsoftAuthRole = ProsoftAuthRole(2);
	pub const Admin: ProsoftAuthRole = ProsoftAuthRole(3);
	pub const SuperUser: ProsoftAuthRole = ProsoftAuthRole(4);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		0 => map!{i, take!(4), |v:&[u8]| Attribute::VsaProsoftHomeAgentAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		1 => map!{i, take!(4), |v:&[u8]| Attribute::VsaProsoftDefaultGateway(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		2 => map!{i, take!(4), |v:&[u8]| Attribute::VsaProsoftPrimaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		3 => map!{i, take!(4), |v:&[u8]| Attribute::VsaProsoftSecondaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		4 => map!{i, be_u32, |v| Attribute::VsaProsoftSecurityParameterIndex(v)},
		5 => value!(i, Attribute::VsaProsoftSecurityKey(i)),
		7 => value!(i, Attribute::VsaProsoftMacAddress(i)),
		8 => map!{i, be_u32, |v| Attribute::VsaProsoftAuthenticationReason(v)},
		9 => map!{i, be_u32, |v| Attribute::VsaProsoftAtmInterface(v)},
		10 => map!{i, be_u32, |v| Attribute::VsaProsoftAtmVpi(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaProsoftAtmVci(v)},
		12 => value!(i, Attribute::VsaProsoftRscIdentifier(i)),
		13 => value!(i, Attribute::VsaProsoftNpmIdentifier(i)),
		14 => value!(i, Attribute::VsaProsoftNpmIp(i)),
		15 => value!(i, Attribute::VsaProsoftSectorId(i)),
		16 => map! {i, be_u32, |v| Attribute::VsaProsoftAuthRole(ProsoftAuthRole(v))},
        _ => value!(i, Attribute::VsaUnknown(4735, typ, i)),
    }
}
