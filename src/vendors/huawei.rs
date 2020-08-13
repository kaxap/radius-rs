/// Definitions for vendor Huawei, vendor value 2011
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HuaweiResultCode(pub u32);
 
#[allow(non_upper_case_globals)]
impl HuaweiResultCode {
	pub const Succeeded: HuaweiResultCode = HuaweiResultCode(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HuaweiPortalMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl HuaweiPortalMode {
	pub const Padm: HuaweiPortalMode = HuaweiPortalMode(0);
	pub const Redirectional: HuaweiPortalMode = HuaweiPortalMode(1);
	pub const NonCaptive: HuaweiPortalMode = HuaweiPortalMode(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HuaweiQosProfileType(pub u32);
 
#[allow(non_upper_case_globals)]
impl HuaweiQosProfileType {
	pub const Original: HuaweiQosProfileType = HuaweiQosProfileType(0);
	pub const L2TpInbound: HuaweiQosProfileType = HuaweiQosProfileType(1);
	pub const L2TpOutbound: HuaweiQosProfileType = HuaweiQosProfileType(2);
	pub const L2Tp: HuaweiQosProfileType = HuaweiQosProfileType(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HuaweiPublicIpAddrState(pub u32);
 
#[allow(non_upper_case_globals)]
impl HuaweiPublicIpAddrState {
	pub const Safe: HuaweiPublicIpAddrState = HuaweiPublicIpAddrState(0);
	pub const Warning: HuaweiPublicIpAddrState = HuaweiPublicIpAddrState(1);
	pub const Danger: HuaweiPublicIpAddrState = HuaweiPublicIpAddrState(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HuaweiAuthType(pub u32);
 
#[allow(non_upper_case_globals)]
impl HuaweiAuthType {
	pub const Ppp: HuaweiAuthType = HuaweiAuthType(1);
	pub const Web: HuaweiAuthType = HuaweiAuthType(2);
	pub const Dot1X: HuaweiAuthType = HuaweiAuthType(3);
	pub const Fast: HuaweiAuthType = HuaweiAuthType(4);
	pub const Bind: HuaweiAuthType = HuaweiAuthType(5);
	pub const Wlan: HuaweiAuthType = HuaweiAuthType(6);
	pub const Administrative: HuaweiAuthType = HuaweiAuthType(7);
	pub const Tunnel: HuaweiAuthType = HuaweiAuthType(8);
	pub const Mip: HuaweiAuthType = HuaweiAuthType(9);
	pub const None: HuaweiAuthType = HuaweiAuthType(10);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HuaweiPortMirror(pub u32);
 
#[allow(non_upper_case_globals)]
impl HuaweiPortMirror {
	pub const Disable: HuaweiPortMirror = HuaweiPortMirror(0);
	pub const UplinkEnable: HuaweiPortMirror = HuaweiPortMirror(1);
	pub const DownlinkEnable: HuaweiPortMirror = HuaweiPortMirror(2);
	pub const Enable: HuaweiPortMirror = HuaweiPortMirror(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HuaweiMngIpv6(pub u32);
 
#[allow(non_upper_case_globals)]
impl HuaweiMngIpv6 {
	pub const Unsupported: HuaweiMngIpv6 = HuaweiMngIpv6(0);
	pub const Supported: HuaweiMngIpv6 = HuaweiMngIpv6(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HuaweiApplicationType(pub u32);
 
#[allow(non_upper_case_globals)]
impl HuaweiApplicationType {
	pub const Fixed: HuaweiApplicationType = HuaweiApplicationType(1);
	pub const Nomadic: HuaweiApplicationType = HuaweiApplicationType(2);
	pub const Portable: HuaweiApplicationType = HuaweiApplicationType(3);
	pub const SimpleMobile: HuaweiApplicationType = HuaweiApplicationType(4);
	pub const FullMobile: HuaweiApplicationType = HuaweiApplicationType(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HuaweiUserPriority(pub u32);
 
#[allow(non_upper_case_globals)]
impl HuaweiUserPriority {
	pub const Common: HuaweiUserPriority = HuaweiUserPriority(0);
	pub const Copper: HuaweiUserPriority = HuaweiUserPriority(1);
	pub const Silver: HuaweiUserPriority = HuaweiUserPriority(2);
	pub const Gold: HuaweiUserPriority = HuaweiUserPriority(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaHuaweiInputBurstSize(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaHuaweiInputAverageRate(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaHuaweiInputPeakRate(v)},
		4 => map!{i, be_u32, |v| Attribute::VsaHuaweiOutputBurstSize(v)},
		5 => map!{i, be_u32, |v| Attribute::VsaHuaweiOutputAverageRate(v)},
		6 => map!{i, be_u32, |v| Attribute::VsaHuaweiOutputPeakRate(v)},
		7 => map!{i, be_u32, |v| Attribute::VsaHuaweiInKbBeforeTSwitch(v)},
		8 => map!{i, be_u32, |v| Attribute::VsaHuaweiOutKbBeforeTSwitch(v)},
		9 => map!{i, be_u32, |v| Attribute::VsaHuaweiInPktBeforeTSwitch(v)},
		10 => map!{i, be_u32, |v| Attribute::VsaHuaweiOutPktBeforeTSwitch(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaHuaweiInKbAfterTSwitch(v)},
		12 => map!{i, be_u32, |v| Attribute::VsaHuaweiOutKbAfterTSwitch(v)},
		13 => map!{i, be_u32, |v| Attribute::VsaHuaweiInPktAfterTSwitch(v)},
		14 => map!{i, be_u32, |v| Attribute::VsaHuaweiOutPktAfterTSwitch(v)},
		15 => map!{i, be_u32, |v| Attribute::VsaHuaweiRemanentVolume(v)},
		16 => map!{i, be_u32, |v| Attribute::VsaHuaweiTariffSwitchInterval(v)},
		17 => value!(i, Attribute::VsaHuaweiIspId(i)),
		18 => map!{i, be_u32, |v| Attribute::VsaHuaweiMaxUsersPerLogicPort(v)},
		20 => map!{i, be_u32, |v| Attribute::VsaHuaweiCommand(v)},
		22 => map!{i, be_u32, |v| Attribute::VsaHuaweiPriority(v)},
		24 => map!{i, be_u32, |v| Attribute::VsaHuaweiControlIdentifier(v)},
		25 => map! {i, be_u32, |v| Attribute::VsaHuaweiResultCode(HuaweiResultCode(v))},
		26 => map!{i, be_u32, |v| Attribute::VsaHuaweiConnectId(v)},
		27 => value!(i, Attribute::VsaHuaweiPortalurl(i)),
		28 => value!(i, Attribute::VsaHuaweiFtpDirectory(i)),
		29 => map!{i, be_u32, |v| Attribute::VsaHuaweiExecPrivilege(v)},
		30 => map!{i, be_u32, |v| Attribute::VsaHuaweiIpAddress(v)},
		31 => value!(i, Attribute::VsaHuaweiQosProfileName(i)),
		32 => value!(i, Attribute::VsaHuaweiSipServer(i)),
		33 => value!(i, Attribute::VsaHuaweiUserPassword(i)),
		34 => value!(i, Attribute::VsaHuaweiCommandMode(i)),
		35 => map!{i, be_u32, |v| Attribute::VsaHuaweiRenewalTime(v)},
		36 => map!{i, be_u32, |v| Attribute::VsaHuaweiRebindingTime(v)},
		37 => map!{i, be_u32, |v| Attribute::VsaHuaweiIgmpEnable(v)},
		39 => value!(i, Attribute::VsaHuaweiDestnationIpAddr(i)),
		40 => value!(i, Attribute::VsaHuaweiDestnationVolume(i)),
		59 => map!{i, be_u32, |v| Attribute::VsaHuaweiStartupStamp(v)},
		60 => value!(i, Attribute::VsaHuaweiIphostAddr(i)),
		61 => map!{i, be_u32, |v| Attribute::VsaHuaweiUpPriority(v)},
		62 => map!{i, be_u32, |v| Attribute::VsaHuaweiDownPriority(v)},
		63 => value!(i, Attribute::VsaHuaweiTunnelVpnInstance(i)),
		64 => map!{i, be_u32, |v| Attribute::VsaHuaweiVtName(v)},
		65 => value!(i, Attribute::VsaHuaweiUserDate(i)),
		66 => value!(i, Attribute::VsaHuaweiUserClass(i)),
		70 => map!{i, be_u32, |v| Attribute::VsaHuaweiPppNcpType(v)},
		71 => value!(i, Attribute::VsaHuaweiVsiName(i)),
		72 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiSubnetMask(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		73 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiGatewayAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		74 => map!{i, be_u32, |v| Attribute::VsaHuaweiLeaseTime(v)},
		75 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiPrimaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		76 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiSecondaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		77 => map!{i, be_u32, |v| Attribute::VsaHuaweiInputPeakBurstSize(v)},
		78 => map!{i, be_u32, |v| Attribute::VsaHuaweiOutputPeakBurstSize(v)},
		79 => map!{i, be_u32, |v| Attribute::VsaHuaweiReducedCir(v)},
		80 => map!{i, be_u32, |v| Attribute::VsaHuaweiTunnelSessionLimit(v)},
		81 => value!(i, Attribute::VsaHuaweiZoneName(i)),
		82 => value!(i, Attribute::VsaHuaweiDataFilter(i)),
		83 => value!(i, Attribute::VsaHuaweiAccessService(i)),
		84 => map!{i, be_u32, |v| Attribute::VsaHuaweiAccountingLevel(v)},
		85 => map! {i, be_u32, |v| Attribute::VsaHuaweiPortalMode(HuaweiPortalMode(v))},
		86 => value!(i, Attribute::VsaHuaweiDpiPolicyName(i)),
		87 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiPolicyRoute(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		88 => value!(i, Attribute::VsaHuaweiFramedPool(i)),
		89 => value!(i, Attribute::VsaHuaweiL2TpTerminateCause(i)),
		90 => map!{i, be_u32, |v| Attribute::VsaHuaweiMultiAccountMode(v)},
		91 => value!(i, Attribute::VsaHuaweiQueueProfile(i)),
		92 => map!{i, be_u32, |v| Attribute::VsaHuaweiLayer4SessionLimit(v)},
		93 => value!(i, Attribute::VsaHuaweiMulticastProfile(i)),
		94 => value!(i, Attribute::VsaHuaweiVpnInstance(i)),
		95 => value!(i, Attribute::VsaHuaweiPolicyName(i)),
		96 => value!(i, Attribute::VsaHuaweiTunnelGroupName(i)),
		97 => value!(i, Attribute::VsaHuaweiMulticastSourceGroup(i)),
		98 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiMulticastReceiveGroup(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		99 => map!{i, be_u32, |v| Attribute::VsaHuaweiUserMulticastType(v)},
		100 => map!{i, be_u32, |v| Attribute::VsaHuaweiReducedPir(v)},
		101 => value!(i, Attribute::VsaHuaweiLiId(i)),
		102 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiLiMdAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		103 => map!{i, be_u32, |v| Attribute::VsaHuaweiLiMdPort(v)},
		104 => value!(i, Attribute::VsaHuaweiLiMdVpninstance(i)),
		105 => map!{i, be_u32, |v| Attribute::VsaHuaweiServiceChgCmd(v)},
		106 => map!{i, be_u32, |v| Attribute::VsaHuaweiAcctPacketType(v)},
		107 => map!{i, be_u32, |v| Attribute::VsaHuaweiCallReference(v)},
		108 => map!{i, be_u32, |v| Attribute::VsaHuaweiPstnPort(v)},
		109 => map!{i, be_u32, |v| Attribute::VsaHuaweiVoipServiceType(v)},
		110 => map!{i, be_u32, |v| Attribute::VsaHuaweiAcctConnectionTime(v)},
		112 => map!{i, be_u32, |v| Attribute::VsaHuaweiErrorReason(v)},
		113 => map!{i, be_u32, |v| Attribute::VsaHuaweiRemainMonney(v)},
		123 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiOrgGkIpaddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		124 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiOrgGwIpaddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		125 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiDstGkIpaddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		126 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiDstGwIpaddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		127 => value!(i, Attribute::VsaHuaweiAccessNum(i)),
		128 => map!{i, be_u32, |v| Attribute::VsaHuaweiRemainTime(v)},
		131 => map!{i, be_u32, |v| Attribute::VsaHuaweiCodecType(v)},
		132 => value!(i, Attribute::VsaHuaweiTransferNum(i)),
		133 => value!(i, Attribute::VsaHuaweiNewUserName(i)),
		134 => value!(i, Attribute::VsaHuaweiTransferStationId(i)),
		135 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiPrimaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		136 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiSecondaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		137 => map!{i, be_u32, |v| Attribute::VsaHuaweiOnlyAccountType(v)},
		138 => value!(i, Attribute::VsaHuaweiDomainName(i)),
		139 => value!(i, Attribute::VsaHuaweiAncpProfile(i)),
		140 => value!(i, Attribute::VsaHuaweiHttpRedirectUrl(i)),
		141 => value!(i, Attribute::VsaHuaweiLoopbackAddress(i)),
		142 => map! {i, be_u32, |v| Attribute::VsaHuaweiQosProfileType(HuaweiQosProfileType(v))},
		143 => map!{i, be_u32, |v| Attribute::VsaHuaweiMaxListNum(v)},
		144 => map!{i, be_u32, |v| Attribute::VsaHuaweiAcctIpv6InputOctets(v)},
		145 => map!{i, be_u32, |v| Attribute::VsaHuaweiAcctIpv6OutputOctets(v)},
		146 => map!{i, be_u32, |v| Attribute::VsaHuaweiAcctIpv6InputPackets(v)},
		147 => map!{i, be_u32, |v| Attribute::VsaHuaweiAcctIpv6OutputPackets(v)},
		148 => map!{i, be_u32, |v| Attribute::VsaHuaweiAcctIpv6InputGigawords(v)},
		149 => map!{i, be_u32, |v| Attribute::VsaHuaweiAcctIpv6OutputGigawords(v)},
		150 => value!(i, Attribute::VsaHuaweiDhcpv6Option37(i)),
		151 => value!(i, Attribute::VsaHuaweiDhcpv6Option38(i)),
		153 => value!(i, Attribute::VsaHuaweiUserMac(i)),
		154 => value!(i, Attribute::VsaHuaweiDnsServerIpv6Address(i)),
		155 => value!(i, Attribute::VsaHuaweiDhcpv4Option121(i)),
		156 => value!(i, Attribute::VsaHuaweiDhcpv4Option43(i)),
		157 => value!(i, Attribute::VsaHuaweiFramedPoolGroup(i)),
		158 => value!(i, Attribute::VsaHuaweiFramedIpv6Address(i)),
		159 => map!{i, be_u32, |v| Attribute::VsaHuaweiAcctUpdateAddress(v)},
		160 => value!(i, Attribute::VsaHuaweiNatPolicyName(i)),
		161 => value!(i, Attribute::VsaHuaweiNatPublicAddress(i)),
		162 => value!(i, Attribute::VsaHuaweiNatStartPort(i)),
		163 => value!(i, Attribute::VsaHuaweiNatEndPort(i)),
		164 => value!(i, Attribute::VsaHuaweiNatPortForwarding(i)),
		165 => map!{i, be_u32, |v| Attribute::VsaHuaweiNatPortRangeUpdate(v)},
		166 => value!(i, Attribute::VsaHuaweiDsLiteTunnelName(i)),
		167 => value!(i, Attribute::VsaHuaweiPcpServerName(i)),
		168 => map! {i, be_u32, |v| Attribute::VsaHuaweiPublicIpAddrState(HuaweiPublicIpAddrState(v))},
		180 => map! {i, be_u32, |v| Attribute::VsaHuaweiAuthType(HuaweiAuthType(v))},
		181 => value!(i, Attribute::VsaHuaweiAcctTerminateSubcause(i)),
		182 => value!(i, Attribute::VsaHuaweiDownQosProfileName(i)),
		183 => map! {i, be_u32, |v| Attribute::VsaHuaweiPortMirror(HuaweiPortMirror(v))},
		184 => value!(i, Attribute::VsaHuaweiAccountInfo(i)),
		185 => value!(i, Attribute::VsaHuaweiServiceInfo(i)),
		187 => value!(i, Attribute::VsaHuaweiDhcpOption(i)),
		188 => value!(i, Attribute::VsaHuaweiAvpair(i)),
		191 => value!(i, Attribute::VsaHuaweiDelegatedIpv6PrefixPool(i)),
		192 => value!(i, Attribute::VsaHuaweiIpv6PrefixLease(i)),
		193 => value!(i, Attribute::VsaHuaweiIpv6AddressLease(i)),
		194 => value!(i, Attribute::VsaHuaweiIpv6PolicyRoute(i)),
		196 => map! {i, be_u32, |v| Attribute::VsaHuaweiMngIpv6(HuaweiMngIpv6(v))},
		211 => value!(i, Attribute::VsaHuaweiFlowInfo(i)),
		212 => map!{i, be_u32, |v| Attribute::VsaHuaweiFlowId(v)},
		214 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiDhcpServerIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		215 => map! {i, be_u32, |v| Attribute::VsaHuaweiApplicationType(HuaweiApplicationType(v))},
		216 => value!(i, Attribute::VsaHuaweiIndicationFlag(i)),
		217 => map!{i, take!(4), |v:&[u8]| Attribute::VsaHuaweiOriginalNasIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		218 => map! {i, be_u32, |v| Attribute::VsaHuaweiUserPriority(HuaweiUserPriority(v))},
		219 => value!(i, Attribute::VsaHuaweiAcsUrl(i)),
		220 => value!(i, Attribute::VsaHuaweiProvisionCode(i)),
		221 => value!(i, Attribute::VsaHuaweiApplicationScene(i)),
		222 => value!(i, Attribute::VsaHuaweiMsMaximumMacStudyNumber(i)),
		232 => value!(i, Attribute::VsaHuaweiGgsnVendor(i)),
		233 => value!(i, Attribute::VsaHuaweiGgsnVersion(i)),
		253 => value!(i, Attribute::VsaHuaweiWebUrl(i)),
		254 => value!(i, Attribute::VsaHuaweiVersion(i)),
		255 => value!(i, Attribute::VsaHuaweiProductId(i)),
        _ => value!(i, Attribute::VsaUnknown(2011, typ, i)),
    }
}
