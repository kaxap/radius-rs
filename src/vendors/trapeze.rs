/// Definitions for vendor Trapeze, vendor value 14525
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaTrapezeVlanName(i)),
		2 => value!(i, Attribute::VsaTrapezeMobilityProfile(i)),
		3 => value!(i, Attribute::VsaTrapezeEncryptionType(i)),
		4 => value!(i, Attribute::VsaTrapezeTimeOfDay(i)),
		5 => value!(i, Attribute::VsaTrapezeSsid(i)),
		6 => value!(i, Attribute::VsaTrapezeEndDate(i)),
		7 => value!(i, Attribute::VsaTrapezeStartDate(i)),
		8 => value!(i, Attribute::VsaTrapezeUrl(i)),
		9 => value!(i, Attribute::VsaTrapezeUserGroupName(i)),
		10 => value!(i, Attribute::VsaTrapezeQosProfile(i)),
		11 => value!(i, Attribute::VsaTrapezeSimultaneousLogins(i)),
		12 => value!(i, Attribute::VsaTrapezeCoaUsername(i)),
		13 => value!(i, Attribute::VsaTrapezeAudit(i)),
		14 => value!(i, Attribute::VsaTrapezeNmsUserGroup(i)),
		15 => value!(i, Attribute::VsaTrapezeNmsPlatformLocalUser(i)),
		16 => value!(i, Attribute::VsaTrapezeSipCallDetailRecord(i)),
		17 => value!(i, Attribute::VsaTrapezeSmartpassAccessControl(i)),
		18 => value!(i, Attribute::VsaTrapezeDeviceProfile(i)),
		19 => value!(i, Attribute::VsaTrapezeDeviceType(i)),
		20 => value!(i, Attribute::VsaTrapezeAllowedDevices(i)),
		21 => value!(i, Attribute::VsaTrapezeDeviceGroup(i)),
        _ => value!(i, Attribute::VsaUnknown(14525, typ, i)),
    }
}
