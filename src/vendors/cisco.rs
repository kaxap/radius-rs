/// Definitions for vendor Cisco, vendor value 9
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CiscoDisconnectCause(pub u32);
 
#[allow(non_upper_case_globals)]
impl CiscoDisconnectCause {
	pub const NoReason: CiscoDisconnectCause = CiscoDisconnectCause(0);
	pub const NoDisconnect: CiscoDisconnectCause = CiscoDisconnectCause(1);
	pub const Unknown: CiscoDisconnectCause = CiscoDisconnectCause(2);
	pub const CallDisconnect: CiscoDisconnectCause = CiscoDisconnectCause(3);
	pub const ClidAuthenticationFailure: CiscoDisconnectCause = CiscoDisconnectCause(4);
	pub const NoModemAvailable: CiscoDisconnectCause = CiscoDisconnectCause(9);
	pub const NoCarrier: CiscoDisconnectCause = CiscoDisconnectCause(10);
	pub const LostCarrier: CiscoDisconnectCause = CiscoDisconnectCause(11);
	pub const NoDetectedResultCodes: CiscoDisconnectCause = CiscoDisconnectCause(12);
	pub const UserEndsSession: CiscoDisconnectCause = CiscoDisconnectCause(20);
	pub const IdleTimeout: CiscoDisconnectCause = CiscoDisconnectCause(21);
	pub const ExitTelnetSession: CiscoDisconnectCause = CiscoDisconnectCause(22);
	pub const NoRemoteIpAddr: CiscoDisconnectCause = CiscoDisconnectCause(23);
	pub const ExitRawTcp: CiscoDisconnectCause = CiscoDisconnectCause(24);
	pub const PasswordFail: CiscoDisconnectCause = CiscoDisconnectCause(25);
	pub const RawTcpDisabled: CiscoDisconnectCause = CiscoDisconnectCause(26);
	pub const ControlCDetected: CiscoDisconnectCause = CiscoDisconnectCause(27);
	pub const ExecProgramDestroyed: CiscoDisconnectCause = CiscoDisconnectCause(28);
	pub const CloseVirtualConnection: CiscoDisconnectCause = CiscoDisconnectCause(29);
	pub const EndVirtualConnection: CiscoDisconnectCause = CiscoDisconnectCause(30);
	pub const ExitRlogin: CiscoDisconnectCause = CiscoDisconnectCause(31);
	pub const InvalidRloginOption: CiscoDisconnectCause = CiscoDisconnectCause(32);
	pub const InsufficientResources: CiscoDisconnectCause = CiscoDisconnectCause(33);
	pub const TimeoutPppLcp: CiscoDisconnectCause = CiscoDisconnectCause(40);
	pub const FailedPppLcpNegotiation: CiscoDisconnectCause = CiscoDisconnectCause(41);
	pub const FailedPppPapAuthFail: CiscoDisconnectCause = CiscoDisconnectCause(42);
	pub const FailedPppChapAuth: CiscoDisconnectCause = CiscoDisconnectCause(43);
	pub const FailedPppRemoteAuth: CiscoDisconnectCause = CiscoDisconnectCause(44);
	pub const PppRemoteTerminate: CiscoDisconnectCause = CiscoDisconnectCause(45);
	pub const PppClosedEvent: CiscoDisconnectCause = CiscoDisconnectCause(46);
	pub const NcpClosedPpp: CiscoDisconnectCause = CiscoDisconnectCause(47);
	pub const MpErrorPpp: CiscoDisconnectCause = CiscoDisconnectCause(48);
	pub const PppMaximumChannels: CiscoDisconnectCause = CiscoDisconnectCause(49);
	pub const TablesFull: CiscoDisconnectCause = CiscoDisconnectCause(50);
	pub const ResourcesFull: CiscoDisconnectCause = CiscoDisconnectCause(51);
	pub const InvalidIpAddress: CiscoDisconnectCause = CiscoDisconnectCause(52);
	pub const BadHostname: CiscoDisconnectCause = CiscoDisconnectCause(53);
	pub const BadPort: CiscoDisconnectCause = CiscoDisconnectCause(54);
	pub const ResetTcp: CiscoDisconnectCause = CiscoDisconnectCause(60);
	pub const TcpConnectionRefused: CiscoDisconnectCause = CiscoDisconnectCause(61);
	pub const TimeoutTcp: CiscoDisconnectCause = CiscoDisconnectCause(62);
	pub const ForeignHostCloseTcp: CiscoDisconnectCause = CiscoDisconnectCause(63);
	pub const TcpNetworkUnreachable: CiscoDisconnectCause = CiscoDisconnectCause(64);
	pub const TcpHostUnreachable: CiscoDisconnectCause = CiscoDisconnectCause(65);
	pub const TcpNetworkAdminUnreachable: CiscoDisconnectCause = CiscoDisconnectCause(66);
	pub const TcpPortUnreachable: CiscoDisconnectCause = CiscoDisconnectCause(67);
	pub const SessionTimeout: CiscoDisconnectCause = CiscoDisconnectCause(100);
	pub const SessionFailedSecurity: CiscoDisconnectCause = CiscoDisconnectCause(101);
	pub const SessionEndCallback: CiscoDisconnectCause = CiscoDisconnectCause(102);
	pub const InvalidProtocol: CiscoDisconnectCause = CiscoDisconnectCause(120);
	pub const RadiusDisconnect: CiscoDisconnectCause = CiscoDisconnectCause(150);
	pub const LocalAdminDisconnect: CiscoDisconnectCause = CiscoDisconnectCause(151);
	pub const SnmpDisconnect: CiscoDisconnectCause = CiscoDisconnectCause(152);
	pub const V110Retries: CiscoDisconnectCause = CiscoDisconnectCause(160);
	pub const PppAuthenticationTimeout: CiscoDisconnectCause = CiscoDisconnectCause(170);
	pub const LocalHangup: CiscoDisconnectCause = CiscoDisconnectCause(180);
	pub const RemoteHangup: CiscoDisconnectCause = CiscoDisconnectCause(185);
	pub const T1Quiesced: CiscoDisconnectCause = CiscoDisconnectCause(190);
	pub const CallDuration: CiscoDisconnectCause = CiscoDisconnectCause(195);
	pub const VpnUserDisconnect: CiscoDisconnectCause = CiscoDisconnectCause(600);
	pub const VpnCarrierLoss: CiscoDisconnectCause = CiscoDisconnectCause(601);
	pub const VpnNoResources: CiscoDisconnectCause = CiscoDisconnectCause(602);
	pub const VpnBadControlPacket: CiscoDisconnectCause = CiscoDisconnectCause(603);
	pub const VpnAdminDisconnect: CiscoDisconnectCause = CiscoDisconnectCause(604);
	pub const VpnTunnelShut: CiscoDisconnectCause = CiscoDisconnectCause(605);
	pub const VpnLocalDisconnect: CiscoDisconnectCause = CiscoDisconnectCause(606);
	pub const VpnSessionLimit: CiscoDisconnectCause = CiscoDisconnectCause(607);
	pub const VpnCallRedirect: CiscoDisconnectCause = CiscoDisconnectCause(608);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaCiscoAvpair(i)),
		2 => value!(i, Attribute::VsaCiscoNasPort(i)),
		3 => value!(i, Attribute::VsaCiscoFaxAccountIdOrigin(i)),
		4 => value!(i, Attribute::VsaCiscoFaxMsgId(i)),
		5 => value!(i, Attribute::VsaCiscoFaxPages(i)),
		6 => value!(i, Attribute::VsaCiscoFaxCoverpageFlag(i)),
		7 => value!(i, Attribute::VsaCiscoFaxModemTime(i)),
		8 => value!(i, Attribute::VsaCiscoFaxConnectSpeed(i)),
		9 => value!(i, Attribute::VsaCiscoFaxRecipientCount(i)),
		10 => value!(i, Attribute::VsaCiscoFaxProcessAbortFlag(i)),
		11 => value!(i, Attribute::VsaCiscoFaxDsnAddress(i)),
		12 => value!(i, Attribute::VsaCiscoFaxDsnFlag(i)),
		13 => value!(i, Attribute::VsaCiscoFaxMdnAddress(i)),
		14 => value!(i, Attribute::VsaCiscoFaxMdnFlag(i)),
		15 => value!(i, Attribute::VsaCiscoFaxAuthStatus(i)),
		16 => value!(i, Attribute::VsaCiscoEmailServerAddress(i)),
		17 => value!(i, Attribute::VsaCiscoEmailServerAckFlag(i)),
		18 => value!(i, Attribute::VsaCiscoGatewayId(i)),
		19 => value!(i, Attribute::VsaCiscoCallType(i)),
		20 => value!(i, Attribute::VsaCiscoPortUsed(i)),
		21 => value!(i, Attribute::VsaCiscoAbortCause(i)),
		23 => value!(i, Attribute::VsaH323RemoteAddress(i)),
		24 => value!(i, Attribute::VsaH323ConfId(i)),
		25 => value!(i, Attribute::VsaH323SetupTime(i)),
		26 => value!(i, Attribute::VsaH323CallOrigin(i)),
		27 => value!(i, Attribute::VsaH323CallType(i)),
		28 => value!(i, Attribute::VsaH323ConnectTime(i)),
		29 => value!(i, Attribute::VsaH323DisconnectTime(i)),
		30 => value!(i, Attribute::VsaH323DisconnectCause(i)),
		31 => value!(i, Attribute::VsaH323VoiceQuality(i)),
		33 => value!(i, Attribute::VsaH323GwId(i)),
		35 => value!(i, Attribute::VsaH323IncomingConfId(i)),
		37 => value!(i, Attribute::VsaCiscoPolicyUp(i)),
		38 => value!(i, Attribute::VsaCiscoPolicyDown(i)),
		100 => value!(i, Attribute::VsaSipConfId(i)),
		101 => value!(i, Attribute::VsaH323CreditAmount(i)),
		102 => value!(i, Attribute::VsaH323CreditTime(i)),
		103 => value!(i, Attribute::VsaH323ReturnCode(i)),
		104 => value!(i, Attribute::VsaH323PromptId(i)),
		105 => value!(i, Attribute::VsaH323TimeAndDay(i)),
		106 => value!(i, Attribute::VsaH323RedirectNumber(i)),
		107 => value!(i, Attribute::VsaH323PreferredLang(i)),
		108 => value!(i, Attribute::VsaH323RedirectIpAddress(i)),
		109 => value!(i, Attribute::VsaH323BillingModel(i)),
		110 => value!(i, Attribute::VsaH323Currency(i)),
		111 => value!(i, Attribute::VsaSubscriber(i)),
		112 => value!(i, Attribute::VsaGwRxdCdn(i)),
		113 => value!(i, Attribute::VsaGwFinalXlatedCdn(i)),
		114 => value!(i, Attribute::VsaRemoteMediaAddress(i)),
		115 => value!(i, Attribute::VsaReleaseSource(i)),
		116 => value!(i, Attribute::VsaGwRxdCgn(i)),
		117 => value!(i, Attribute::VsaGwFinalXlatedCgn(i)),
		141 => value!(i, Attribute::VsaCallId(i)),
		142 => value!(i, Attribute::VsaSessionProtocol(i)),
		143 => value!(i, Attribute::VsaMethod(i)),
		144 => value!(i, Attribute::VsaPrevHopVia(i)),
		145 => value!(i, Attribute::VsaPrevHopIp(i)),
		146 => value!(i, Attribute::VsaIncomingReqUri(i)),
		147 => value!(i, Attribute::VsaOutgoingReqUri(i)),
		148 => value!(i, Attribute::VsaNextHopIp(i)),
		149 => value!(i, Attribute::VsaNextHopDn(i)),
		150 => value!(i, Attribute::VsaSipHdr(i)),
		151 => value!(i, Attribute::VsaDspId(i)),
		187 => map!{i, be_u32, |v| Attribute::VsaCiscoMultilinkId(v)},
		188 => map!{i, be_u32, |v| Attribute::VsaCiscoNumInMultilink(v)},
		190 => map!{i, be_u32, |v| Attribute::VsaCiscoPreInputOctets(v)},
		191 => map!{i, be_u32, |v| Attribute::VsaCiscoPreOutputOctets(v)},
		192 => map!{i, be_u32, |v| Attribute::VsaCiscoPreInputPackets(v)},
		193 => map!{i, be_u32, |v| Attribute::VsaCiscoPreOutputPackets(v)},
		194 => map!{i, be_u32, |v| Attribute::VsaCiscoMaximumTime(v)},
		195 => map! {i, be_u32, |v| Attribute::VsaCiscoDisconnectCause(CiscoDisconnectCause(v))},
		197 => map!{i, be_u32, |v| Attribute::VsaCiscoDataRate(v)},
		198 => map!{i, be_u32, |v| Attribute::VsaCiscoPresessionTime(v)},
		208 => map!{i, be_u32, |v| Attribute::VsaCiscoPwLifetime(v)},
		209 => map!{i, be_u32, |v| Attribute::VsaCiscoIpDirect(v)},
		210 => map!{i, be_u32, |v| Attribute::VsaCiscoPppVjSlotComp(v)},
		212 => map!{i, be_u32, |v| Attribute::VsaCiscoPppAsyncMap(v)},
		217 => value!(i, Attribute::VsaCiscoIpPoolDefinition(i)),
		218 => map!{i, be_u32, |v| Attribute::VsaCiscoAssignIpPool(v)},
		228 => map!{i, be_u32, |v| Attribute::VsaCiscoRouteIp(v)},
		233 => map!{i, be_u32, |v| Attribute::VsaCiscoLinkCompression(v)},
		234 => map!{i, be_u32, |v| Attribute::VsaCiscoTargetUtil(v)},
		235 => map!{i, be_u32, |v| Attribute::VsaCiscoMaximumChannels(v)},
		242 => map!{i, be_u32, |v| Attribute::VsaCiscoDataFilter(v)},
		243 => map!{i, be_u32, |v| Attribute::VsaCiscoCallFilter(v)},
		244 => map!{i, be_u32, |v| Attribute::VsaCiscoIdleLimit(v)},
		249 => value!(i, Attribute::VsaCiscoSubscriberPassword(i)),
		250 => value!(i, Attribute::VsaCiscoAccountInfo(i)),
		251 => value!(i, Attribute::VsaCiscoServiceInfo(i)),
		252 => value!(i, Attribute::VsaCiscoCommandCode(i)),
		253 => value!(i, Attribute::VsaCiscoControlInfo(i)),
		255 => map!{i, be_u32, |v| Attribute::VsaCiscoXmitRate(v)},
        _ => value!(i, Attribute::VsaUnknown(9, typ, i)),
    }
}
