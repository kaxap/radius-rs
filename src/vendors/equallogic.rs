/// Definitions for vendor Equallogic, vendor value 12740
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EquallogicEqlAdminPrivilege(pub u32);
 
#[allow(non_upper_case_globals)]
impl EquallogicEqlAdminPrivilege {
	pub const GroupAdministrator: EquallogicEqlAdminPrivilege = EquallogicEqlAdminPrivilege(0);
	pub const PoolAdministrator: EquallogicEqlAdminPrivilege = EquallogicEqlAdminPrivilege(1);
	pub const PoolAdministratorRoEntireGroup: EquallogicEqlAdminPrivilege = EquallogicEqlAdminPrivilege(2);
	pub const VolumeAdministrator: EquallogicEqlAdminPrivilege = EquallogicEqlAdminPrivilege(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaEquallogicAdminFullName(i)),
		2 => value!(i, Attribute::VsaEquallogicAdminEmail(i)),
		3 => value!(i, Attribute::VsaEquallogicAdminPhone(i)),
		4 => value!(i, Attribute::VsaEquallogicAdminMobile(i)),
		5 => map!{i, be_u32, |v| Attribute::VsaEquallogicPollInterval(v)},
		6 => map! {i, be_u32, |v| Attribute::VsaEquallogicEqlAdminPrivilege(EquallogicEqlAdminPrivilege(v))},
		7 => value!(i, Attribute::VsaEquallogicAdminPoolAccess(i)),
		8 => value!(i, Attribute::VsaEquallogicAdminReplSiteAccess(i)),
		9 => value!(i, Attribute::VsaEquallogicAdminAccountType(i)),
        _ => value!(i, Attribute::VsaUnknown(12740, typ, i)),
    }
}
