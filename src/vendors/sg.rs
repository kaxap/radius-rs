/// Definitions for vendor SG, vendor value 2454
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgAccounting(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgAccounting {
	pub const Disable: SgAccounting = SgAccounting(1);
	pub const Enable: SgAccounting = SgAccounting(2);
	pub const Lastpacket: SgAccounting = SgAccounting(3);
	pub const EnableOnIpUpdate: SgAccounting = SgAccounting(4);
	pub const ResetAcctSessionId: SgAccounting = SgAccounting(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgAuthType(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgAuthType {
	pub const PreAuth: SgAuthType = SgAuthType(1);
	pub const ServiceSelection: SgAuthType = SgAuthType(2);
	pub const WebAuth: SgAuthType = SgAuthType(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgAction {
	pub const Reject: SgAction = SgAction(1);
	pub const Echo: SgAction = SgAction(2);
	pub const L2Echo: SgAction = SgAction(6);
	pub const Macantispoof: SgAction = SgAction(3);
	pub const UserSpaceOverwrite: SgAction = SgAction(4);
	pub const UserSpaceOverwriteOnNextService: SgAction = SgAction(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgDiscoverAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgDiscoverAction {
	pub const Normal: SgDiscoverAction = SgDiscoverAction(1);
	pub const Update: SgDiscoverAction = SgDiscoverAction(2);
	pub const PassThrough: SgDiscoverAction = SgDiscoverAction(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgReleaseAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgReleaseAction {
	pub const Disconnect: SgReleaseAction = SgReleaseAction(1);
	pub const Update: SgReleaseAction = SgReleaseAction(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgProtocolType(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgProtocolType {
	pub const Mlp: SgProtocolType = SgProtocolType(1);
	pub const Roam: SgProtocolType = SgProtocolType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgAuthSource(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgAuthSource {
	pub const Service: SgAuthSource = SgAuthSource(1);
	pub const User: SgAuthSource = SgAuthSource(2);
	pub const Cli: SgAuthSource = SgAuthSource(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgServiceCache(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgServiceCache {
	pub const Off: SgServiceCache = SgServiceCache(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgRoaming(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgRoaming {
	pub const Disable: SgRoaming = SgRoaming(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgAdvertiseProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgAdvertiseProtocol {
	pub const Ripv2: SgAdvertiseProtocol = SgAdvertiseProtocol(1);
	pub const Ospf: SgAdvertiseProtocol = SgAdvertiseProtocol(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgNativeip(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgNativeip {
	pub const Ppp: SgNativeip = SgNativeip(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgWimaxReducedResources(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgWimaxReducedResources {
	pub const Enable: SgWimaxReducedResources = SgWimaxReducedResources(1);
	pub const Disable: SgWimaxReducedResources = SgWimaxReducedResources(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgWimaxDmActionCode(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgWimaxDmActionCode {
	pub const ReconnectDisable: SgWimaxDmActionCode = SgWimaxDmActionCode(0x000010E0);
	pub const ReconnectDisableOnIdle: SgWimaxDmActionCode = SgWimaxDmActionCode(0x000410E0);
	pub const Idle: SgWimaxDmActionCode = SgWimaxDmActionCode(0x00000005);
	pub const IdleOnSessionIdle: SgWimaxDmActionCode = SgWimaxDmActionCode(0x00040005);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgWimaxMobilityFeaturesSupported(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgWimaxMobilityFeaturesSupported {
	pub const Mobility: SgWimaxMobilityFeaturesSupported = SgWimaxMobilityFeaturesSupported(1);
	pub const Sleep: SgWimaxMobilityFeaturesSupported = SgWimaxMobilityFeaturesSupported(2);
	pub const Idle: SgWimaxMobilityFeaturesSupported = SgWimaxMobilityFeaturesSupported(4);
	pub const Nomobility: SgWimaxMobilityFeaturesSupported = SgWimaxMobilityFeaturesSupported(4294967294);
	pub const Nosleep: SgWimaxMobilityFeaturesSupported = SgWimaxMobilityFeaturesSupported(4294967293);
	pub const Noidle: SgWimaxMobilityFeaturesSupported = SgWimaxMobilityFeaturesSupported(4294967291);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgWimaxNodeDisconnect(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgWimaxNodeDisconnect {
	pub const Enable: SgWimaxNodeDisconnect = SgWimaxNodeDisconnect(0);
	pub const Disable: SgWimaxNodeDisconnect = SgWimaxNodeDisconnect(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SgWimaxServiceFlowModification(pub u32);
 
#[allow(non_upper_case_globals)]
impl SgWimaxServiceFlowModification {
	pub const On: SgWimaxServiceFlowModification = SgWimaxServiceFlowModification(2);
	pub const Off: SgWimaxServiceFlowModification = SgWimaxServiceFlowModification(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AcctStatusType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AcctStatusType {
	pub const SgServiceStart: AcctStatusType = AcctStatusType(0x09960001);
	pub const SgServiceStop: AcctStatusType = AcctStatusType(0x09960002);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSgFilterRedirectGw(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		10 => map! {i, be_u32, |v| Attribute::VsaSgAccounting(SgAccounting(v))},
		12 => value!(i, Attribute::VsaSgOrigName(i)),
		13 => map! {i, be_u32, |v| Attribute::VsaSgAuthType(SgAuthType(v))},
		14 => map! {i, be_u32, |v| Attribute::VsaSgAction(SgAction(v))},
		15 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSgSscHost(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		16 => value!(i, Attribute::VsaSgServiceName(i)),
		17 => value!(i, Attribute::VsaSgPersonalSite(i)),
		18 => value!(i, Attribute::VsaSgMacAddress(i)),
		19 => map!{i, be_u32, |v| Attribute::VsaSgUserGroup(v)},
		20 => map!{i, be_u32, |v| Attribute::VsaSgMaxAllowedSessions(v)},
		21 => value!(i, Attribute::VsaSgClass(i)),
		22 => value!(i, Attribute::VsaSgEdsEncKey(i)),
		23 => value!(i, Attribute::VsaSgEdsCookie(i)),
		24 => value!(i, Attribute::VsaSgOriginalUrlPrefix(i)),
		25 => map!{i, be_u32, |v| Attribute::VsaSgMaxAllowedNodes(v)},
		26 => value!(i, Attribute::VsaSgParentUserName(i)),
		27 => map!{i, be_u32, |v| Attribute::VsaSgNodeGroup(v)},
		28 => value!(i, Attribute::VsaSgNodeDefaultService(i)),
		29 => value!(i, Attribute::VsaSgNodeDynamicService(i)),
		30 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSgDhcpServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		31 => value!(i, Attribute::VsaSgOpt82RelayRemoteId(i)),
		32 => map! {i, be_u32, |v| Attribute::VsaSgDiscoverAction(SgDiscoverAction(v))},
		33 => map! {i, be_u32, |v| Attribute::VsaSgReleaseAction(SgReleaseAction(v))},
		34 => value!(i, Attribute::VsaSgFixedIpAddress(i)),
		35 => value!(i, Attribute::VsaSgNodeFixedIpAddress(i)),
		36 => map!{i, be_u32, |v| Attribute::VsaSgLeaseTime(v)},
		40 => map! {i, be_u32, |v| Attribute::VsaSgProtocolType(SgProtocolType(v))},
		50 => map!{i, be_u32, |v| Attribute::VsaSgServiceTimeout(v)},
		51 => value!(i, Attribute::VsaSgNextServiceName(i)),
		52 => value!(i, Attribute::VsaSgAutoServiceName(i)),
		53 => map! {i, be_u32, |v| Attribute::VsaSgAuthSource(SgAuthSource(v))},
		54 => value!(i, Attribute::VsaSgDataQuota(i)),
		55 => value!(i, Attribute::VsaSgAclDataQuota(i)),
		56 => map! {i, be_u32, |v| Attribute::VsaSgServiceCache(SgServiceCache(v))},
		57 => value!(i, Attribute::VsaSgDataQuotaUsed(i)),
		58 => value!(i, Attribute::VsaSgAclDataQuotaUsed(i)),
		59 => value!(i, Attribute::VsaSgAclPacketQuota(i)),
		60 => value!(i, Attribute::VsaSgAclPacketQuotaUsed(i)),
		61 => map! {i, be_u32, |v| Attribute::VsaSgRoaming(SgRoaming(v))},
		62 => value!(i, Attribute::VsaSgAclEdsAction(i)),
		63 => value!(i, Attribute::VsaSgAclIdleIgnore(i)),
		65 => value!(i, Attribute::VsaSgServiceQuotaIgnore(i)),
		66 => value!(i, Attribute::VsaSgServiceAclQuotaIgnore(i)),
		67 => value!(i, Attribute::VsaSgServiceAclQuotaIndication(i)),
		70 => value!(i, Attribute::VsaSgRemoteFilterRedirectGw(i)),
		71 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSgNextHop(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		72 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSgNipPipeNextHop(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		73 => map! {i, be_u32, |v| Attribute::VsaSgAdvertiseProtocol(SgAdvertiseProtocol(v))},
		74 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSgForwardAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		75 => value!(i, Attribute::VsaSgAclTcpNatRedirect(i)),
		76 => value!(i, Attribute::VsaSgAclNextHop(i)),
		80 => value!(i, Attribute::VsaSgTunnelId(i)),
		81 => value!(i, Attribute::VsaSgL2TpTunnelPassword(i)),
		82 => value!(i, Attribute::VsaSgIpAddress(i)),
		83 => map!{i, be_u32, |v| Attribute::VsaSgTunnelAssignmentId(v)},
		84 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSgTunnelClientIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		85 => map! {i, be_u32, |v| Attribute::VsaSgNativeip(SgNativeip(v))},
		86 => value!(i, Attribute::VsaSgIpTunnel(i)),
		90 => value!(i, Attribute::VsaSgUpMeanRate(i)),
		91 => value!(i, Attribute::VsaSgDownMeanRate(i)),
		92 => value!(i, Attribute::VsaSgAclUpMeanRate(i)),
		93 => value!(i, Attribute::VsaSgAclDownMeanRate(i)),
		94 => value!(i, Attribute::VsaSgCos(i)),
		95 => value!(i, Attribute::VsaSgAclPriority(i)),
		96 => map!{i, be_u32, |v| Attribute::VsaSgBurstSize(v)},
		100 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSgIpPrimary(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		101 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSgIpSecondary(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		110 => map! {i, be_u32, |v| Attribute::VsaSgWimaxReducedResources(SgWimaxReducedResources(v))},
		111 => value!(i, Attribute::VsaSgWimaxAclScheduleType(i)),
		112 => value!(i, Attribute::VsaSgWimaxAclMinReservedTrafficRate(i)),
		113 => value!(i, Attribute::VsaSgWimaxAclMaximumTrafficBurst(i)),
		114 => value!(i, Attribute::VsaSgWimaxAclToleratedJitter(i)),
		115 => value!(i, Attribute::VsaSgWimaxAclMaximumLatency(i)),
		116 => value!(i, Attribute::VsaSgWimaxAclUnsolicitedGrantInt(i)),
		117 => value!(i, Attribute::VsaSgWimaxAclSduSize(i)),
		118 => value!(i, Attribute::VsaSgWimaxAclUnsolicitedPollingInt(i)),
		119 => map!{i, be_u32, |v| Attribute::VsaSgWimaxMskLifetime(v)},
		120 => map! {i, be_u32, |v| Attribute::VsaSgWimaxDmActionCode(SgWimaxDmActionCode(v))},
		121 => value!(i, Attribute::VsaSgWimaxAclArqEnable(i)),
		122 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSgWimaxBsidNextHop(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		123 => map! {i, be_u32, |v| Attribute::VsaSgWimaxMobilityFeaturesSupported(SgWimaxMobilityFeaturesSupported(v))},
		124 => map! {i, be_u32, |v| Attribute::VsaSgWimaxNodeDisconnect(SgWimaxNodeDisconnect(v))},
		125 => map! {i, be_u32, |v| Attribute::VsaSgWimaxServiceFlowModification(SgWimaxServiceFlowModification(v))},
		126 => value!(i, Attribute::VsaSgWimaxServiceFlowDown(i)),
		130 => value!(i, Attribute::VsaSgNodeAcctUsername(i)),
        _ => value!(i, Attribute::VsaUnknown(2454, typ, i)),
    }
}
