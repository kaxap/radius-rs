/// Definitions for vendor Livingston, vendor value 307
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeIpsecDenyAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl LeIpsecDenyAction {
	pub const Drop: LeIpsecDenyAction = LeIpsecDenyAction(1);
	pub const IcmpReject: LeIpsecDenyAction = LeIpsecDenyAction(2);
	pub const PassThrough: LeIpsecDenyAction = LeIpsecDenyAction(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeIpsecLogOptions(pub u32);
 
#[allow(non_upper_case_globals)]
impl LeIpsecLogOptions {
	pub const SaSuccessOn: LeIpsecLogOptions = LeIpsecLogOptions(1);
	pub const SaFailureOn: LeIpsecLogOptions = LeIpsecLogOptions(2);
	pub const ConsoleOn: LeIpsecLogOptions = LeIpsecLogOptions(3);
	pub const SyslogOn: LeIpsecLogOptions = LeIpsecLogOptions(4);
	pub const SaSuccessOff: LeIpsecLogOptions = LeIpsecLogOptions(5);
	pub const SaFailureOff: LeIpsecLogOptions = LeIpsecLogOptions(6);
	pub const ConsoleOff: LeIpsecLogOptions = LeIpsecLogOptions(7);
	pub const SyslogOff: LeIpsecLogOptions = LeIpsecLogOptions(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeNatSessDirFailAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl LeNatSessDirFailAction {
	pub const Drop: LeNatSessDirFailAction = LeNatSessDirFailAction(1);
	pub const IcmpReject: LeNatSessDirFailAction = LeNatSessDirFailAction(2);
	pub const PassThrough: LeNatSessDirFailAction = LeNatSessDirFailAction(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeNatLogOptions(pub u32);
 
#[allow(non_upper_case_globals)]
impl LeNatLogOptions {
	pub const SessionSuccessOn: LeNatLogOptions = LeNatLogOptions(1);
	pub const SessionFailureOn: LeNatLogOptions = LeNatLogOptions(2);
	pub const ConsoleOn: LeNatLogOptions = LeNatLogOptions(3);
	pub const SyslogOn: LeNatLogOptions = LeNatLogOptions(4);
	pub const SuccessOff: LeNatLogOptions = LeNatLogOptions(5);
	pub const FailureOff: LeNatLogOptions = LeNatLogOptions(6);
	pub const ConsoleOff: LeNatLogOptions = LeNatLogOptions(7);
	pub const SyslogOff: LeNatLogOptions = LeNatLogOptions(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeMulticastClient(pub u32);
 
#[allow(non_upper_case_globals)]
impl LeMulticastClient {
	pub const On: LeMulticastClient = LeMulticastClient(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		2 => value!(i, Attribute::VsaLeTerminateDetail(i)),
		3 => value!(i, Attribute::VsaLeAdviceOfCharge(i)),
		4 => value!(i, Attribute::VsaLeConnectDetail(i)),
		6 => value!(i, Attribute::VsaLeIpPool(i)),
		7 => map!{i, take!(4), |v:&[u8]| Attribute::VsaLeIpGateway(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		8 => value!(i, Attribute::VsaLeModemInfo(i)),
		9 => map! {i, be_u32, |v| Attribute::VsaLeIpsecLogOptions(LeIpsecLogOptions(v))},
		10 => map! {i, be_u32, |v| Attribute::VsaLeIpsecDenyAction(LeIpsecDenyAction(v))},
		11 => value!(i, Attribute::VsaLeIpsecActiveProfile(i)),
		12 => value!(i, Attribute::VsaLeIpsecOutsourceProfile(i)),
		13 => value!(i, Attribute::VsaLeIpsecPassiveProfile(i)),
		14 => map!{i, be_u32, |v| Attribute::VsaLeNatTcpSessionTimeout(v)},
		15 => map!{i, be_u32, |v| Attribute::VsaLeNatOtherSessionTimeout(v)},
		16 => map! {i, be_u32, |v| Attribute::VsaLeNatLogOptions(LeNatLogOptions(v))},
		17 => map! {i, be_u32, |v| Attribute::VsaLeNatSessDirFailAction(LeNatSessDirFailAction(v))},
		18 => value!(i, Attribute::VsaLeNatInmap(i)),
		19 => value!(i, Attribute::VsaLeNatOutmap(i)),
		20 => value!(i, Attribute::VsaLeNatOutsourceInmap(i)),
		21 => value!(i, Attribute::VsaLeNatOutsourceOutmap(i)),
		22 => value!(i, Attribute::VsaLeAdminGroup(i)),
		23 => map! {i, be_u32, |v| Attribute::VsaLeMulticastClient(LeMulticastClient(v))},
        _ => value!(i, Attribute::VsaUnknown(307, typ, i)),
    }
}
