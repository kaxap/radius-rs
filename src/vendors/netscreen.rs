/// Definitions for vendor Netscreen, vendor value 3224
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NsAdminPrivilege(pub u32);
 
#[allow(non_upper_case_globals)]
impl NsAdminPrivilege {
	pub const RootAdmin: NsAdminPrivilege = NsAdminPrivilege(1);
	pub const AllVsysRootAdmin: NsAdminPrivilege = NsAdminPrivilege(2);
	pub const VsysAdmin: NsAdminPrivilege = NsAdminPrivilege(3);
	pub const ReadOnlyAdmin: NsAdminPrivilege = NsAdminPrivilege(4);
	pub const ReadOnlyVsysAdmin: NsAdminPrivilege = NsAdminPrivilege(5);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaNsAdminPrivilege(NsAdminPrivilege(v))},
		2 => value!(i, Attribute::VsaNsVsysName(i)),
		3 => value!(i, Attribute::VsaNsUserGroup(i)),
		4 => map!{i, take!(4), |v:&[u8]| Attribute::VsaNsPrimaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		5 => map!{i, take!(4), |v:&[u8]| Attribute::VsaNsSecondaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		6 => map!{i, take!(4), |v:&[u8]| Attribute::VsaNsPrimaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		7 => map!{i, take!(4), |v:&[u8]| Attribute::VsaNsSecondaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		220 => value!(i, Attribute::VsaNsNsmUserDomainName(i)),
		221 => value!(i, Attribute::VsaNsNsmUserRoleMapping(i)),
        _ => value!(i, Attribute::VsaUnknown(3224, typ, i)),
    }
}
