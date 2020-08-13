/// Definitions for vendor Ascend, vendor value 529
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ServiceType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ServiceType {
	pub const DialoutFramedUser: ServiceType = ServiceType(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FramedProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl FramedProtocol {
	pub const AscendAra: FramedProtocol = FramedProtocol(255);
	pub const AscendMpp: FramedProtocol = FramedProtocol(256);
	pub const AscendEuraw: FramedProtocol = FramedProtocol(257);
	pub const AscendEuui: FramedProtocol = FramedProtocol(258);
	pub const AscendX25: FramedProtocol = FramedProtocol(259);
	pub const AscendComb: FramedProtocol = FramedProtocol(260);
	pub const AscendFr: FramedProtocol = FramedProtocol(261);
	pub const AscendMp: FramedProtocol = FramedProtocol(262);
	pub const AscendFrCir: FramedProtocol = FramedProtocol(263);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendSourceIpCheck(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendSourceIpCheck {
	pub const SourceIpCheckNo: AscendSourceIpCheck = AscendSourceIpCheck(0);
	pub const SourceIpCheckYes: AscendSourceIpCheck = AscendSourceIpCheck(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendCbcpEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendCbcpEnable {
	pub const CbcpNotEnabled: AscendCbcpEnable = AscendCbcpEnable(0);
	pub const CbcpEnabled: AscendCbcpEnable = AscendCbcpEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendCbcpMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendCbcpMode {
	pub const CbcpNoCallback: AscendCbcpMode = AscendCbcpMode(1);
	pub const CbcpUserCallback: AscendCbcpMode = AscendCbcpMode(2);
	pub const CbcpProfileCallback: AscendCbcpMode = AscendCbcpMode(3);
	pub const CbcpAnyOrNo: AscendCbcpMode = AscendCbcpMode(7);
	pub const CbcpOff: AscendCbcpMode = AscendCbcpMode(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendFrDirect(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendFrDirect {
	pub const FrDirectNo: AscendFrDirect = AscendFrDirect(0);
	pub const FrDirectYes: AscendFrDirect = AscendFrDirect(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendHandleIpx(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendHandleIpx {
	pub const HandleIpxNone: AscendHandleIpx = AscendHandleIpx(0);
	pub const HandleIpxClient: AscendHandleIpx = AscendHandleIpx(1);
	pub const HandleIpxServer: AscendHandleIpx = AscendHandleIpx(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendIpxPeerMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendIpxPeerMode {
	pub const IpxPeerRouter: AscendIpxPeerMode = AscendIpxPeerMode(0);
	pub const IpxPeerDialin: AscendIpxPeerMode = AscendIpxPeerMode(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendCallType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendCallType {
	pub const Switched: AscendCallType = AscendCallType(0);
	pub const Nailed: AscendCallType = AscendCallType(1);
	pub const NailedOrMpp: AscendCallType = AscendCallType(2);
	pub const PermOrSwitched: AscendCallType = AscendCallType(3);
	pub const AoOrDi: AscendCallType = AscendCallType(6);
	pub const Megamax: AscendCallType = AscendCallType(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendFt1Caller(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendFt1Caller {
	pub const Ft1No: AscendFt1Caller = AscendFt1Caller(0);
	pub const Ft1Yes: AscendFt1Caller = AscendFt1Caller(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendPriNumberType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendPriNumberType {
	pub const UnknownNumber: AscendPriNumberType = AscendPriNumberType(0);
	pub const IntlNumber: AscendPriNumberType = AscendPriNumberType(1);
	pub const NationalNumber: AscendPriNumberType = AscendPriNumberType(2);
	pub const NetSpecificNumber: AscendPriNumberType = AscendPriNumberType(3);
	pub const LocalNumber: AscendPriNumberType = AscendPriNumberType(4);
	pub const AbbrevNumber: AscendPriNumberType = AscendPriNumberType(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendRouteIp(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendRouteIp {
	pub const RouteIpNo: AscendRouteIp = AscendRouteIp(0);
	pub const RouteIpYes: AscendRouteIp = AscendRouteIp(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendRouteIpx(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendRouteIpx {
	pub const RouteIpxNo: AscendRouteIpx = AscendRouteIpx(0);
	pub const RouteIpxYes: AscendRouteIpx = AscendRouteIpx(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendBridge(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendBridge {
	pub const BridgeNo: AscendBridge = AscendBridge(0);
	pub const BridgeYes: AscendBridge = AscendBridge(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendTsIdleMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendTsIdleMode {
	pub const TsIdleNone: AscendTsIdleMode = AscendTsIdleMode(0);
	pub const TsIdleInput: AscendTsIdleMode = AscendTsIdleMode(1);
	pub const TsIdleInputOutput: AscendTsIdleMode = AscendTsIdleMode(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendSendAuth(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendSendAuth {
	pub const SendAuthNone: AscendSendAuth = AscendSendAuth(0);
	pub const SendAuthPap: AscendSendAuth = AscendSendAuth(1);
	pub const SendAuthChap: AscendSendAuth = AscendSendAuth(2);
	pub const SendAuthMsChap: AscendSendAuth = AscendSendAuth(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendLinkCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendLinkCompression {
	pub const LinkCompNone: AscendLinkCompression = AscendLinkCompression(0);
	pub const LinkCompStac: AscendLinkCompression = AscendLinkCompression(1);
	pub const LinkCompStacDraft9: AscendLinkCompression = AscendLinkCompression(2);
	pub const LinkCompMsStac: AscendLinkCompression = AscendLinkCompression(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendHistoryWeighType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendHistoryWeighType {
	pub const HistoryConstant: AscendHistoryWeighType = AscendHistoryWeighType(0);
	pub const HistoryLinear: AscendHistoryWeighType = AscendHistoryWeighType(1);
	pub const HistoryQuadratic: AscendHistoryWeighType = AscendHistoryWeighType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendCallback(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendCallback {
	pub const CallbackNo: AscendCallback = AscendCallback(0);
	pub const CallbackYes: AscendCallback = AscendCallback(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendExpectCallback(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendExpectCallback {
	pub const ExpectCallbackNo: AscendExpectCallback = AscendExpectCallback(0);
	pub const ExpectCallbackYes: AscendExpectCallback = AscendExpectCallback(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendDataSvc(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendDataSvc {
	pub const SwitchedVoiceBearer: AscendDataSvc = AscendDataSvc(0);
	pub const Nailed56Kr: AscendDataSvc = AscendDataSvc(1);
	pub const Nailed64K: AscendDataSvc = AscendDataSvc(2);
	pub const Switched64Kr: AscendDataSvc = AscendDataSvc(3);
	pub const Switched56K: AscendDataSvc = AscendDataSvc(4);
	pub const Switched384Kr: AscendDataSvc = AscendDataSvc(5);
	pub const Switched384K: AscendDataSvc = AscendDataSvc(6);
	pub const Switched1536K: AscendDataSvc = AscendDataSvc(7);
	pub const Switched1536Kr: AscendDataSvc = AscendDataSvc(8);
	pub const Switched128K: AscendDataSvc = AscendDataSvc(9);
	pub const Switched192K: AscendDataSvc = AscendDataSvc(10);
	pub const Switched256K: AscendDataSvc = AscendDataSvc(11);
	pub const Switched320K: AscendDataSvc = AscendDataSvc(12);
	pub const Switched384KMr: AscendDataSvc = AscendDataSvc(13);
	pub const Switched448K: AscendDataSvc = AscendDataSvc(14);
	pub const Switched512K: AscendDataSvc = AscendDataSvc(15);
	pub const Switched576K: AscendDataSvc = AscendDataSvc(16);
	pub const Switched640K: AscendDataSvc = AscendDataSvc(17);
	pub const Switched704K: AscendDataSvc = AscendDataSvc(18);
	pub const Switched768K: AscendDataSvc = AscendDataSvc(19);
	pub const Switched832K: AscendDataSvc = AscendDataSvc(20);
	pub const Switched896K: AscendDataSvc = AscendDataSvc(21);
	pub const Switched960K: AscendDataSvc = AscendDataSvc(22);
	pub const Switched1024K: AscendDataSvc = AscendDataSvc(23);
	pub const Switched1088K: AscendDataSvc = AscendDataSvc(24);
	pub const Switched1152K: AscendDataSvc = AscendDataSvc(25);
	pub const Switched1216K: AscendDataSvc = AscendDataSvc(26);
	pub const Switched1280K: AscendDataSvc = AscendDataSvc(27);
	pub const Switched1344K: AscendDataSvc = AscendDataSvc(28);
	pub const Switched1408K: AscendDataSvc = AscendDataSvc(29);
	pub const Switched1472K: AscendDataSvc = AscendDataSvc(30);
	pub const Switched1600K: AscendDataSvc = AscendDataSvc(31);
	pub const Switched1664K: AscendDataSvc = AscendDataSvc(32);
	pub const Switched1728K: AscendDataSvc = AscendDataSvc(33);
	pub const Switched1792K: AscendDataSvc = AscendDataSvc(34);
	pub const Switched1856K: AscendDataSvc = AscendDataSvc(35);
	pub const Switched1920K: AscendDataSvc = AscendDataSvc(36);
	pub const SwitchedInherited: AscendDataSvc = AscendDataSvc(37);
	pub const SwitchedRestrictedBearerX30: AscendDataSvc = AscendDataSvc(38);
	pub const SwitchedClearBearerV110: AscendDataSvc = AscendDataSvc(39);
	pub const SwitchedRestricted64X30: AscendDataSvc = AscendDataSvc(40);
	pub const SwitchedClear56V110: AscendDataSvc = AscendDataSvc(41);
	pub const SwitchedModem: AscendDataSvc = AscendDataSvc(42);
	pub const SwitchedAtmodem: AscendDataSvc = AscendDataSvc(43);
	pub const SwitchedV1102456: AscendDataSvc = AscendDataSvc(45);
	pub const SwitchedV1104856: AscendDataSvc = AscendDataSvc(46);
	pub const SwitchedV1109656: AscendDataSvc = AscendDataSvc(47);
	pub const SwitchedV11019256: AscendDataSvc = AscendDataSvc(48);
	pub const SwitchedV11038456: AscendDataSvc = AscendDataSvc(49);
	pub const SwitchedV1102456R: AscendDataSvc = AscendDataSvc(50);
	pub const SwitchedV1104856R: AscendDataSvc = AscendDataSvc(51);
	pub const SwitchedV1109656R: AscendDataSvc = AscendDataSvc(52);
	pub const SwitchedV11019256R: AscendDataSvc = AscendDataSvc(53);
	pub const SwitchedV11038456R: AscendDataSvc = AscendDataSvc(54);
	pub const SwitchedV1102464: AscendDataSvc = AscendDataSvc(55);
	pub const SwitchedV1104864: AscendDataSvc = AscendDataSvc(56);
	pub const SwitchedV1109664: AscendDataSvc = AscendDataSvc(57);
	pub const SwitchedV11019264: AscendDataSvc = AscendDataSvc(58);
	pub const SwitchedV11038464: AscendDataSvc = AscendDataSvc(59);
	pub const SwitchedV1102464R: AscendDataSvc = AscendDataSvc(60);
	pub const SwitchedV1104864R: AscendDataSvc = AscendDataSvc(61);
	pub const SwitchedV1109664R: AscendDataSvc = AscendDataSvc(62);
	pub const SwitchedV11038464R: AscendDataSvc = AscendDataSvc(64);
	pub const SwitchedV11019264R: AscendDataSvc = AscendDataSvc(63);
	pub const SwitchedPots: AscendDataSvc = AscendDataSvc(68);
	pub const SwitchedAtm: AscendDataSvc = AscendDataSvc(69);
	pub const SwitchedFr: AscendDataSvc = AscendDataSvc(70);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendForce56(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendForce56 {
	pub const Force56No: AscendForce56 = AscendForce56(0);
	pub const Force56Yes: AscendForce56 = AscendForce56(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendPwLifetime(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendPwLifetime {
	pub const LifetimeInDays: AscendPwLifetime = AscendPwLifetime(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendPwWarntime(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendPwWarntime {
	pub const DaysOfWarning: AscendPwWarntime = AscendPwWarntime(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendPppVj1172(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendPppVj1172 {
	pub const PppVj1172: AscendPppVj1172 = AscendPppVj1172(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendPppVjSlotComp(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendPppVjSlotComp {
	pub const VjSlotCompNo: AscendPppVjSlotComp = AscendPppVjSlotComp(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendRequireAuth(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendRequireAuth {
	pub const NotRequireAuth: AscendRequireAuth = AscendRequireAuth(0);
	pub const RequireAuth: AscendRequireAuth = AscendRequireAuth(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendTokenImmediate(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendTokenImmediate {
	pub const TokImmNo: AscendTokenImmediate = AscendTokenImmediate(0);
	pub const TokImmYes: AscendTokenImmediate = AscendTokenImmediate(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendDbaMonitor(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendDbaMonitor {
	pub const DbaTransmit: AscendDbaMonitor = AscendDbaMonitor(0);
	pub const DbaTransmitRecv: AscendDbaMonitor = AscendDbaMonitor(1);
	pub const DbaNone: AscendDbaMonitor = AscendDbaMonitor(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendFrType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendFrType {
	pub const AscendFrDte: AscendFrType = AscendFrType(0);
	pub const AscendFrDce: AscendFrType = AscendFrType(1);
	pub const AscendFrNni: AscendFrType = AscendFrType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendFrLinkMgt(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendFrLinkMgt {
	pub const AscendFrNoLinkMgt: AscendFrLinkMgt = AscendFrLinkMgt(0);
	pub const AscendFrT1617D: AscendFrLinkMgt = AscendFrLinkMgt(1);
	pub const AscendFrQ933A: AscendFrLinkMgt = AscendFrLinkMgt(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendFrLinkup(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendFrLinkup {
	pub const AscendLinkupDefault: AscendFrLinkup = AscendFrLinkup(0);
	pub const AscendLinkupAlwaysup: AscendFrLinkup = AscendFrLinkup(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendMulticastClient(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendMulticastClient {
	pub const MulticastNo: AscendMulticastClient = AscendMulticastClient(0);
	pub const MulticastYes: AscendMulticastClient = AscendMulticastClient(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendUserAcctType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendUserAcctType {
	pub const AscendUserAcctNone: AscendUserAcctType = AscendUserAcctType(0);
	pub const AscendUserAcctUser: AscendUserAcctType = AscendUserAcctType(1);
	pub const AscendUserAcctUserDefault: AscendUserAcctType = AscendUserAcctType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendUserAcctBase(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendUserAcctBase {
	pub const Base10: AscendUserAcctBase = AscendUserAcctBase(0);
	pub const Base16: AscendUserAcctBase = AscendUserAcctBase(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendDhcpReply(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendDhcpReply {
	pub const DhcpReplyNo: AscendDhcpReply = AscendDhcpReply(0);
	pub const DhcpReplyYes: AscendDhcpReply = AscendDhcpReply(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendClientAssignDns(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendClientAssignDns {
	pub const DnsAssignNo: AscendClientAssignDns = AscendClientAssignDns(0);
	pub const DnsAssignYes: AscendClientAssignDns = AscendClientAssignDns(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendEventType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendEventType {
	pub const AscendColdstart: AscendEventType = AscendEventType(1);
	pub const AscendSessionEvent: AscendEventType = AscendEventType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendBacpEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendBacpEnable {
	pub const BacpNo: AscendBacpEnable = AscendBacpEnable(0);
	pub const BacpYes: AscendBacpEnable = AscendBacpEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendDialoutAllowed(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendDialoutAllowed {
	pub const DialoutNotAllowed: AscendDialoutAllowed = AscendDialoutAllowed(0);
	pub const DialoutAllowed: AscendDialoutAllowed = AscendDialoutAllowed(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendSharedProfileEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendSharedProfileEnable {
	pub const SharedProfileNo: AscendSharedProfileEnable = AscendSharedProfileEnable(0);
	pub const SharedProfileYes: AscendSharedProfileEnable = AscendSharedProfileEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendTemporaryRtes(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendTemporaryRtes {
	pub const TempRtesNo: AscendTemporaryRtes = AscendTemporaryRtes(0);
	pub const TempRtesYes: AscendTemporaryRtes = AscendTemporaryRtes(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendDisconnectCause(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendDisconnectCause {
	pub const NoReason: AscendDisconnectCause = AscendDisconnectCause(0);
	pub const NotApplicable: AscendDisconnectCause = AscendDisconnectCause(1);
	pub const Unknown: AscendDisconnectCause = AscendDisconnectCause(2);
	pub const CallDisconnected: AscendDisconnectCause = AscendDisconnectCause(3);
	pub const ClidAuthenticationFailed: AscendDisconnectCause = AscendDisconnectCause(4);
	pub const ClidRadiusTimeout: AscendDisconnectCause = AscendDisconnectCause(5);
	pub const ModemNoDcd: AscendDisconnectCause = AscendDisconnectCause(10);
	pub const DcdDetectedThenInactive: AscendDisconnectCause = AscendDisconnectCause(11);
	pub const ModemInvalidResultCodes: AscendDisconnectCause = AscendDisconnectCause(12);
	pub const TermsrvUserQuit: AscendDisconnectCause = AscendDisconnectCause(20);
	pub const TermsrvIdleTimeout: AscendDisconnectCause = AscendDisconnectCause(21);
	pub const TermsrvExitTelnet: AscendDisconnectCause = AscendDisconnectCause(22);
	pub const TermsrvNoIpaddr: AscendDisconnectCause = AscendDisconnectCause(23);
	pub const TermsrvExitRawTcp: AscendDisconnectCause = AscendDisconnectCause(24);
	pub const TermsrvExitLoginFailed: AscendDisconnectCause = AscendDisconnectCause(25);
	pub const TermsrvExitRawTcpDisabled: AscendDisconnectCause = AscendDisconnectCause(26);
	pub const TermsrvCtrlCInLogin: AscendDisconnectCause = AscendDisconnectCause(27);
	pub const TermsrvDestroyed: AscendDisconnectCause = AscendDisconnectCause(28);
	pub const TermsrvUserClosedVcon: AscendDisconnectCause = AscendDisconnectCause(29);
	pub const TermsrvVconDestroyed: AscendDisconnectCause = AscendDisconnectCause(30);
	pub const TermsrvExitRlogin: AscendDisconnectCause = AscendDisconnectCause(31);
	pub const TermsrvBadRloginOption: AscendDisconnectCause = AscendDisconnectCause(32);
	pub const TermsrvNotEnoughResources: AscendDisconnectCause = AscendDisconnectCause(33);
	pub const MppNoNullMsgTimeout: AscendDisconnectCause = AscendDisconnectCause(35);
	pub const PppLcpTimeout: AscendDisconnectCause = AscendDisconnectCause(40);
	pub const PppLcpNegotionFailed: AscendDisconnectCause = AscendDisconnectCause(41);
	pub const PppPapAuthFailed: AscendDisconnectCause = AscendDisconnectCause(42);
	pub const PppChapAuthFailed: AscendDisconnectCause = AscendDisconnectCause(43);
	pub const PppRmtAuthFailed: AscendDisconnectCause = AscendDisconnectCause(44);
	pub const PppRcvTerminateReq: AscendDisconnectCause = AscendDisconnectCause(45);
	pub const PppRcvCloseEvent: AscendDisconnectCause = AscendDisconnectCause(46);
	pub const PppNoNcpsOpen: AscendDisconnectCause = AscendDisconnectCause(47);
	pub const PppMpBundleUnknown: AscendDisconnectCause = AscendDisconnectCause(48);
	pub const PppLcpCloseMpAddFail: AscendDisconnectCause = AscendDisconnectCause(49);
	pub const SessionTableFull: AscendDisconnectCause = AscendDisconnectCause(50);
	pub const OutOfResources: AscendDisconnectCause = AscendDisconnectCause(51);
	pub const InvalidIpAddress: AscendDisconnectCause = AscendDisconnectCause(52);
	pub const HostnameResolutionFailed: AscendDisconnectCause = AscendDisconnectCause(53);
	pub const BadOrMissingPortNumber: AscendDisconnectCause = AscendDisconnectCause(54);
	pub const HostReset: AscendDisconnectCause = AscendDisconnectCause(60);
	pub const ConnectionRefused: AscendDisconnectCause = AscendDisconnectCause(61);
	pub const ConnectionTimeout: AscendDisconnectCause = AscendDisconnectCause(62);
	pub const ConnectionClosed: AscendDisconnectCause = AscendDisconnectCause(63);
	pub const NetworkUnreachable: AscendDisconnectCause = AscendDisconnectCause(64);
	pub const HostUnreachable: AscendDisconnectCause = AscendDisconnectCause(65);
	pub const NetworkUnreachableAdmin: AscendDisconnectCause = AscendDisconnectCause(66);
	pub const HostUnreachableAdmin: AscendDisconnectCause = AscendDisconnectCause(67);
	pub const PortUnreachable: AscendDisconnectCause = AscendDisconnectCause(68);
	pub const SessionTimeout: AscendDisconnectCause = AscendDisconnectCause(100);
	pub const InvalidIncomingUser: AscendDisconnectCause = AscendDisconnectCause(101);
	pub const DisconnectDueToCallback: AscendDisconnectCause = AscendDisconnectCause(102);
	pub const ProtoDisabledOrUnsupported: AscendDisconnectCause = AscendDisconnectCause(120);
	pub const DisconnectReqByRadius: AscendDisconnectCause = AscendDisconnectCause(150);
	pub const DisconnectReqByLocalAdmin: AscendDisconnectCause = AscendDisconnectCause(151);
	pub const V110TimeoutSyncRetryExceed: AscendDisconnectCause = AscendDisconnectCause(160);
	pub const PppAuthTimeoutExceeded: AscendDisconnectCause = AscendDisconnectCause(170);
	pub const UserExecutedDoHangup: AscendDisconnectCause = AscendDisconnectCause(180);
	pub const RemoteEndHungUp: AscendDisconnectCause = AscendDisconnectCause(185);
	pub const ResourceHasBeenQuiesced: AscendDisconnectCause = AscendDisconnectCause(190);
	pub const MaxCallDurationReached: AscendDisconnectCause = AscendDisconnectCause(195);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendConnectProgress(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendConnectProgress {
	pub const NoProgress: AscendConnectProgress = AscendConnectProgress(0);
	pub const CallUp: AscendConnectProgress = AscendConnectProgress(10);
	pub const ModemUp: AscendConnectProgress = AscendConnectProgress(30);
	pub const ModemAwaitingDcd: AscendConnectProgress = AscendConnectProgress(31);
	pub const ModemAwaitingCodes: AscendConnectProgress = AscendConnectProgress(32);
	pub const TermsrvStarted: AscendConnectProgress = AscendConnectProgress(40);
	pub const TermsrvRawTcpStarted: AscendConnectProgress = AscendConnectProgress(41);
	pub const TermsrvTelnetStarted: AscendConnectProgress = AscendConnectProgress(42);
	pub const TermsrvRawTcpConnected: AscendConnectProgress = AscendConnectProgress(43);
	pub const TermsrvTelnetConnected: AscendConnectProgress = AscendConnectProgress(44);
	pub const TermsrvRloginStarted: AscendConnectProgress = AscendConnectProgress(45);
	pub const TermsrvRloginConnected: AscendConnectProgress = AscendConnectProgress(46);
	pub const ModemOutdialCallUp: AscendConnectProgress = AscendConnectProgress(50);
	pub const LanSessionUp: AscendConnectProgress = AscendConnectProgress(60);
	pub const LcpOpening: AscendConnectProgress = AscendConnectProgress(61);
	pub const CcpOpening: AscendConnectProgress = AscendConnectProgress(62);
	pub const IpncpOpening: AscendConnectProgress = AscendConnectProgress(63);
	pub const BncpOpening: AscendConnectProgress = AscendConnectProgress(64);
	pub const LcpOpened: AscendConnectProgress = AscendConnectProgress(65);
	pub const CcpOpened: AscendConnectProgress = AscendConnectProgress(66);
	pub const IpncpOpened: AscendConnectProgress = AscendConnectProgress(67);
	pub const BncpOpened: AscendConnectProgress = AscendConnectProgress(68);
	pub const LcpStateInitial: AscendConnectProgress = AscendConnectProgress(69);
	pub const LcpStateStarting: AscendConnectProgress = AscendConnectProgress(70);
	pub const LcpStateClosed: AscendConnectProgress = AscendConnectProgress(71);
	pub const LcpStateStopped: AscendConnectProgress = AscendConnectProgress(72);
	pub const LcpStateClosing: AscendConnectProgress = AscendConnectProgress(73);
	pub const LcpStateStopping: AscendConnectProgress = AscendConnectProgress(74);
	pub const LcpStateRequestSent: AscendConnectProgress = AscendConnectProgress(75);
	pub const LcpStateAckReceived: AscendConnectProgress = AscendConnectProgress(76);
	pub const LcpStateAckSent: AscendConnectProgress = AscendConnectProgress(77);
	pub const IpxncpOpened: AscendConnectProgress = AscendConnectProgress(80);
	pub const AtncpOpened: AscendConnectProgress = AscendConnectProgress(81);
	pub const BacpOpening: AscendConnectProgress = AscendConnectProgress(82);
	pub const BacpOpened: AscendConnectProgress = AscendConnectProgress(83);
	pub const V110Up: AscendConnectProgress = AscendConnectProgress(90);
	pub const V110StateOpened: AscendConnectProgress = AscendConnectProgress(91);
	pub const V110StateCarrier: AscendConnectProgress = AscendConnectProgress(92);
	pub const V110StateReset: AscendConnectProgress = AscendConnectProgress(93);
	pub const V110StateClosed: AscendConnectProgress = AscendConnectProgress(94);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendAtmDirect(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendAtmDirect {
	pub const AtmDirectNo: AscendAtmDirect = AscendAtmDirect(0);
	pub const AtmDirectYes: AscendAtmDirect = AscendAtmDirect(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendAtmFaultManagement(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendAtmFaultManagement {
	pub const VcEndToEndLoopback: AscendAtmFaultManagement = AscendAtmFaultManagement(2);
	pub const VcNoLoopback: AscendAtmFaultManagement = AscendAtmFaultManagement(0);
	pub const VcSegmentLoopback: AscendAtmFaultManagement = AscendAtmFaultManagement(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendAppletalkPeerMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendAppletalkPeerMode {
	pub const AppletalkPeerDialin: AscendAppletalkPeerMode = AscendAppletalkPeerMode(1);
	pub const AppletalkPeerRouter: AscendAppletalkPeerMode = AscendAppletalkPeerMode(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendAuthType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendAuthType {
	pub const AuthAny: AscendAuthType = AscendAuthType(2);
	pub const AuthChap: AscendAuthType = AscendAuthType(4);
	pub const AuthDefault: AscendAuthType = AscendAuthType(1);
	pub const AuthMsChap: AscendAuthType = AscendAuthType(5);
	pub const AuthNone: AscendAuthType = AscendAuthType(0);
	pub const AuthPap: AscendAuthType = AscendAuthType(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendBirEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendBirEnable {
	pub const BirEnableNo: AscendBirEnable = AscendBirEnable(0);
	pub const BirEnableYes: AscendBirEnable = AscendBirEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendBirProxy(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendBirProxy {
	pub const BirProxyNo: AscendBirProxy = AscendBirProxy(0);
	pub const BirProxyYes: AscendBirProxy = AscendBirProxy(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendBiDirectionalAuth(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendBiDirectionalAuth {
	pub const BiDirectionalAuthAllowed: AscendBiDirectionalAuth = AscendBiDirectionalAuth(1);
	pub const BiDirectionalAuthNone: AscendBiDirectionalAuth = AscendBiDirectionalAuth(0);
	pub const BiDirectionalAuthRequired: AscendBiDirectionalAuth = AscendBiDirectionalAuth(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendBridgeNonPppoe(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendBridgeNonPppoe {
	pub const BridgeNonPppoeNo: AscendBridgeNonPppoe = AscendBridgeNonPppoe(0);
	pub const BridgeNonPppoeYes: AscendBridgeNonPppoe = AscendBridgeNonPppoe(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendCacheRefresh(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendCacheRefresh {
	pub const RefreshNo: AscendCacheRefresh = AscendCacheRefresh(0);
	pub const RefreshYes: AscendCacheRefresh = AscendCacheRefresh(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendCallDirection(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendCallDirection {
	pub const AscendCallDirectionIncoming: AscendCallDirection = AscendCallDirection(0);
	pub const AscendCallDirectionOutgoing: AscendCallDirection = AscendCallDirection(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendCallingIdNumberPlan(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendCallingIdNumberPlan {
	pub const Data: AscendCallingIdNumberPlan = AscendCallingIdNumberPlan(3);
	pub const IsdnTelephony: AscendCallingIdNumberPlan = AscendCallingIdNumberPlan(1);
	pub const National: AscendCallingIdNumberPlan = AscendCallingIdNumberPlan(8);
	pub const Private: AscendCallingIdNumberPlan = AscendCallingIdNumberPlan(9);
	pub const Telex: AscendCallingIdNumberPlan = AscendCallingIdNumberPlan(4);
	pub const Unknown: AscendCallingIdNumberPlan = AscendCallingIdNumberPlan(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendCallingIdPresentatn(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendCallingIdPresentatn {
	pub const Allowed: AscendCallingIdPresentatn = AscendCallingIdPresentatn(0);
	pub const NumberNotAvailable: AscendCallingIdPresentatn = AscendCallingIdPresentatn(2);
	pub const Restricted: AscendCallingIdPresentatn = AscendCallingIdPresentatn(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendCallingIdScreening(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendCallingIdScreening {
	pub const NetworkProvided: AscendCallingIdScreening = AscendCallingIdScreening(3);
	pub const UserNotScreened: AscendCallingIdScreening = AscendCallingIdScreening(0);
	pub const UserProvidedFailed: AscendCallingIdScreening = AscendCallingIdScreening(2);
	pub const UserProvidedPassed: AscendCallingIdScreening = AscendCallingIdScreening(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendCallingIdTypeOfNum(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendCallingIdTypeOfNum {
	pub const AbbreviatedNumber: AscendCallingIdTypeOfNum = AscendCallingIdTypeOfNum(6);
	pub const InternationalNumber: AscendCallingIdTypeOfNum = AscendCallingIdTypeOfNum(1);
	pub const NationalNumber: AscendCallingIdTypeOfNum = AscendCallingIdTypeOfNum(2);
	pub const NetworkSpecific: AscendCallingIdTypeOfNum = AscendCallingIdTypeOfNum(3);
	pub const SubscriberNumber: AscendCallingIdTypeOfNum = AscendCallingIdTypeOfNum(4);
	pub const Unknown: AscendCallingIdTypeOfNum = AscendCallingIdTypeOfNum(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendCktType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendCktType {
	pub const AscendPvc: AscendCktType = AscendCktType(0);
	pub const AscendSvc: AscendCktType = AscendCktType(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendClientAssignWins(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendClientAssignWins {
	pub const WinsAssignNo: AscendClientAssignWins = AscendClientAssignWins(0);
	pub const WinsAssignYes: AscendClientAssignWins = AscendClientAssignWins(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendDslDownstreamLimit(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendDslDownstreamLimit {
	pub const AdslcapDn1280000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(10);
	pub const AdslcapDn1600000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(9);
	pub const AdslcapDn1920000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(8);
	pub const AdslcapDn2240000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(7);
	pub const AdslcapDn2560000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(6);
	pub const AdslcapDn2688000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(5);
	pub const AdslcapDn3200000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(4);
	pub const AdslcapDn4480000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(3);
	pub const AdslcapDn5120000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(2);
	pub const AdslcapDn6272000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(1);
	pub const AdslcapDn640000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(12);
	pub const AdslcapDn7168000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(0);
	pub const AdslcapDn960000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(11);
	pub const AdsldmtDn128000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(121);
	pub const AdsldmtDn1280000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(114);
	pub const AdsldmtDn1600000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(113);
	pub const AdsldmtDn1920000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(112);
	pub const AdsldmtDn2240000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(111);
	pub const AdsldmtDn256000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(120);
	pub const AdsldmtDn2560000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(110);
	pub const AdsldmtDn2688000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(109);
	pub const AdsldmtDn3200000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(108);
	pub const AdsldmtDn384000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(119);
	pub const AdsldmtDn4480000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(107);
	pub const AdsldmtDn512000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(118);
	pub const AdsldmtDn5120000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(106);
	pub const AdsldmtDn6272000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(105);
	pub const AdsldmtDn640000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(117);
	pub const AdsldmtDn7168000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(104);
	pub const AdsldmtDn768000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(116);
	pub const AdsldmtDn8000000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(103);
	pub const AdsldmtDn8960000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(102);
	pub const AdsldmtDn9504000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(101);
	pub const AdsldmtDn960000: AscendDslDownstreamLimit = AscendDslDownstreamLimit(115);
	pub const AdsldmtDnAuto: AscendDslDownstreamLimit = AscendDslDownstreamLimit(100);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendDslRateMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendDslRateMode {
	pub const RateModeAutobaud: AscendDslRateMode = AscendDslRateMode(1);
	pub const RateModeSingle: AscendDslRateMode = AscendDslRateMode(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendDslRateType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendDslRateType {
	pub const RateTypeAdslcap: AscendDslRateType = AscendDslRateType(2);
	pub const RateTypeAdsldmt: AscendDslRateType = AscendDslRateType(4);
	pub const RateTypeAdsldmtcell: AscendDslRateType = AscendDslRateType(3);
	pub const RateTypeDisabled: AscendDslRateType = AscendDslRateType(0);
	pub const RateTypeSdsl: AscendDslRateType = AscendDslRateType(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendDslUpstreamLimit(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendDslUpstreamLimit {
	pub const AdsldmtUp896000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(153);
	pub const AdslcapUp1088000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(50);
	pub const AdslcapUp272000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(56);
	pub const AdslcapUp408000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(55);
	pub const AdslcapUp544000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(54);
	pub const AdslcapUp680000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(53);
	pub const AdslcapUp816000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(52);
	pub const AdslcapUp952000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(51);
	pub const AdsldmtUp1088000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(151);
	pub const AdsldmtUp128000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(160);
	pub const AdsldmtUp256000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(159);
	pub const AdsldmtUp384000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(158);
	pub const AdsldmtUp512000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(157);
	pub const AdsldmtUp640000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(156);
	pub const AdsldmtUp768000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(155);
	pub const AdsldmtUp800000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(154);
	pub const AdsldmtUp928000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(152);
	pub const AdsldmtUpAuto: AscendDslUpstreamLimit = AscendDslUpstreamLimit(150);
	pub const Sdsl1168000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(5);
	pub const Sdsl144000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(0);
	pub const Sdsl1552000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(6);
	pub const Sdsl2320000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(7);
	pub const Sdsl272000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(1);
	pub const Sdsl400000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(2);
	pub const Sdsl528000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(3);
	pub const Sdsl784000: AscendDslUpstreamLimit = AscendDslUpstreamLimit(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendFrLinkStatusDlci(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendFrLinkStatusDlci {
	pub const AscendFrLmiDlci0: AscendFrLinkStatusDlci = AscendFrLinkStatusDlci(0);
	pub const AscendFrLmiDlci1023: AscendFrLinkStatusDlci = AscendFrLinkStatusDlci(1023);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendFilterRequired(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendFilterRequired {
	pub const RequiredNo: AscendFilterRequired = AscendFilterRequired(0);
	pub const RequiredYes: AscendFilterRequired = AscendFilterRequired(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendIpPoolChaining(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendIpPoolChaining {
	pub const IpPoolChainingNo: AscendIpPoolChaining = AscendIpPoolChaining(0);
	pub const IpPoolChainingYes: AscendIpPoolChaining = AscendIpPoolChaining(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendIpTos(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendIpTos {
	pub const IpTosCost: AscendIpTos = AscendIpTos(2);
	pub const IpTosDisabled: AscendIpTos = AscendIpTos(1);
	pub const IpTosLatency: AscendIpTos = AscendIpTos(16);
	pub const IpTosNormal: AscendIpTos = AscendIpTos(0);
	pub const IpTosReliability: AscendIpTos = AscendIpTos(4);
	pub const IpTosThroughput: AscendIpTos = AscendIpTos(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendIpTosApplyTo(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendIpTosApplyTo {
	pub const IpTosApplyToBoth: AscendIpTosApplyTo = AscendIpTosApplyTo(3072);
	pub const IpTosApplyToIncoming: AscendIpTosApplyTo = AscendIpTosApplyTo(1024);
	pub const IpTosApplyToOutgoing: AscendIpTosApplyTo = AscendIpTosApplyTo(2048);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendIpTosPrecedence(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendIpTosPrecedence {
	pub const IpTosPrecedencePriFive: AscendIpTosPrecedence = AscendIpTosPrecedence(160);
	pub const IpTosPrecedencePriFour: AscendIpTosPrecedence = AscendIpTosPrecedence(128);
	pub const IpTosPrecedencePriNormal: AscendIpTosPrecedence = AscendIpTosPrecedence(0);
	pub const IpTosPrecedencePriOne: AscendIpTosPrecedence = AscendIpTosPrecedence(32);
	pub const IpTosPrecedencePriSeven: AscendIpTosPrecedence = AscendIpTosPrecedence(224);
	pub const IpTosPrecedencePriSix: AscendIpTosPrecedence = AscendIpTosPrecedence(192);
	pub const IpTosPrecedencePriThree: AscendIpTosPrecedence = AscendIpTosPrecedence(96);
	pub const IpTosPrecedencePriTwo: AscendIpTosPrecedence = AscendIpTosPrecedence(64);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendIpxHeaderCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendIpxHeaderCompression {
	pub const IpxHeaderCompressionNo: AscendIpxHeaderCompression = AscendIpxHeaderCompression(0);
	pub const IpxHeaderCompressionYes: AscendIpxHeaderCompression = AscendIpxHeaderCompression(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendNasPortFormat(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendNasPortFormat {
	pub const One22: AscendNasPortFormat = AscendNasPortFormat(3);
	pub const Two455: AscendNasPortFormat = AscendNasPortFormat(2);
	pub const Two464: AscendNasPortFormat = AscendNasPortFormat(1);
	pub const Unknown: AscendNasPortFormat = AscendNasPortFormat(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendNumberingPlanId(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendNumberingPlanId {
	pub const IsdnNumberingPlan: AscendNumberingPlanId = AscendNumberingPlanId(1);
	pub const PrivateNumberingPlan: AscendNumberingPlanId = AscendNumberingPlanId(9);
	pub const UnknownNumberingPlan: AscendNumberingPlanId = AscendNumberingPlanId(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendPppoeEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendPppoeEnable {
	pub const PppoeNo: AscendPppoeEnable = AscendPppoeEnable(0);
	pub const PppoeYes: AscendPppoeEnable = AscendPppoeEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendPortRedirProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendPortRedirProtocol {
	pub const AscendProtoTcp: AscendPortRedirProtocol = AscendPortRedirProtocol(6);
	pub const AscendProtoUdp: AscendPortRedirProtocol = AscendPortRedirProtocol(17);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendPrivateRouteRequired(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendPrivateRouteRequired {
	pub const RequiredNo: AscendPrivateRouteRequired = AscendPrivateRouteRequired(0);
	pub const RequiredYes: AscendPrivateRouteRequired = AscendPrivateRouteRequired(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendRouteAppletalk(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendRouteAppletalk {
	pub const RouteAppletalkNo: AscendRouteAppletalk = AscendRouteAppletalk(0);
	pub const RouteAppletalkYes: AscendRouteAppletalk = AscendRouteAppletalk(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendSvcEnabled(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendSvcEnabled {
	pub const AscendSvcEnabledNo: AscendSvcEnabled = AscendSvcEnabled(0);
	pub const AscendSvcEnabledYes: AscendSvcEnabled = AscendSvcEnabled(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendServiceType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendServiceType {
	pub const AscendServiceTypeAtm: AscendServiceType = AscendServiceType(20);
	pub const AscendServiceTypeCombinet: AscendServiceType = AscendServiceType(7);
	pub const AscendServiceTypeEuraw: AscendServiceType = AscendServiceType(9);
	pub const AscendServiceTypeEuui: AscendServiceType = AscendServiceType(10);
	pub const AscendServiceTypeFr: AscendServiceType = AscendServiceType(8);
	pub const AscendServiceTypeHdlcnrm: AscendServiceType = AscendServiceType(21);
	pub const AscendServiceTypeIpfax: AscendServiceType = AscendServiceType(19);
	pub const AscendServiceTypeMp: AscendServiceType = AscendServiceType(15);
	pub const AscendServiceTypeMpp: AscendServiceType = AscendServiceType(5);
	pub const AscendServiceTypeNettonet: AscendServiceType = AscendServiceType(25);
	pub const AscendServiceTypeNone: AscendServiceType = AscendServiceType(1);
	pub const AscendServiceTypeNotused: AscendServiceType = AscendServiceType(0);
	pub const AscendServiceTypeOther: AscendServiceType = AscendServiceType(2);
	pub const AscendServiceTypePpp: AscendServiceType = AscendServiceType(3);
	pub const AscendServiceTypePseutunppp: AscendServiceType = AscendServiceType(18);
	pub const AscendServiceTypeRawtcp: AscendServiceType = AscendServiceType(13);
	pub const AscendServiceTypeSlip: AscendServiceType = AscendServiceType(4);
	pub const AscendServiceTypeTelnet: AscendServiceType = AscendServiceType(11);
	pub const AscendServiceTypeTelnetbin: AscendServiceType = AscendServiceType(12);
	pub const AscendServiceTypeTermserver: AscendServiceType = AscendServiceType(14);
	pub const AscendServiceTypeVirtualconn: AscendServiceType = AscendServiceType(16);
	pub const AscendServiceTypeVisa2: AscendServiceType = AscendServiceType(23);
	pub const AscendServiceTypeVoip: AscendServiceType = AscendServiceType(22);
	pub const AscendServiceTypeX25: AscendServiceType = AscendServiceType(6);
	pub const AscendServiceTypeX25Dchan: AscendServiceType = AscendServiceType(17);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendSessionType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendSessionType {
	pub const AscendSessionG711Alaw: AscendSessionType = AscendSessionType(3);
	pub const AscendSessionG711Ulaw: AscendSessionType = AscendSessionType(2);
	pub const AscendSessionG723: AscendSessionType = AscendSessionType(4);
	pub const AscendSessionG72364Kps: AscendSessionType = AscendSessionType(6);
	pub const AscendSessionG728: AscendSessionType = AscendSessionType(7);
	pub const AscendSessionG729: AscendSessionType = AscendSessionType(5);
	pub const AscendSessionRt24: AscendSessionType = AscendSessionType(8);
	pub const AscendSessionUnknown: AscendSessionType = AscendSessionType(1);
	pub const AscendSessionUnused: AscendSessionType = AscendSessionType(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendTunnelingProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendTunnelingProtocol {
	pub const AtmpTunnel: AscendTunnelingProtocol = AscendTunnelingProtocol(0);
	pub const VtpTunnel: AscendTunnelingProtocol = AscendTunnelingProtocol(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendX25PadX3Profile(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendX25PadX3Profile {
	pub const CcSsp: AscendX25PadX3Profile = AscendX25PadX3Profile(4);
	pub const CcTsp: AscendX25PadX3Profile = AscendX25PadX3Profile(5);
	pub const Crt: AscendX25PadX3Profile = AscendX25PadX3Profile(0);
	pub const Custom: AscendX25PadX3Profile = AscendX25PadX3Profile(11);
	pub const Default: AscendX25PadX3Profile = AscendX25PadX3Profile(2);
	pub const Hardcopy: AscendX25PadX3Profile = AscendX25PadX3Profile(6);
	pub const Hdx: AscendX25PadX3Profile = AscendX25PadX3Profile(7);
	pub const Infonet: AscendX25PadX3Profile = AscendX25PadX3Profile(1);
	pub const Null: AscendX25PadX3Profile = AscendX25PadX3Profile(10);
	pub const Pos: AscendX25PadX3Profile = AscendX25PadX3Profile(9);
	pub const Scen: AscendX25PadX3Profile = AscendX25PadX3Profile(3);
	pub const Shark: AscendX25PadX3Profile = AscendX25PadX3Profile(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AscendX25ReverseCharging(pub u32);
 
#[allow(non_upper_case_globals)]
impl AscendX25ReverseCharging {
	pub const ReverseChargingNo: AscendX25ReverseCharging = AscendX25ReverseCharging(0);
	pub const ReverseChargingYes: AscendX25ReverseCharging = AscendX25ReverseCharging(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		2 => map!{i, be_u32, |v| Attribute::VsaAscendMaxSharedUsers(v)},
		7 => value!(i, Attribute::VsaAscendUuInfo(i)),
		9 => map!{i, be_u32, |v| Attribute::VsaAscendCirTimer(v)},
		10 => map!{i, be_u32, |v| Attribute::VsaAscendFr08Mode(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaAscendDestinationNasPort(v)},
		12 => value!(i, Attribute::VsaAscendFrSvcAddr(i)),
		13 => map! {i, be_u32, |v| Attribute::VsaAscendNasPortFormat(AscendNasPortFormat(v))},
		14 => map! {i, be_u32, |v| Attribute::VsaAscendAtmFaultManagement(AscendAtmFaultManagement(v))},
		15 => map!{i, be_u32, |v| Attribute::VsaAscendAtmLoopbackCellLoss(v)},
		16 => map! {i, be_u32, |v| Attribute::VsaAscendCktType(AscendCktType(v))},
		17 => map! {i, be_u32, |v| Attribute::VsaAscendSvcEnabled(AscendSvcEnabled(v))},
		18 => map! {i, be_u32, |v| Attribute::VsaAscendSessionType(AscendSessionType(v))},
		19 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendH323Gatekeeper(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		20 => value!(i, Attribute::VsaAscendGlobalCallId(i)),
		21 => map!{i, be_u32, |v| Attribute::VsaAscendH323ConferenceId(v)},
		22 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendH323FegwAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		23 => map!{i, be_u32, |v| Attribute::VsaAscendH323DialedTime(v)},
		24 => value!(i, Attribute::VsaAscendDialedNumber(i)),
		25 => map!{i, be_u32, |v| Attribute::VsaAscendInterArrivalJitter(v)},
		26 => map!{i, be_u32, |v| Attribute::VsaAscendDroppedOctets(v)},
		27 => map!{i, be_u32, |v| Attribute::VsaAscendDroppedPackets(v)},
		28 => map!{i, be_u32, |v| Attribute::VsaAscendAuthDelay(v)},
		29 => map! {i, be_u32, |v| Attribute::VsaAscendX25PadX3Profile(AscendX25PadX3Profile(v))},
		30 => value!(i, Attribute::VsaAscendX25PadX3Parameters(i)),
		31 => value!(i, Attribute::VsaAscendTunnelVrouterName(i)),
		32 => map! {i, be_u32, |v| Attribute::VsaAscendX25ReverseCharging(AscendX25ReverseCharging(v))},
		33 => value!(i, Attribute::VsaAscendX25NuiPrompt(i)),
		34 => value!(i, Attribute::VsaAscendX25NuiPasswordPrompt(i)),
		35 => value!(i, Attribute::VsaAscendX25Cug(i)),
		36 => value!(i, Attribute::VsaAscendX25PadAlias1(i)),
		37 => value!(i, Attribute::VsaAscendX25PadAlias2(i)),
		38 => value!(i, Attribute::VsaAscendX25PadAlias3(i)),
		39 => value!(i, Attribute::VsaAscendX25X121Address(i)),
		40 => value!(i, Attribute::VsaAscendX25Nui(i)),
		41 => value!(i, Attribute::VsaAscendX25Rpoa(i)),
		42 => value!(i, Attribute::VsaAscendX25PadPrompt(i)),
		43 => value!(i, Attribute::VsaAscendX25PadBanner(i)),
		44 => value!(i, Attribute::VsaAscendX25ProfileName(i)),
		45 => value!(i, Attribute::VsaAscendRecvName(i)),
		46 => map! {i, be_u32, |v| Attribute::VsaAscendBiDirectionalAuth(AscendBiDirectionalAuth(v))},
		47 => map!{i, be_u32, |v| Attribute::VsaAscendMtu(v)},
		48 => map! {i, be_u32, |v| Attribute::VsaAscendCallDirection(AscendCallDirection(v))},
		49 => map! {i, be_u32, |v| Attribute::VsaAscendServiceType(AscendServiceType(v))},
		50 => map! {i, be_u32, |v| Attribute::VsaAscendFilterRequired(AscendFilterRequired(v))},
		51 => map!{i, be_u32, |v| Attribute::VsaAscendTrafficShaper(v)},
		52 => value!(i, Attribute::VsaAscendAccessInterceptLea(i)),
		53 => value!(i, Attribute::VsaAscendAccessInterceptLog(i)),
		54 => value!(i, Attribute::VsaAscendPrivateRouteTableId(i)),
		55 => map! {i, be_u32, |v| Attribute::VsaAscendPrivateRouteRequired(AscendPrivateRouteRequired(v))},
		56 => map! {i, be_u32, |v| Attribute::VsaAscendCacheRefresh(AscendCacheRefresh(v))},
		57 => map!{i, be_u32, |v| Attribute::VsaAscendCacheTime(v)},
		58 => map!{i, be_u32, |v| Attribute::VsaAscendEgressEnabled(v)},
		59 => value!(i, Attribute::VsaAscendQosUpstream(i)),
		60 => value!(i, Attribute::VsaAscendQosDownstream(i)),
		61 => map!{i, be_u32, |v| Attribute::VsaAscendAtmConnectVpi(v)},
		62 => map!{i, be_u32, |v| Attribute::VsaAscendAtmConnectVci(v)},
		63 => map!{i, be_u32, |v| Attribute::VsaAscendAtmConnectGroup(v)},
		64 => map!{i, be_u32, |v| Attribute::VsaAscendAtmGroup(v)},
		65 => map! {i, be_u32, |v| Attribute::VsaAscendIpxHeaderCompression(AscendIpxHeaderCompression(v))},
		66 => map! {i, be_u32, |v| Attribute::VsaAscendCallingIdTypeOfNum(AscendCallingIdTypeOfNum(v))},
		67 => map! {i, be_u32, |v| Attribute::VsaAscendCallingIdNumberPlan(AscendCallingIdNumberPlan(v))},
		68 => map! {i, be_u32, |v| Attribute::VsaAscendCallingIdPresentatn(AscendCallingIdPresentatn(v))},
		69 => map! {i, be_u32, |v| Attribute::VsaAscendCallingIdScreening(AscendCallingIdScreening(v))},
		70 => map! {i, be_u32, |v| Attribute::VsaAscendBirEnable(AscendBirEnable(v))},
		71 => map! {i, be_u32, |v| Attribute::VsaAscendBirProxy(AscendBirProxy(v))},
		72 => map!{i, be_u32, |v| Attribute::VsaAscendBirBridgeGroup(v)},
		73 => value!(i, Attribute::VsaAscendIpsecProfile(i)),
		74 => map! {i, be_u32, |v| Attribute::VsaAscendPppoeEnable(AscendPppoeEnable(v))},
		75 => map! {i, be_u32, |v| Attribute::VsaAscendBridgeNonPppoe(AscendBridgeNonPppoe(v))},
		76 => map! {i, be_u32, |v| Attribute::VsaAscendAtmDirect(AscendAtmDirect(v))},
		77 => value!(i, Attribute::VsaAscendAtmDirectProfile(i)),
		78 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendClientPrimaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		79 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendClientSecondaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		80 => map! {i, be_u32, |v| Attribute::VsaAscendClientAssignWins(AscendClientAssignWins(v))},
		81 => map! {i, be_u32, |v| Attribute::VsaAscendAuthType(AscendAuthType(v))},
		82 => map! {i, be_u32, |v| Attribute::VsaAscendPortRedirProtocol(AscendPortRedirProtocol(v))},
		83 => map!{i, be_u32, |v| Attribute::VsaAscendPortRedirPortnum(v)},
		84 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendPortRedirServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		85 => map! {i, be_u32, |v| Attribute::VsaAscendIpPoolChaining(AscendIpPoolChaining(v))},
		86 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendOwnerIpAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		87 => map! {i, be_u32, |v| Attribute::VsaAscendIpTos(AscendIpTos(v))},
		88 => map! {i, be_u32, |v| Attribute::VsaAscendIpTosPrecedence(AscendIpTosPrecedence(v))},
		89 => map! {i, be_u32, |v| Attribute::VsaAscendIpTosApplyTo(AscendIpTosApplyTo(v))},
		90 => value!(i, Attribute::VsaAscendFilter(i)),
		91 => value!(i, Attribute::VsaAscendTelnetProfile(i)),
		92 => map! {i, be_u32, |v| Attribute::VsaAscendDslRateType(AscendDslRateType(v))},
		93 => value!(i, Attribute::VsaAscendRedirectNumber(i)),
		94 => map!{i, be_u32, |v| Attribute::VsaAscendAtmVpi(v)},
		95 => map!{i, be_u32, |v| Attribute::VsaAscendAtmVci(v)},
		96 => map! {i, be_u32, |v| Attribute::VsaAscendSourceIpCheck(AscendSourceIpCheck(v))},
		97 => map! {i, be_u32, |v| Attribute::VsaAscendDslRateMode(AscendDslRateMode(v))},
		98 => map! {i, be_u32, |v| Attribute::VsaAscendDslUpstreamLimit(AscendDslUpstreamLimit(v))},
		99 => map! {i, be_u32, |v| Attribute::VsaAscendDslDownstreamLimit(AscendDslDownstreamLimit(v))},
		100 => map!{i, be_u32, |v| Attribute::VsaAscendDslCirRecvLimit(v)},
		101 => map!{i, be_u32, |v| Attribute::VsaAscendDslCirXmitLimit(v)},
		102 => value!(i, Attribute::VsaAscendVrouterName(i)),
		103 => value!(i, Attribute::VsaAscendSourceAuth(i)),
		104 => value!(i, Attribute::VsaAscendPrivateRoute(i)),
		105 => map! {i, be_u32, |v| Attribute::VsaAscendNumberingPlanId(AscendNumberingPlanId(v))},
		106 => map! {i, be_u32, |v| Attribute::VsaAscendFrLinkStatusDlci(AscendFrLinkStatusDlci(v))},
		107 => value!(i, Attribute::VsaAscendCallingSubaddress(i)),
		108 => map!{i, be_u32, |v| Attribute::VsaAscendCallbackDelay(v)},
		109 => value!(i, Attribute::VsaAscendEndpointDisc(i)),
		110 => value!(i, Attribute::VsaAscendRemoteFw(i)),
		111 => map!{i, be_u32, |v| Attribute::VsaAscendMulticastGleaveDelay(v)},
		112 => map! {i, be_u32, |v| Attribute::VsaAscendCbcpEnable(AscendCbcpEnable(v))},
		113 => map! {i, be_u32, |v| Attribute::VsaAscendCbcpMode(AscendCbcpMode(v))},
		114 => map!{i, be_u32, |v| Attribute::VsaAscendCbcpDelay(v)},
		115 => map!{i, be_u32, |v| Attribute::VsaAscendCbcpTrunkGroup(v)},
		116 => value!(i, Attribute::VsaAscendAppletalkRoute(i)),
		117 => map! {i, be_u32, |v| Attribute::VsaAscendAppletalkPeerMode(AscendAppletalkPeerMode(v))},
		118 => map! {i, be_u32, |v| Attribute::VsaAscendRouteAppletalk(AscendRouteAppletalk(v))},
		119 => value!(i, Attribute::VsaAscendFcpParameter(i)),
		120 => map!{i, be_u32, |v| Attribute::VsaAscendModemPortno(v)},
		121 => map!{i, be_u32, |v| Attribute::VsaAscendModemSlotno(v)},
		122 => map!{i, be_u32, |v| Attribute::VsaAscendModemShelfno(v)},
		123 => map!{i, be_u32, |v| Attribute::VsaAscendCallAttemptLimit(v)},
		124 => map!{i, be_u32, |v| Attribute::VsaAscendCallBlockDuration(v)},
		125 => map!{i, be_u32, |v| Attribute::VsaAscendMaximumCallDuration(v)},
		126 => map! {i, be_u32, |v| Attribute::VsaAscendTemporaryRtes(AscendTemporaryRtes(v))},
		127 => map! {i, be_u32, |v| Attribute::VsaAscendTunnelingProtocol(AscendTunnelingProtocol(v))},
		128 => map! {i, be_u32, |v| Attribute::VsaAscendSharedProfileEnable(AscendSharedProfileEnable(v))},
		129 => value!(i, Attribute::VsaAscendPrimaryHomeAgent(i)),
		130 => value!(i, Attribute::VsaAscendSecondaryHomeAgent(i)),
		131 => map! {i, be_u32, |v| Attribute::VsaAscendDialoutAllowed(AscendDialoutAllowed(v))},
		132 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendClientGateway(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		133 => map! {i, be_u32, |v| Attribute::VsaAscendBacpEnable(AscendBacpEnable(v))},
		134 => map!{i, be_u32, |v| Attribute::VsaAscendDhcpMaximumLeases(v)},
		135 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendClientPrimaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		136 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendClientSecondaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		137 => map! {i, be_u32, |v| Attribute::VsaAscendClientAssignDns(AscendClientAssignDns(v))},
		138 => map! {i, be_u32, |v| Attribute::VsaAscendUserAcctType(AscendUserAcctType(v))},
		139 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendUserAcctHost(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		140 => map!{i, be_u32, |v| Attribute::VsaAscendUserAcctPort(v)},
		141 => value!(i, Attribute::VsaAscendUserAcctKey(i)),
		142 => map! {i, be_u32, |v| Attribute::VsaAscendUserAcctBase(AscendUserAcctBase(v))},
		143 => map!{i, be_u32, |v| Attribute::VsaAscendUserAcctTime(v)},
		144 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendAssignIpClient(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		145 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendAssignIpServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		146 => value!(i, Attribute::VsaAscendAssignIpGlobalPool(i)),
		147 => map! {i, be_u32, |v| Attribute::VsaAscendDhcpReply(AscendDhcpReply(v))},
		148 => map!{i, be_u32, |v| Attribute::VsaAscendDhcpPoolNumber(v)},
		149 => map! {i, be_u32, |v| Attribute::VsaAscendExpectCallback(AscendExpectCallback(v))},
		150 => map! {i, be_u32, |v| Attribute::VsaAscendEventType(AscendEventType(v))},
		151 => value!(i, Attribute::VsaAscendSessionSvrKey(i)),
		152 => map!{i, be_u32, |v| Attribute::VsaAscendMulticastRateLimit(v)},
		153 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendIfNetmask(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		154 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendRemoteAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		155 => map! {i, be_u32, |v| Attribute::VsaAscendMulticastClient(AscendMulticastClient(v))},
		156 => value!(i, Attribute::VsaAscendFrCircuitName(i)),
		157 => map! {i, be_u32, |v| Attribute::VsaAscendFrLinkup(AscendFrLinkup(v))},
		158 => map!{i, be_u32, |v| Attribute::VsaAscendFrNailedGrp(v)},
		159 => map! {i, be_u32, |v| Attribute::VsaAscendFrType(AscendFrType(v))},
		160 => map! {i, be_u32, |v| Attribute::VsaAscendFrLinkMgt(AscendFrLinkMgt(v))},
		161 => map!{i, be_u32, |v| Attribute::VsaAscendFrN391(v)},
		162 => map!{i, be_u32, |v| Attribute::VsaAscendFrDceN392(v)},
		163 => map!{i, be_u32, |v| Attribute::VsaAscendFrDteN392(v)},
		164 => map!{i, be_u32, |v| Attribute::VsaAscendFrDceN393(v)},
		165 => map!{i, be_u32, |v| Attribute::VsaAscendFrDteN393(v)},
		166 => map!{i, be_u32, |v| Attribute::VsaAscendFrT391(v)},
		167 => map!{i, be_u32, |v| Attribute::VsaAscendFrT392(v)},
		168 => value!(i, Attribute::VsaAscendBridgeAddress(i)),
		169 => map!{i, be_u32, |v| Attribute::VsaAscendTsIdleLimit(v)},
		170 => map! {i, be_u32, |v| Attribute::VsaAscendTsIdleMode(AscendTsIdleMode(v))},
		171 => map! {i, be_u32, |v| Attribute::VsaAscendDbaMonitor(AscendDbaMonitor(v))},
		172 => map!{i, be_u32, |v| Attribute::VsaAscendBaseChannelCount(v)},
		173 => map!{i, be_u32, |v| Attribute::VsaAscendMinimumChannels(v)},
		174 => value!(i, Attribute::VsaAscendIpxRoute(i)),
		175 => map! {i, be_u32, |v| Attribute::VsaAscendFt1Caller(AscendFt1Caller(v))},
		176 => value!(i, Attribute::VsaAscendBackup(i)),
		177 => map! {i, be_u32, |v| Attribute::VsaAscendCallType(AscendCallType(v))},
		178 => value!(i, Attribute::VsaAscendGroup(i)),
		179 => map!{i, be_u32, |v| Attribute::VsaAscendFrDlci(v)},
		180 => value!(i, Attribute::VsaAscendFrProfileName(i)),
		181 => value!(i, Attribute::VsaAscendAraPw(i)),
		182 => value!(i, Attribute::VsaAscendIpxNodeAddr(i)),
		183 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendHomeAgentIpAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		184 => value!(i, Attribute::VsaAscendHomeAgentPassword(i)),
		185 => value!(i, Attribute::VsaAscendHomeNetworkName(i)),
		186 => map!{i, be_u32, |v| Attribute::VsaAscendHomeAgentUdpPort(v)},
		187 => map!{i, be_u32, |v| Attribute::VsaAscendMultilinkId(v)},
		188 => map!{i, be_u32, |v| Attribute::VsaAscendNumInMultilink(v)},
		189 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendFirstDest(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		190 => map!{i, be_u32, |v| Attribute::VsaAscendPreInputOctets(v)},
		191 => map!{i, be_u32, |v| Attribute::VsaAscendPreOutputOctets(v)},
		192 => map!{i, be_u32, |v| Attribute::VsaAscendPreInputPackets(v)},
		193 => map!{i, be_u32, |v| Attribute::VsaAscendPreOutputPackets(v)},
		194 => map!{i, be_u32, |v| Attribute::VsaAscendMaximumTime(v)},
		195 => map! {i, be_u32, |v| Attribute::VsaAscendDisconnectCause(AscendDisconnectCause(v))},
		196 => map! {i, be_u32, |v| Attribute::VsaAscendConnectProgress(AscendConnectProgress(v))},
		197 => map!{i, be_u32, |v| Attribute::VsaAscendDataRate(v)},
		198 => map!{i, be_u32, |v| Attribute::VsaAscendPresessionTime(v)},
		199 => map!{i, be_u32, |v| Attribute::VsaAscendTokenIdle(v)},
		200 => map! {i, be_u32, |v| Attribute::VsaAscendTokenImmediate(AscendTokenImmediate(v))},
		201 => map! {i, be_u32, |v| Attribute::VsaAscendRequireAuth(AscendRequireAuth(v))},
		202 => value!(i, Attribute::VsaAscendNumberSessions(i)),
		203 => value!(i, Attribute::VsaAscendAuthenAlias(i)),
		204 => map!{i, be_u32, |v| Attribute::VsaAscendTokenExpiry(v)},
		205 => value!(i, Attribute::VsaAscendMenuSelector(i)),
		206 => value!(i, Attribute::VsaAscendMenuItem(i)),
		207 => map! {i, be_u32, |v| Attribute::VsaAscendPwWarntime(AscendPwWarntime(v))},
		208 => map! {i, be_u32, |v| Attribute::VsaAscendPwLifetime(AscendPwLifetime(v))},
		209 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendIpDirect(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		210 => map! {i, be_u32, |v| Attribute::VsaAscendPppVjSlotComp(AscendPppVjSlotComp(v))},
		211 => map! {i, be_u32, |v| Attribute::VsaAscendPppVj1172(AscendPppVj1172(v))},
		212 => map!{i, be_u32, |v| Attribute::VsaAscendPppAsyncMap(v)},
		213 => value!(i, Attribute::VsaAscendThirdPrompt(i)),
		214 => value!(i, Attribute::VsaAscendSendSecret(i)),
		215 => value!(i, Attribute::VsaAscendReceiveSecret(i)),
		216 => map! {i, be_u32, |v| Attribute::VsaAscendIpxPeerMode(AscendIpxPeerMode(v))},
		217 => value!(i, Attribute::VsaAscendIpPoolDefinition(i)),
		218 => map!{i, be_u32, |v| Attribute::VsaAscendAssignIpPool(v)},
		219 => map! {i, be_u32, |v| Attribute::VsaAscendFrDirect(AscendFrDirect(v))},
		220 => value!(i, Attribute::VsaAscendFrDirectProfile(i)),
		221 => map!{i, be_u32, |v| Attribute::VsaAscendFrDirectDlci(v)},
		222 => map! {i, be_u32, |v| Attribute::VsaAscendHandleIpx(AscendHandleIpx(v))},
		223 => map!{i, be_u32, |v| Attribute::VsaAscendNetwareTimeout(v)},
		224 => map!{i, be_u32, |v| Attribute::VsaAscendIpxAlias(v)},
		225 => map!{i, be_u32, |v| Attribute::VsaAscendMetric(v)},
		226 => map! {i, be_u32, |v| Attribute::VsaAscendPriNumberType(AscendPriNumberType(v))},
		227 => value!(i, Attribute::VsaAscendDialNumber(i)),
		228 => map! {i, be_u32, |v| Attribute::VsaAscendRouteIp(AscendRouteIp(v))},
		229 => map! {i, be_u32, |v| Attribute::VsaAscendRouteIpx(AscendRouteIpx(v))},
		230 => map! {i, be_u32, |v| Attribute::VsaAscendBridge(AscendBridge(v))},
		231 => map! {i, be_u32, |v| Attribute::VsaAscendSendAuth(AscendSendAuth(v))},
		232 => value!(i, Attribute::VsaAscendSendPasswd(i)),
		233 => map! {i, be_u32, |v| Attribute::VsaAscendLinkCompression(AscendLinkCompression(v))},
		234 => map!{i, be_u32, |v| Attribute::VsaAscendTargetUtil(v)},
		235 => map!{i, be_u32, |v| Attribute::VsaAscendMaximumChannels(v)},
		236 => map!{i, be_u32, |v| Attribute::VsaAscendIncChannelCount(v)},
		237 => map!{i, be_u32, |v| Attribute::VsaAscendDecChannelCount(v)},
		238 => map!{i, be_u32, |v| Attribute::VsaAscendSecondsOfHistory(v)},
		239 => map! {i, be_u32, |v| Attribute::VsaAscendHistoryWeighType(AscendHistoryWeighType(v))},
		240 => map!{i, be_u32, |v| Attribute::VsaAscendAddSeconds(v)},
		241 => map!{i, be_u32, |v| Attribute::VsaAscendRemoveSeconds(v)},
		242 => value!(i, Attribute::VsaAscendDataFilter(i)),
		243 => value!(i, Attribute::VsaAscendCallFilter(i)),
		244 => map!{i, be_u32, |v| Attribute::VsaAscendIdleLimit(v)},
		245 => map!{i, be_u32, |v| Attribute::VsaAscendPreemptLimit(v)},
		246 => map! {i, be_u32, |v| Attribute::VsaAscendCallback(AscendCallback(v))},
		247 => map! {i, be_u32, |v| Attribute::VsaAscendDataSvc(AscendDataSvc(v))},
		248 => map! {i, be_u32, |v| Attribute::VsaAscendForce56(AscendForce56(v))},
		249 => value!(i, Attribute::VsaAscendBillingNumber(i)),
		250 => map!{i, be_u32, |v| Attribute::VsaAscendCallByCall(v)},
		251 => value!(i, Attribute::VsaAscendTransitNumber(i)),
		252 => value!(i, Attribute::VsaAscendHostInfo(i)),
		253 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAscendPppAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		254 => map!{i, be_u32, |v| Attribute::VsaAscendMppIdlePercent(v)},
		255 => map!{i, be_u32, |v| Attribute::VsaAscendXmitRate(v)},
        _ => value!(i, Attribute::VsaUnknown(529, typ, i)),
    }
}
