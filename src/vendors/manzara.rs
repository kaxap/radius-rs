/// Definitions for vendor Manzara, vendor value 19382
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ManzaraTariffType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ManzaraTariffType {
	pub const MmsPicture: ManzaraTariffType = ManzaraTariffType(1);
	pub const Unused: ManzaraTariffType = ManzaraTariffType(2);
	pub const Internet: ManzaraTariffType = ManzaraTariffType(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AcctStatusType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AcctStatusType {
	pub const OneTime: AcctStatusType = AcctStatusType(17);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaManzaraUserUid(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaManzaraUserGid(v)},
		3 => value!(i, Attribute::VsaManzaraUserHome(i)),
		4 => value!(i, Attribute::VsaManzaraUserShell(i)),
		5 => value!(i, Attribute::VsaManzaraPppAddrString(i)),
		6 => value!(i, Attribute::VsaManzaraFullLoginString(i)),
		7 => map!{i, be_u32, |v| Attribute::VsaManzaraTariffUnits(v)},
		8 => map! {i, be_u32, |v| Attribute::VsaManzaraTariffType(ManzaraTariffType(v))},
		9 => value!(i, Attribute::VsaManzaraEcpSessionKey(i)),
		10 => value!(i, Attribute::VsaManzaraMapName(i)),
		11 => value!(i, Attribute::VsaManzaraMapKey(i)),
		12 => value!(i, Attribute::VsaManzaraMapValue(i)),
		13 => value!(i, Attribute::VsaManzaraMapError(i)),
		14 => value!(i, Attribute::VsaManzaraServiceType(i)),
        _ => value!(i, Attribute::VsaUnknown(19382, typ, i)),
    }
}
