/// Definitions for vendor H3C, vendor value 25506
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct H3CCommand(pub u32);
 
#[allow(non_upper_case_globals)]
impl H3CCommand {
	pub const TriggerRequest: H3CCommand = H3CCommand(1);
	pub const TerminateRequest: H3CCommand = H3CCommand(2);
	pub const Setpolicy: H3CCommand = H3CCommand(3);
	pub const Result: H3CCommand = H3CCommand(4);
	pub const Portalclear: H3CCommand = H3CCommand(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct H3CExecPrivilege(pub u32);
 
#[allow(non_upper_case_globals)]
impl H3CExecPrivilege {
	pub const Visit: H3CExecPrivilege = H3CExecPrivilege(0);
	pub const Monitor: H3CExecPrivilege = H3CExecPrivilege(1);
	pub const System: H3CExecPrivilege = H3CExecPrivilege(2);
	pub const Manage: H3CExecPrivilege = H3CExecPrivilege(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaH3CInputPeakRate(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaH3CInputAverageRate(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaH3CInputBasicRate(v)},
		15 => map!{i, be_u32, |v| Attribute::VsaH3CRemanentVolume(v)},
		20 => map! {i, be_u32, |v| Attribute::VsaH3CCommand(H3CCommand(v))},
		24 => map!{i, be_u32, |v| Attribute::VsaH3CControlIdentifier(v)},
		25 => map!{i, be_u32, |v| Attribute::VsaH3CResultCode(v)},
		26 => map!{i, be_u32, |v| Attribute::VsaH3CConnectId(v)},
		28 => value!(i, Attribute::VsaH3CFtpDirectory(i)),
		29 => map! {i, be_u32, |v| Attribute::VsaH3CExecPrivilege(H3CExecPrivilege(v))},
		59 => map!{i, be_u32, |v| Attribute::VsaH3CNasStartupTimestamp(v)},
		60 => value!(i, Attribute::VsaH3CIpHostAddr(i)),
		61 => value!(i, Attribute::VsaH3CUserNotify(i)),
		62 => value!(i, Attribute::VsaH3CUserHeartbeat(i)),
		140 => value!(i, Attribute::VsaH3CUserGroup(i)),
		141 => map!{i, be_u32, |v| Attribute::VsaH3CSecurityLevel(v)},
		201 => map!{i, be_u32, |v| Attribute::VsaH3CInputIntervalOctets(v)},
		202 => map!{i, be_u32, |v| Attribute::VsaH3COutputIntervalOctets(v)},
		203 => map!{i, be_u32, |v| Attribute::VsaH3CInputIntervalPackets(v)},
		204 => map!{i, be_u32, |v| Attribute::VsaH3COutputIntervalPackets(v)},
		205 => map!{i, be_u32, |v| Attribute::VsaH3CInputIntervalGigawords(v)},
		206 => map!{i, be_u32, |v| Attribute::VsaH3COutputIntervalGigawords(v)},
		207 => map!{i, take!(4), |v:&[u8]| Attribute::VsaH3CBackupNasIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		255 => value!(i, Attribute::VsaH3CProductId(i)),
        _ => value!(i, Attribute::VsaUnknown(25506, typ, i)),
    }
}
