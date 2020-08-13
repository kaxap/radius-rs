/// Definitions for vendor Unisphere, vendor value 4874
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereIngressStatistics(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereIngressStatistics {
	pub const Disable: UnisphereIngressStatistics = UnisphereIngressStatistics(0);
	pub const Enable: UnisphereIngressStatistics = UnisphereIngressStatistics(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereEgressStatistics(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereEgressStatistics {
	pub const Disable: UnisphereEgressStatistics = UnisphereEgressStatistics(0);
	pub const Enable: UnisphereEgressStatistics = UnisphereEgressStatistics(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereServiceCategory(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereServiceCategory {
	pub const Ubr: UnisphereServiceCategory = UnisphereServiceCategory(1);
	pub const Ubrpcr: UnisphereServiceCategory = UnisphereServiceCategory(2);
	pub const Nrtvbr: UnisphereServiceCategory = UnisphereServiceCategory(3);
	pub const Cbr: UnisphereServiceCategory = UnisphereServiceCategory(4);
	pub const Rtvbr: UnisphereServiceCategory = UnisphereServiceCategory(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereAllowAllVrAccess(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereAllowAllVrAccess {
	pub const Disable: UnisphereAllowAllVrAccess = UnisphereAllowAllVrAccess(0);
	pub const Enable: UnisphereAllowAllVrAccess = UnisphereAllowAllVrAccess(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereSaValidate(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereSaValidate {
	pub const Disable: UnisphereSaValidate = UnisphereSaValidate(0);
	pub const Enable: UnisphereSaValidate = UnisphereSaValidate(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereIgmpEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereIgmpEnable {
	pub const Disable: UnisphereIgmpEnable = UnisphereIgmpEnable(0);
	pub const Enable: UnisphereIgmpEnable = UnisphereIgmpEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereQosProfileInterfaceType(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereQosProfileInterfaceType {
	pub const Ip: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(1);
	pub const Atm: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(2);
	pub const Hdlc: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(3);
	pub const Ethernet: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(4);
	pub const ServerPort: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(5);
	pub const Atm1483: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(6);
	pub const FrameRelay: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(7);
	pub const MplsMinor: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(8);
	pub const Cbf: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(9);
	pub const IpTunnel: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(10);
	pub const VlanSub: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(11);
	pub const PppoeSub: UnisphereQosProfileInterfaceType = UnisphereQosProfileInterfaceType(12);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereNasPortMethod(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereNasPortMethod {
	pub const None: UnisphereNasPortMethod = UnisphereNasPortMethod(0);
	pub const CiscoClid: UnisphereNasPortMethod = UnisphereNasPortMethod(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnispherePppProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnispherePppProtocol {
	pub const None: UnispherePppProtocol = UnispherePppProtocol(0);
	pub const Pap: UnispherePppProtocol = UnispherePppProtocol(1);
	pub const Chap: UnispherePppProtocol = UnispherePppProtocol(2);
	pub const PapChap: UnispherePppProtocol = UnispherePppProtocol(3);
	pub const ChapPap: UnispherePppProtocol = UnispherePppProtocol(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereTunnelBearerType(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereTunnelBearerType {
	pub const None: UnisphereTunnelBearerType = UnisphereTunnelBearerType(0);
	pub const Analog: UnisphereTunnelBearerType = UnisphereTunnelBearerType(1);
	pub const Digital: UnisphereTunnelBearerType = UnisphereTunnelBearerType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereLiAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereLiAction {
	pub const Off: UnisphereLiAction = UnisphereLiAction(0);
	pub const On: UnisphereLiAction = UnisphereLiAction(1);
	pub const Noop: UnisphereLiAction = UnisphereLiAction(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereDfBit(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereDfBit {
	pub const DontIgnoreDfBit: UnisphereDfBit = UnisphereDfBit(0);
	pub const IgnoreDfBit: UnisphereDfBit = UnisphereDfBit(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereMldVersion(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereMldVersion {
	pub const V1: UnisphereMldVersion = UnisphereMldVersion(1);
	pub const V2: UnisphereMldVersion = UnisphereMldVersion(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereIgmpVersion(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereIgmpVersion {
	pub const V1: UnisphereIgmpVersion = UnisphereIgmpVersion(1);
	pub const V2: UnisphereIgmpVersion = UnisphereIgmpVersion(2);
	pub const V3: UnisphereIgmpVersion = UnisphereIgmpVersion(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereServiceStats(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereServiceStats {
	pub const Disabled: UnisphereServiceStats = UnisphereServiceStats(0);
	pub const Time: UnisphereServiceStats = UnisphereServiceStats(1);
	pub const TimeVolume: UnisphereServiceStats = UnisphereServiceStats(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereL2TpResynchMethod(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereL2TpResynchMethod {
	pub const Disable: UnisphereL2TpResynchMethod = UnisphereL2TpResynchMethod(0);
	pub const Failover: UnisphereL2TpResynchMethod = UnisphereL2TpResynchMethod(1);
	pub const SilentFailover: UnisphereL2TpResynchMethod = UnisphereL2TpResynchMethod(2);
	pub const FailoverWithSilentBackup: UnisphereL2TpResynchMethod = UnisphereL2TpResynchMethod(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereTunnelTxSpeedMethod(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereTunnelTxSpeedMethod {
	pub const StaticLayer2: UnisphereTunnelTxSpeedMethod = UnisphereTunnelTxSpeedMethod(1);
	pub const DynamicLayer2: UnisphereTunnelTxSpeedMethod = UnisphereTunnelTxSpeedMethod(2);
	pub const Qos: UnisphereTunnelTxSpeedMethod = UnisphereTunnelTxSpeedMethod(3);
	pub const Actual: UnisphereTunnelTxSpeedMethod = UnisphereTunnelTxSpeedMethod(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereIgmpImmediateLeave(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereIgmpImmediateLeave {
	pub const Disabled: UnisphereIgmpImmediateLeave = UnisphereIgmpImmediateLeave(0);
	pub const Enabled: UnisphereIgmpImmediateLeave = UnisphereIgmpImmediateLeave(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereMldImmediateLeave(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereMldImmediateLeave {
	pub const Disabled: UnisphereMldImmediateLeave = UnisphereMldImmediateLeave(0);
	pub const Enabled: UnisphereMldImmediateLeave = UnisphereMldImmediateLeave(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereIpBlockMulticast(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereIpBlockMulticast {
	pub const Disabled: UnisphereIpBlockMulticast = UnisphereIpBlockMulticast(0);
	pub const Enabled: UnisphereIpBlockMulticast = UnisphereIpBlockMulticast(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereIgmpExplicitTracking(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereIgmpExplicitTracking {
	pub const Disabled: UnisphereIgmpExplicitTracking = UnisphereIgmpExplicitTracking(0);
	pub const Enabled: UnisphereIgmpExplicitTracking = UnisphereIgmpExplicitTracking(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereIgmpNoTrackingV2Grps(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereIgmpNoTrackingV2Grps {
	pub const Disabled: UnisphereIgmpNoTrackingV2Grps = UnisphereIgmpNoTrackingV2Grps(0);
	pub const Enabled: UnisphereIgmpNoTrackingV2Grps = UnisphereIgmpNoTrackingV2Grps(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereMldExplicitTracking(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereMldExplicitTracking {
	pub const Disabled: UnisphereMldExplicitTracking = UnisphereMldExplicitTracking(0);
	pub const Enabled: UnisphereMldExplicitTracking = UnisphereMldExplicitTracking(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereMldNoTrackingV1Grps(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereMldNoTrackingV1Grps {
	pub const Disabled: UnisphereMldNoTrackingV1Grps = UnisphereMldNoTrackingV1Grps(0);
	pub const Enabled: UnisphereMldNoTrackingV1Grps = UnisphereMldNoTrackingV1Grps(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereDslLineState(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereDslLineState {
	pub const Showtime: UnisphereDslLineState = UnisphereDslLineState(1);
	pub const Idle: UnisphereDslLineState = UnisphereDslLineState(2);
	pub const Silent: UnisphereDslLineState = UnisphereDslLineState(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnisphereDslType(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnisphereDslType {
	pub const Adsl1: UnisphereDslType = UnisphereDslType(1);
	pub const Adsl2: UnisphereDslType = UnisphereDslType(2);
	pub const Adsl2Plus: UnisphereDslType = UnisphereDslType(3);
	pub const Vdsl1: UnisphereDslType = UnisphereDslType(4);
	pub const Vdsl2: UnisphereDslType = UnisphereDslType(5);
	pub const Sdsl: UnisphereDslType = UnisphereDslType(6);
	pub const Unknown: UnisphereDslType = UnisphereDslType(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnispherePppIngressOnly(pub u32);
 
#[allow(non_upper_case_globals)]
impl UnispherePppIngressOnly {
	pub const Disabled: UnispherePppIngressOnly = UnispherePppIngressOnly(0);
	pub const Enabled: UnispherePppIngressOnly = UnispherePppIngressOnly(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaUnisphereVirtualRouter(i)),
		2 => value!(i, Attribute::VsaUnisphereLocalAddressPool(i)),
		3 => value!(i, Attribute::VsaUnisphereLocalInterface(i)),
		4 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUnispherePrimaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		5 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUnisphereSecondaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		6 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUnispherePrimaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		7 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUnisphereSecondaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		8 => value!(i, Attribute::VsaUnisphereTunnelVirtualRouter(i)),
		9 => value!(i, Attribute::VsaUnisphereTunnelPassword(i)),
		10 => value!(i, Attribute::VsaUnisphereIngressPolicyName(i)),
		11 => value!(i, Attribute::VsaUnisphereEgressPolicyName(i)),
		12 => map! {i, be_u32, |v| Attribute::VsaUnisphereIngressStatistics(UnisphereIngressStatistics(v))},
		13 => map! {i, be_u32, |v| Attribute::VsaUnisphereEgressStatistics(UnisphereEgressStatistics(v))},
		14 => map! {i, be_u32, |v| Attribute::VsaUnisphereServiceCategory(UnisphereServiceCategory(v))},
		15 => map!{i, be_u32, |v| Attribute::VsaUnispherePcr(v)},
		16 => map!{i, be_u32, |v| Attribute::VsaUnisphereScr(v)},
		17 => map!{i, be_u32, |v| Attribute::VsaUnisphereMbs(v)},
		18 => value!(i, Attribute::VsaUnisphereInitCliAccessLevel(i)),
		19 => map! {i, be_u32, |v| Attribute::VsaUnisphereAllowAllVrAccess(UnisphereAllowAllVrAccess(v))},
		20 => value!(i, Attribute::VsaUnisphereAltCliAccessLevel(i)),
		21 => value!(i, Attribute::VsaUnisphereAltCliVrouterName(i)),
		22 => map! {i, be_u32, |v| Attribute::VsaUnisphereSaValidate(UnisphereSaValidate(v))},
		23 => map! {i, be_u32, |v| Attribute::VsaUnisphereIgmpEnable(UnisphereIgmpEnable(v))},
		24 => value!(i, Attribute::VsaUnispherePppoeDescription(i)),
		25 => value!(i, Attribute::VsaUnisphereRedirectVrouterName(i)),
		26 => value!(i, Attribute::VsaUnisphereQosProfileName(i)),
		27 => map!{i, be_u32, |v| Attribute::VsaUnispherePppoeMaxSessions(v)},
		28 => value!(i, Attribute::VsaUnispherePppoeUrl(i)),
		29 => map! {i, be_u32, |v| Attribute::VsaUnisphereQosProfileInterfaceType(UnisphereQosProfileInterfaceType(v))},
		30 => map! {i, be_u32, |v| Attribute::VsaUnisphereNasPortMethod(UnisphereNasPortMethod(v))},
		31 => value!(i, Attribute::VsaUnisphereServiceBundle(i)),
		32 => map!{i, be_u32, |v| Attribute::VsaUnisphereTunnelTos(v)},
		33 => map!{i, be_u32, |v| Attribute::VsaUnisphereTunnelMaxSessions(v)},
		34 => value!(i, Attribute::VsaUnisphereFramedIpRouteTag(i)),
		35 => value!(i, Attribute::VsaUnisphereTunnelDialoutNumber(i)),
		36 => value!(i, Attribute::VsaUnispherePppUsername(i)),
		37 => value!(i, Attribute::VsaUnispherePppPassword(i)),
		38 => map! {i, be_u32, |v| Attribute::VsaUnispherePppProtocol(UnispherePppProtocol(v))},
		39 => map!{i, be_u32, |v| Attribute::VsaUnisphereTunnelMinBps(v)},
		40 => map!{i, be_u32, |v| Attribute::VsaUnisphereTunnelMaxBps(v)},
		41 => map! {i, be_u32, |v| Attribute::VsaUnisphereTunnelBearerType(UnisphereTunnelBearerType(v))},
		42 => map!{i, be_u32, |v| Attribute::VsaUnisphereInputGigapackets(v)},
		43 => map!{i, be_u32, |v| Attribute::VsaUnisphereOutputGigapackets(v)},
		44 => value!(i, Attribute::VsaUnisphereTunnelInterfaceId(i)),
		45 => value!(i, Attribute::VsaUnisphereIpv6VirtualRouter(i)),
		46 => value!(i, Attribute::VsaUnisphereIpv6LocalInterface(i)),
		47 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaUnisphereIpv6PrimaryDns(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		48 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaUnisphereIpv6SecondaryDns(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		49 => value!(i, Attribute::VsaUnisphereServiceName(i)),
		50 => value!(i, Attribute::VsaUnisphereSessionVolumeQuota(i)),
		51 => value!(i, Attribute::VsaUnisphereDisconnectCause(i)),
		52 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUnisphereRadiusClientAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		53 => value!(i, Attribute::VsaUnisphereServiceDescription(i)),
		54 => map!{i, be_u32, |v| Attribute::VsaUnisphereL2TpRecvWindowSize(v)},
		55 => value!(i, Attribute::VsaUnisphereDhcpOptions(i)),
		56 => value!(i, Attribute::VsaUnisphereDhcpMacAddr(i)),
		57 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUnisphereDhcpGiAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		58 => map! {i, be_u32, |v| Attribute::VsaUnisphereLiAction(UnisphereLiAction(v))},
		59 => value!(i, Attribute::VsaUnisphereMedDevHandle(i)),
		60 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUnisphereMedIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		61 => map!{i, be_u32, |v| Attribute::VsaUnisphereMedPortNumber(v)},
		62 => value!(i, Attribute::VsaUnisphereMlpppBundleName(i)),
		63 => value!(i, Attribute::VsaUnisphereInterfaceDesc(i)),
		64 => value!(i, Attribute::VsaUnisphereTunnelGroup(i)),
		65 => value!(i, Attribute::VsaUnisphereServiceActivate(i)),
		66 => value!(i, Attribute::VsaUnisphereServiceDeactivate(i)),
		67 => map!{i, be_u32, |v| Attribute::VsaUnisphereServiceVolume(v)},
		68 => map!{i, be_u32, |v| Attribute::VsaUnisphereServiceTimeout(v)},
		69 => map! {i, be_u32, |v| Attribute::VsaUnisphereServiceStats(UnisphereServiceStats(v))},
		70 => map! {i, be_u32, |v| Attribute::VsaUnisphereDfBit(UnisphereDfBit(v))},
		71 => value!(i, Attribute::VsaUnisphereIgmpAccessName(i)),
		72 => value!(i, Attribute::VsaUnisphereIgmpAccessSrcName(i)),
		73 => value!(i, Attribute::VsaUnisphereIgmpOifMapName(i)),
		74 => value!(i, Attribute::VsaUnisphereMldAccessName(i)),
		75 => value!(i, Attribute::VsaUnisphereMldAccessSrcName(i)),
		76 => value!(i, Attribute::VsaUnisphereMldOifMapName(i)),
		77 => map! {i, be_u32, |v| Attribute::VsaUnisphereMldVersion(UnisphereMldVersion(v))},
		78 => map! {i, be_u32, |v| Attribute::VsaUnisphereIgmpVersion(UnisphereIgmpVersion(v))},
		79 => map!{i, be_u32, |v| Attribute::VsaUnisphereIpMcastAdmBwLimit(v)},
		80 => map!{i, be_u32, |v| Attribute::VsaUnisphereIpv6McastAdmBwLimit(v)},
		81 => value!(i, Attribute::VsaUnisphereL2CAccessLoopParameters(i)),
		82 => value!(i, Attribute::VsaUnisphereQosParameters(i)),
		83 => value!(i, Attribute::VsaUnisphereServiceSession(i)),
		84 => map!{i, be_u32, |v| Attribute::VsaUnisphereMobileIpAlgorithm(v)},
		85 => map!{i, be_u32, |v| Attribute::VsaUnisphereMobileIpSpi(v)},
		86 => value!(i, Attribute::VsaUnisphereMobileIpKey(i)),
		87 => map!{i, be_u32, |v| Attribute::VsaUnisphereMobileIpReplay(v)},
		88 => value!(i, Attribute::VsaUnisphereMobileIpAccessControl(i)),
		89 => map!{i, be_u32, |v| Attribute::VsaUnisphereMobileIpLifetime(v)},
		90 => map! {i, be_u32, |v| Attribute::VsaUnisphereL2TpResynchMethod(UnisphereL2TpResynchMethod(v))},
		91 => value!(i, Attribute::VsaUnisphereTunnelSwitchProfile(i)),
		92 => value!(i, Attribute::VsaUnisphereL2CUpStreamData(i)),
		93 => value!(i, Attribute::VsaUnisphereL2CDownStreamData(i)),
		94 => map! {i, be_u32, |v| Attribute::VsaUnisphereTunnelTxSpeedMethod(UnisphereTunnelTxSpeedMethod(v))},
		95 => map!{i, be_u32, |v| Attribute::VsaUnisphereIgmpQueryInterval(v)},
		96 => map!{i, be_u32, |v| Attribute::VsaUnisphereIgmpMaxRespTime(v)},
		97 => map! {i, be_u32, |v| Attribute::VsaUnisphereIgmpImmediateLeave(UnisphereIgmpImmediateLeave(v))},
		98 => map!{i, be_u32, |v| Attribute::VsaUnisphereMldQueryInterval(v)},
		99 => map!{i, be_u32, |v| Attribute::VsaUnisphereMldMaxRespTime(v)},
		100 => map! {i, be_u32, |v| Attribute::VsaUnisphereMldImmediateLeave(UnisphereMldImmediateLeave(v))},
		101 => map! {i, be_u32, |v| Attribute::VsaUnisphereIpBlockMulticast(UnisphereIpBlockMulticast(v))},
		102 => map! {i, be_u32, |v| Attribute::VsaUnisphereIgmpExplicitTracking(UnisphereIgmpExplicitTracking(v))},
		103 => map! {i, be_u32, |v| Attribute::VsaUnisphereIgmpNoTrackingV2Grps(UnisphereIgmpNoTrackingV2Grps(v))},
		104 => map! {i, be_u32, |v| Attribute::VsaUnisphereMldExplicitTracking(UnisphereMldExplicitTracking(v))},
		105 => map! {i, be_u32, |v| Attribute::VsaUnisphereMldNoTrackingV1Grps(UnisphereMldNoTrackingV1Grps(v))},
		106 => value!(i, Attribute::VsaJnprIpv6IngressPolicyName(i)),
		107 => value!(i, Attribute::VsaJnprIpv6EgressPolicyName(i)),
		108 => value!(i, Attribute::VsaJnprCosParameterType(i)),
		109 => map!{i, take!(4), |v:&[u8]| Attribute::VsaJnprDhcpGuidedRelayServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		110 => value!(i, Attribute::VsaUnisphereAccLoopCirId(i)),
		111 => value!(i, Attribute::VsaUnisphereAccAggrCirIdBin(i)),
		112 => value!(i, Attribute::VsaUnisphereAccAggrCirIdAsc(i)),
		113 => map!{i, be_u32, |v| Attribute::VsaUnisphereActDataRateUp(v)},
		114 => map!{i, be_u32, |v| Attribute::VsaUnisphereActDataRateDn(v)},
		115 => map!{i, be_u32, |v| Attribute::VsaUnisphereMinDataRateUp(v)},
		116 => map!{i, be_u32, |v| Attribute::VsaUnisphereMinDataRateDn(v)},
		117 => map!{i, be_u32, |v| Attribute::VsaUnisphereAttDataRateUp(v)},
		118 => map!{i, be_u32, |v| Attribute::VsaUnisphereAttDataRateDn(v)},
		119 => map!{i, be_u32, |v| Attribute::VsaUnisphereMaxDataRateUp(v)},
		120 => map!{i, be_u32, |v| Attribute::VsaUnisphereMaxDataRateDn(v)},
		121 => map!{i, be_u32, |v| Attribute::VsaUnisphereMinLpDataRateUp(v)},
		122 => map!{i, be_u32, |v| Attribute::VsaUnisphereMinLpDataRateDn(v)},
		123 => map!{i, be_u32, |v| Attribute::VsaUnisphereMaxInterlvDelayUp(v)},
		124 => map!{i, be_u32, |v| Attribute::VsaUnisphereActInterlvDelayUp(v)},
		125 => map!{i, be_u32, |v| Attribute::VsaUnisphereMaxInterlvDelayDn(v)},
		126 => map!{i, be_u32, |v| Attribute::VsaUnisphereActInterlvDelayDn(v)},
		127 => map! {i, be_u32, |v| Attribute::VsaUnisphereDslLineState(UnisphereDslLineState(v))},
		128 => map! {i, be_u32, |v| Attribute::VsaUnisphereDslType(UnisphereDslType(v))},
		129 => value!(i, Attribute::VsaUnisphereIpv6NdraPrefix(i)),
		130 => value!(i, Attribute::VsaUnisphereQosSetName(i)),
		140 => map!{i, be_u32, |v| Attribute::VsaUnisphereServiceAcctint(v)},
		141 => map!{i, be_u32, |v| Attribute::VsaUnisphereDownstreamCalcRate(v)},
		142 => map!{i, be_u32, |v| Attribute::VsaUnisphereUpstreamCalcRate(v)},
		143 => map!{i, be_u32, |v| Attribute::VsaJnprMaxClientsPerInterface(v)},
		144 => map! {i, be_u32, |v| Attribute::VsaUnispherePppIngressOnly(UnispherePppIngressOnly(v))},
		146 => value!(i, Attribute::VsaJnprCosSchedulerPmtType(i)),
		147 => value!(i, Attribute::VsaUnisphereBackupAddressPool(i)),
		150 => value!(i, Attribute::VsaUnisphereIcrPartitionId(i)),
		151 => map!{i, be_u32, |v| Attribute::VsaUnisphereIpv6AcctInputOctets(v)},
		152 => map!{i, be_u32, |v| Attribute::VsaUnisphereIpv6AcctOutputOctets(v)},
		153 => map!{i, be_u32, |v| Attribute::VsaUnisphereIpv6AcctInputPackets(v)},
		154 => map!{i, be_u32, |v| Attribute::VsaUnisphereIpv6AcctOutputPackets(v)},
		155 => map!{i, be_u32, |v| Attribute::VsaUnisphereIpv6AcctInputGigawords(v)},
		156 => map!{i, be_u32, |v| Attribute::VsaUnisphereIpv6AcctOutputGigawords(v)},
		157 => value!(i, Attribute::VsaJnprIpv6NdraPoolName(i)),
		158 => value!(i, Attribute::VsaJnprPppoePadn(i)),
		159 => value!(i, Attribute::VsaUnisphereDhcpOption82(i)),
		160 => map!{i, be_u32, |v| Attribute::VsaJnprVlanMapId(v)},
		161 => value!(i, Attribute::VsaJnprIpv6DelegatedPoolName(i)),
		162 => map!{i, be_u32, |v| Attribute::VsaJnprTxConnectSpeed(v)},
		163 => map!{i, be_u32, |v| Attribute::VsaJnprRxConnectSpeed(v)},
		164 => value!(i, Attribute::VsaUnisphereIpv4ReleaseControl(i)),
		165 => value!(i, Attribute::VsaPcpServerName(i)),
		174 => value!(i, Attribute::VsaUnisphereClientProfileName(i)),
		175 => map!{i, take!(4), |v:&[u8]| Attribute::VsaJnprRedirectGwAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		176 => value!(i, Attribute::VsaJnprApnName(i)),
		177 => value!(i, Attribute::VsaUnisphereCosShapingRate(i)),
		178 => value!(i, Attribute::VsaUnisphereActionReason(i)),
		179 => map!{i, be_u32, |v| Attribute::VsaUnisphereServiceVolumeGigawords(v)},
		180 => value!(i, Attribute::VsaUnisphereUpdateService(i)),
		182 => value!(i, Attribute::VsaUnisphereAccLoopRemoteId(i)),
		183 => value!(i, Attribute::VsaUnisphereAccLoopEncap(i)),
		184 => map!{i, be_u32, |v| Attribute::VsaUnisphereInnerVlanMapId(v)},
		185 => value!(i, Attribute::VsaUnisphereCoreFacingInterface(i)),
		186 => value!(i, Attribute::VsaUnispherePcpPortMap(i)),
		187 => value!(i, Attribute::VsaUnisphereVcpeLanExtension(i)),
		188 => value!(i, Attribute::VsaUnisphereVcpeIpv4Offload(i)),
		191 => value!(i, Attribute::VsaJnprInputInterfaceFilter(i)),
		192 => value!(i, Attribute::VsaJnprOutputInterfaceFilter(i)),
		194 => map!{i, be_u32, |v| Attribute::VsaErxBulkCoaTransactionId(v)},
		195 => map!{i, be_u32, |v| Attribute::VsaErxBulkCoaIdentifier(v)},
		196 => value!(i, Attribute::VsaUnisphereIpv4InputServiceSet(i)),
		197 => value!(i, Attribute::VsaUnisphereIpv4OutputServiceSet(i)),
		198 => value!(i, Attribute::VsaUnisphereIpv4InputServiceFilter(i)),
		199 => value!(i, Attribute::VsaUnisphereIpv4OutputServiceFilter(i)),
		200 => value!(i, Attribute::VsaUnisphereIpv6InputServiceSet(i)),
		201 => value!(i, Attribute::VsaUnisphereIpv6OutputServiceSet(i)),
		202 => value!(i, Attribute::VsaUnisphereIpv6InputServiceFilter(i)),
		203 => value!(i, Attribute::VsaUnisphereIpv6OutputServiceFilter(i)),
		204 => value!(i, Attribute::VsaUnisphereAdvPcefProfileName(i)),
		205 => value!(i, Attribute::VsaUnisphereAdvPcefRuleName(i)),
        _ => value!(i, Attribute::VsaUnknown(4874, typ, i)),
    }
}
