/// Definitions for vendor Starent, vendor value 8164
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1DisconnectReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1DisconnectReason {
	pub const NotDefined: Sn1DisconnectReason = Sn1DisconnectReason(0);
	pub const AdminDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(1);
	pub const RemoteDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(2);
	pub const LocalDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(3);
	pub const DiscNoResource: Sn1DisconnectReason = Sn1DisconnectReason(4);
	pub const DiscExcdServiceLimit: Sn1DisconnectReason = Sn1DisconnectReason(5);
	pub const PppLcpNegFailed: Sn1DisconnectReason = Sn1DisconnectReason(6);
	pub const PppLcpNoResponse: Sn1DisconnectReason = Sn1DisconnectReason(7);
	pub const PppLcpLoopback: Sn1DisconnectReason = Sn1DisconnectReason(8);
	pub const PppLcpMaxRetry: Sn1DisconnectReason = Sn1DisconnectReason(9);
	pub const PppEchoFailed: Sn1DisconnectReason = Sn1DisconnectReason(10);
	pub const PppAuthFailed: Sn1DisconnectReason = Sn1DisconnectReason(11);
	pub const PppAuthFailedNoAaaResp: Sn1DisconnectReason = Sn1DisconnectReason(12);
	pub const PppAuthNoResponse: Sn1DisconnectReason = Sn1DisconnectReason(13);
	pub const PppAuthMaxRetry: Sn1DisconnectReason = Sn1DisconnectReason(14);
	pub const InvalidAaaAttr: Sn1DisconnectReason = Sn1DisconnectReason(15);
	pub const FailedUserFilter: Sn1DisconnectReason = Sn1DisconnectReason(16);
	pub const FailedProvideService: Sn1DisconnectReason = Sn1DisconnectReason(17);
	pub const InvalidIpAddressAaa: Sn1DisconnectReason = Sn1DisconnectReason(18);
	pub const InvalidIpPoolAaa: Sn1DisconnectReason = Sn1DisconnectReason(19);
	pub const PppIpcpNegFailed: Sn1DisconnectReason = Sn1DisconnectReason(20);
	pub const PppIpcpNoResponse: Sn1DisconnectReason = Sn1DisconnectReason(21);
	pub const PppIpcpMaxRetry: Sn1DisconnectReason = Sn1DisconnectReason(22);
	pub const PppNoRemIpAddress: Sn1DisconnectReason = Sn1DisconnectReason(23);
	pub const InactivityTimeout: Sn1DisconnectReason = Sn1DisconnectReason(24);
	pub const SessionTimeout: Sn1DisconnectReason = Sn1DisconnectReason(25);
	pub const MaxDataExcd: Sn1DisconnectReason = Sn1DisconnectReason(26);
	pub const InvalidIpSourceAddress: Sn1DisconnectReason = Sn1DisconnectReason(27);
	pub const MsidAuthFailed: Sn1DisconnectReason = Sn1DisconnectReason(28);
	pub const MsidAuthFauiledNoAaaResp: Sn1DisconnectReason = Sn1DisconnectReason(29);
	pub const A11MaxRetry: Sn1DisconnectReason = Sn1DisconnectReason(30);
	pub const A11LifetimeExpired: Sn1DisconnectReason = Sn1DisconnectReason(31);
	pub const A11MessageIntegrityFailure: Sn1DisconnectReason = Sn1DisconnectReason(32);
	pub const PppLcpRemoteDisc: Sn1DisconnectReason = Sn1DisconnectReason(33);
	pub const SessionSetupTimeout: Sn1DisconnectReason = Sn1DisconnectReason(34);
	pub const PppKeepaliveFailure: Sn1DisconnectReason = Sn1DisconnectReason(35);
	pub const FlowAddFailed: Sn1DisconnectReason = Sn1DisconnectReason(36);
	pub const CallTypeDetectionFailed: Sn1DisconnectReason = Sn1DisconnectReason(37);
	pub const WrongIpcpParams: Sn1DisconnectReason = Sn1DisconnectReason(38);
	pub const MipRemoteDereg: Sn1DisconnectReason = Sn1DisconnectReason(39);
	pub const MipLifetimeExpiry: Sn1DisconnectReason = Sn1DisconnectReason(40);
	pub const MipProtoError: Sn1DisconnectReason = Sn1DisconnectReason(41);
	pub const MipAuthFailure: Sn1DisconnectReason = Sn1DisconnectReason(42);
	pub const MipRegTimeout: Sn1DisconnectReason = Sn1DisconnectReason(43);
	pub const InvalidDestContext: Sn1DisconnectReason = Sn1DisconnectReason(44);
	pub const SourceContextRemoved: Sn1DisconnectReason = Sn1DisconnectReason(45);
	pub const DestinationContextRemoved: Sn1DisconnectReason = Sn1DisconnectReason(46);
	pub const ReqServiceAddrUnavailable: Sn1DisconnectReason = Sn1DisconnectReason(47);
	pub const DemuxMgrFailed: Sn1DisconnectReason = Sn1DisconnectReason(48);
	pub const InternalError: Sn1DisconnectReason = Sn1DisconnectReason(49);
	pub const AaaContextRemoved: Sn1DisconnectReason = Sn1DisconnectReason(50);
	pub const InvalidServiceType: Sn1DisconnectReason = Sn1DisconnectReason(51);
	pub const MipRelayReqFailed: Sn1DisconnectReason = Sn1DisconnectReason(52);
	pub const MipRcvdRelayFailure: Sn1DisconnectReason = Sn1DisconnectReason(53);
	pub const PppRestartInterPdsnHandoff: Sn1DisconnectReason = Sn1DisconnectReason(54);
	pub const GreKeyMismatch: Sn1DisconnectReason = Sn1DisconnectReason(55);
	pub const InvalidTunnelContext: Sn1DisconnectReason = Sn1DisconnectReason(56);
	pub const NoPeerLnsAddress: Sn1DisconnectReason = Sn1DisconnectReason(57);
	pub const FailedTunnelConnect: Sn1DisconnectReason = Sn1DisconnectReason(58);
	pub const L2TpTunnelDisconnectRemote: Sn1DisconnectReason = Sn1DisconnectReason(59);
	pub const L2TpTunnelTimeout: Sn1DisconnectReason = Sn1DisconnectReason(60);
	pub const L2TpProtocolErrorRemote: Sn1DisconnectReason = Sn1DisconnectReason(61);
	pub const L2TpProtocolErrorLocal: Sn1DisconnectReason = Sn1DisconnectReason(62);
	pub const L2TpAuthFailedRemote: Sn1DisconnectReason = Sn1DisconnectReason(63);
	pub const L2TpAuthFailedLocal: Sn1DisconnectReason = Sn1DisconnectReason(64);
	pub const L2TpTryAnotherLnsFromRemote: Sn1DisconnectReason = Sn1DisconnectReason(65);
	pub const L2TpNoResourceLocal: Sn1DisconnectReason = Sn1DisconnectReason(66);
	pub const L2TpNoResourceRemote: Sn1DisconnectReason = Sn1DisconnectReason(67);
	pub const L2TpTunnelDisconnectLocal: Sn1DisconnectReason = Sn1DisconnectReason(68);
	pub const L2TpAdminDisconnectRemote: Sn1DisconnectReason = Sn1DisconnectReason(69);
	pub const L2TpmgrReachedMaxCapacity: Sn1DisconnectReason = Sn1DisconnectReason(70);
	pub const MipRegRevocation: Sn1DisconnectReason = Sn1DisconnectReason(71);
	pub const PathFailure: Sn1DisconnectReason = Sn1DisconnectReason(72);
	pub const DhcpRelayIpValidationFailed: Sn1DisconnectReason = Sn1DisconnectReason(73);
	pub const GtpUnknownPdpAddrOrPdpType: Sn1DisconnectReason = Sn1DisconnectReason(74);
	pub const GtpAllDynamicPdpAddrOccupied: Sn1DisconnectReason = Sn1DisconnectReason(75);
	pub const GtpNoMemoryIsAvailable: Sn1DisconnectReason = Sn1DisconnectReason(76);
	pub const DhcpRelayStaticIpAddrNotAllowed: Sn1DisconnectReason = Sn1DisconnectReason(77);
	pub const DhcpNoIpAddrAllocated: Sn1DisconnectReason = Sn1DisconnectReason(78);
	pub const DhcpIpAddrAllocationTmrExp: Sn1DisconnectReason = Sn1DisconnectReason(79);
	pub const DhcpIpValidationFailed: Sn1DisconnectReason = Sn1DisconnectReason(80);
	pub const DhcpStaticAddrNotAllowed: Sn1DisconnectReason = Sn1DisconnectReason(81);
	pub const DhcpIpAddrNotAvailableAtPresent: Sn1DisconnectReason = Sn1DisconnectReason(82);
	pub const DhcpLeaseExpired: Sn1DisconnectReason = Sn1DisconnectReason(83);
	pub const LpoolIpValidationFailed: Sn1DisconnectReason = Sn1DisconnectReason(84);
	pub const LpoolStaticIpAddrNotAllowed: Sn1DisconnectReason = Sn1DisconnectReason(85);
	pub const StaticIpValidationFailed: Sn1DisconnectReason = Sn1DisconnectReason(86);
	pub const StaticIpAddrNotPresent: Sn1DisconnectReason = Sn1DisconnectReason(87);
	pub const StaticIpAddrNotAllowed: Sn1DisconnectReason = Sn1DisconnectReason(88);
	pub const RadiusIpValidationFailed: Sn1DisconnectReason = Sn1DisconnectReason(89);
	pub const RadiusIpAddrNotProvided: Sn1DisconnectReason = Sn1DisconnectReason(90);
	pub const InvalidIpAddrFromSgsn: Sn1DisconnectReason = Sn1DisconnectReason(91);
	pub const NoMoreSessionsInAaa: Sn1DisconnectReason = Sn1DisconnectReason(92);
	pub const GgsnAaaAuthReqFailed: Sn1DisconnectReason = Sn1DisconnectReason(93);
	pub const ConflictInIpAddrAssignment: Sn1DisconnectReason = Sn1DisconnectReason(94);
	pub const ApnRemoved: Sn1DisconnectReason = Sn1DisconnectReason(95);
	pub const CreditsUsedBytesIn: Sn1DisconnectReason = Sn1DisconnectReason(96);
	pub const CreditsUsedBytesOut: Sn1DisconnectReason = Sn1DisconnectReason(97);
	pub const CreditsUsedBytesTotal: Sn1DisconnectReason = Sn1DisconnectReason(98);
	pub const PrepaidFailed: Sn1DisconnectReason = Sn1DisconnectReason(99);
	pub const L2TpIpsecTunnelFailure: Sn1DisconnectReason = Sn1DisconnectReason(100);
	pub const L2TpIpsecTunnelDisconnected: Sn1DisconnectReason = Sn1DisconnectReason(101);
	pub const MipIpsecSaInactive: Sn1DisconnectReason = Sn1DisconnectReason(102);
	pub const LongDurationTimeout: Sn1DisconnectReason = Sn1DisconnectReason(103);
	pub const ProxyMipRegistrationFailure: Sn1DisconnectReason = Sn1DisconnectReason(104);
	pub const ProxyMipBindingUpdate: Sn1DisconnectReason = Sn1DisconnectReason(105);
	pub const ProxyMipInterPdsnHandoffRequireIpAddress: Sn1DisconnectReason = Sn1DisconnectReason(106);
	pub const ProxyMipInterPdsnHandoffMismatchedAddress: Sn1DisconnectReason = Sn1DisconnectReason(107);
	pub const LocalPurge: Sn1DisconnectReason = Sn1DisconnectReason(108);
	pub const FailedUpdateHandoff: Sn1DisconnectReason = Sn1DisconnectReason(109);
	pub const ClosedRpHandoffComplete: Sn1DisconnectReason = Sn1DisconnectReason(110);
	pub const ClosedRpDuplicateSession: Sn1DisconnectReason = Sn1DisconnectReason(111);
	pub const ClosedRpHandoffSessionNotFound: Sn1DisconnectReason = Sn1DisconnectReason(112);
	pub const ClosedRpHandoffFailed: Sn1DisconnectReason = Sn1DisconnectReason(113);
	pub const PcfMonitorKeepAliveFailed: Sn1DisconnectReason = Sn1DisconnectReason(114);
	pub const CallInternalReject: Sn1DisconnectReason = Sn1DisconnectReason(115);
	pub const CallRestarted: Sn1DisconnectReason = Sn1DisconnectReason(116);
	pub const A11MnHaAuthFailure: Sn1DisconnectReason = Sn1DisconnectReason(117);
	pub const A11BadlyFormed: Sn1DisconnectReason = Sn1DisconnectReason(118);
	pub const A11TBitNotSet: Sn1DisconnectReason = Sn1DisconnectReason(119);
	pub const A11UnsupportedVendorId: Sn1DisconnectReason = Sn1DisconnectReason(120);
	pub const A11MismatchedId: Sn1DisconnectReason = Sn1DisconnectReason(121);
	pub const MiphaDupHomeAddrReq: Sn1DisconnectReason = Sn1DisconnectReason(122);
	pub const MiphaDupImsiSession: Sn1DisconnectReason = Sn1DisconnectReason(123);
	pub const HaUnreachable: Sn1DisconnectReason = Sn1DisconnectReason(124);
	pub const IpspAddrInUse: Sn1DisconnectReason = Sn1DisconnectReason(125);
	pub const MipfaDupHomeAddrReq: Sn1DisconnectReason = Sn1DisconnectReason(126);
	pub const MiphaIpPoolBusyout: Sn1DisconnectReason = Sn1DisconnectReason(127);
	pub const InterPdsnHandoff: Sn1DisconnectReason = Sn1DisconnectReason(128);
	pub const ActiveToDormant: Sn1DisconnectReason = Sn1DisconnectReason(129);
	pub const PppRenegotiation: Sn1DisconnectReason = Sn1DisconnectReason(130);
	pub const ActiveStartParamChange: Sn1DisconnectReason = Sn1DisconnectReason(131);
	pub const TarrifBoundary: Sn1DisconnectReason = Sn1DisconnectReason(132);
	pub const A11DisconnectNoActiveStop: Sn1DisconnectReason = Sn1DisconnectReason(133);
	pub const NwReachabilityFailedReject: Sn1DisconnectReason = Sn1DisconnectReason(134);
	pub const NwReachabilityFailedRedirect: Sn1DisconnectReason = Sn1DisconnectReason(135);
	pub const ContainerMaxExceeded: Sn1DisconnectReason = Sn1DisconnectReason(136);
	pub const StaticAddrNotAllowedInApn: Sn1DisconnectReason = Sn1DisconnectReason(137);
	pub const StaticAddrRequiredByRadius: Sn1DisconnectReason = Sn1DisconnectReason(138);
	pub const StaticAddrNotAllowedByRadius: Sn1DisconnectReason = Sn1DisconnectReason(139);
	pub const MipRegistrationDropped: Sn1DisconnectReason = Sn1DisconnectReason(140);
	pub const CounterRollover: Sn1DisconnectReason = Sn1DisconnectReason(141);
	pub const ConstructedNaiAuthFail: Sn1DisconnectReason = Sn1DisconnectReason(142);
	pub const InterPdsnServiceOptimizeHandoffDisabled: Sn1DisconnectReason = Sn1DisconnectReason(143);
	pub const GreKeyCollision: Sn1DisconnectReason = Sn1DisconnectReason(144);
	pub const InterPdsnServiceOptimizeHandoffTriggered: Sn1DisconnectReason = Sn1DisconnectReason(145);
	pub const IntraPdsnHandoffTriggered: Sn1DisconnectReason = Sn1DisconnectReason(146);
	pub const DelayedAbortTimerExpired: Sn1DisconnectReason = Sn1DisconnectReason(147);
	pub const AdminAaaDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(148);
	pub const AdminAaaDisconnectHandoff: Sn1DisconnectReason = Sn1DisconnectReason(149);
	pub const PppIpv6CpNegFailed: Sn1DisconnectReason = Sn1DisconnectReason(150);
	pub const PppIpv6CpNoResponse: Sn1DisconnectReason = Sn1DisconnectReason(151);
	pub const PppIpv6CpMaxRetry: Sn1DisconnectReason = Sn1DisconnectReason(152);
	pub const PppRestartInvalidSourceIpv4Address: Sn1DisconnectReason = Sn1DisconnectReason(153);
	pub const A11DisconnectHandoffNoActiveStop: Sn1DisconnectReason = Sn1DisconnectReason(154);
	pub const CallRestartedInterPdsnHandoff: Sn1DisconnectReason = Sn1DisconnectReason(155);
	pub const CallRestartedPppTermination: Sn1DisconnectReason = Sn1DisconnectReason(156);
	pub const MipfaResourceConflict: Sn1DisconnectReason = Sn1DisconnectReason(157);
	pub const FailedAuthWithChargingSvc: Sn1DisconnectReason = Sn1DisconnectReason(158);
	pub const MiphaDupImsiSessionPurge: Sn1DisconnectReason = Sn1DisconnectReason(159);
	pub const MiphaRevPendingNewcall: Sn1DisconnectReason = Sn1DisconnectReason(160);
	pub const VolumeQuotaReached: Sn1DisconnectReason = Sn1DisconnectReason(161);
	pub const DurationQuotaReached: Sn1DisconnectReason = Sn1DisconnectReason(162);
	pub const GtpUserAuthenticationFailed: Sn1DisconnectReason = Sn1DisconnectReason(163);
	pub const MipRegRevocationNoLcpTerm: Sn1DisconnectReason = Sn1DisconnectReason(164);
	pub const MipPrivateIpNoRevTunnel: Sn1DisconnectReason = Sn1DisconnectReason(165);
	pub const InvalidPrepaidAaaAttrInAuthResponse: Sn1DisconnectReason = Sn1DisconnectReason(166);
	pub const MiphaPrepaidResetDynamicNewcall: Sn1DisconnectReason = Sn1DisconnectReason(167);
	pub const GreFlowControlTimeout: Sn1DisconnectReason = Sn1DisconnectReason(168);
	pub const MipPaaaBcQueryNotFound: Sn1DisconnectReason = Sn1DisconnectReason(169);
	pub const MiphaDynamicIpAddrNotAvailable: Sn1DisconnectReason = Sn1DisconnectReason(170);
	pub const A11MismatchedIdOnHandoff: Sn1DisconnectReason = Sn1DisconnectReason(171);
	pub const A11BadlyFormedOnHandoff: Sn1DisconnectReason = Sn1DisconnectReason(172);
	pub const A11UnsupportedVendorIdOnHandoff: Sn1DisconnectReason = Sn1DisconnectReason(173);
	pub const A11TBitNotSetOnHandoff: Sn1DisconnectReason = Sn1DisconnectReason(174);
	pub const MipRegRevocationIBitOn: Sn1DisconnectReason = Sn1DisconnectReason(175);
	pub const A11RrqDenyMaxCount: Sn1DisconnectReason = Sn1DisconnectReason(176);
	pub const DormantTransitionDuringSessionSetup: Sn1DisconnectReason = Sn1DisconnectReason(177);
	pub const PppRemRenegDiscAlwaysCfg: Sn1DisconnectReason = Sn1DisconnectReason(178);
	pub const PppRemRenegDiscNaiMsidMismatch: Sn1DisconnectReason = Sn1DisconnectReason(179);
	pub const MiphaSubscriberIpsecTunnelDown: Sn1DisconnectReason = Sn1DisconnectReason(180);
	pub const MiphaSubscriberIpsecTunnelFailed: Sn1DisconnectReason = Sn1DisconnectReason(181);
	pub const MiphaSubscriberIpsecmgrDeath: Sn1DisconnectReason = Sn1DisconnectReason(182);
	pub const FlowIsDeactivated: Sn1DisconnectReason = Sn1DisconnectReason(183);
	pub const Ecsv2LicenseExceeded: Sn1DisconnectReason = Sn1DisconnectReason(184);
	pub const IpsgAuthFailed: Sn1DisconnectReason = Sn1DisconnectReason(185);
	pub const DriverInitiated: Sn1DisconnectReason = Sn1DisconnectReason(186);
	pub const ImsAuthorizationFailed: Sn1DisconnectReason = Sn1DisconnectReason(187);
	pub const ServiceInstanceReleased: Sn1DisconnectReason = Sn1DisconnectReason(188);
	pub const FlowReleased: Sn1DisconnectReason = Sn1DisconnectReason(189);
	pub const PppRenegoNoHaAddr: Sn1DisconnectReason = Sn1DisconnectReason(190);
	pub const IntraPdsnHandoff: Sn1DisconnectReason = Sn1DisconnectReason(191);
	pub const OverloadDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(192);
	pub const CssServiceNotFound: Sn1DisconnectReason = Sn1DisconnectReason(193);
	pub const AuthFailed: Sn1DisconnectReason = Sn1DisconnectReason(194);
	pub const DhcpClientSentRelease: Sn1DisconnectReason = Sn1DisconnectReason(195);
	pub const DhcpClientSentNak: Sn1DisconnectReason = Sn1DisconnectReason(196);
	pub const MsidDhcpChaddrMismatch: Sn1DisconnectReason = Sn1DisconnectReason(197);
	pub const LinkBroken: Sn1DisconnectReason = Sn1DisconnectReason(198);
	pub const ProgEndTimeout: Sn1DisconnectReason = Sn1DisconnectReason(199);
	pub const QosUpdateWaitTimeout: Sn1DisconnectReason = Sn1DisconnectReason(200);
	pub const CssSynchCause: Sn1DisconnectReason = Sn1DisconnectReason(201);
	pub const GtpContextReplacement: Sn1DisconnectReason = Sn1DisconnectReason(202);
	pub const PdifAuthFailed: Sn1DisconnectReason = Sn1DisconnectReason(203);
	pub const L2TpUnknownApn: Sn1DisconnectReason = Sn1DisconnectReason(204);
	pub const MsUnexpectedNetworkReentry: Sn1DisconnectReason = Sn1DisconnectReason(205);
	pub const R6InvalidNai: Sn1DisconnectReason = Sn1DisconnectReason(206);
	pub const EapMaxRetryReached: Sn1DisconnectReason = Sn1DisconnectReason(207);
	pub const VbmHoaSessionDisconnected: Sn1DisconnectReason = Sn1DisconnectReason(208);
	pub const VbmVoaSessionDisconnected: Sn1DisconnectReason = Sn1DisconnectReason(209);
	pub const InAclDisconnectOnViolation: Sn1DisconnectReason = Sn1DisconnectReason(210);
	pub const EapMskLifetimeExpiry: Sn1DisconnectReason = Sn1DisconnectReason(211);
	pub const EapMskLifetimeTooLow: Sn1DisconnectReason = Sn1DisconnectReason(212);
	pub const MipfaInterTechHandoff: Sn1DisconnectReason = Sn1DisconnectReason(213);
	pub const R6MaxRetryReached: Sn1DisconnectReason = Sn1DisconnectReason(214);
	pub const R6NwexitRecd: Sn1DisconnectReason = Sn1DisconnectReason(215);
	pub const R6DeregReqRecd: Sn1DisconnectReason = Sn1DisconnectReason(216);
	pub const R6RemoteFailure: Sn1DisconnectReason = Sn1DisconnectReason(217);
	pub const R6R4ProtocolErrors: Sn1DisconnectReason = Sn1DisconnectReason(218);
	pub const WimaxQosInvalidAaaAttr: Sn1DisconnectReason = Sn1DisconnectReason(219);
	pub const NpuGreFlowsNotAvailable: Sn1DisconnectReason = Sn1DisconnectReason(220);
	pub const R4MaxRetryReached: Sn1DisconnectReason = Sn1DisconnectReason(221);
	pub const R4NwexitRecd: Sn1DisconnectReason = Sn1DisconnectReason(222);
	pub const R4DeregReqRecd: Sn1DisconnectReason = Sn1DisconnectReason(223);
	pub const R4RemoteFailure: Sn1DisconnectReason = Sn1DisconnectReason(224);
	pub const ImsAuthorizationRevoked: Sn1DisconnectReason = Sn1DisconnectReason(225);
	pub const ImsAuthorizationReleased: Sn1DisconnectReason = Sn1DisconnectReason(226);
	pub const ImsAuthDecisionInvalid: Sn1DisconnectReason = Sn1DisconnectReason(227);
	pub const MacAddrValidationFailed: Sn1DisconnectReason = Sn1DisconnectReason(228);
	pub const ExcessiveWimaxPdFlowsCfgd: Sn1DisconnectReason = Sn1DisconnectReason(229);
	pub const SgsnCancLocSub: Sn1DisconnectReason = Sn1DisconnectReason(230);
	pub const SgsnCancLocUpd: Sn1DisconnectReason = Sn1DisconnectReason(231);
	pub const SgsnMnrExp: Sn1DisconnectReason = Sn1DisconnectReason(232);
	pub const SgsnIdentFail: Sn1DisconnectReason = Sn1DisconnectReason(233);
	pub const SgsnSecFail: Sn1DisconnectReason = Sn1DisconnectReason(234);
	pub const SgsnAuthFail: Sn1DisconnectReason = Sn1DisconnectReason(235);
	pub const SgsnGluFail: Sn1DisconnectReason = Sn1DisconnectReason(236);
	pub const SgsnImpDet: Sn1DisconnectReason = Sn1DisconnectReason(237);
	pub const SgsnSmgrPurge: Sn1DisconnectReason = Sn1DisconnectReason(238);
	pub const SgsnSubsHandedToPeer: Sn1DisconnectReason = Sn1DisconnectReason(239);
	pub const SgsnDnsFailInterRau: Sn1DisconnectReason = Sn1DisconnectReason(240);
	pub const SgsnContRspFail: Sn1DisconnectReason = Sn1DisconnectReason(241);
	pub const SgsnHlrNotFoundForImsi: Sn1DisconnectReason = Sn1DisconnectReason(242);
	pub const SgsnMsInitDet: Sn1DisconnectReason = Sn1DisconnectReason(243);
	pub const SgsnOprPolicyFail: Sn1DisconnectReason = Sn1DisconnectReason(244);
	pub const SgsnDuplicateContext: Sn1DisconnectReason = Sn1DisconnectReason(245);
	pub const HssProfileUpdateFailed: Sn1DisconnectReason = Sn1DisconnectReason(246);
	pub const SgsnNoPdpActivated: Sn1DisconnectReason = Sn1DisconnectReason(247);
	pub const AsnpcIdleModeTimeout: Sn1DisconnectReason = Sn1DisconnectReason(248);
	pub const AsnpcIdleModeExit: Sn1DisconnectReason = Sn1DisconnectReason(249);
	pub const AsnpcIdleModeAuthFailed: Sn1DisconnectReason = Sn1DisconnectReason(250);
	pub const AsngwInvalidQosConfiguration: Sn1DisconnectReason = Sn1DisconnectReason(251);
	pub const SgsnDsdAllgprswithdrawn: Sn1DisconnectReason = Sn1DisconnectReason(252);
	pub const R6PmkKeyChangeFailure: Sn1DisconnectReason = Sn1DisconnectReason(253);
	pub const SgsnIllegalMe: Sn1DisconnectReason = Sn1DisconnectReason(254);
	pub const SessTerminationTimeout: Sn1DisconnectReason = Sn1DisconnectReason(255);
	pub const SgsnSaiFail: Sn1DisconnectReason = Sn1DisconnectReason(256);
	pub const SgsnRncRemoval: Sn1DisconnectReason = Sn1DisconnectReason(257);
	pub const SgsnRaiRemoval: Sn1DisconnectReason = Sn1DisconnectReason(258);
	pub const SgsnInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(259);
	pub const GgsnInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(260);
	pub const HlrInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(261);
	pub const MsInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(262);
	pub const SgsnDetachInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(263);
	pub const SgsnRabRelInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(264);
	pub const SgsnIuRelInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(265);
	pub const SgsnGtpuPathFailure: Sn1DisconnectReason = Sn1DisconnectReason(266);
	pub const SgsnGtpcPathFailure: Sn1DisconnectReason = Sn1DisconnectReason(267);
	pub const SgsnLocalHandoffInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(268);
	pub const SgsnRemoteHandoffInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(269);
	pub const SgsnGtpNoResource: Sn1DisconnectReason = Sn1DisconnectReason(270);
	pub const SgsnRncNoResource: Sn1DisconnectReason = Sn1DisconnectReason(271);
	pub const SgsnOdbInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(272);
	pub const SgsnInvalidTi: Sn1DisconnectReason = Sn1DisconnectReason(273);
	pub const SgsnGgsnCtxtNonExistent: Sn1DisconnectReason = Sn1DisconnectReason(274);
	pub const SgsnApnRestrictVio: Sn1DisconnectReason = Sn1DisconnectReason(275);
	pub const SgsnRegularDeact: Sn1DisconnectReason = Sn1DisconnectReason(276);
	pub const SgsnAbnormalDeact: Sn1DisconnectReason = Sn1DisconnectReason(277);
	pub const SgsnActvRejectedByPeer: Sn1DisconnectReason = Sn1DisconnectReason(278);
	pub const SgsnErrInd: Sn1DisconnectReason = Sn1DisconnectReason(279);
	pub const AsngwNonAnchorProhibited: Sn1DisconnectReason = Sn1DisconnectReason(280);
	pub const AsngwImEntryProhibited: Sn1DisconnectReason = Sn1DisconnectReason(281);
	pub const SessionIdleModeEntryTimeout: Sn1DisconnectReason = Sn1DisconnectReason(282);
	pub const SessionIdleModeExitTimeout: Sn1DisconnectReason = Sn1DisconnectReason(283);
	pub const AsnpcMsPowerDownNwexit: Sn1DisconnectReason = Sn1DisconnectReason(284);
	pub const AsnpcR4NwexitRecd: Sn1DisconnectReason = Sn1DisconnectReason(285);
	pub const SgsnIuRelBeforeCallEst: Sn1DisconnectReason = Sn1DisconnectReason(286);
	pub const Ikev2SubscriberIpsecmgrDeath: Sn1DisconnectReason = Sn1DisconnectReason(287);
	pub const AllDynamicPoolAddrOccupied: Sn1DisconnectReason = Sn1DisconnectReason(288);
	pub const Mip6HaIpAddrNotAvailable: Sn1DisconnectReason = Sn1DisconnectReason(289);
	pub const BsMonitorKeepAliveFailed: Sn1DisconnectReason = Sn1DisconnectReason(290);
	pub const SgsnAttInRegState: Sn1DisconnectReason = Sn1DisconnectReason(291);
	pub const SgsnInboundSrnsInRegState: Sn1DisconnectReason = Sn1DisconnectReason(292);
	pub const DtGgsnTunReestablishFailed: Sn1DisconnectReason = Sn1DisconnectReason(293);
	pub const SgsnUnknownPdp: Sn1DisconnectReason = Sn1DisconnectReason(294);
	pub const SgsnPdpAuthFailure: Sn1DisconnectReason = Sn1DisconnectReason(295);
	pub const SgsnDuplicatePdpContext: Sn1DisconnectReason = Sn1DisconnectReason(296);
	pub const SgsnNoRspFromGgsn: Sn1DisconnectReason = Sn1DisconnectReason(297);
	pub const SgsnFailureRspFromGgsn: Sn1DisconnectReason = Sn1DisconnectReason(298);
	pub const SgsnApnUnknown: Sn1DisconnectReason = Sn1DisconnectReason(299);
	pub const SgsnServReqInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(300);
	pub const SgsnAttachOnAttchInitAbort: Sn1DisconnectReason = Sn1DisconnectReason(301);
	pub const SgsnIuRelInIsrauInitAbort: Sn1DisconnectReason = Sn1DisconnectReason(302);
	pub const SgsnSmgrInitAbort: Sn1DisconnectReason = Sn1DisconnectReason(303);
	pub const SgsnMmCtxCleanupInitAbort: Sn1DisconnectReason = Sn1DisconnectReason(304);
	pub const SgsnUnknownAbort: Sn1DisconnectReason = Sn1DisconnectReason(305);
	pub const SgsnGuardTimeoutAbort: Sn1DisconnectReason = Sn1DisconnectReason(306);
	pub const VpnBounceDhcpipValidateReq: Sn1DisconnectReason = Sn1DisconnectReason(307);
	pub const Mipv6IdMismatch: Sn1DisconnectReason = Sn1DisconnectReason(308);
	pub const AaaSessionIdNotFound: Sn1DisconnectReason = Sn1DisconnectReason(309);
	pub const X1MaxRetryReached: Sn1DisconnectReason = Sn1DisconnectReason(310);
	pub const X1NwexitRecd: Sn1DisconnectReason = Sn1DisconnectReason(311);
	pub const X1DeregReqRecd: Sn1DisconnectReason = Sn1DisconnectReason(312);
	pub const X1RemoteFailure: Sn1DisconnectReason = Sn1DisconnectReason(313);
	pub const X1X2ProtocolErrors: Sn1DisconnectReason = Sn1DisconnectReason(314);
	pub const X2MaxRetryReached: Sn1DisconnectReason = Sn1DisconnectReason(315);
	pub const X2NwexitRecd: Sn1DisconnectReason = Sn1DisconnectReason(316);
	pub const X2DeregReqRecd: Sn1DisconnectReason = Sn1DisconnectReason(317);
	pub const X2RemoteFailure: Sn1DisconnectReason = Sn1DisconnectReason(318);
	pub const X1PmkKeyChangeFailure: Sn1DisconnectReason = Sn1DisconnectReason(319);
	pub const SaRekeyingFailure: Sn1DisconnectReason = Sn1DisconnectReason(320);
	pub const SessSleepModeEntryTimeout: Sn1DisconnectReason = Sn1DisconnectReason(321);
	pub const PhsgwNonAnchorProhibited: Sn1DisconnectReason = Sn1DisconnectReason(322);
	pub const AsnpcPcRelocationFailed: Sn1DisconnectReason = Sn1DisconnectReason(323);
	pub const AsnpcPcRelocation: Sn1DisconnectReason = Sn1DisconnectReason(324);
	pub const AuthPolicyMismatch: Sn1DisconnectReason = Sn1DisconnectReason(325);
	pub const SaLifetimeExpiry: Sn1DisconnectReason = Sn1DisconnectReason(326);
	pub const AsnpcDelMsEntryRecd: Sn1DisconnectReason = Sn1DisconnectReason(327);
	pub const PhspcSleepModeTimeout: Sn1DisconnectReason = Sn1DisconnectReason(328);
	pub const PhspcSleepModeExit: Sn1DisconnectReason = Sn1DisconnectReason(329);
	pub const PhspcSleepModeAuthFailed: Sn1DisconnectReason = Sn1DisconnectReason(330);
	pub const PhspcMsPowerDownNwexit: Sn1DisconnectReason = Sn1DisconnectReason(331);
	pub const PhspcX2NwexitRecd: Sn1DisconnectReason = Sn1DisconnectReason(332);
	pub const InvalidNatConfig: Sn1DisconnectReason = Sn1DisconnectReason(333);
	pub const AsngwTidEntryNotFound: Sn1DisconnectReason = Sn1DisconnectReason(334);
	pub const NoNatIpAddress: Sn1DisconnectReason = Sn1DisconnectReason(335);
	pub const ExcessivePhsPdFlowsCfgd: Sn1DisconnectReason = Sn1DisconnectReason(336);
	pub const PhsgwInvalidQosConfiguration: Sn1DisconnectReason = Sn1DisconnectReason(337);
	pub const InterimUpdate: Sn1DisconnectReason = Sn1DisconnectReason(338);
	pub const SgsnAttachAbrtRadLost: Sn1DisconnectReason = Sn1DisconnectReason(339);
	pub const SgsnInbndIrauAbrtRadLost: Sn1DisconnectReason = Sn1DisconnectReason(340);
	pub const IkeKeepaliveFailed: Sn1DisconnectReason = Sn1DisconnectReason(341);
	pub const SgsnAttachAbrtMsSuspend: Sn1DisconnectReason = Sn1DisconnectReason(342);
	pub const SgsnInbndIrauAbrtMsSuspend: Sn1DisconnectReason = Sn1DisconnectReason(343);
	pub const DuplicateSessionDetected: Sn1DisconnectReason = Sn1DisconnectReason(344);
	pub const SgsnXidResponseFailure: Sn1DisconnectReason = Sn1DisconnectReason(345);
	pub const SgsnNseCleanup: Sn1DisconnectReason = Sn1DisconnectReason(346);
	pub const SgsnGtpReqFailure: Sn1DisconnectReason = Sn1DisconnectReason(347);
	pub const SgsnImsiMismatch: Sn1DisconnectReason = Sn1DisconnectReason(348);
	pub const SgsnBvcBlocked: Sn1DisconnectReason = Sn1DisconnectReason(349);
	pub const SgsnAttachOnInboundIrau: Sn1DisconnectReason = Sn1DisconnectReason(350);
	pub const SgsnAttachOnOutboundIrau: Sn1DisconnectReason = Sn1DisconnectReason(351);
	pub const SgsnIncorrectState: Sn1DisconnectReason = Sn1DisconnectReason(352);
	pub const SgsnT3350Expiry: Sn1DisconnectReason = Sn1DisconnectReason(353);
	pub const SgsnPageTimerExpiry: Sn1DisconnectReason = Sn1DisconnectReason(354);
	pub const PhsgwTidEntryNotFound: Sn1DisconnectReason = Sn1DisconnectReason(355);
	pub const PhspcDelMsEntryRecd: Sn1DisconnectReason = Sn1DisconnectReason(356);
	pub const SgsnPdpLocalPurge: Sn1DisconnectReason = Sn1DisconnectReason(357);
	pub const PhsInvalidNai: Sn1DisconnectReason = Sn1DisconnectReason(358);
	pub const SessionSleepModeExitTimeout: Sn1DisconnectReason = Sn1DisconnectReason(359);
	pub const SgsnOffloadPhase2: Sn1DisconnectReason = Sn1DisconnectReason(360);
	pub const PhsThirdpartyAuthFail: Sn1DisconnectReason = Sn1DisconnectReason(361);
	pub const RemoteErrorNotify: Sn1DisconnectReason = Sn1DisconnectReason(362);
	pub const NoResponse: Sn1DisconnectReason = Sn1DisconnectReason(363);
	pub const PdgAuthFailed: Sn1DisconnectReason = Sn1DisconnectReason(364);
	pub const MmeS1ApSendFailed: Sn1DisconnectReason = Sn1DisconnectReason(365);
	pub const MmeEgtpcConnectionFailed: Sn1DisconnectReason = Sn1DisconnectReason(366);
	pub const MmeEgtpcCreateSessionFailed: Sn1DisconnectReason = Sn1DisconnectReason(367);
	pub const MmeAuthenticationFailure: Sn1DisconnectReason = Sn1DisconnectReason(368);
	pub const MmeUeDetach: Sn1DisconnectReason = Sn1DisconnectReason(369);
	pub const MmeMmeDetach: Sn1DisconnectReason = Sn1DisconnectReason(370);
	pub const MmeHssDetach: Sn1DisconnectReason = Sn1DisconnectReason(371);
	pub const MmePgwDetach: Sn1DisconnectReason = Sn1DisconnectReason(372);
	pub const MmeSubValidationFailure: Sn1DisconnectReason = Sn1DisconnectReason(373);
	pub const MmeHssConnectionFailure: Sn1DisconnectReason = Sn1DisconnectReason(374);
	pub const MmeHssUserUnknown: Sn1DisconnectReason = Sn1DisconnectReason(375);
	pub const DhcpLeaseMismatchDetected: Sn1DisconnectReason = Sn1DisconnectReason(376);
	pub const NemoLinkLayerDown: Sn1DisconnectReason = Sn1DisconnectReason(377);
	pub const EapolMaxRetryReached: Sn1DisconnectReason = Sn1DisconnectReason(378);
	pub const SgsnOffloadPhase3: Sn1DisconnectReason = Sn1DisconnectReason(379);
	pub const MbmsBearerServiceDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(380);
	pub const DisconnectOnViolationOdb: Sn1DisconnectReason = Sn1DisconnectReason(381);
	pub const DisconnOnViolationFocsOdb: Sn1DisconnectReason = Sn1DisconnectReason(382);
	pub const CscfRegAdminDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(383);
	pub const CscfRegUserDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(384);
	pub const CscfRegInactivityTimeout: Sn1DisconnectReason = Sn1DisconnectReason(385);
	pub const CscfRegNetworkDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(386);
	pub const CscfCallAdminDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(387);
	pub const CscfCallUserDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(388);
	pub const CscfCallLocalDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(389);
	pub const CscfCallNoResource: Sn1DisconnectReason = Sn1DisconnectReason(390);
	pub const CscfCallNoRespone: Sn1DisconnectReason = Sn1DisconnectReason(391);
	pub const CscfCallInactivityTimeout: Sn1DisconnectReason = Sn1DisconnectReason(392);
	pub const CscfCallMediaAuthFailure: Sn1DisconnectReason = Sn1DisconnectReason(393);
	pub const CscfRegNoResource: Sn1DisconnectReason = Sn1DisconnectReason(394);
	pub const MsUnexpectedIdleModeEntry: Sn1DisconnectReason = Sn1DisconnectReason(395);
	pub const ReAuthFailed: Sn1DisconnectReason = Sn1DisconnectReason(396);
	pub const SgsnPdpNseCleanup: Sn1DisconnectReason = Sn1DisconnectReason(397);
	pub const SgsnMmCtxtGtpNoResource: Sn1DisconnectReason = Sn1DisconnectReason(398);
	pub const UnknownApn: Sn1DisconnectReason = Sn1DisconnectReason(399);
	pub const GtpcPathFailure: Sn1DisconnectReason = Sn1DisconnectReason(400);
	pub const GtpuPathFailure: Sn1DisconnectReason = Sn1DisconnectReason(401);
	pub const ActvRejectedBySgsn: Sn1DisconnectReason = Sn1DisconnectReason(402);
	pub const SgsnPdpGprsCamelRelease: Sn1DisconnectReason = Sn1DisconnectReason(403);
	pub const SgsnCheckImeiFailure: Sn1DisconnectReason = Sn1DisconnectReason(404);
	pub const SgsnSndcpInitDeact: Sn1DisconnectReason = Sn1DisconnectReason(405);
	pub const SgsnPdpInactivityTimeout: Sn1DisconnectReason = Sn1DisconnectReason(406);
	pub const FwAndNatPolicyRemoved: Sn1DisconnectReason = Sn1DisconnectReason(407);
	pub const FngAuthFailed: Sn1DisconnectReason = Sn1DisconnectReason(408);
	pub const HaStaleKeyDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(409);
	pub const NoIpv6AddressForSubscriber: Sn1DisconnectReason = Sn1DisconnectReason(410);
	pub const PrefixRegistrationFailure: Sn1DisconnectReason = Sn1DisconnectReason(411);
	pub const DisconnectFromPolicyServer: Sn1DisconnectReason = Sn1DisconnectReason(412);
	pub const S6BAuthFailed: Sn1DisconnectReason = Sn1DisconnectReason(413);
	pub const GtpcErrInd: Sn1DisconnectReason = Sn1DisconnectReason(414);
	pub const GtpuErrInd: Sn1DisconnectReason = Sn1DisconnectReason(415);
	pub const InvalidPdnType: Sn1DisconnectReason = Sn1DisconnectReason(416);
	pub const AaaAuthReqFailed: Sn1DisconnectReason = Sn1DisconnectReason(417);
	pub const ApnDeniedNoSubscription: Sn1DisconnectReason = Sn1DisconnectReason(418);
	pub const SgwContextReplacement: Sn1DisconnectReason = Sn1DisconnectReason(419);
	pub const DupStaticIpAddrReq: Sn1DisconnectReason = Sn1DisconnectReason(420);
	pub const ApnRestrictViolation: Sn1DisconnectReason = Sn1DisconnectReason(421);
	pub const InvalidWapn: Sn1DisconnectReason = Sn1DisconnectReason(422);
	pub const TtgNsapiAllocationFailed: Sn1DisconnectReason = Sn1DisconnectReason(423);
	pub const MandatoryGtpIeMissing: Sn1DisconnectReason = Sn1DisconnectReason(424);
	pub const AaaUnreachable: Sn1DisconnectReason = Sn1DisconnectReason(425);
	pub const AsngwServiceFlowDeletion: Sn1DisconnectReason = Sn1DisconnectReason(426);
	pub const CtPmipRrqNvseValueChange: Sn1DisconnectReason = Sn1DisconnectReason(427);
	pub const TcpReadFailed: Sn1DisconnectReason = Sn1DisconnectReason(428);
	pub const TcpWriteFailed: Sn1DisconnectReason = Sn1DisconnectReason(429);
	pub const SslHandshakeFailed: Sn1DisconnectReason = Sn1DisconnectReason(430);
	pub const SslRenegotiateFailed: Sn1DisconnectReason = Sn1DisconnectReason(431);
	pub const SslBadMessage: Sn1DisconnectReason = Sn1DisconnectReason(432);
	pub const SslAlertReceived: Sn1DisconnectReason = Sn1DisconnectReason(433);
	pub const SslDisconnect: Sn1DisconnectReason = Sn1DisconnectReason(434);
	pub const SslMigration: Sn1DisconnectReason = Sn1DisconnectReason(435);
	pub const SgsnArdFailure: Sn1DisconnectReason = Sn1DisconnectReason(436);
	pub const SgsnCamelRelease: Sn1DisconnectReason = Sn1DisconnectReason(437);
	pub const HotliningStatusChange: Sn1DisconnectReason = Sn1DisconnectReason(447);
	pub const GgsnNoRspFromSgsn: Sn1DisconnectReason = Sn1DisconnectReason(448);
	pub const DiameterProtocolError: Sn1DisconnectReason = Sn1DisconnectReason(449);
	pub const DiameterRequestTimeout: Sn1DisconnectReason = Sn1DisconnectReason(450);
	pub const OperatorPolicy: Sn1DisconnectReason = Sn1DisconnectReason(451);
	pub const SprConnectionTimeout: Sn1DisconnectReason = Sn1DisconnectReason(452);
	pub const MiphaDupWimaxSession: Sn1DisconnectReason = Sn1DisconnectReason(453);
	pub const InvalidVersionAttr: Sn1DisconnectReason = Sn1DisconnectReason(454);
	pub const SgsnZoneCodeFailure: Sn1DisconnectReason = Sn1DisconnectReason(455);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PppProgressCode(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PppProgressCode {
	pub const NotDefined: Sn1PppProgressCode = Sn1PppProgressCode(0);
	pub const CallLcpDown: Sn1PppProgressCode = Sn1PppProgressCode(1);
	pub const CallDisconnecting: Sn1PppProgressCode = Sn1PppProgressCode(2);
	pub const CallPppRenegotiating: Sn1PppProgressCode = Sn1PppProgressCode(3);
	pub const CallLcpDown1: Sn1PppProgressCode = Sn1PppProgressCode(10);
	pub const CallArrived: Sn1PppProgressCode = Sn1PppProgressCode(11);
	pub const CallLcpUp: Sn1PppProgressCode = Sn1PppProgressCode(12);
	pub const CallAuthenticating: Sn1PppProgressCode = Sn1PppProgressCode(13);
	pub const CallAuthenticated: Sn1PppProgressCode = Sn1PppProgressCode(14);
	pub const CallIpcpUp: Sn1PppProgressCode = Sn1PppProgressCode(15);
	pub const CallSimpleIpConnected: Sn1PppProgressCode = Sn1PppProgressCode(16);
	pub const CallMobileIpConnected: Sn1PppProgressCode = Sn1PppProgressCode(17);
	pub const CallPdgTcpConnecting: Sn1PppProgressCode = Sn1PppProgressCode(45);
	pub const CallPdgSslConnecting: Sn1PppProgressCode = Sn1PppProgressCode(46);
	pub const CallBcmcsAuthenticating: Sn1PppProgressCode = Sn1PppProgressCode(70);
	pub const CallTunnelConnecting: Sn1PppProgressCode = Sn1PppProgressCode(85);
	pub const CallImsaAuthorizing: Sn1PppProgressCode = Sn1PppProgressCode(95);
	pub const CallImsaAuthorized: Sn1PppProgressCode = Sn1PppProgressCode(97);
	pub const CallMbmsUeAuthorizing: Sn1PppProgressCode = Sn1PppProgressCode(98);
	pub const CallMbmsBearerAuthorizing: Sn1PppProgressCode = Sn1PppProgressCode(99);
	pub const CallTunnelConnected: Sn1PppProgressCode = Sn1PppProgressCode(115);
	pub const CallPdpTypeIpConnected: Sn1PppProgressCode = Sn1PppProgressCode(120);
	pub const CallPdpTypeIpv6Connected: Sn1PppProgressCode = Sn1PppProgressCode(125);
	pub const CallPdpTypePppConnected: Sn1PppProgressCode = Sn1PppProgressCode(130);
	pub const CallProxyMobileIpConnected: Sn1PppProgressCode = Sn1PppProgressCode(140);
	pub const CallPdgConnected: Sn1PppProgressCode = Sn1PppProgressCode(142);
	pub const CallPdgSslConnected: Sn1PppProgressCode = Sn1PppProgressCode(141);
	pub const CallIpsgConnected: Sn1PppProgressCode = Sn1PppProgressCode(145);
	pub const CallBcmcsConnected: Sn1PppProgressCode = Sn1PppProgressCode(150);
	pub const CallMbmsUeConnected: Sn1PppProgressCode = Sn1PppProgressCode(155);
	pub const CallMbmsBearerConnected: Sn1PppProgressCode = Sn1PppProgressCode(156);
	pub const CallPendingAddrFromDhcp: Sn1PppProgressCode = Sn1PppProgressCode(160);
	pub const CallGotAddrFromDhcp: Sn1PppProgressCode = Sn1PppProgressCode(170);
	pub const CallHaIpsecTunnelConnecting: Sn1PppProgressCode = Sn1PppProgressCode(180);
	pub const CallHaIpsecConnected: Sn1PppProgressCode = Sn1PppProgressCode(190);
	pub const CallAsnNonAnchorConnected: Sn1PppProgressCode = Sn1PppProgressCode(200);
	pub const CallAsnpcConnected: Sn1PppProgressCode = Sn1PppProgressCode(210);
	pub const CallMobileIpv6Connected: Sn1PppProgressCode = Sn1PppProgressCode(220);
	pub const CallPmipv6Connected: Sn1PppProgressCode = Sn1PppProgressCode(221);
	pub const CallPhspcConnected: Sn1PppProgressCode = Sn1PppProgressCode(230);
	pub const CallGtpIpv4Connected: Sn1PppProgressCode = Sn1PppProgressCode(235);
	pub const CallGtpIpv6Connected: Sn1PppProgressCode = Sn1PppProgressCode(236);
	pub const CallGtpIpv4Ipv6Connected: Sn1PppProgressCode = Sn1PppProgressCode(237);
	pub const CallSgwConnected: Sn1PppProgressCode = Sn1PppProgressCode(245);
	pub const CallMmeAttached: Sn1PppProgressCode = Sn1PppProgressCode(246);
	pub const CallAuthOnlyConnected: Sn1PppProgressCode = Sn1PppProgressCode(247);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PppDataCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PppDataCompression {
	pub const None: Sn1PppDataCompression = Sn1PppDataCompression(0);
	pub const StacLzs: Sn1PppDataCompression = Sn1PppDataCompression(1);
	pub const Mppc: Sn1PppDataCompression = Sn1PppDataCompression(2);
	pub const MpccStacLzs: Sn1PppDataCompression = Sn1PppDataCompression(3);
	pub const Deflate: Sn1PppDataCompression = Sn1PppDataCompression(4);
	pub const DeflateStacLzs: Sn1PppDataCompression = Sn1PppDataCompression(5);
	pub const DeflateMpcc: Sn1PppDataCompression = Sn1PppDataCompression(6);
	pub const DeflateMpccStacLzs: Sn1PppDataCompression = Sn1PppDataCompression(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1IpSourceValidation(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1IpSourceValidation {
	pub const No: Sn1IpSourceValidation = Sn1IpSourceValidation(0);
	pub const Yes: Sn1IpSourceValidation = Sn1IpSourceValidation(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubscriberPermission(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubscriberPermission {
	pub const None: Sn1SubscriberPermission = Sn1SubscriberPermission(0);
	pub const SimpleIp: Sn1SubscriberPermission = Sn1SubscriberPermission(1);
	pub const MobileIp: Sn1SubscriberPermission = Sn1SubscriberPermission(2);
	pub const SimpleIpMobileIp: Sn1SubscriberPermission = Sn1SubscriberPermission(3);
	pub const HaMobileIp: Sn1SubscriberPermission = Sn1SubscriberPermission(4);
	pub const SimpleIpHaMobileIp: Sn1SubscriberPermission = Sn1SubscriberPermission(5);
	pub const MobileIpHaMobileIp: Sn1SubscriberPermission = Sn1SubscriberPermission(6);
	pub const All: Sn1SubscriberPermission = Sn1SubscriberPermission(7);
	pub const GgsnPdpTypeIp: Sn1SubscriberPermission = Sn1SubscriberPermission(8);
	pub const GgsnPdpTypePpp: Sn1SubscriberPermission = Sn1SubscriberPermission(16);
	pub const NetworkMobility: Sn1SubscriberPermission = Sn1SubscriberPermission(32);
	pub const FaHaNemo: Sn1SubscriberPermission = Sn1SubscriberPermission(38);
	pub const AllUnderscore: Sn1SubscriberPermission = Sn1SubscriberPermission(63);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1AdminPermission(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1AdminPermission {
	pub const None: Sn1AdminPermission = Sn1AdminPermission(0);
	pub const Cli: Sn1AdminPermission = Sn1AdminPermission(1);
	pub const Ftp: Sn1AdminPermission = Sn1AdminPermission(2);
	pub const CliFtp: Sn1AdminPermission = Sn1AdminPermission(3);
	pub const Intercept: Sn1AdminPermission = Sn1AdminPermission(4);
	pub const CliIntercept: Sn1AdminPermission = Sn1AdminPermission(5);
	pub const CliInterceptFtp: Sn1AdminPermission = Sn1AdminPermission(7);
	pub const Ecs: Sn1AdminPermission = Sn1AdminPermission(8);
	pub const CliEcs: Sn1AdminPermission = Sn1AdminPermission(9);
	pub const CliFtpEcs: Sn1AdminPermission = Sn1AdminPermission(11);
	pub const CliInterceptEcs: Sn1AdminPermission = Sn1AdminPermission(13);
	pub const CliInterceptFtpEcs: Sn1AdminPermission = Sn1AdminPermission(15);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SimultaneousSipMip(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SimultaneousSipMip {
	pub const Disabled: Sn1SimultaneousSipMip = Sn1SimultaneousSipMip(0);
	pub const Enabled: Sn1SimultaneousSipMip = Sn1SimultaneousSipMip(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PppDataCompressionMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PppDataCompressionMode {
	pub const Normal: Sn1PppDataCompressionMode = Sn1PppDataCompressionMode(0);
	pub const Stateless: Sn1PppDataCompressionMode = Sn1PppDataCompressionMode(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1AccessLinkIpFrag(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1AccessLinkIpFrag {
	pub const Normal: Sn1AccessLinkIpFrag = Sn1AccessLinkIpFrag(0);
	pub const DfIgnore: Sn1AccessLinkIpFrag = Sn1AccessLinkIpFrag(1);
	pub const DfFragmentIcmpNotify: Sn1AccessLinkIpFrag = Sn1AccessLinkIpFrag(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1ChangeCondition(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1ChangeCondition {
	pub const Qoschange: Sn1ChangeCondition = Sn1ChangeCondition(0);
	pub const Tarifftimechange: Sn1ChangeCondition = Sn1ChangeCondition(1);
	pub const Sgsnchange: Sn1ChangeCondition = Sn1ChangeCondition(500);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1DataTunnelIgnoreDfBit(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1DataTunnelIgnoreDfBit {
	pub const Disabled: Sn1DataTunnelIgnoreDfBit = Sn1DataTunnelIgnoreDfBit(0);
	pub const Enabled: Sn1DataTunnelIgnoreDfBit = Sn1DataTunnelIgnoreDfBit(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1DhcpLeaseExpiryPolicy(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1DhcpLeaseExpiryPolicy {
	pub const AutoRenew: Sn1DhcpLeaseExpiryPolicy = Sn1DhcpLeaseExpiryPolicy(0);
	pub const Disconnect: Sn1DhcpLeaseExpiryPolicy = Sn1DhcpLeaseExpiryPolicy(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1Direction(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1Direction {
	pub const Any: Sn1Direction = Sn1Direction(0);
	pub const Uplink: Sn1Direction = Sn1Direction(1);
	pub const Downlink: Sn1Direction = Sn1Direction(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1DnsProxyUseSubscrAddr(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1DnsProxyUseSubscrAddr {
	pub const Disable: Sn1DnsProxyUseSubscrAddr = Sn1DnsProxyUseSubscrAddr(0);
	pub const Enable: Sn1DnsProxyUseSubscrAddr = Sn1DnsProxyUseSubscrAddr(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1EnableQosRenegotiation(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1EnableQosRenegotiation {
	pub const No: Sn1EnableQosRenegotiation = Sn1EnableQosRenegotiation(0);
	pub const Yes: Sn1EnableQosRenegotiation = Sn1EnableQosRenegotiation(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1FirewallEnabled(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1FirewallEnabled {
	pub const False: Sn1FirewallEnabled = Sn1FirewallEnabled(0);
	pub const True: Sn1FirewallEnabled = Sn1FirewallEnabled(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1Ggsn1MipRequired(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1Ggsn1MipRequired {
	pub const Disabled: Sn1Ggsn1MipRequired = Sn1Ggsn1MipRequired(0);
	pub const Enabled: Sn1Ggsn1MipRequired = Sn1Ggsn1MipRequired(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1GratuitousArpAggressive(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1GratuitousArpAggressive {
	pub const Disabled: Sn1GratuitousArpAggressive = Sn1GratuitousArpAggressive(0);
	pub const Enabled: Sn1GratuitousArpAggressive = Sn1GratuitousArpAggressive(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1GtpVersion(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1GtpVersion {
	pub const GtpVersion0: Sn1GtpVersion = Sn1GtpVersion(0);
	pub const GtpVersion1: Sn1GtpVersion = Sn1GtpVersion(1);
	pub const GtpVersion2: Sn1GtpVersion = Sn1GtpVersion(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1HaSendDnsAddress(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1HaSendDnsAddress {
	pub const Disabled: Sn1HaSendDnsAddress = Sn1HaSendDnsAddress(0);
	pub const Enabled: Sn1HaSendDnsAddress = Sn1HaSendDnsAddress(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1HomeSubUseGgsn(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1HomeSubUseGgsn {
	pub const Deny: Sn1HomeSubUseGgsn = Sn1HomeSubUseGgsn(0);
	pub const Accept: Sn1HomeSubUseGgsn = Sn1HomeSubUseGgsn(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1IpAllocMethod(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1IpAllocMethod {
	pub const AllocLocalPool: Sn1IpAllocMethod = Sn1IpAllocMethod(0);
	pub const AllocDhcpClient: Sn1IpAllocMethod = Sn1IpAllocMethod(1);
	pub const AllocRadius: Sn1IpAllocMethod = Sn1IpAllocMethod(2);
	pub const AllocNoAlloc: Sn1IpAllocMethod = Sn1IpAllocMethod(3);
	pub const AllocStaticAlloc: Sn1IpAllocMethod = Sn1IpAllocMethod(4);
	pub const AllocDhcpRelay: Sn1IpAllocMethod = Sn1IpAllocMethod(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1IpHeaderCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1IpHeaderCompression {
	pub const None: Sn1IpHeaderCompression = Sn1IpHeaderCompression(0);
	pub const Vj: Sn1IpHeaderCompression = Sn1IpHeaderCompression(1);
	pub const Rohc: Sn1IpHeaderCompression = Sn1IpHeaderCompression(2);
	pub const VjRohc: Sn1IpHeaderCompression = Sn1IpHeaderCompression(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1IpHideServiceAddress(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1IpHideServiceAddress {
	pub const Disabled: Sn1IpHideServiceAddress = Sn1IpHideServiceAddress(0);
	pub const Enabled: Sn1IpHideServiceAddress = Sn1IpHideServiceAddress(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1IpSourceViolateNoAcct(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1IpSourceViolateNoAcct {
	pub const Disabled: Sn1IpSourceViolateNoAcct = Sn1IpSourceViolateNoAcct(0);
	pub const Enabled: Sn1IpSourceViolateNoAcct = Sn1IpSourceViolateNoAcct(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1Ipv6DnsProxy(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1Ipv6DnsProxy {
	pub const Disabled: Sn1Ipv6DnsProxy = Sn1Ipv6DnsProxy(0);
	pub const Enabled: Sn1Ipv6DnsProxy = Sn1Ipv6DnsProxy(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1Ipv6EgressFiltering(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1Ipv6EgressFiltering {
	pub const Disabled: Sn1Ipv6EgressFiltering = Sn1Ipv6EgressFiltering(0);
	pub const Enabled: Sn1Ipv6EgressFiltering = Sn1Ipv6EgressFiltering(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1L3ToL2TunAddrPolicy(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1L3ToL2TunAddrPolicy {
	pub const NoLocalAllocValidate: Sn1L3ToL2TunAddrPolicy = Sn1L3ToL2TunAddrPolicy(0);
	pub const LocalAlloc: Sn1L3ToL2TunAddrPolicy = Sn1L3ToL2TunAddrPolicy(1);
	pub const LocalAllocValidate: Sn1L3ToL2TunAddrPolicy = Sn1L3ToL2TunAddrPolicy(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1LongDurationAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1LongDurationAction {
	pub const Detection: Sn1LongDurationAction = Sn1LongDurationAction(1);
	pub const Disconnection: Sn1LongDurationAction = Sn1LongDurationAction(2);
	pub const DormantOnlyDisconnection: Sn1LongDurationAction = Sn1LongDurationAction(3);
	pub const DormantOnlyDetection: Sn1LongDurationAction = Sn1LongDurationAction(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1LongDurationNotification(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1LongDurationNotification {
	pub const Suppress: Sn1LongDurationNotification = Sn1LongDurationNotification(0);
	pub const Send: Sn1LongDurationNotification = Sn1LongDurationNotification(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1MediationAcctRspAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1MediationAcctRspAction {
	pub const None: Sn1MediationAcctRspAction = Sn1MediationAcctRspAction(0);
	pub const NoEarlyPdus: Sn1MediationAcctRspAction = Sn1MediationAcctRspAction(1);
	pub const DelayGtpResponse: Sn1MediationAcctRspAction = Sn1MediationAcctRspAction(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1MediationEnabled(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1MediationEnabled {
	pub const Disabled: Sn1MediationEnabled = Sn1MediationEnabled(0);
	pub const Enabled: Sn1MediationEnabled = Sn1MediationEnabled(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1MediationNoInterims(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1MediationNoInterims {
	pub const Disabled: Sn1MediationNoInterims = Sn1MediationNoInterims(0);
	pub const Enabled: Sn1MediationNoInterims = Sn1MediationNoInterims(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1MipAaaAssignAddr(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1MipAaaAssignAddr {
	pub const Disabled: Sn1MipAaaAssignAddr = Sn1MipAaaAssignAddr(0);
	pub const Enabled: Sn1MipAaaAssignAddr = Sn1MipAaaAssignAddr(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1MipDualAnchor(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1MipDualAnchor {
	pub const Disabled: Sn1MipDualAnchor = Sn1MipDualAnchor(0);
	pub const Enabled: Sn1MipDualAnchor = Sn1MipDualAnchor(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1MipMatchAaaAssignAddr(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1MipMatchAaaAssignAddr {
	pub const Disabled: Sn1MipMatchAaaAssignAddr = Sn1MipMatchAaaAssignAddr(0);
	pub const Enabled: Sn1MipMatchAaaAssignAddr = Sn1MipMatchAaaAssignAddr(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1MipSendAncid(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1MipSendAncid {
	pub const Disabled: Sn1MipSendAncid = Sn1MipSendAncid(0);
	pub const Enabled: Sn1MipSendAncid = Sn1MipSendAncid(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1MipSendCorrelationInfo(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1MipSendCorrelationInfo {
	pub const Disabled: Sn1MipSendCorrelationInfo = Sn1MipSendCorrelationInfo(0);
	pub const EnabledornvseStarent: Sn1MipSendCorrelationInfo = Sn1MipSendCorrelationInfo(1);
	pub const NvseCustom1: Sn1MipSendCorrelationInfo = Sn1MipSendCorrelationInfo(2);
	pub const NvseCustom2: Sn1MipSendCorrelationInfo = Sn1MipSendCorrelationInfo(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1MipSendImsi(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1MipSendImsi {
	pub const Noneordisabled: Sn1MipSendImsi = Sn1MipSendImsi(0);
	pub const StarentNvse: Sn1MipSendImsi = Sn1MipSendImsi(1);
	pub const NvseCustom1: Sn1MipSendImsi = Sn1MipSendImsi(2);
	pub const NvseCustom2: Sn1MipSendImsi = Sn1MipSendImsi(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1MipSendTermVerification(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1MipSendTermVerification {
	pub const Disabled: Sn1MipSendTermVerification = Sn1MipSendTermVerification(0);
	pub const EnabledornvseCustom1: Sn1MipSendTermVerification = Sn1MipSendTermVerification(1);
	pub const NvseCustom2: Sn1MipSendTermVerification = Sn1MipSendTermVerification(2);
	pub const NvseStarent: Sn1MipSendTermVerification = Sn1MipSendTermVerification(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1MnHaHashAlgorithm(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1MnHaHashAlgorithm {
	pub const Md5: Sn1MnHaHashAlgorithm = Sn1MnHaHashAlgorithm(1);
	pub const Md5Rfc2002: Sn1MnHaHashAlgorithm = Sn1MnHaHashAlgorithm(2);
	pub const HmacMd5: Sn1MnHaHashAlgorithm = Sn1MnHaHashAlgorithm(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1Mode(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1Mode {
	pub const Reliable: Sn1Mode = Sn1Mode(0);
	pub const Optimistic: Sn1Mode = Sn1Mode(1);
	pub const Unidirectional: Sn1Mode = Sn1Mode(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1NpuQosPriority(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1NpuQosPriority {
	pub const BestEffort: Sn1NpuQosPriority = Sn1NpuQosPriority(0);
	pub const Bronze: Sn1NpuQosPriority = Sn1NpuQosPriority(1);
	pub const Silver: Sn1NpuQosPriority = Sn1NpuQosPriority(2);
	pub const Gold: Sn1NpuQosPriority = Sn1NpuQosPriority(3);
	pub const FromDscp: Sn1NpuQosPriority = Sn1NpuQosPriority(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1NtkSessionDisconnectFlag(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1NtkSessionDisconnectFlag {
	pub const SessionDisconnect: Sn1NtkSessionDisconnectFlag = Sn1NtkSessionDisconnectFlag(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PdifMipReleaseTia(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PdifMipReleaseTia {
	pub const No: Sn1PdifMipReleaseTia = Sn1PdifMipReleaseTia(0);
	pub const Yes: Sn1PdifMipReleaseTia = Sn1PdifMipReleaseTia(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PdifMipRequired(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PdifMipRequired {
	pub const No: Sn1PdifMipRequired = Sn1PdifMipRequired(0);
	pub const Yes: Sn1PdifMipRequired = Sn1PdifMipRequired(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PdifMipSimpleIpFallback(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PdifMipSimpleIpFallback {
	pub const No: Sn1PdifMipSimpleIpFallback = Sn1PdifMipSimpleIpFallback(0);
	pub const Yes: Sn1PdifMipSimpleIpFallback = Sn1PdifMipSimpleIpFallback(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1Pdsn1HandoffReqIpAddr(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1Pdsn1HandoffReqIpAddr {
	pub const Disabled: Sn1Pdsn1HandoffReqIpAddr = Sn1Pdsn1HandoffReqIpAddr(0);
	pub const Enabled: Sn1Pdsn1HandoffReqIpAddr = Sn1Pdsn1HandoffReqIpAddr(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PermitUserMcastPdus(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PermitUserMcastPdus {
	pub const Disabled: Sn1PermitUserMcastPdus = Sn1PermitUserMcastPdus(0);
	pub const Enabled: Sn1PermitUserMcastPdus = Sn1PermitUserMcastPdus(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PppAcceptPeerV6Ifid(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PppAcceptPeerV6Ifid {
	pub const Disabled: Sn1PppAcceptPeerV6Ifid = Sn1PppAcceptPeerV6Ifid(0);
	pub const Enabled: Sn1PppAcceptPeerV6Ifid = Sn1PppAcceptPeerV6Ifid(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PppAlwaysOnVse(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PppAlwaysOnVse {
	pub const Disabled: Sn1PppAlwaysOnVse = Sn1PppAlwaysOnVse(0);
	pub const Enabled: Sn1PppAlwaysOnVse = Sn1PppAlwaysOnVse(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PppNwLayerIpv4(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PppNwLayerIpv4 {
	pub const Disabled: Sn1PppNwLayerIpv4 = Sn1PppNwLayerIpv4(0);
	pub const Enabled: Sn1PppNwLayerIpv4 = Sn1PppNwLayerIpv4(1);
	pub const Passive: Sn1PppNwLayerIpv4 = Sn1PppNwLayerIpv4(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PppNwLayerIpv6(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PppNwLayerIpv6 {
	pub const Disabled: Sn1PppNwLayerIpv6 = Sn1PppNwLayerIpv6(0);
	pub const Enabled: Sn1PppNwLayerIpv6 = Sn1PppNwLayerIpv6(1);
	pub const Passive: Sn1PppNwLayerIpv6 = Sn1PppNwLayerIpv6(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PppRenegDisc(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PppRenegDisc {
	pub const Never: Sn1PppRenegDisc = Sn1PppRenegDisc(0);
	pub const Always: Sn1PppRenegDisc = Sn1PppRenegDisc(1);
	pub const NaiPrefixMsidMismatch: Sn1PppRenegDisc = Sn1PppRenegDisc(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1Prepaid(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1Prepaid {
	pub const NoPrepaid: Sn1Prepaid = Sn1Prepaid(0);
	pub const CustomPrepaid: Sn1Prepaid = Sn1Prepaid(1);
	pub const StandardPrepaid: Sn1Prepaid = Sn1Prepaid(2);
	pub const WimaxPrepaid: Sn1Prepaid = Sn1Prepaid(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PrepaidCompressedCount(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PrepaidCompressedCount {
	pub const Uncompressed: Sn1PrepaidCompressedCount = Sn1PrepaidCompressedCount(0);
	pub const Compressed: Sn1PrepaidCompressedCount = Sn1PrepaidCompressedCount(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PrepaidFinalDurationAlg(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PrepaidFinalDurationAlg {
	pub const CurrentTime: Sn1PrepaidFinalDurationAlg = Sn1PrepaidFinalDurationAlg(0);
	pub const LastUserLayer3ActivityTime: Sn1PrepaidFinalDurationAlg = Sn1PrepaidFinalDurationAlg(1);
	pub const LastAirlinkActivityTime: Sn1PrepaidFinalDurationAlg = Sn1PrepaidFinalDurationAlg(2);
	pub const LastAirlinkActivityTimeLastReported: Sn1PrepaidFinalDurationAlg = Sn1PrepaidFinalDurationAlg(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1PrepaidPreference(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1PrepaidPreference {
	pub const PrepaidDuration: Sn1PrepaidPreference = Sn1PrepaidPreference(0);
	pub const PrepaidVolume: Sn1PrepaidPreference = Sn1PrepaidPreference(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1ProxyMip(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1ProxyMip {
	pub const Disabled: Sn1ProxyMip = Sn1ProxyMip(0);
	pub const Enabled: Sn1ProxyMip = Sn1ProxyMip(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1QosClassBackgroundPhb(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1QosClassBackgroundPhb {
	pub const BestEffort: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(0);
	pub const PassThrough: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(1);
	pub const Af11: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(10);
	pub const Af12: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(12);
	pub const Af13: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(14);
	pub const Af21: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(18);
	pub const Af22: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(20);
	pub const Af23: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(22);
	pub const Af31: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(26);
	pub const Af32: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(28);
	pub const Af33: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(30);
	pub const Af41: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(34);
	pub const Af42: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(36);
	pub const Af43: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(38);
	pub const Ef: Sn1QosClassBackgroundPhb = Sn1QosClassBackgroundPhb(46);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1QosClassConversationalPhb(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1QosClassConversationalPhb {
	pub const BestEffort: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(0);
	pub const PassThrough: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(1);
	pub const Af11: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(10);
	pub const Af12: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(12);
	pub const Af13: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(14);
	pub const Af21: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(18);
	pub const Af22: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(20);
	pub const Af23: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(22);
	pub const Af31: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(26);
	pub const Af32: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(28);
	pub const Af33: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(30);
	pub const Af41: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(34);
	pub const Af42: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(36);
	pub const Af43: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(38);
	pub const Ef: Sn1QosClassConversationalPhb = Sn1QosClassConversationalPhb(46);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1QosClassInteractive1Phb(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1QosClassInteractive1Phb {
	pub const BestEffort: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(0);
	pub const PassThrough: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(1);
	pub const Af11: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(10);
	pub const Af12: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(12);
	pub const Af13: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(14);
	pub const Af21: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(18);
	pub const Af22: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(20);
	pub const Af23: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(22);
	pub const Af31: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(26);
	pub const Af32: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(28);
	pub const Af33: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(30);
	pub const Af41: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(34);
	pub const Af42: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(36);
	pub const Af43: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(38);
	pub const Ef: Sn1QosClassInteractive1Phb = Sn1QosClassInteractive1Phb(46);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1QosClassInteractive2Phb(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1QosClassInteractive2Phb {
	pub const BestEffort: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(0);
	pub const PassThrough: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(1);
	pub const Af11: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(10);
	pub const Af12: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(12);
	pub const Af13: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(14);
	pub const Af21: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(18);
	pub const Af22: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(20);
	pub const Af23: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(22);
	pub const Af31: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(26);
	pub const Af32: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(28);
	pub const Af33: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(30);
	pub const Af41: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(34);
	pub const Af42: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(36);
	pub const Af43: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(38);
	pub const Ef: Sn1QosClassInteractive2Phb = Sn1QosClassInteractive2Phb(46);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1QosClassInteractive3Phb(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1QosClassInteractive3Phb {
	pub const BestEffort: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(0);
	pub const PassThrough: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(1);
	pub const Af11: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(10);
	pub const Af12: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(12);
	pub const Af13: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(14);
	pub const Af21: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(18);
	pub const Af22: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(20);
	pub const Af23: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(22);
	pub const Af31: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(26);
	pub const Af32: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(28);
	pub const Af33: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(30);
	pub const Af41: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(34);
	pub const Af42: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(36);
	pub const Af43: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(38);
	pub const Ef: Sn1QosClassInteractive3Phb = Sn1QosClassInteractive3Phb(46);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1QosClassStreamingPhb(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1QosClassStreamingPhb {
	pub const BestEffort: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(0);
	pub const PassThrough: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(1);
	pub const Af11: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(10);
	pub const Af12: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(12);
	pub const Af13: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(14);
	pub const Af21: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(18);
	pub const Af22: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(20);
	pub const Af23: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(22);
	pub const Af31: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(26);
	pub const Af32: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(28);
	pub const Af33: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(30);
	pub const Af41: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(34);
	pub const Af42: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(36);
	pub const Af43: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(38);
	pub const Ef: Sn1QosClassStreamingPhb = Sn1QosClassStreamingPhb(46);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1QosTpDnlk(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1QosTpDnlk {
	pub const Disabled: Sn1QosTpDnlk = Sn1QosTpDnlk(0);
	pub const Policing: Sn1QosTpDnlk = Sn1QosTpDnlk(1);
	pub const Shaping: Sn1QosTpDnlk = Sn1QosTpDnlk(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1QosTpUplk(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1QosTpUplk {
	pub const Disabled: Sn1QosTpUplk = Sn1QosTpUplk(0);
	pub const Policing: Sn1QosTpUplk = Sn1QosTpUplk(1);
	pub const Shaping: Sn1QosTpUplk = Sn1QosTpUplk(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1RadiusReturnedUsername(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1RadiusReturnedUsername {
	pub const No: Sn1RadiusReturnedUsername = Sn1RadiusReturnedUsername(0);
	pub const Yes: Sn1RadiusReturnedUsername = Sn1RadiusReturnedUsername(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1RoamingSubUseGgsn(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1RoamingSubUseGgsn {
	pub const Deny: Sn1RoamingSubUseGgsn = Sn1RoamingSubUseGgsn(0);
	pub const Accept: Sn1RoamingSubUseGgsn = Sn1RoamingSubUseGgsn(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1ServiceType(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1ServiceType {
	pub const None: Sn1ServiceType = Sn1ServiceType(0);
	pub const Pdsn: Sn1ServiceType = Sn1ServiceType(1);
	pub const Management: Sn1ServiceType = Sn1ServiceType(2);
	pub const Ha: Sn1ServiceType = Sn1ServiceType(3);
	pub const Ggsn: Sn1ServiceType = Sn1ServiceType(4);
	pub const Lns: Sn1ServiceType = Sn1ServiceType(5);
	pub const Ipsg: Sn1ServiceType = Sn1ServiceType(6);
	pub const Cscf: Sn1ServiceType = Sn1ServiceType(7);
	pub const Asngw: Sn1ServiceType = Sn1ServiceType(8);
	pub const Pdif: Sn1ServiceType = Sn1ServiceType(9);
	pub const StandaloneFa: Sn1ServiceType = Sn1ServiceType(10);
	pub const Sgsn: Sn1ServiceType = Sn1ServiceType(11);
	pub const Phsgw: Sn1ServiceType = Sn1ServiceType(12);
	pub const Pdg: Sn1ServiceType = Sn1ServiceType(13);
	pub const Mipv6Ha: Sn1ServiceType = Sn1ServiceType(14);
	pub const Pgw: Sn1ServiceType = Sn1ServiceType(15);
	pub const Sgw: Sn1ServiceType = Sn1ServiceType(16);
	pub const Fng: Sn1ServiceType = Sn1ServiceType(17);
	pub const Ogw: Sn1ServiceType = Sn1ServiceType(18);
	pub const Hnbgw: Sn1ServiceType = Sn1ServiceType(19);
	pub const Bng: Sn1ServiceType = Sn1ServiceType(20);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubsAccFlowTrafficValid(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubsAccFlowTrafficValid {
	pub const Disabled: Sn1SubsAccFlowTrafficValid = Sn1SubsAccFlowTrafficValid(0);
	pub const Enabled: Sn1SubsAccFlowTrafficValid = Sn1SubsAccFlowTrafficValid(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubscriberAccounting(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubscriberAccounting {
	pub const None: Sn1SubscriberAccounting = Sn1SubscriberAccounting(0);
	pub const Radius: Sn1SubscriberAccounting = Sn1SubscriberAccounting(1);
	pub const Gtpp: Sn1SubscriberAccounting = Sn1SubscriberAccounting(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubscriberAcctInterim(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubscriberAcctInterim {
	pub const Normal: Sn1SubscriberAcctInterim = Sn1SubscriberAcctInterim(0);
	pub const Suppress: Sn1SubscriberAcctInterim = Sn1SubscriberAcctInterim(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubscriberAcctMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubscriberAcctMode {
	pub const FlowBasedAuxilliary: Sn1SubscriberAcctMode = Sn1SubscriberAcctMode(0);
	pub const FlowBasedAll: Sn1SubscriberAcctMode = Sn1SubscriberAcctMode(1);
	pub const FlowBasedNone: Sn1SubscriberAcctMode = Sn1SubscriberAcctMode(2);
	pub const SessionBased: Sn1SubscriberAcctMode = Sn1SubscriberAcctMode(3);
	pub const MainA10Only: Sn1SubscriberAcctMode = Sn1SubscriberAcctMode(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubscriberAcctRspAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubscriberAcctRspAction {
	pub const None: Sn1SubscriberAcctRspAction = Sn1SubscriberAcctRspAction(0);
	pub const NoEarlyPdus: Sn1SubscriberAcctRspAction = Sn1SubscriberAcctRspAction(1);
	pub const DelayGtpResponse: Sn1SubscriberAcctRspAction = Sn1SubscriberAcctRspAction(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubscriberAcctStart(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubscriberAcctStart {
	pub const Normal: Sn1SubscriberAcctStart = Sn1SubscriberAcctStart(0);
	pub const Suppress: Sn1SubscriberAcctStart = Sn1SubscriberAcctStart(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubscriberAcctStop(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubscriberAcctStop {
	pub const Normal: Sn1SubscriberAcctStop = Sn1SubscriberAcctStop(0);
	pub const Suppress: Sn1SubscriberAcctStop = Sn1SubscriberAcctStop(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubscriberClass(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubscriberClass {
	pub const NormalSubscriber: Sn1SubscriberClass = Sn1SubscriberClass(0);
	pub const Ting100: Sn1SubscriberClass = Sn1SubscriberClass(1);
	pub const Ting500: Sn1SubscriberClass = Sn1SubscriberClass(2);
	pub const TingBuddy: Sn1SubscriberClass = Sn1SubscriberClass(3);
	pub const TingStar: Sn1SubscriberClass = Sn1SubscriberClass(4);
	pub const TingNolimitSms: Sn1SubscriberClass = Sn1SubscriberClass(5);
	pub const KidsLocator: Sn1SubscriberClass = Sn1SubscriberClass(6);
	pub const Ting2000: Sn1SubscriberClass = Sn1SubscriberClass(7);
	pub const HandicappedWelfare: Sn1SubscriberClass = Sn1SubscriberClass(8);
	pub const Reserved: Sn1SubscriberClass = Sn1SubscriberClass(9);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubscriberIpHdrNegMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubscriberIpHdrNegMode {
	pub const Force: Sn1SubscriberIpHdrNegMode = Sn1SubscriberIpHdrNegMode(0);
	pub const Detect: Sn1SubscriberIpHdrNegMode = Sn1SubscriberIpHdrNegMode(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubscriberIpTosCopy(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubscriberIpTosCopy {
	pub const None: Sn1SubscriberIpTosCopy = Sn1SubscriberIpTosCopy(0);
	pub const AccessTunnel: Sn1SubscriberIpTosCopy = Sn1SubscriberIpTosCopy(1);
	pub const DataTunnel: Sn1SubscriberIpTosCopy = Sn1SubscriberIpTosCopy(2);
	pub const Both: Sn1SubscriberIpTosCopy = Sn1SubscriberIpTosCopy(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubscriberNoInterims(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubscriberNoInterims {
	pub const Disabled: Sn1SubscriberNoInterims = Sn1SubscriberNoInterims(0);
	pub const Enabled: Sn1SubscriberNoInterims = Sn1SubscriberNoInterims(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1SubsVjSlotidCmpNegMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1SubsVjSlotidCmpNegMode {
	pub const None: Sn1SubsVjSlotidCmpNegMode = Sn1SubsVjSlotidCmpNegMode(0);
	pub const Receive: Sn1SubsVjSlotidCmpNegMode = Sn1SubsVjSlotidCmpNegMode(1);
	pub const Transmit: Sn1SubsVjSlotidCmpNegMode = Sn1SubsVjSlotidCmpNegMode(2);
	pub const Both: Sn1SubsVjSlotidCmpNegMode = Sn1SubsVjSlotidCmpNegMode(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1TpDnlkExceedAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1TpDnlkExceedAction {
	pub const Transmit: Sn1TpDnlkExceedAction = Sn1TpDnlkExceedAction(0);
	pub const Drop: Sn1TpDnlkExceedAction = Sn1TpDnlkExceedAction(1);
	pub const LowerIpPrecedence: Sn1TpDnlkExceedAction = Sn1TpDnlkExceedAction(2);
	pub const Buffer: Sn1TpDnlkExceedAction = Sn1TpDnlkExceedAction(3);
	pub const TransmitOnBufferFull: Sn1TpDnlkExceedAction = Sn1TpDnlkExceedAction(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1TpDnlkViolateAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1TpDnlkViolateAction {
	pub const Transmit: Sn1TpDnlkViolateAction = Sn1TpDnlkViolateAction(0);
	pub const Drop: Sn1TpDnlkViolateAction = Sn1TpDnlkViolateAction(1);
	pub const LowerIpPrecedence: Sn1TpDnlkViolateAction = Sn1TpDnlkViolateAction(2);
	pub const Buffer: Sn1TpDnlkViolateAction = Sn1TpDnlkViolateAction(3);
	pub const TransmitOnBufferFull: Sn1TpDnlkViolateAction = Sn1TpDnlkViolateAction(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1TpUplkExceedAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1TpUplkExceedAction {
	pub const Transmit: Sn1TpUplkExceedAction = Sn1TpUplkExceedAction(0);
	pub const Drop: Sn1TpUplkExceedAction = Sn1TpUplkExceedAction(1);
	pub const LowerIpPrecedence: Sn1TpUplkExceedAction = Sn1TpUplkExceedAction(2);
	pub const Buffer: Sn1TpUplkExceedAction = Sn1TpUplkExceedAction(3);
	pub const TransmitOnBufferFull: Sn1TpUplkExceedAction = Sn1TpUplkExceedAction(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1TpUplkViolateAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1TpUplkViolateAction {
	pub const Transmit: Sn1TpUplkViolateAction = Sn1TpUplkViolateAction(0);
	pub const Drop: Sn1TpUplkViolateAction = Sn1TpUplkViolateAction(1);
	pub const LowerIpPrecedence: Sn1TpUplkViolateAction = Sn1TpUplkViolateAction(2);
	pub const Buffer: Sn1TpUplkViolateAction = Sn1TpUplkViolateAction(3);
	pub const TransmitOnBufferFull: Sn1TpUplkViolateAction = Sn1TpUplkViolateAction(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1TunAddrPolicy(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1TunAddrPolicy {
	pub const NoLocalAllocValidate: Sn1TunAddrPolicy = Sn1TunAddrPolicy(0);
	pub const LocalAlloc: Sn1TunAddrPolicy = Sn1TunAddrPolicy(1);
	pub const LocalAllocValidate: Sn1TunAddrPolicy = Sn1TunAddrPolicy(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1TunnelGn(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1TunnelGn {
	pub const Disabled: Sn1TunnelGn = Sn1TunnelGn(0);
	pub const Enabled: Sn1TunnelGn = Sn1TunnelGn(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1TunnelLoadBalancing(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1TunnelLoadBalancing {
	pub const Random: Sn1TunnelLoadBalancing = Sn1TunnelLoadBalancing(1);
	pub const Balanced: Sn1TunnelLoadBalancing = Sn1TunnelLoadBalancing(2);
	pub const Prioritized: Sn1TunnelLoadBalancing = Sn1TunnelLoadBalancing(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sn1VisitingSubUseGgsn(pub u32);
 
#[allow(non_upper_case_globals)]
impl Sn1VisitingSubUseGgsn {
	pub const Deny: Sn1VisitingSubUseGgsn = Sn1VisitingSubUseGgsn(0);
	pub const Accept: Sn1VisitingSubUseGgsn = Sn1VisitingSubUseGgsn(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaSn1VpnId(v)},
		2 => value!(i, Attribute::VsaSn1VpnName(i)),
		3 => map! {i, be_u32, |v| Attribute::VsaSn1DisconnectReason(Sn1DisconnectReason(v))},
		4 => map! {i, be_u32, |v| Attribute::VsaSn1PppProgressCode(Sn1PppProgressCode(v))},
		5 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSn1PrimaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		6 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSn1SecondaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		7 => map!{i, be_u32, |v| Attribute::VsaSn1ReChapInterval(v)},
		8 => value!(i, Attribute::VsaSn1IpPoolName(i)),
		9 => map! {i, be_u32, |v| Attribute::VsaSn1PppDataCompression(Sn1PppDataCompression(v))},
		10 => value!(i, Attribute::VsaSn1IpFilterIn(i)),
		11 => value!(i, Attribute::VsaSn1IpFilterOut(i)),
		13 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSn1LocalIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		14 => map! {i, be_u32, |v| Attribute::VsaSn1IpSourceValidation(Sn1IpSourceValidation(v))},
		15 => value!(i, Attribute::VsaSn1PppOutboundPassword(i)),
		16 => map!{i, be_u32, |v| Attribute::VsaSn1PppKeepalive(v)},
		17 => value!(i, Attribute::VsaSn1IpInAcl(i)),
		18 => value!(i, Attribute::VsaSn1IpOutAcl(i)),
		19 => map! {i, be_u32, |v| Attribute::VsaSn1PppDataCompressionMode(Sn1PppDataCompressionMode(v))},
		20 => map! {i, be_u32, |v| Attribute::VsaSn1SubscriberPermission(Sn1SubscriberPermission(v))},
		21 => map! {i, be_u32, |v| Attribute::VsaSn1AdminPermission(Sn1AdminPermission(v))},
		22 => map! {i, be_u32, |v| Attribute::VsaSn1SimultaneousSipMip(Sn1SimultaneousSipMip(v))},
		23 => map!{i, be_u32, |v| Attribute::VsaSn1MinCompressSize(v)},
		24 => map! {i, be_u32, |v| Attribute::VsaSn1ServiceType(Sn1ServiceType(v))},
		25 => map! {i, be_u32, |v| Attribute::VsaSn1DnsProxyUseSubscrAddr(Sn1DnsProxyUseSubscrAddr(v))},
		26 => value!(i, Attribute::VsaSn1TunnelPassword(i)),
		27 => map! {i, be_u32, |v| Attribute::VsaSn1TunnelLoadBalancing(Sn1TunnelLoadBalancing(v))},
		30 => map!{i, be_u32, |v| Attribute::VsaSn1MnHaTimestampTolerance(v)},
		31 => map! {i, be_u32, |v| Attribute::VsaSn1PrepaidCompressedCount(Sn1PrepaidCompressedCount(v))},
		32 => map!{i, be_u32, |v| Attribute::VsaSn1PrepaidInboundOctets(v)},
		33 => map!{i, be_u32, |v| Attribute::VsaSn1PrepaidOutboundOctets(v)},
		34 => map!{i, be_u32, |v| Attribute::VsaSn1PrepaidTotalOctets(v)},
		35 => map!{i, be_u32, |v| Attribute::VsaSn1PrepaidTimeout(v)},
		36 => map!{i, be_u32, |v| Attribute::VsaSn1PrepaidWatermark(v)},
		37 => value!(i, Attribute::VsaSn1NaiConstructionDomain(i)),
		38 => value!(i, Attribute::VsaSn1TunnelIsakmpCryptoMap(i)),
		39 => value!(i, Attribute::VsaSn1TunnelIsakmpSecret(i)),
		41 => value!(i, Attribute::VsaSn1ExtInlineSrvrContext(i)),
		43 => map! {i, be_u32, |v| Attribute::VsaSn1L3ToL2TunAddrPolicy(Sn1L3ToL2TunAddrPolicy(v))},
		44 => map!{i, be_u32, |v| Attribute::VsaSn1LongDurationTimeout(v)},
		45 => map! {i, be_u32, |v| Attribute::VsaSn1LongDurationAction(Sn1LongDurationAction(v))},
		46 => map! {i, be_u32, |v| Attribute::VsaSn1Pdsn1HandoffReqIpAddr(Sn1Pdsn1HandoffReqIpAddr(v))},
		47 => map! {i, be_u32, |v| Attribute::VsaSn1HaSendDnsAddress(Sn1HaSendDnsAddress(v))},
		48 => map! {i, be_u32, |v| Attribute::VsaSn1MipSendTermVerification(Sn1MipSendTermVerification(v))},
		49 => map! {i, be_u32, |v| Attribute::VsaSn1DataTunnelIgnoreDfBit(Sn1DataTunnelIgnoreDfBit(v))},
		50 => map! {i, be_u32, |v| Attribute::VsaSn1MipAaaAssignAddr(Sn1MipAaaAssignAddr(v))},
		52 => map! {i, be_u32, |v| Attribute::VsaSn1ProxyMip(Sn1ProxyMip(v))},
		51 => map! {i, be_u32, |v| Attribute::VsaSn1MipMatchAaaAssignAddr(Sn1MipMatchAaaAssignAddr(v))},
		53 => map! {i, be_u32, |v| Attribute::VsaSn1IpAllocMethod(Sn1IpAllocMethod(v))},
		54 => map! {i, be_u32, |v| Attribute::VsaSn1GratuitousArpAggressive(Sn1GratuitousArpAggressive(v))},
		55 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSn1ExtInlineSrvrUpAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		56 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSn1ExtInlineSrvrDownAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		57 => map!{i, be_u32, |v| Attribute::VsaSn1ExtInlineSrvrPreference(v)},
		58 => value!(i, Attribute::VsaSn1ExtInlineSrvrUpVlan(i)),
		59 => value!(i, Attribute::VsaSn1ExtInlineSrvrDownVlan(i)),
		60 => map! {i, be_u32, |v| Attribute::VsaSn1IpHideServiceAddress(Sn1IpHideServiceAddress(v))},
		61 => value!(i, Attribute::VsaSn1PppOutboundUsername(i)),
		62 => map! {i, be_u32, |v| Attribute::VsaSn1GtpVersion(Sn1GtpVersion(v))},
		63 => map! {i, be_u32, |v| Attribute::VsaSn1AccessLinkIpFrag(Sn1AccessLinkIpFrag(v))},
		64 => map! {i, be_u32, |v| Attribute::VsaSn1SubscriberAccounting(Sn1SubscriberAccounting(v))},
		65 => value!(i, Attribute::VsaSn1NwReachabilityServerName(i)),
		67 => map! {i, be_u32, |v| Attribute::VsaSn1SubscriberIpHdrNegMode(Sn1SubscriberIpHdrNegMode(v))},
		68 => map! {i, be_u32, |v| Attribute::VsaSn1Ggsn1MipRequired(Sn1Ggsn1MipRequired(v))},
		69 => map! {i, be_u32, |v| Attribute::VsaSn1SubscriberAcctStart(Sn1SubscriberAcctStart(v))},
		70 => map! {i, be_u32, |v| Attribute::VsaSn1SubscriberAcctInterim(Sn1SubscriberAcctInterim(v))},
		71 => map! {i, be_u32, |v| Attribute::VsaSn1SubscriberAcctStop(Sn1SubscriberAcctStop(v))},
		73 => map! {i, be_u32, |v| Attribute::VsaSn1QosTpDnlk(Sn1QosTpDnlk(v))},
		74 => map!{i, be_u32, |v| Attribute::VsaSn1TpDnlkCommittedDataRate(v)},
		75 => map!{i, be_u32, |v| Attribute::VsaSn1TpDnlkPeakDataRate(v)},
		76 => map!{i, be_u32, |v| Attribute::VsaSn1TpDnlkBurstSize(v)},
		77 => map! {i, be_u32, |v| Attribute::VsaSn1TpDnlkExceedAction(Sn1TpDnlkExceedAction(v))},
		78 => map! {i, be_u32, |v| Attribute::VsaSn1TpDnlkViolateAction(Sn1TpDnlkViolateAction(v))},
		79 => map! {i, be_u32, |v| Attribute::VsaSn1QosTpUplk(Sn1QosTpUplk(v))},
		80 => map!{i, be_u32, |v| Attribute::VsaSn1TpUplkCommittedDataRate(v)},
		81 => map!{i, be_u32, |v| Attribute::VsaSn1TpUplkPeakDataRate(v)},
		82 => map!{i, be_u32, |v| Attribute::VsaSn1TpUplkBurstSize(v)},
		83 => map! {i, be_u32, |v| Attribute::VsaSn1TpUplkExceedAction(Sn1TpUplkExceedAction(v))},
		84 => map! {i, be_u32, |v| Attribute::VsaSn1TpUplkViolateAction(Sn1TpUplkViolateAction(v))},
		85 => map! {i, be_u32, |v| Attribute::VsaSn1SubscriberIpTosCopy(Sn1SubscriberIpTosCopy(v))},
		86 => value!(i, Attribute::VsaSn1QosConversationClass(i)),
		87 => value!(i, Attribute::VsaSn1QosStreamingClass(i)),
		88 => value!(i, Attribute::VsaSn1QosInteractive1Class(i)),
		89 => value!(i, Attribute::VsaSn1QosInteractive2Class(i)),
		90 => value!(i, Attribute::VsaSn1QosInteractive3Class(i)),
		91 => value!(i, Attribute::VsaSn1QosBackgroundClass(i)),
		92 => map! {i, be_u32, |v| Attribute::VsaSn1PppNwLayerIpv4(Sn1PppNwLayerIpv4(v))},
		93 => map! {i, be_u32, |v| Attribute::VsaSn1PppNwLayerIpv6(Sn1PppNwLayerIpv6(v))},
		94 => value!(i, Attribute::VsaSn1VirtualApnName(i)),
		95 => map! {i, be_u32, |v| Attribute::VsaSn1PppAcceptPeerV6Ifid(Sn1PppAcceptPeerV6Ifid(v))},
		96 => map!{i, be_u32, |v| Attribute::VsaSn1Ipv6RtrAdvtInterval(v)},
		97 => map!{i, be_u32, |v| Attribute::VsaSn1Ipv6NumRtrAdvt(v)},
		98 => map! {i, be_u32, |v| Attribute::VsaSn1NpuQosPriority(Sn1NpuQosPriority(v))},
		99 => map! {i, be_u32, |v| Attribute::VsaSn1MnHaHashAlgorithm(Sn1MnHaHashAlgorithm(v))},
		100 => map! {i, be_u32, |v| Attribute::VsaSn1SubscriberAcctRspAction(Sn1SubscriberAcctRspAction(v))},
		101 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaSn1Ipv6PrimaryDns(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		102 => value!(i, Attribute::VsaSn1Ipv6SecondaryDns(i)),
		103 => map! {i, be_u32, |v| Attribute::VsaSn1Ipv6EgressFiltering(Sn1Ipv6EgressFiltering(v))},
		104 => value!(i, Attribute::VsaSn1MediationVpnName(i)),
		105 => map! {i, be_u32, |v| Attribute::VsaSn1MediationAcctRspAction(Sn1MediationAcctRspAction(v))},
		106 => map! {i, be_u32, |v| Attribute::VsaSn1HomeSubUseGgsn(Sn1HomeSubUseGgsn(v))},
		107 => map! {i, be_u32, |v| Attribute::VsaSn1VisitingSubUseGgsn(Sn1VisitingSubUseGgsn(v))},
		108 => map! {i, be_u32, |v| Attribute::VsaSn1RoamingSubUseGgsn(Sn1RoamingSubUseGgsn(v))},
		109 => map!{i, be_u32, |v| Attribute::VsaSn1HomeProfile(v)},
		110 => map!{i, be_u32, |v| Attribute::VsaSn1IpSrcValidationDropLimit(v)},
		111 => map! {i, be_u32, |v| Attribute::VsaSn1QosClassConversationalPhb(Sn1QosClassConversationalPhb(v))},
		112 => map! {i, be_u32, |v| Attribute::VsaSn1QosClassStreamingPhb(Sn1QosClassStreamingPhb(v))},
		113 => map! {i, be_u32, |v| Attribute::VsaSn1QosClassBackgroundPhb(Sn1QosClassBackgroundPhb(v))},
		114 => map! {i, be_u32, |v| Attribute::VsaSn1QosClassInteractive1Phb(Sn1QosClassInteractive1Phb(v))},
		115 => map! {i, be_u32, |v| Attribute::VsaSn1QosClassInteractive2Phb(Sn1QosClassInteractive2Phb(v))},
		116 => map! {i, be_u32, |v| Attribute::VsaSn1QosClassInteractive3Phb(Sn1QosClassInteractive3Phb(v))},
		117 => map!{i, be_u32, |v| Attribute::VsaSn1VisitingProfile(v)},
		118 => map!{i, be_u32, |v| Attribute::VsaSn1RoamingProfile(v)},
		119 => map!{i, be_u32, |v| Attribute::VsaSn1HomeBehavior(v)},
		120 => map!{i, be_u32, |v| Attribute::VsaSn1VisitingBehavior(v)},
		121 => map!{i, be_u32, |v| Attribute::VsaSn1RoamingBehavior(v)},
		122 => map!{i, be_u32, |v| Attribute::VsaSn1InternalSmIndex(v)},
		123 => map! {i, be_u32, |v| Attribute::VsaSn1MediationEnabled(Sn1MediationEnabled(v))},
		124 => value!(i, Attribute::VsaSn1Ipv6SecPool(i)),
		125 => value!(i, Attribute::VsaSn1Ipv6SecPrefix(i)),
		126 => map! {i, be_u32, |v| Attribute::VsaSn1Ipv6DnsProxy(Sn1Ipv6DnsProxy(v))},
		127 => map!{i, be_u32, |v| Attribute::VsaSn1SubscriberNexthopAddress(v)},
		128 => map! {i, be_u32, |v| Attribute::VsaSn1Prepaid(Sn1Prepaid(v))},
		129 => map! {i, be_u32, |v| Attribute::VsaSn1PrepaidPreference(Sn1PrepaidPreference(v))},
		130 => map! {i, be_u32, |v| Attribute::VsaSn1PppAlwaysOnVse(Sn1PppAlwaysOnVse(v))},
		131 => value!(i, Attribute::VsaSn1VoicePushListName(i)),
		132 => value!(i, Attribute::VsaSn1UnclassifyListName(i)),
		133 => map! {i, be_u32, |v| Attribute::VsaSn1SubscriberNoInterims(Sn1SubscriberNoInterims(v))},
		134 => map! {i, be_u32, |v| Attribute::VsaSn1PermitUserMcastPdus(Sn1PermitUserMcastPdus(v))},
		135 => map! {i, be_u32, |v| Attribute::VsaSn1PrepaidFinalDurationAlg(Sn1PrepaidFinalDurationAlg(v))},
		136 => map!{i, be_u32, |v| Attribute::VsaSn1Ipv6MinLinkMtu(v)},
		137 => value!(i, Attribute::VsaSn1ChargingVpnName(i)),
		138 => map!{i, be_u32, |v| Attribute::VsaSn1ChrgCharSelectionMode(v)},
		139 => map!{i, be_u32, |v| Attribute::VsaSn1CauseForRecClosing(v)},
		140 => map! {i, be_u32, |v| Attribute::VsaSn1ChangeCondition(Sn1ChangeCondition(v))},
		141 => value!(i, Attribute::VsaSn1DynamicAddrAllocIndFlag(i)),
		142 => value!(i, Attribute::VsaSn1NtkInitiatedCtxIndFlag(i)),
		143 => map! {i, be_u32, |v| Attribute::VsaSn1NtkSessionDisconnectFlag(Sn1NtkSessionDisconnectFlag(v))},
		144 => map! {i, be_u32, |v| Attribute::VsaSn1EnableQosRenegotiation(Sn1EnableQosRenegotiation(v))},
		145 => map!{i, be_u32, |v| Attribute::VsaSn1QosRenegotiationTimeout(v)},
		147 => value!(i, Attribute::VsaSn1QosNegotiated(i)),
		146 => map! {i, be_u32, |v| Attribute::VsaSn1MediationNoInterims(Sn1MediationNoInterims(v))},
		148 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSn1PrimaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		149 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSn1SecondaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		150 => map! {i, be_u32, |v| Attribute::VsaSn1IpHeaderCompression(Sn1IpHeaderCompression(v))},
		151 => map! {i, be_u32, |v| Attribute::VsaSn1Mode(Sn1Mode(v))},
		152 => map!{i, be_u16, |v| Attribute::VsaSn1AssignedVlanId(v)},
		153 => map! {i, be_u32, |v| Attribute::VsaSn1Direction(Sn1Direction(v))},
		154 => value!(i, Attribute::VsaSn1MipHaAssignmentTable(i)),
		156 => map! {i, be_u32, |v| Attribute::VsaSn1TunAddrPolicy(Sn1TunAddrPolicy(v))},
		157 => map! {i, be_u32, |v| Attribute::VsaSn1DhcpLeaseExpiryPolicy(Sn1DhcpLeaseExpiryPolicy(v))},
		158 => value!(i, Attribute::VsaSn1SubscriberTemplateName(i)),
		159 => value!(i, Attribute::VsaSn1SubsImsaServiceName(i)),
		161 => map!{i, be_u32, |v| Attribute::VsaSn1TrafficGroup(v)},
		162 => value!(i, Attribute::VsaSn1RadApnName(i)),
		163 => map! {i, be_u32, |v| Attribute::VsaSn1MipSendAncid(Sn1MipSendAncid(v))},
		164 => map! {i, be_u32, |v| Attribute::VsaSn1MipSendImsi(Sn1MipSendImsi(v))},
		165 => map! {i, be_u32, |v| Attribute::VsaSn1MipDualAnchor(Sn1MipDualAnchor(v))},
		166 => value!(i, Attribute::VsaSn1MipAncid(i)),
		167 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSn1ImsAmAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		168 => value!(i, Attribute::VsaSn1ImsAmDomainName(i)),
		169 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSn1ServiceAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		170 => map! {i, be_u32, |v| Attribute::VsaSn1PdifMipRequired(Sn1PdifMipRequired(v))},
		171 => value!(i, Attribute::VsaSn1FmcLocation(i)),
		172 => map! {i, be_u32, |v| Attribute::VsaSn1PdifMipReleaseTia(Sn1PdifMipReleaseTia(v))},
		173 => map! {i, be_u32, |v| Attribute::VsaSn1PdifMipSimpleIpFallback(Sn1PdifMipSimpleIpFallback(v))},
		174 => map! {i, be_u32, |v| Attribute::VsaSn1TunnelGn(Sn1TunnelGn(v))},
		175 => map!{i, be_u32, |v| Attribute::VsaSn1MipRegLifetimeRealm(v)},
		176 => value!(i, Attribute::VsaSn1EcsDataVolume(i)),
		177 => value!(i, Attribute::VsaSn1QosTrafficPolicy(i)),
		178 => value!(i, Attribute::VsaSn1Anid(i)),
		187 => map! {i, be_u32, |v| Attribute::VsaSn1PppRenegDisc(Sn1PppRenegDisc(v))},
		188 => map! {i, be_u32, |v| Attribute::VsaSn1MipSendCorrelationInfo(Sn1MipSendCorrelationInfo(v))},
		189 => value!(i, Attribute::VsaSn1Pdsn1CorrelationId(i)),
		190 => value!(i, Attribute::VsaSn1Pdsn1NasId(i)),
		191 => map!{i, take!(4), |v:&[u8]| Attribute::VsaSn1Pdsn1NasIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		192 => map! {i, be_u32, |v| Attribute::VsaSn1SubscriberAcctMode(Sn1SubscriberAcctMode(v))},
		193 => value!(i, Attribute::VsaSn1IpInPlcyGrp(i)),
		194 => value!(i, Attribute::VsaSn1IpOutPlcyGrp(i)),
		196 => map! {i, be_u32, |v| Attribute::VsaSn1IpSourceViolateNoAcct(Sn1IpSourceViolateNoAcct(v))},
		198 => map! {i, be_u32, |v| Attribute::VsaSn1FirewallEnabled(Sn1FirewallEnabled(v))},
		200 => map!{i, be_u32, |v| Attribute::VsaSnaPppUnfrDataInOct(v)},
		201 => map!{i, be_u32, |v| Attribute::VsaSnaPppUnfrDataOutOct(v)},
		202 => map!{i, be_u32, |v| Attribute::VsaSnaPppUnfrDataInGig(v)},
		203 => map!{i, be_u32, |v| Attribute::VsaSnaPppUnfrDataOutGig(v)},
		204 => map!{i, be_u32, |v| Attribute::VsaSn1AdminExpiry(v)},
		206 => map!{i, be_u32, |v| Attribute::VsaSnaInputGigawords(v)},
		207 => map!{i, be_u32, |v| Attribute::VsaSnaOutputGigawords(v)},
		214 => value!(i, Attribute::VsaSn1DnsProxyInterceptList(i)),
		219 => map! {i, be_u32, |v| Attribute::VsaSn1SubscriberClass(Sn1SubscriberClass(v))},
		220 => map!{i, be_u32, |v| Attribute::VsaSn1CfpolicyId(v)},
		221 => map! {i, be_u32, |v| Attribute::VsaSn1SubsVjSlotidCmpNegMode(Sn1SubsVjSlotidCmpNegMode(v))},
		223 => value!(i, Attribute::VsaSn1PrimaryDccaPeer(i)),
		224 => value!(i, Attribute::VsaSn1SecondaryDccaPeer(i)),
		225 => map! {i, be_u32, |v| Attribute::VsaSn1SubsAccFlowTrafficValid(Sn1SubsAccFlowTrafficValid(v))},
		226 => map!{i, be_u32, |v| Attribute::VsaSn1AcctInputPacketsDropped(v)},
		227 => map!{i, be_u32, |v| Attribute::VsaSn1AcctOutputPacketsDropped(v)},
		228 => map!{i, be_u64, |v| Attribute::VsaSn1AcctInputOctetsDropped(v)},
		229 => map!{i, be_u64, |v| Attribute::VsaSn1AcctOutputOctetsDropped(v)},
		230 => map!{i, be_u32, |v| Attribute::VsaSn1AcctInputGigaDropped(v)},
		231 => map!{i, be_u32, |v| Attribute::VsaSn1AcctOutputGigaDropped(v)},
		233 => map!{i, be_u32, |v| Attribute::VsaSn1OverloadDiscConnectTime(v)},
		235 => map!{i, be_u32, |v| Attribute::VsaSn1OverloadDisconnect(v)},
		236 => map! {i, be_u32, |v| Attribute::VsaSn1RadiusReturnedUsername(Sn1RadiusReturnedUsername(v))},
		238 => value!(i, Attribute::VsaSn1RohcProfileName(i)),
		239 => value!(i, Attribute::VsaSn1FirewallPolicy(i)),
		247 => value!(i, Attribute::VsaSn1TransparentData(i)),
		248 => value!(i, Attribute::VsaSn1MsIsdn(i)),
		249 => value!(i, Attribute::VsaSn1RoutingAreaId(i)),
		251 => map!{i, be_u32, |v| Attribute::VsaSn1CallId(v)},
		252 => value!(i, Attribute::VsaSn1Imsi(i)),
		253 => map! {i, be_u32, |v| Attribute::VsaSn1LongDurationNotification(Sn1LongDurationNotification(v))},
		254 => map!{i, be_u32, |v| Attribute::VsaSn1SipMethod(v)},
		255 => value!(i, Attribute::VsaSn1Event(i)),
        _ => value!(i, Attribute::VsaUnknown(8164, typ, i)),
    }
}
