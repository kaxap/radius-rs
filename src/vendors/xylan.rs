/// Definitions for vendor Xylan, vendor value 800
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XylanAccessPriv(pub u32);
 
#[allow(non_upper_case_globals)]
impl XylanAccessPriv {
	pub const XylanReadPriv: XylanAccessPriv = XylanAccessPriv(1);
	pub const XylanWritePriv: XylanAccessPriv = XylanAccessPriv(2);
	pub const XylanAdminPriv: XylanAccessPriv = XylanAccessPriv(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaXylanAuthGroup(v)},
		2 => value!(i, Attribute::VsaXylanSlotPort(i)),
		3 => value!(i, Attribute::VsaXylanTimeOfDay(i)),
		4 => map!{i, take!(4), |v:&[u8]| Attribute::VsaXylanClientIpAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		5 => value!(i, Attribute::VsaXylanGroupDesc(i)),
		6 => value!(i, Attribute::VsaXylanPortDesc(i)),
		7 => map!{i, be_u32, |v| Attribute::VsaXylanProfilNumb(v)},
		8 => value!(i, Attribute::VsaXylanAuthGroupProtocol(i)),
		9 => value!(i, Attribute::VsaXylanAsaAccess(i)),
		10 => map!{i, be_u32, |v| Attribute::VsaXylanEndUserProfile(v)},
		16 => map! {i, be_u32, |v| Attribute::VsaXylanAccessPriv(XylanAccessPriv(v))},
		20 => value!(i, Attribute::VsaXylanNmsGroup(i)),
		21 => value!(i, Attribute::VsaXylanNmsFirstName(i)),
		22 => value!(i, Attribute::VsaXylanNmsLastName(i)),
		23 => value!(i, Attribute::VsaXylanNmsDescription(i)),
		33 => value!(i, Attribute::VsaXylanAccePrivR1(i)),
		34 => value!(i, Attribute::VsaXylanAccePrivR2(i)),
		35 => value!(i, Attribute::VsaXylanAccePrivW1(i)),
		36 => value!(i, Attribute::VsaXylanAccePrivW2(i)),
		37 => value!(i, Attribute::VsaXylanAccePrivG1(i)),
		38 => value!(i, Attribute::VsaXylanAccePrivG2(i)),
		39 => value!(i, Attribute::VsaXylanAccePrivFR1(i)),
		40 => value!(i, Attribute::VsaXylanAccePrivFR2(i)),
		41 => value!(i, Attribute::VsaXylanAccePrivFW1(i)),
		42 => value!(i, Attribute::VsaXylanAccePrivFW2(i)),
        _ => value!(i, Attribute::VsaUnknown(800, typ, i)),
    }
}
