/// Definitions for vendor APC, vendor value 318
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ApcServiceType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ApcServiceType {
	pub const Admin: ApcServiceType = ApcServiceType(1);
	pub const Device: ApcServiceType = ApcServiceType(2);
	pub const Readonly: ApcServiceType = ApcServiceType(3);
	pub const Outlet: ApcServiceType = ApcServiceType(4);
	pub const Card: ApcServiceType = ApcServiceType(5);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaApcServiceType(ApcServiceType(v))},
		2 => value!(i, Attribute::VsaApcOutlets(i)),
		3 => value!(i, Attribute::VsaApcPerms(i)),
		4 => value!(i, Attribute::VsaApcUsername(i)),
		5 => value!(i, Attribute::VsaApcContact(i)),
		6 => value!(i, Attribute::VsaApcAccpxDoors(i)),
		7 => value!(i, Attribute::VsaApcAccpxStatus(i)),
		8 => value!(i, Attribute::VsaApcAccpxAccess1(i)),
		9 => value!(i, Attribute::VsaApcAccpxAccess2(i)),
		10 => value!(i, Attribute::VsaApcAccpxAccess3(i)),
		11 => value!(i, Attribute::VsaApcAccpxAccess4(i)),
		12 => value!(i, Attribute::VsaApcAccpxAccess5(i)),
		13 => value!(i, Attribute::VsaApcAccpxAccess6(i)),
		14 => value!(i, Attribute::VsaApcAccpxAccess7(i)),
        _ => value!(i, Attribute::VsaUnknown(318, typ, i)),
    }
}
