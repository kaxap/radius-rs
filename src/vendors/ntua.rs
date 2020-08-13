/// Definitions for vendor NTUA, vendor value 969
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserlogonType(pub u32);
 
#[allow(non_upper_case_globals)]
impl UserlogonType {
	pub const Ftp: UserlogonType = UserlogonType(1);
	pub const Web: UserlogonType = UserlogonType(2);
	pub const Pop: UserlogonType = UserlogonType(3);
	pub const Imap: UserlogonType = UserlogonType(4);
	pub const WindowsLogon: UserlogonType = UserlogonType(5);
	pub const UnixLogon: UserlogonType = UserlogonType(6);
	pub const SmtpAuth: UserlogonType = UserlogonType(7);
	pub const Other: UserlogonType = UserlogonType(200);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserlogonRestriction(pub u32);
 
#[allow(non_upper_case_globals)]
impl UserlogonRestriction {
	pub const AnonymousUser: UserlogonRestriction = UserlogonRestriction(1);
	pub const AdminUser: UserlogonRestriction = UserlogonRestriction(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		10 => map!{i, be_u32, |v| Attribute::VsaUserlogonUid(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaUserlogonGid(v)},
		12 => value!(i, Attribute::VsaUserlogonHomedir(i)),
		13 => map! {i, be_u32, |v| Attribute::VsaUserlogonType(UserlogonType(v))},
		14 => map!{i, be_u32, |v| Attribute::VsaUserlogonQuotabytes(v)},
		15 => map!{i, be_u32, |v| Attribute::VsaUserlogonQuotafiles(v)},
		16 => value!(i, Attribute::VsaUserlogonShell(i)),
		17 => map! {i, be_u32, |v| Attribute::VsaUserlogonRestriction(UserlogonRestriction(v))},
		18 => value!(i, Attribute::VsaUserlogonGroupnames(i)),
		19 => value!(i, Attribute::VsaUserlogonDrivenames(i)),
		20 => value!(i, Attribute::VsaUserlogonUserdescription(i)),
		21 => value!(i, Attribute::VsaUserlogonUserfullname(i)),
		22 => value!(i, Attribute::VsaUserlogonUserdomain(i)),
		23 => value!(i, Attribute::VsaUserlogonLogontask(i)),
		24 => value!(i, Attribute::VsaUserlogonLogofftask(i)),
		25 => value!(i, Attribute::VsaUserlogonExpiration(i)),
		26 => value!(i, Attribute::VsaUserlogonUserprofile(i)),
		50 => value!(i, Attribute::VsaUserlogonAcctTerminatecause(i)),
        _ => value!(i, Attribute::VsaUnknown(969, typ, i)),
    }
}
