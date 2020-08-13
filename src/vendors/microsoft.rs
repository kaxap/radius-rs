/// Definitions for vendor Microsoft, vendor value 311
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsAfwZone(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsAfwZone {
	pub const MsAfwZoneBoundaryPolicy: MsAfwZone = MsAfwZone(1);
	pub const MsAfwZoneUnprotectedPolicy: MsAfwZone = MsAfwZone(2);
	pub const MsAfwZoneProtectedPolicy: MsAfwZone = MsAfwZone(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsAfwProtectionLevel(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsAfwProtectionLevel {
	pub const HecpResponseSignOnly: MsAfwProtectionLevel = MsAfwProtectionLevel(1);
	pub const HecpResponseSignAndEncrypt: MsAfwProtectionLevel = MsAfwProtectionLevel(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsRnapNotQuarantineCapable(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsRnapNotQuarantineCapable {
	pub const SohSent: MsRnapNotQuarantineCapable = MsRnapNotQuarantineCapable(0);
	pub const SohNotSent: MsRnapNotQuarantineCapable = MsRnapNotQuarantineCapable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsMppeEncryptionPolicy(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsMppeEncryptionPolicy {
	pub const EncryptionAllowed: MsMppeEncryptionPolicy = MsMppeEncryptionPolicy(1);
	pub const EncryptionRequired: MsMppeEncryptionPolicy = MsMppeEncryptionPolicy(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsMppeEncryptionTypes(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsMppeEncryptionTypes {
	pub const None: MsMppeEncryptionTypes = MsMppeEncryptionTypes(0);
	pub const Rc440: MsMppeEncryptionTypes = MsMppeEncryptionTypes(2);
	pub const Rc4128: MsMppeEncryptionTypes = MsMppeEncryptionTypes(4);
	pub const Rc440128: MsMppeEncryptionTypes = MsMppeEncryptionTypes(6);
	pub const Stateless: MsMppeEncryptionTypes = MsMppeEncryptionTypes(8);
	pub const Rc440Stateless: MsMppeEncryptionTypes = MsMppeEncryptionTypes(10);
	pub const Rc4128Stateless: MsMppeEncryptionTypes = MsMppeEncryptionTypes(12);
	pub const Rc440128Stateless: MsMppeEncryptionTypes = MsMppeEncryptionTypes(14);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsBapUsage(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsBapUsage {
	pub const NotAllowed: MsBapUsage = MsBapUsage(0);
	pub const Allowed: MsBapUsage = MsBapUsage(1);
	pub const Required: MsBapUsage = MsBapUsage(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsArapPwChangeReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsArapPwChangeReason {
	pub const JustChangePassword: MsArapPwChangeReason = MsArapPwChangeReason(1);
	pub const ExpiredPassword: MsArapPwChangeReason = MsArapPwChangeReason(2);
	pub const AdminRequiresPasswordChange: MsArapPwChangeReason = MsArapPwChangeReason(3);
	pub const PasswordTooShort: MsArapPwChangeReason = MsArapPwChangeReason(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsAcctAuthType(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsAcctAuthType {
	pub const Pap: MsAcctAuthType = MsAcctAuthType(1);
	pub const Chap: MsAcctAuthType = MsAcctAuthType(2);
	pub const MsChap1: MsAcctAuthType = MsAcctAuthType(3);
	pub const MsChap2: MsAcctAuthType = MsAcctAuthType(4);
	pub const Eap: MsAcctAuthType = MsAcctAuthType(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsAcctEapType(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsAcctEapType {
	pub const Md5: MsAcctEapType = MsAcctEapType(4);
	pub const Otp: MsAcctEapType = MsAcctEapType(5);
	pub const GenericTokenCard: MsAcctEapType = MsAcctEapType(6);
	pub const Tls: MsAcctEapType = MsAcctEapType(13);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsIdentityType(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsIdentityType {
	pub const MachineHealthCheck: MsIdentityType = MsIdentityType(1);
	pub const IgnoreUserLookupFailure: MsIdentityType = MsIdentityType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsQuarantineState(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsQuarantineState {
	pub const FullAccess: MsQuarantineState = MsQuarantineState(0);
	pub const Quarantine: MsQuarantineState = MsQuarantineState(1);
	pub const Probation: MsQuarantineState = MsQuarantineState(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsNetworkAccessServerType(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsNetworkAccessServerType {
	pub const Unspecified: MsNetworkAccessServerType = MsNetworkAccessServerType(0);
	pub const TerminalServerGateway: MsNetworkAccessServerType = MsNetworkAccessServerType(1);
	pub const RemoteAccessServer: MsNetworkAccessServerType = MsNetworkAccessServerType(2);
	pub const DhcpServer: MsNetworkAccessServerType = MsNetworkAccessServerType(3);
	pub const WirelessAccessPoint: MsNetworkAccessServerType = MsNetworkAccessServerType(4);
	pub const Hra: MsNetworkAccessServerType = MsNetworkAccessServerType(5);
	pub const HcapServer: MsNetworkAccessServerType = MsNetworkAccessServerType(6);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsExtendedQuarantineState(pub u32);
 
#[allow(non_upper_case_globals)]
impl MsExtendedQuarantineState {
	pub const Transition: MsExtendedQuarantineState = MsExtendedQuarantineState(1);
	pub const Infected: MsExtendedQuarantineState = MsExtendedQuarantineState(2);
	pub const Unknown: MsExtendedQuarantineState = MsExtendedQuarantineState(3);
	pub const NoData: MsExtendedQuarantineState = MsExtendedQuarantineState(4);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaMsChapResponse(i)),
		2 => value!(i, Attribute::VsaMsChapError(i)),
		3 => value!(i, Attribute::VsaMsChapCpw1(i)),
		4 => value!(i, Attribute::VsaMsChapCpw2(i)),
		5 => value!(i, Attribute::VsaMsChapLmEncPw(i)),
		6 => value!(i, Attribute::VsaMsChapNtEncPw(i)),
		7 => map! {i, be_u32, |v| Attribute::VsaMsMppeEncryptionPolicy(MsMppeEncryptionPolicy(v))},
		8 => map!{i, be_u32, |v| Attribute::VsaMsMppeEncryptionType(v)},
		9 => map!{i, be_u32, |v| Attribute::VsaMsRasVendor(v)},
		10 => value!(i, Attribute::VsaMsChapDomain(i)),
		11 => value!(i, Attribute::VsaMsChapChallenge(i)),
		12 => value!(i, Attribute::VsaMsChapMppeKeys(i)),
		13 => map! {i, be_u32, |v| Attribute::VsaMsBapUsage(MsBapUsage(v))},
		14 => map!{i, be_u32, |v| Attribute::VsaMsLinkUtilizationThreshold(v)},
		15 => map!{i, be_u32, |v| Attribute::VsaMsLinkDropTimeLimit(v)},
		16 => value!(i, Attribute::VsaMsMppeSendKey(i)),
		17 => value!(i, Attribute::VsaMsMppeRecvKey(i)),
		18 => value!(i, Attribute::VsaMsRasVersion(i)),
		19 => value!(i, Attribute::VsaMsOldArapPassword(i)),
		20 => value!(i, Attribute::VsaMsNewArapPassword(i)),
		21 => map! {i, be_u32, |v| Attribute::VsaMsArapPwChangeReason(MsArapPwChangeReason(v))},
		22 => value!(i, Attribute::VsaMsFilter(i)),
		23 => map! {i, be_u32, |v| Attribute::VsaMsAcctAuthType(MsAcctAuthType(v))},
		24 => map! {i, be_u32, |v| Attribute::VsaMsAcctEapType(MsAcctEapType(v))},
		25 => value!(i, Attribute::VsaMsChap2Response(i)),
		26 => value!(i, Attribute::VsaMsChap2Success(i)),
		27 => value!(i, Attribute::VsaMsChap2Cpw(i)),
		28 => map!{i, take!(4), |v:&[u8]| Attribute::VsaMsPrimaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		29 => map!{i, take!(4), |v:&[u8]| Attribute::VsaMsSecondaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		30 => map!{i, take!(4), |v:&[u8]| Attribute::VsaMsPrimaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		31 => map!{i, take!(4), |v:&[u8]| Attribute::VsaMsSecondaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		34 => value!(i, Attribute::VsaMsRasClientName(i)),
		35 => value!(i, Attribute::VsaMsRasClientVersion(i)),
		36 => value!(i, Attribute::VsaMsQuarantineIpfilter(i)),
		37 => map!{i, be_u32, |v| Attribute::VsaMsQuarantineSessionTimeout(v)},
		40 => value!(i, Attribute::VsaMsUserSecurityIdentity(i)),
		41 => map! {i, be_u32, |v| Attribute::VsaMsIdentityType(MsIdentityType(v))},
		42 => value!(i, Attribute::VsaMsServiceClass(i)),
		44 => value!(i, Attribute::VsaMsQuarantineUserClass(i)),
		45 => map! {i, be_u32, |v| Attribute::VsaMsQuarantineState(MsQuarantineState(v))},
		46 => map!{i, be_u32, |v| Attribute::VsaMsQuarantineGraceTime(v)},
		47 => map! {i, be_u32, |v| Attribute::VsaMsNetworkAccessServerType(MsNetworkAccessServerType(v))},
		48 => map! {i, be_u32, |v| Attribute::VsaMsAfwZone(MsAfwZone(v))},
		49 => map! {i, be_u32, |v| Attribute::VsaMsAfwProtectionLevel(MsAfwProtectionLevel(v))},
		50 => value!(i, Attribute::VsaMsMachineName(i)),
		51 => value!(i, Attribute::VsaMsIpv6Filter(i)),
		52 => value!(i, Attribute::VsaMsIpv4RemediationServers(i)),
		53 => value!(i, Attribute::VsaMsIpv6RemediationServers(i)),
		54 => map! {i, be_u32, |v| Attribute::VsaMsRnapNotQuarantineCapable(MsRnapNotQuarantineCapable(v))},
		55 => value!(i, Attribute::VsaMsQuarantineSoh(i)),
		56 => value!(i, Attribute::VsaMsRasCorrelation(i)),
		57 => map! {i, be_u32, |v| Attribute::VsaMsExtendedQuarantineState(MsExtendedQuarantineState(v))},
		58 => value!(i, Attribute::VsaMsHcapUserGroups(i)),
		59 => value!(i, Attribute::VsaMsHcapLocationGroupName(i)),
		60 => value!(i, Attribute::VsaMsHcapUserName(i)),
		61 => map!{i, take!(4), |v:&[u8]| Attribute::VsaMsUserIpv4Address(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		62 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaMsUserIpv6Address(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		63 => map!{i, be_u32, |v| Attribute::VsaMsTsgDeviceRedirection(v)},
        _ => value!(i, Attribute::VsaUnknown(311, typ, i)),
    }
}
