/// Definitions for vendor Extreme, vendor value 1916
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtremeCliAuthorization(pub u32);
 
#[allow(non_upper_case_globals)]
impl ExtremeCliAuthorization {
	pub const Disabled: ExtremeCliAuthorization = ExtremeCliAuthorization(0);
	pub const Enabled: ExtremeCliAuthorization = ExtremeCliAuthorization(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtremeNetloginOnly(pub u32);
 
#[allow(non_upper_case_globals)]
impl ExtremeNetloginOnly {
	pub const Disabled: ExtremeNetloginOnly = ExtremeNetloginOnly(0);
	pub const Enabled: ExtremeNetloginOnly = ExtremeNetloginOnly(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		201 => map! {i, be_u32, |v| Attribute::VsaExtremeCliAuthorization(ExtremeCliAuthorization(v))},
		202 => value!(i, Attribute::VsaExtremeShellCommand(i)),
		203 => value!(i, Attribute::VsaExtremeNetloginVlan(i)),
		204 => value!(i, Attribute::VsaExtremeNetloginUrl(i)),
		205 => value!(i, Attribute::VsaExtremeNetloginUrlDesc(i)),
		206 => map! {i, be_u32, |v| Attribute::VsaExtremeNetloginOnly(ExtremeNetloginOnly(v))},
		208 => value!(i, Attribute::VsaExtremeUserLocation(i)),
		209 => map!{i, be_u32, |v| Attribute::VsaExtremeNetloginVlanTag(v)},
		211 => value!(i, Attribute::VsaExtremeNetloginExtendedVlan(i)),
		212 => value!(i, Attribute::VsaExtremeSecurityProfile(i)),
		213 => value!(i, Attribute::VsaExtremeVmName(i)),
		214 => value!(i, Attribute::VsaExtremeVmVppName(i)),
		215 => map!{i, take!(4), |v:&[u8]| Attribute::VsaExtremeVmIpAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		216 => map!{i, be_u32, |v| Attribute::VsaExtremeVmVlanId(v)},
		217 => value!(i, Attribute::VsaExtremeVmVrName(i)),
        _ => value!(i, Attribute::VsaUnknown(1916, typ, i)),
    }
}
