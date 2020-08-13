/// Definitions for vendor Ericsson-AB, vendor value 2352
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PvcEncapsulationType(pub u32);
 
#[allow(non_upper_case_globals)]
impl PvcEncapsulationType {
	pub const AaaEncapsAtmRaw: PvcEncapsulationType = PvcEncapsulationType(1);
	pub const AaaEncapsAtmRoute1483: PvcEncapsulationType = PvcEncapsulationType(2);
	pub const AaaEncapsAtmAuto1483: PvcEncapsulationType = PvcEncapsulationType(3);
	pub const AaaEncapsAtmMulti: PvcEncapsulationType = PvcEncapsulationType(4);
	pub const AaaEncapsAtmBridge1483: PvcEncapsulationType = PvcEncapsulationType(5);
	pub const AaaEncapsAtmPpp: PvcEncapsulationType = PvcEncapsulationType(6);
	pub const AaaEncapsAtmPppSerial: PvcEncapsulationType = PvcEncapsulationType(7);
	pub const AaaEncapsAtmPppNlpid: PvcEncapsulationType = PvcEncapsulationType(8);
	pub const AaaEncapsAtmPppAuto: PvcEncapsulationType = PvcEncapsulationType(9);
	pub const AaaEncapsAtmPppoe: PvcEncapsulationType = PvcEncapsulationType(10);
	pub const AaaEncapsAtmL2Tp: PvcEncapsulationType = PvcEncapsulationType(11);
	pub const AaaEncapsAtmPppLlc: PvcEncapsulationType = PvcEncapsulationType(12);
	pub const AaaEncapsFrameAuto1490: PvcEncapsulationType = PvcEncapsulationType(13);
	pub const AaaEncapsFrameMulti: PvcEncapsulationType = PvcEncapsulationType(14);
	pub const AaaEncapsFrameBridge1490: PvcEncapsulationType = PvcEncapsulationType(15);
	pub const AaaEncapsFramePpp: PvcEncapsulationType = PvcEncapsulationType(16);
	pub const AaaEncapsFramePppAuto: PvcEncapsulationType = PvcEncapsulationType(17);
	pub const AaaEncapsFramePppoe: PvcEncapsulationType = PvcEncapsulationType(18);
	pub const AaaEncapsFrameRoute1490: PvcEncapsulationType = PvcEncapsulationType(19);
	pub const AaaEncapsFrameL2Tp: PvcEncapsulationType = PvcEncapsulationType(20);
	pub const AaaEncapsL2TpVcMuxed: PvcEncapsulationType = PvcEncapsulationType(21);
	pub const AaaEncapsEth: PvcEncapsulationType = PvcEncapsulationType(22);
	pub const AaaEncapsEthPppoe: PvcEncapsulationType = PvcEncapsulationType(23);
	pub const AaaEncapsEthMulti: PvcEncapsulationType = PvcEncapsulationType(24);
	pub const AaaEncapsEthDot1Q: PvcEncapsulationType = PvcEncapsulationType(25);
	pub const AaaEncapsEthDot1QPppoe: PvcEncapsulationType = PvcEncapsulationType(26);
	pub const AaaEncapsAtmMultiPppoe: PvcEncapsulationType = PvcEncapsulationType(27);
	pub const AaaEncapsAtmMultiIpv6Oe: PvcEncapsulationType = PvcEncapsulationType(28);
	pub const AaaEncapsAtmMultiPppoeNIpv6Oe: PvcEncapsulationType = PvcEncapsulationType(29);
	pub const AaaEncapsEthDot1QTunnel: PvcEncapsulationType = PvcEncapsulationType(30);
	pub const AaaEncapsEthDot1QTunnelPppoe: PvcEncapsulationType = PvcEncapsulationType(31);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PvcCircuitPadding(pub u32);
 
#[allow(non_upper_case_globals)]
impl PvcCircuitPadding {
	pub const AaaCircuitPadding: PvcCircuitPadding = PvcCircuitPadding(1);
	pub const AaaCircuitNoPadding: PvcCircuitPadding = PvcCircuitPadding(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BindType(pub u32);
 
#[allow(non_upper_case_globals)]
impl BindType {
	pub const AaaAuthBind: BindType = BindType(1);
	pub const AaaBypassBind: BindType = BindType(2);
	pub const AaaInterfaceBind: BindType = BindType(3);
	pub const AaaSubscribeBind: BindType = BindType(4);
	pub const AaaTunnelBind: BindType = BindType(5);
	pub const AaaSessionBind: BindType = BindType(6);
	pub const AaaQ8021Bind: BindType = BindType(7);
	pub const AaaMultiBind: BindType = BindType(8);
	pub const AaaDhcpBind: BindType = BindType(9);
	pub const AaaMultiBindSub: BindType = BindType(10);
	pub const AaaBridgeGroupBind: BindType = BindType(11);
	pub const AaaVlanBind: BindType = BindType(12);
	pub const AaaVlanGroupBind: BindType = BindType(13);
	pub const AaaAutoSubscriberBind: BindType = BindType(14);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BindAuthProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl BindAuthProtocol {
	pub const AaaPppPap: BindAuthProtocol = BindAuthProtocol(1);
	pub const AaaPppChap: BindAuthProtocol = BindAuthProtocol(2);
	pub const AaaPppChapWait: BindAuthProtocol = BindAuthProtocol(3);
	pub const AaaPppChapPap: BindAuthProtocol = BindAuthProtocol(4);
	pub const AaaPppChapWaitPap: BindAuthProtocol = BindAuthProtocol(5);
	pub const AaaPppEap: BindAuthProtocol = BindAuthProtocol(6);
	pub const AaaPppPapChap: BindAuthProtocol = BindAuthProtocol(7);
	pub const AaaPppPapChapWait: BindAuthProtocol = BindAuthProtocol(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceValidation(pub u32);
 
#[allow(non_upper_case_globals)]
impl SourceValidation {
	pub const Enabled: SourceValidation = SourceValidation(1);
	pub const Disabled: SourceValidation = SourceValidation(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TunnelDomain(pub u32);
 
#[allow(non_upper_case_globals)]
impl TunnelDomain {
	pub const Enabled: TunnelDomain = TunnelDomain(1);
	pub const Disabled: TunnelDomain = TunnelDomain(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TunnelFunction(pub u32);
 
#[allow(non_upper_case_globals)]
impl TunnelFunction {
	pub const LacOnly: TunnelFunction = TunnelFunction(1);
	pub const LnsOnly: TunnelFunction = TunnelFunction(2);
	pub const LacLns: TunnelFunction = TunnelFunction(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TunnelSessionAuth(pub u32);
 
#[allow(non_upper_case_globals)]
impl TunnelSessionAuth {
	pub const Chap: TunnelSessionAuth = TunnelSessionAuth(1);
	pub const Pap: TunnelSessionAuth = TunnelSessionAuth(2);
	pub const ChapPap: TunnelSessionAuth = TunnelSessionAuth(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TunnelGroup(pub u32);
 
#[allow(non_upper_case_globals)]
impl TunnelGroup {
	pub const Enabled: TunnelGroup = TunnelGroup(1);
	pub const Disabled: TunnelGroup = TunnelGroup(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TunnelAlgorithm(pub u32);
 
#[allow(non_upper_case_globals)]
impl TunnelAlgorithm {
	pub const First: TunnelAlgorithm = TunnelAlgorithm(1);
	pub const LoadBalance: TunnelAlgorithm = TunnelAlgorithm(2);
	pub const Wrr: TunnelAlgorithm = TunnelAlgorithm(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct McastSend(pub u32);
 
#[allow(non_upper_case_globals)]
impl McastSend {
	pub const NoSend: McastSend = McastSend(1);
	pub const Send: McastSend = McastSend(2);
	pub const UnsolicitedSend: McastSend = McastSend(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct McastReceive(pub u32);
 
#[allow(non_upper_case_globals)]
impl McastReceive {
	pub const NoReceive: McastReceive = McastReceive(1);
	pub const Receive: McastReceive = McastReceive(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TunnelDnis(pub u32);
 
#[allow(non_upper_case_globals)]
impl TunnelDnis {
	pub const Dnis: TunnelDnis = TunnelDnis(1);
	pub const DnisOnly: TunnelDnis = TunnelDnis(2);
	pub const DnisGenerate: TunnelDnis = TunnelDnis(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlatformType(pub u32);
 
#[allow(non_upper_case_globals)]
impl PlatformType {
	pub const Sms: PlatformType = PlatformType(1);
	pub const Smartedge800: PlatformType = PlatformType(2);
	pub const Se400: PlatformType = PlatformType(3);
	pub const Se100: PlatformType = PlatformType(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CircuitProtocolEncap(pub u32);
 
#[allow(non_upper_case_globals)]
impl CircuitProtocolEncap {
	pub const EncapsPppoe: CircuitProtocolEncap = CircuitProtocolEncap(27);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MediumType(pub u32);
 
#[allow(non_upper_case_globals)]
impl MediumType {
	pub const Dsl: MediumType = MediumType(11);
	pub const Cable: MediumType = MediumType(12);
	pub const Wireless: MediumType = MediumType(13);
	pub const Satellite: MediumType = MediumType(14);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IpTosField(pub u32);
 
#[allow(non_upper_case_globals)]
impl IpTosField {
	pub const Normal: IpTosField = IpTosField(0);
	pub const MinCostOnly: IpTosField = IpTosField(1);
	pub const MaxReliabilityOnly: IpTosField = IpTosField(2);
	pub const MaxReliabilityPlusMinCost: IpTosField = IpTosField(3);
	pub const MaxThroughputOnly: IpTosField = IpTosField(4);
	pub const MaxThroughputPlusMinCost: IpTosField = IpTosField(5);
	pub const MaxThroughputPlusMaxReliability: IpTosField = IpTosField(6);
	pub const MaxThroughputPlusMaxReliabilityPlusMinCost: IpTosField = IpTosField(7);
	pub const MinDelayOnly: IpTosField = IpTosField(8);
	pub const MinDelayPlusMinCost: IpTosField = IpTosField(9);
	pub const MinDelayPlusMaxReliability: IpTosField = IpTosField(10);
	pub const MinDelayPlusMaxReliabilityPlusMinCost: IpTosField = IpTosField(11);
	pub const MinDelayPlusMaxThroughput: IpTosField = IpTosField(12);
	pub const MinDelayPlusMaxThroughputPlusMinCost: IpTosField = IpTosField(13);
	pub const MinDelayPlusMaxThroughputPlusMaxReliability: IpTosField = IpTosField(14);
	pub const MinDelayPlusMaxThroughputPlusMaxReliabilityPlusMinCost: IpTosField = IpTosField(15);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LacPortType(pub u32);
 
#[allow(non_upper_case_globals)]
impl LacPortType {
	pub const NasPortType10Bt: LacPortType = LacPortType(40);
	pub const NasPortType100Bt: LacPortType = LacPortType(41);
	pub const NasPortTypeDs3Fr: LacPortType = LacPortType(42);
	pub const NasPortTypeDs3Atm: LacPortType = LacPortType(43);
	pub const NasPortTypeOc3: LacPortType = LacPortType(44);
	pub const NasPortTypeHssi: LacPortType = LacPortType(45);
	pub const NasPortTypeEia530: LacPortType = LacPortType(46);
	pub const NasPortTypeT1: LacPortType = LacPortType(47);
	pub const NasPortTypeChanT3: LacPortType = LacPortType(48);
	pub const NasPortTypeDs1Fr: LacPortType = LacPortType(49);
	pub const NasPortTypeE3Atm: LacPortType = LacPortType(50);
	pub const NasPortTypeImaAtm: LacPortType = LacPortType(51);
	pub const NasPortTypeDs3Atm2: LacPortType = LacPortType(52);
	pub const NasPortTypeOc3Atm2: LacPortType = LacPortType(53);
	pub const NasPortType1000Bsx: LacPortType = LacPortType(54);
	pub const NasPortTypeE1Fr: LacPortType = LacPortType(55);
	pub const NasPortTypeE1Atm: LacPortType = LacPortType(56);
	pub const NasPortTypeE3Fr: LacPortType = LacPortType(57);
	pub const NasPortTypeOc3Pos: LacPortType = LacPortType(58);
	pub const NasPortTypeOc12Pos: LacPortType = LacPortType(59);
	pub const NasPortTypePppoe: LacPortType = LacPortType(60);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LacRealPortType(pub u32);
 
#[allow(non_upper_case_globals)]
impl LacRealPortType {
	pub const NasPortType10Bt: LacRealPortType = LacRealPortType(40);
	pub const NasPortType100Bt: LacRealPortType = LacRealPortType(41);
	pub const NasPortTypeDs3Fr: LacRealPortType = LacRealPortType(42);
	pub const NasPortTypeDs3Atm: LacRealPortType = LacRealPortType(43);
	pub const NasPortTypeOc3: LacRealPortType = LacRealPortType(44);
	pub const NasPortTypeHssi: LacRealPortType = LacRealPortType(45);
	pub const NasPortTypeEia530: LacRealPortType = LacRealPortType(46);
	pub const NasPortTypeT1: LacRealPortType = LacRealPortType(47);
	pub const NasPortTypeChanT3: LacRealPortType = LacRealPortType(48);
	pub const NasPortTypeDs1Fr: LacRealPortType = LacRealPortType(49);
	pub const NasPortTypeE3Atm: LacRealPortType = LacRealPortType(50);
	pub const NasPortTypeImaAtm: LacRealPortType = LacRealPortType(51);
	pub const NasPortTypeDs3Atm2: LacRealPortType = LacRealPortType(52);
	pub const NasPortTypeOc3Atm2: LacRealPortType = LacRealPortType(53);
	pub const NasPortType1000Bsx: LacRealPortType = LacRealPortType(54);
	pub const NasPortTypeE1Fr: LacRealPortType = LacRealPortType(55);
	pub const NasPortTypeE1Atm: LacRealPortType = LacRealPortType(56);
	pub const NasPortTypeE3Fr: LacRealPortType = LacRealPortType(57);
	pub const NasPortTypeOc3Pos: LacRealPortType = LacRealPortType(58);
	pub const NasPortTypeOc12Pos: LacRealPortType = LacRealPortType(59);
	pub const NasPortTypePppoe: LacRealPortType = LacRealPortType(60);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AcctUpdateReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl AcctUpdateReason {
	pub const AaaLoadAcctSessionUp: AcctUpdateReason = AcctUpdateReason(1);
	pub const AaaLoadAcctSessionDown: AcctUpdateReason = AcctUpdateReason(2);
	pub const AaaLoadAcctPeriodic: AcctUpdateReason = AcctUpdateReason(3);
	pub const AaaLoadAcctDynAcEntStart: AcctUpdateReason = AcctUpdateReason(4);
	pub const AaaLoadAcctDynAcEntStop: AcctUpdateReason = AcctUpdateReason(5);
	pub const AaaLoadAcctDynAcEntTimeout: AcctUpdateReason = AcctUpdateReason(6);
	pub const AaaLoadAcctSubscriberReauthor: AcctUpdateReason = AcctUpdateReason(7);
	pub const AaaLoadAcctPppIpcpUp: AcctUpdateReason = AcctUpdateReason(8);
	pub const AaaLoadAcctPppMpLinkUp: AcctUpdateReason = AcctUpdateReason(9);
	pub const AaaLoadAcctDhcpIpAddrGranted: AcctUpdateReason = AcctUpdateReason(10);
	pub const AaaLoadAcctDhcpIpAddrReleased: AcctUpdateReason = AcctUpdateReason(11);
	pub const AaaLoadAcctAclTimeredAction: AcctUpdateReason = AcctUpdateReason(12);
	pub const AaaLoadAcctAclAction: AcctUpdateReason = AcctUpdateReason(13);
	pub const AaaLoadAcctCmd: AcctUpdateReason = AcctUpdateReason(14);
	pub const AaaLoadAcctTest: AcctUpdateReason = AcctUpdateReason(15);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DslLineState(pub u32);
 
#[allow(non_upper_case_globals)]
impl DslLineState {
	pub const Showtime: DslLineState = DslLineState(1);
	pub const Idle: DslLineState = DslLineState(2);
	pub const Silent: DslLineState = DslLineState(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DslTransmissionSystem(pub u32);
 
#[allow(non_upper_case_globals)]
impl DslTransmissionSystem {
	pub const Adsl1: DslTransmissionSystem = DslTransmissionSystem(1);
	pub const Adsl2: DslTransmissionSystem = DslTransmissionSystem(2);
	pub const Adsl2Plus: DslTransmissionSystem = DslTransmissionSystem(3);
	pub const Vdsl1: DslTransmissionSystem = DslTransmissionSystem(4);
	pub const Vdsl2: DslTransmissionSystem = DslTransmissionSystem(5);
	pub const Sdsl: DslTransmissionSystem = DslTransmissionSystem(6);
	pub const Unknown: DslTransmissionSystem = DslTransmissionSystem(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ServiceAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl ServiceAction {
	pub const DeActivate: ServiceAction = ServiceAction(0);
	pub const ActivateWithAcct: ServiceAction = ServiceAction(1);
	pub const ActivateWithoutAcct: ServiceAction = ServiceAction(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, take!(4), |v:&[u8]| Attribute::VsaClientDnsPri(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		2 => map!{i, take!(4), |v:&[u8]| Attribute::VsaClientDnsSec(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		3 => map!{i, be_u32, |v| Attribute::VsaDhcpMaxLeases(v)},
		4 => value!(i, Attribute::VsaContextName(i)),
		5 => value!(i, Attribute::VsaBridgeGroup(i)),
		6 => value!(i, Attribute::VsaBgAgingTime(i)),
		7 => value!(i, Attribute::VsaBgPathCost(i)),
		8 => value!(i, Attribute::VsaBgSpanDis(i)),
		9 => value!(i, Attribute::VsaBgTransBpdu(i)),
		10 => map!{i, be_u32, |v| Attribute::VsaRateLimitRate(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaRateLimitBurst(v)},
		12 => map!{i, be_u32, |v| Attribute::VsaPoliceRate(v)},
		13 => map!{i, be_u32, |v| Attribute::VsaPoliceBurst(v)},
		14 => map! {i, be_u32, |v| Attribute::VsaSourceValidation(SourceValidation(v))},
		15 => map! {i, be_u32, |v| Attribute::VsaTunnelDomain(TunnelDomain(v))},
		16 => value!(i, Attribute::VsaTunnelLocalName(i)),
		17 => value!(i, Attribute::VsaTunnelRemoteName(i)),
		18 => map! {i, be_u32, |v| Attribute::VsaTunnelFunction(TunnelFunction(v))},
		19 => map!{i, be_u32, |v| Attribute::VsaTunnelFlowControl(v)},
		20 => map!{i, be_u32, |v| Attribute::VsaTunnelStatic(v)},
		21 => map!{i, be_u32, |v| Attribute::VsaTunnelMaxSessions(v)},
		22 => map!{i, be_u32, |v| Attribute::VsaTunnelMaxTunnels(v)},
		23 => map! {i, be_u32, |v| Attribute::VsaTunnelSessionAuth(TunnelSessionAuth(v))},
		24 => map!{i, be_u32, |v| Attribute::VsaTunnelWindow(v)},
		25 => map!{i, be_u32, |v| Attribute::VsaTunnelRetransmit(v)},
		26 => map!{i, be_u32, |v| Attribute::VsaTunnelCmdTimeout(v)},
		27 => value!(i, Attribute::VsaPppoeUrl(i)),
		28 => value!(i, Attribute::VsaPppoeMotm(i)),
		29 => map! {i, be_u32, |v| Attribute::VsaTunnelGroup(TunnelGroup(v))},
		30 => value!(i, Attribute::VsaTunnelContext(i)),
		31 => map! {i, be_u32, |v| Attribute::VsaTunnelAlgorithm(TunnelAlgorithm(v))},
		32 => map!{i, be_u32, |v| Attribute::VsaTunnelDeadtime(v)},
		33 => map! {i, be_u32, |v| Attribute::VsaMcastSend(McastSend(v))},
		34 => map! {i, be_u32, |v| Attribute::VsaMcastReceive(McastReceive(v))},
		35 => map!{i, be_u32, |v| Attribute::VsaMcastMaxgroups(v)},
		36 => value!(i, Attribute::VsaIpAddressPoolName(i)),
		37 => map! {i, be_u32, |v| Attribute::VsaTunnelDnis(TunnelDnis(v))},
		38 => map! {i, be_u32, |v| Attribute::VsaMediumType(MediumType(v))},
		39 => map! {i, be_u32, |v| Attribute::VsaPvcEncapsulationType(PvcEncapsulationType(v))},
		40 => value!(i, Attribute::VsaPvcProfileName(i)),
		41 => map! {i, be_u32, |v| Attribute::VsaPvcCircuitPadding(PvcCircuitPadding(v))},
		42 => map! {i, be_u32, |v| Attribute::VsaBindType(BindType(v))},
		43 => map! {i, be_u32, |v| Attribute::VsaBindAuthProtocol(BindAuthProtocol(v))},
		44 => map!{i, be_u32, |v| Attribute::VsaBindAuthMaxSessions(v)},
		45 => value!(i, Attribute::VsaBindBypassBypass(i)),
		46 => value!(i, Attribute::VsaBindAuthContext(i)),
		47 => value!(i, Attribute::VsaBindAuthServiceGrp(i)),
		48 => value!(i, Attribute::VsaBindBypassContext(i)),
		49 => value!(i, Attribute::VsaBindIntContext(i)),
		50 => value!(i, Attribute::VsaBindTunContext(i)),
		51 => value!(i, Attribute::VsaBindSesContext(i)),
		52 => map!{i, be_u32, |v| Attribute::VsaBindDot1QSlot(v)},
		53 => map!{i, be_u32, |v| Attribute::VsaBindDot1QPort(v)},
		54 => map!{i, be_u32, |v| Attribute::VsaBindDot1QVlanTagId(v)},
		55 => value!(i, Attribute::VsaBindIntInterfaceName(i)),
		56 => value!(i, Attribute::VsaBindL2TpTunnelName(i)),
		57 => map!{i, be_u32, |v| Attribute::VsaBindL2TpFlowControl(v)},
		58 => value!(i, Attribute::VsaBindSubUserAtContext(i)),
		59 => value!(i, Attribute::VsaBindSubPassword(i)),
		60 => value!(i, Attribute::VsaIpHostAddr(i)),
		61 => map! {i, be_u32, |v| Attribute::VsaIpTosField(IpTosField(v))},
		62 => map!{i, be_u32, |v| Attribute::VsaNasRealPort(v)},
		63 => value!(i, Attribute::VsaTunnelSessionAuthCtx(i)),
		64 => value!(i, Attribute::VsaTunnelSessionAuthServiceGrp(i)),
		65 => map!{i, be_u32, |v| Attribute::VsaTunnelRateLimitRate(v)},
		66 => map!{i, be_u32, |v| Attribute::VsaTunnelRateLimitBurst(v)},
		67 => map!{i, be_u32, |v| Attribute::VsaTunnelPoliceRate(v)},
		68 => map!{i, be_u32, |v| Attribute::VsaTunnelPoliceBurst(v)},
		69 => value!(i, Attribute::VsaTunnelL2FSecondPassword(i)),
		70 => value!(i, Attribute::VsaAclDefinition(i)),
		71 => value!(i, Attribute::VsaPppoeIpRouteAdd(i)),
		72 => map!{i, be_u32, |v| Attribute::VsaTtyLevelMax(v)},
		73 => map!{i, be_u32, |v| Attribute::VsaTtyLevelStart(v)},
		74 => map!{i, be_u32, |v| Attribute::VsaTunnelChecksum(v)},
		75 => value!(i, Attribute::VsaTunnelProfile(i)),
		78 => value!(i, Attribute::VsaTunnelClientVpn(i)),
		79 => value!(i, Attribute::VsaTunnelServerVpn(i)),
		80 => value!(i, Attribute::VsaTunnelClientRhost(i)),
		81 => value!(i, Attribute::VsaTunnelServerRhost(i)),
		82 => map!{i, take!(4), |v:&[u8]| Attribute::VsaTunnelClientIntAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		83 => map!{i, take!(4), |v:&[u8]| Attribute::VsaTunnelServerIntAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		84 => map!{i, be_u32, |v| Attribute::VsaPppCompression(v)},
		85 => map!{i, be_u32, |v| Attribute::VsaTunnelHelloTimer(v)},
		86 => map!{i, be_u32, |v| Attribute::VsaRedbackReason(v)},
		87 => value!(i, Attribute::VsaQosPolicingProfileName(i)),
		88 => value!(i, Attribute::VsaQosMeteringProfileName(i)),
		89 => value!(i, Attribute::VsaQosPolicyQueuing(i)),
		90 => value!(i, Attribute::VsaIgmpServiceProfileName(i)),
		91 => value!(i, Attribute::VsaSubscriberProfileName(i)),
		92 => value!(i, Attribute::VsaForwardPolicy(i)),
		93 => value!(i, Attribute::VsaRemotePort(i)),
		94 => value!(i, Attribute::VsaReauth(i)),
		95 => map!{i, be_u32, |v| Attribute::VsaReauthMore(v)},
		96 => value!(i, Attribute::VsaAgentRemoteId(i)),
		97 => value!(i, Attribute::VsaAgentCircuitId(i)),
		98 => map! {i, be_u32, |v| Attribute::VsaPlatformType(PlatformType(v))},
		99 => map!{i, take!(4), |v:&[u8]| Attribute::VsaClientNbnsPri(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		100 => map!{i, take!(4), |v:&[u8]| Attribute::VsaClientNbnsSec(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		101 => value!(i, Attribute::VsaShapingProfileName(i)),
		103 => map!{i, be_u32, |v| Attribute::VsaBgCctAddrMax(v)},
		104 => value!(i, Attribute::VsaIpInterfaceName(i)),
		105 => value!(i, Attribute::VsaNatPolicyName(i)),
		106 => value!(i, Attribute::VsaRbNpmServiceId(i)),
		107 => value!(i, Attribute::VsaHttpRedirectProfileName(i)),
		108 => value!(i, Attribute::VsaBindAutoSubUser(i)),
		109 => value!(i, Attribute::VsaBindAutoSubContext(i)),
		110 => value!(i, Attribute::VsaBindAutoSubPassword(i)),
		111 => map! {i, be_u32, |v| Attribute::VsaCircuitProtocolEncap(CircuitProtocolEncap(v))},
		112 => value!(i, Attribute::VsaOsVersion(i)),
		113 => value!(i, Attribute::VsaSessionTrafficLimit(i)),
		114 => value!(i, Attribute::VsaQosReference(i)),
		121 => value!(i, Attribute::VsaRateLimitExcessBurst(i)),
		122 => value!(i, Attribute::VsaPoliceExcessBurst(i)),
		123 => value!(i, Attribute::VsaTunnelRateLimitExcessBurst(i)),
		124 => value!(i, Attribute::VsaTunnelPoliceExcessBurst(i)),
		125 => value!(i, Attribute::VsaDhcpVendorClassId(i)),
		126 => value!(i, Attribute::VsaQosRate(i)),
		127 => value!(i, Attribute::VsaDhcpVendorEncapOption(i)),
		128 => value!(i, Attribute::VsaAcctInputOctets64(i)),
		129 => value!(i, Attribute::VsaAcctOutputOctets64(i)),
		130 => value!(i, Attribute::VsaAcctInputPackets64(i)),
		131 => value!(i, Attribute::VsaAcctOutputPackets64(i)),
		132 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAssignedIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		133 => value!(i, Attribute::VsaAcctMcastInOctets64(i)),
		134 => value!(i, Attribute::VsaAcctMcastOutOctets64(i)),
		135 => value!(i, Attribute::VsaAcctMcastInPackets64(i)),
		136 => value!(i, Attribute::VsaAcctMcastOutPackets64(i)),
		137 => map!{i, be_u32, |v| Attribute::VsaLacPort(v)},
		138 => map!{i, be_u32, |v| Attribute::VsaLacRealPort(v)},
		139 => map! {i, be_u32, |v| Attribute::VsaLacPortType(LacPortType(v))},
		140 => map! {i, be_u32, |v| Attribute::VsaLacRealPortType(LacRealPortType(v))},
		141 => value!(i, Attribute::VsaAcctDynAcEnt(i)),
		142 => map!{i, be_u32, |v| Attribute::VsaSessionErrorCode(v)},
		143 => value!(i, Attribute::VsaSessionErrorMsg(i)),
		144 => map! {i, be_u32, |v| Attribute::VsaAcctUpdateReason(AcctUpdateReason(v))},
		145 => value!(i, Attribute::VsaMacAddr(i)),
		146 => value!(i, Attribute::VsaVlanSourceInfo(i)),
		147 => map!{i, be_u32, |v| Attribute::VsaAcctMcastInOctets(v)},
		148 => map!{i, be_u32, |v| Attribute::VsaAcctMcastOutOctets(v)},
		149 => map!{i, be_u32, |v| Attribute::VsaAcctMcastInPackets(v)},
		150 => map!{i, be_u32, |v| Attribute::VsaAcctMcastOutPackets(v)},
		151 => value!(i, Attribute::VsaReauthSessionId(i)),
		156 => value!(i, Attribute::VsaQosRateInbound(i)),
		157 => value!(i, Attribute::VsaQosRateOutbound(i)),
		158 => map!{i, be_u32, |v| Attribute::VsaRouteTag(v)},
		159 => map!{i, be_u32, |v| Attribute::VsaLiId(v)},
		160 => map!{i, take!(4), |v:&[u8]| Attribute::VsaLiMdAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		161 => map!{i, be_u32, |v| Attribute::VsaLiMdPort(v)},
		162 => map!{i, be_u32, |v| Attribute::VsaLiAction(v)},
		163 => value!(i, Attribute::VsaLiProfile(i)),
		164 => value!(i, Attribute::VsaDynamicPolicyFilter(i)),
		165 => value!(i, Attribute::VsaHttpRedirectUrl(i)),
		166 => map!{i, be_u32, |v| Attribute::VsaDslActualRateUp(v)},
		167 => map!{i, be_u32, |v| Attribute::VsaDslActualRateDown(v)},
		168 => map!{i, be_u32, |v| Attribute::VsaDslMinRateUp(v)},
		169 => map!{i, be_u32, |v| Attribute::VsaDslMinRateDown(v)},
		170 => map!{i, be_u32, |v| Attribute::VsaDslAttainableRateUp(v)},
		171 => map!{i, be_u32, |v| Attribute::VsaDslAttainableRateDown(v)},
		172 => map!{i, be_u32, |v| Attribute::VsaDslMaxRateUp(v)},
		173 => map!{i, be_u32, |v| Attribute::VsaDslMaxRateDown(v)},
		174 => map!{i, be_u32, |v| Attribute::VsaDslMinLowPowerRateUp(v)},
		175 => map!{i, be_u32, |v| Attribute::VsaDslMinLowPowerRateDown(v)},
		176 => map!{i, be_u32, |v| Attribute::VsaDslMaxInterDelayUp(v)},
		177 => map!{i, be_u32, |v| Attribute::VsaDslActualInterDelayUp(v)},
		178 => map!{i, be_u32, |v| Attribute::VsaDslMaxInterDelayDown(v)},
		179 => map!{i, be_u32, |v| Attribute::VsaDslActualInterDelayDown(v)},
		180 => map! {i, be_u32, |v| Attribute::VsaDslLineState(DslLineState(v))},
		181 => map!{i, be_u32, |v| Attribute::VsaDslL2Encapsulation(v)},
		182 => map! {i, be_u32, |v| Attribute::VsaDslTransmissionSystem(DslTransmissionSystem(v))},
		183 => map!{i, be_u32, |v| Attribute::VsaDslPppoaPppoeInterWorkFlag(v)},
		185 => map!{i, be_u32, |v| Attribute::VsaDslActualRateDownFactor(v)},
		184 => value!(i, Attribute::VsaDslCombinedLineInfo(i)),
		186 => value!(i, Attribute::VsaClassVolumeLimit(i)),
		187 => value!(i, Attribute::VsaClassVolumeInCounter(i)),
		188 => value!(i, Attribute::VsaClassVolumeOutCounter(i)),
		189 => value!(i, Attribute::VsaFlowFacProfile(i)),
		190 => value!(i, Attribute::VsaServiceName(i)),
		191 => map! {i, be_u32, |v| Attribute::VsaServiceAction(ServiceAction(v))},
		192 => value!(i, Attribute::VsaServiceParameter(i)),
		193 => map!{i, be_u32, |v| Attribute::VsaServiceErrorCause(v)},
		194 => value!(i, Attribute::VsaDeactivateServiceName(i)),
		195 => value!(i, Attribute::VsaQosProfileOverhead(i)),
		196 => value!(i, Attribute::VsaDynamicQosParam(i)),
		197 => value!(i, Attribute::VsaAcctAltSessionId(i)),
		198 => map!{i, be_u32, |v| Attribute::VsaIdleTimeoutThreshold(v)},
		199 => map!{i, be_u32, |v| Attribute::VsaDoubleAuthentication(v)},
		200 => value!(i, Attribute::VsaSbcAdjacency(i)),
		201 => value!(i, Attribute::VsaDhcpField(i)),
		202 => value!(i, Attribute::VsaDhcpOption(i)),
		203 => value!(i, Attribute::VsaSecurityService(i)),
		204 => value!(i, Attribute::VsaReauthServiceName(i)),
		205 => value!(i, Attribute::VsaFlowIpProfile(i)),
		206 => map!{i, be_u32, |v| Attribute::VsaRadiusThrottleWatermark(v)},
		207 => value!(i, Attribute::VsaRbIpv6Dns(i)),
		208 => value!(i, Attribute::VsaRbIpv6Option(i)),
		209 => value!(i, Attribute::VsaClusterPartitionId(i)),
		210 => value!(i, Attribute::VsaCircuitGroupMember(i)),
		212 => map!{i, be_u32, |v| Attribute::VsaDelegatedMaxPrefix(v)},
		213 => value!(i, Attribute::VsaIpv4AddressReleaseControl(i)),
		214 => map!{i, be_u32, |v| Attribute::VsaAcctInputIpv4Octet(v)},
		215 => map!{i, be_u32, |v| Attribute::VsaAcctOutputIpv4Octets(v)},
		216 => map!{i, be_u32, |v| Attribute::VsaAcctInputIpv4Packets(v)},
		217 => map!{i, be_u32, |v| Attribute::VsaAcctOutputIpv4Packets(v)},
		218 => map!{i, be_u32, |v| Attribute::VsaAcctInputIpv4Gigawords(v)},
		219 => map!{i, be_u32, |v| Attribute::VsaAcctOutputIpv4Gigawords(v)},
		220 => map!{i, be_u32, |v| Attribute::VsaAcctInputIpv6Octets(v)},
		221 => map!{i, be_u32, |v| Attribute::VsaAcctOutputIpv6Octets(v)},
		222 => map!{i, be_u32, |v| Attribute::VsaAcctInputIpv6Packets(v)},
		223 => map!{i, be_u32, |v| Attribute::VsaAcctOutputIpv6Packets(v)},
		224 => map!{i, be_u32, |v| Attribute::VsaAcctInputIpv6Gigawords(v)},
		225 => map!{i, be_u32, |v| Attribute::VsaAcctOutputIpv6Gigawords(v)},
        _ => value!(i, Attribute::VsaUnknown(2352, typ, i)),
    }
}
