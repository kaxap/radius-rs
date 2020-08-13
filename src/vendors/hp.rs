/// Definitions for vendor HP, vendor value 11
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HpPortAuthModeDot1X(pub u32);
 
#[allow(non_upper_case_globals)]
impl HpPortAuthModeDot1X {
	pub const PortBased: HpPortAuthModeDot1X = HpPortAuthModeDot1X(1);
	pub const UserBased: HpPortAuthModeDot1X = HpPortAuthModeDot1X(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HpCommandException(pub u32);
 
#[allow(non_upper_case_globals)]
impl HpCommandException {
	pub const PermitList: HpCommandException = HpCommandException(0);
	pub const DenyList: HpCommandException = HpCommandException(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HpManagementProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl HpManagementProtocol {
	pub const Http: HpManagementProtocol = HpManagementProtocol(5);
	pub const Https: HpManagementProtocol = HpManagementProtocol(6);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ServiceType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ServiceType {
	pub const HpOper: ServiceType = ServiceType(252);
	pub const HpUser: ServiceType = ServiceType(255);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaHpPrivilegeLevel(v)},
		2 => value!(i, Attribute::VsaHpCommandString(i)),
		3 => map! {i, be_u32, |v| Attribute::VsaHpCommandException(HpCommandException(v))},
		26 => map! {i, be_u32, |v| Attribute::VsaHpManagementProtocol(HpManagementProtocol(v))},
		10 => map!{i, be_u32, |v| Attribute::VsaHpPortClientLimitDot1X(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaHpPortClientLimitMa(v)},
		12 => map!{i, be_u32, |v| Attribute::VsaHpPortClientLimitWa(v)},
		13 => map! {i, be_u32, |v| Attribute::VsaHpPortAuthModeDot1X(HpPortAuthModeDot1X(v))},
		23 => map!{i, be_u32, |v| Attribute::VsaHpPortBounceHost(v)},
		24 => value!(i, Attribute::VsaHpCaptivePortalUrl(i)),
		40 => value!(i, Attribute::VsaHpPortPriorityRegenerationTable(i)),
		46 => map!{i, be_u32, |v| Attribute::VsaHpBandwidthMaxIngress(v)},
		48 => map!{i, be_u32, |v| Attribute::VsaHpBandwidthMaxEgress(v)},
		61 => value!(i, Attribute::VsaHpIpFilterRaw(i)),
		63 => map!{i, be_u32, |v| Attribute::VsaHpNasRulesIpv6(v)},
		64 => map!{i, be_u32, |v| Attribute::VsaHpEgressVlanid(v)},
		65 => value!(i, Attribute::VsaHpEgressVlanName(i)),
		255 => value!(i, Attribute::VsaHpCapabilityAdvert(i)),
        _ => value!(i, Attribute::VsaUnknown(11, typ, i)),
    }
}
