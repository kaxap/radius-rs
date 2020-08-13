/// Definitions for vendor Alcatel-Lucent-Service-Router, vendor value 6527
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimetraAccess(pub u32);
 
#[allow(non_upper_case_globals)]
impl TimetraAccess {
	pub const Ftp: TimetraAccess = TimetraAccess(1);
	pub const Console: TimetraAccess = TimetraAccess(2);
	pub const Both: TimetraAccess = TimetraAccess(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimetraRestrictToHome(pub u32);
 
#[allow(non_upper_case_globals)]
impl TimetraRestrictToHome {
	pub const True: TimetraRestrictToHome = TimetraRestrictToHome(1);
	pub const False: TimetraRestrictToHome = TimetraRestrictToHome(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimetraDefaultAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl TimetraDefaultAction {
	pub const PermitAll: TimetraDefaultAction = TimetraDefaultAction(1);
	pub const DenyAll: TimetraDefaultAction = TimetraDefaultAction(2);
	pub const None: TimetraDefaultAction = TimetraDefaultAction(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimetraAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl TimetraAction {
	pub const Permit: TimetraAction = TimetraAction(1);
	pub const Deny: TimetraAction = TimetraAction(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaTimetraAccess(TimetraAccess(v))},
		2 => value!(i, Attribute::VsaTimetraHomeDirectory(i)),
		3 => map! {i, be_u32, |v| Attribute::VsaTimetraRestrictToHome(TimetraRestrictToHome(v))},
		4 => value!(i, Attribute::VsaTimetraProfile(i)),
		5 => map! {i, be_u32, |v| Attribute::VsaTimetraDefaultAction(TimetraDefaultAction(v))},
		6 => value!(i, Attribute::VsaTimetraCmd(i)),
		7 => map! {i, be_u32, |v| Attribute::VsaTimetraAction(TimetraAction(v))},
		8 => value!(i, Attribute::VsaTimetraExecFile(i)),
		9 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAlcPrimaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		10 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAlcSecondaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		11 => value!(i, Attribute::VsaAlcSubscIdStr(i)),
		12 => value!(i, Attribute::VsaAlcSubscProfStr(i)),
		13 => value!(i, Attribute::VsaAlcSlaProfStr(i)),
		14 => value!(i, Attribute::VsaAlcForceRenew(i)),
		15 => value!(i, Attribute::VsaAlcCreateHost(i)),
		16 => value!(i, Attribute::VsaAlcAncpStr(i)),
		17 => map!{i, be_u32, |v| Attribute::VsaAlcRetailServId(v)},
		18 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAlcDefaultRouter(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		27 => value!(i, Attribute::VsaAlcClientHardwareAddr(i)),
		19 => map!{i, be_u32, |v| Attribute::VsaAlcAcctIInprofOctets64(v)},
		20 => map!{i, be_u32, |v| Attribute::VsaAlcAcctIOutprofOctets64(v)},
		21 => map!{i, be_u32, |v| Attribute::VsaAlcAcctOInprofOctets64(v)},
		22 => map!{i, be_u32, |v| Attribute::VsaAlcAcctOOutprofOctets64(v)},
		23 => map!{i, be_u32, |v| Attribute::VsaAlcAcctIInprofPkts64(v)},
		24 => map!{i, be_u32, |v| Attribute::VsaAlcAcctIOutprofPkts64(v)},
		25 => map!{i, be_u32, |v| Attribute::VsaAlcAcctOInprofPkts64(v)},
		26 => map!{i, be_u32, |v| Attribute::VsaAlcAcctOOutprofPkts64(v)},
		172 => value!(i, Attribute::VsaAlcWlanPortalRedirect(i)),
		173 => value!(i, Attribute::VsaAlcWlanPortalUrl(i)),
		174 => map!{i, be_u32, |v| Attribute::VsaAlcLeaseTime(v)},
		177 => value!(i, Attribute::VsaAlcPortalUrl(i)),
		181 => value!(i, Attribute::VsaAlcSlaacIpv6Pool(i)),
        _ => value!(i, Attribute::VsaUnknown(6527, typ, i)),
    }
}
