/// Definitions for vendor USR, vendor value 429
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrSyslogTap(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrSyslogTap {
	pub const Off: UsrSyslogTap = UsrSyslogTap(0);
	pub const OnRaw: UsrSyslogTap = UsrSyslogTap(1);
	pub const OnFramed: UsrSyslogTap = UsrSyslogTap(2);
	pub const Unknown: UsrSyslogTap = UsrSyslogTap(4294967295);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrEventId(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrEventId {
	pub const ModuleInserted: UsrEventId = UsrEventId(6);
	pub const ModuleRemoved: UsrEventId = UsrEventId(7);
	pub const PsuVoltageAlarm: UsrEventId = UsrEventId(8);
	pub const PsuFailed: UsrEventId = UsrEventId(9);
	pub const HubTempOutOfRange: UsrEventId = UsrEventId(10);
	pub const FanFailed: UsrEventId = UsrEventId(11);
	pub const WatchdogTimeout: UsrEventId = UsrEventId(12);
	pub const MgmtBusFailure: UsrEventId = UsrEventId(13);
	pub const InConnectionEst: UsrEventId = UsrEventId(14);
	pub const OutConnectionEst: UsrEventId = UsrEventId(15);
	pub const InConnectionTerm: UsrEventId = UsrEventId(16);
	pub const OutConnectionTerm: UsrEventId = UsrEventId(17);
	pub const ConnectionFailed: UsrEventId = UsrEventId(18);
	pub const ConnectionTimeout: UsrEventId = UsrEventId(19);
	pub const DteTransmitIdle: UsrEventId = UsrEventId(20);
	pub const DtrTrue: UsrEventId = UsrEventId(21);
	pub const DtrFalse: UsrEventId = UsrEventId(22);
	pub const BlockErrorAtThreshold: UsrEventId = UsrEventId(23);
	pub const FallbacksAtThreshold: UsrEventId = UsrEventId(24);
	pub const NoDialToneDetected: UsrEventId = UsrEventId(25);
	pub const NoLoopCurrentDetected: UsrEventId = UsrEventId(26);
	pub const YellowAlarm: UsrEventId = UsrEventId(27);
	pub const RedAlarm: UsrEventId = UsrEventId(28);
	pub const LossOfSignal: UsrEventId = UsrEventId(29);
	pub const RcvAlrmIndSignal: UsrEventId = UsrEventId(30);
	pub const TimingSourceSwitch: UsrEventId = UsrEventId(31);
	pub const ModemResetByDte: UsrEventId = UsrEventId(32);
	pub const ModemRingNoAnswer: UsrEventId = UsrEventId(33);
	pub const DteRingNoAnswer: UsrEventId = UsrEventId(34);
	pub const PktBusSessionActive: UsrEventId = UsrEventId(35);
	pub const PktBusSessionCongestion: UsrEventId = UsrEventId(36);
	pub const PktBusSessionLost: UsrEventId = UsrEventId(37);
	pub const PktBusSessionInactive: UsrEventId = UsrEventId(38);
	pub const UserInterfaceReset: UsrEventId = UsrEventId(39);
	pub const GatewayPortOutOfService: UsrEventId = UsrEventId(40);
	pub const GatewayPortLinkActive: UsrEventId = UsrEventId(41);
	pub const DialOutLoginFailure: UsrEventId = UsrEventId(42);
	pub const DialInLoginFailure: UsrEventId = UsrEventId(43);
	pub const DialOutRestrictedNumber: UsrEventId = UsrEventId(44);
	pub const DialBackRestrictedNumber: UsrEventId = UsrEventId(45);
	pub const UserBlacklisted: UsrEventId = UsrEventId(46);
	pub const AttemptedLoginBlacklisted: UsrEventId = UsrEventId(47);
	pub const ResponseAttemptLimitExceeded: UsrEventId = UsrEventId(48);
	pub const LoginAttemptLimitExceeded: UsrEventId = UsrEventId(49);
	pub const DialOutCallDuration: UsrEventId = UsrEventId(50);
	pub const DialInCallDuration: UsrEventId = UsrEventId(51);
	pub const PktBusSessionErrStatus: UsrEventId = UsrEventId(52);
	pub const NmcAutorespnseTrap: UsrEventId = UsrEventId(53);
	pub const AcctServerContactLoss: UsrEventId = UsrEventId(54);
	pub const YellowAlarmClear: UsrEventId = UsrEventId(55);
	pub const RedAlarmClear: UsrEventId = UsrEventId(56);
	pub const LossOfSignalClear: UsrEventId = UsrEventId(57);
	pub const RcvAlrmIndSignalClear: UsrEventId = UsrEventId(58);
	pub const IncomingConnectionEstablished: UsrEventId = UsrEventId(59);
	pub const OutgoingConnectionEstablished: UsrEventId = UsrEventId(60);
	pub const IncomingConnectionTerminated: UsrEventId = UsrEventId(61);
	pub const OutgoingConnectionTerminated: UsrEventId = UsrEventId(62);
	pub const ConnectionAttemptFailure: UsrEventId = UsrEventId(63);
	pub const ContinuousCrcAlarm: UsrEventId = UsrEventId(64);
	pub const ContinuousCrcAlarmClear: UsrEventId = UsrEventId(65);
	pub const PhysicalStateChange: UsrEventId = UsrEventId(66);
	pub const GatewayNetworkFailed: UsrEventId = UsrEventId(71);
	pub const GatewayNetworkRestored: UsrEventId = UsrEventId(72);
	pub const PacketBusClockLost: UsrEventId = UsrEventId(73);
	pub const PacketBusClockRestored: UsrEventId = UsrEventId(74);
	pub const DChannelInService: UsrEventId = UsrEventId(75);
	pub const DChannelOutOfService: UsrEventId = UsrEventId(76);
	pub const Ds0SInService: UsrEventId = UsrEventId(77);
	pub const Ds0SOutOfService: UsrEventId = UsrEventId(78);
	pub const T1OrT1PriOrE1PriCallEvent: UsrEventId = UsrEventId(79);
	pub const PsuIncompatible: UsrEventId = UsrEventId(80);
	pub const T1CommaT1E1OrPriCallArriveEvent: UsrEventId = UsrEventId(81);
	pub const T1CommaT1E1OrPriCallConnectEvent: UsrEventId = UsrEventId(82);
	pub const T1CommaT1E1OrPriCallTerminaEvent: UsrEventId = UsrEventId(83);
	pub const T1CommaT1E1OrPriCallFailedEvent: UsrEventId = UsrEventId(84);
	pub const DnsContactLost: UsrEventId = UsrEventId(85);
	pub const NtpContactLost: UsrEventId = UsrEventId(86);
	pub const NtpContactRestored: UsrEventId = UsrEventId(87);
	pub const IpgwLinkUp: UsrEventId = UsrEventId(88);
	pub const IpgwLinkDown: UsrEventId = UsrEventId(89);
	pub const NtpContactDegraded: UsrEventId = UsrEventId(90);
	pub const InConnectionFailed: UsrEventId = UsrEventId(91);
	pub const OutConnectionFailed: UsrEventId = UsrEventId(92);
	pub const ApplicationProcessorreset: UsrEventId = UsrEventId(93);
	pub const DspReset: UsrEventId = UsrEventId(94);
	pub const ChangedToMaintSrvsState: UsrEventId = UsrEventId(95);
	pub const LoopBackClearedOnChannel: UsrEventId = UsrEventId(96);
	pub const LoopBackOnChannel: UsrEventId = UsrEventId(97);
	pub const TelcoAbnormalResponse: UsrEventId = UsrEventId(98);
	pub const DnsContactRestored: UsrEventId = UsrEventId(99);
	pub const DnsContactDegraded: UsrEventId = UsrEventId(100);
	pub const RadiusAccountingRestored: UsrEventId = UsrEventId(101);
	pub const RadiusAccountingGroupRestore: UsrEventId = UsrEventId(102);
	pub const RadiusAccountingGroupDegrade: UsrEventId = UsrEventId(103);
	pub const RadiusAccountingGroupNonoper: UsrEventId = UsrEventId(104);
	pub const T1OrT1E1OrPriIncallFailEvent: UsrEventId = UsrEventId(119);
	pub const T1OrT1E1OrPriOutcallFailEvent: UsrEventId = UsrEventId(120);
	pub const RmmieRetrainEvent: UsrEventId = UsrEventId(121);
	pub const RmmieSpeedShiftEvent: UsrEventId = UsrEventId(122);
	pub const CdmaCallStart: UsrEventId = UsrEventId(191);
	pub const CdmaCallEnd: UsrEventId = UsrEventId(192);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrCardType(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrCardType {
	pub const Slotempty: UsrCardType = UsrCardType(1);
	pub const Slotunknown: UsrCardType = UsrCardType(2);
	pub const Netwmgtcard: UsrCardType = UsrCardType(3);
	pub const Dualt1Nac: UsrCardType = UsrCardType(4);
	pub const Dualmodemnac: UsrCardType = UsrCardType(5);
	pub const Quadmodemnac: UsrCardType = UsrCardType(6);
	pub const Trgatewaynac: UsrCardType = UsrCardType(7);
	pub const X25Gatewaynac: UsrCardType = UsrCardType(8);
	pub const Dualv34Modemnac: UsrCardType = UsrCardType(9);
	pub const Quadv32Digitalmodemnac: UsrCardType = UsrCardType(10);
	pub const Quadv32Analogmodemnac: UsrCardType = UsrCardType(11);
	pub const Quadv32Diganlmodemnac: UsrCardType = UsrCardType(12);
	pub const Quadv34Digmodemnac: UsrCardType = UsrCardType(13);
	pub const Quadv34Anlmodemnac: UsrCardType = UsrCardType(14);
	pub const Quadv34Diganlmodemnac: UsrCardType = UsrCardType(15);
	pub const Singlet1Nac: UsrCardType = UsrCardType(16);
	pub const Ethernetgatewaynac: UsrCardType = UsrCardType(17);
	pub const Accessserver: UsrCardType = UsrCardType(18);
	pub const Four86Trgatewaynac: UsrCardType = UsrCardType(19);
	pub const Four86Ethernetgatewaynac: UsrCardType = UsrCardType(20);
	pub const Dualrs232Nac: UsrCardType = UsrCardType(22);
	pub const Four86X25Gatewaynac: UsrCardType = UsrCardType(23);
	pub const Applicationservernac: UsrCardType = UsrCardType(25);
	pub const Isdngatewaynac: UsrCardType = UsrCardType(26);
	pub const Isdnprit1Nac: UsrCardType = UsrCardType(27);
	pub const Clkednetmgtcard: UsrCardType = UsrCardType(28);
	pub const Modempoolmanagementnac: UsrCardType = UsrCardType(29);
	pub const Modempoolnetservernac: UsrCardType = UsrCardType(30);
	pub const Modempoolv34Modemnac: UsrCardType = UsrCardType(31);
	pub const Modempoolisdnnac: UsrCardType = UsrCardType(32);
	pub const Ntservernac: UsrCardType = UsrCardType(33);
	pub const Quadv34Digitalg2Nac: UsrCardType = UsrCardType(34);
	pub const Quadv34Analogg2Nac: UsrCardType = UsrCardType(35);
	pub const Quadv34Diganlgg2Nac: UsrCardType = UsrCardType(36);
	pub const Netserverframerelaynac: UsrCardType = UsrCardType(37);
	pub const Netservertokenringnac: UsrCardType = UsrCardType(38);
	pub const X2524Channelnac: UsrCardType = UsrCardType(39);
	pub const Wirelessgatewaynac: UsrCardType = UsrCardType(42);
	pub const Enhancedaccessserver: UsrCardType = UsrCardType(44);
	pub const Enhancedisdngatewaynac: UsrCardType = UsrCardType(45);
	pub const Dualt1Nic: UsrCardType = UsrCardType(1001);
	pub const Dualalogmdmnic: UsrCardType = UsrCardType(1002);
	pub const Quaddgtlmdmnic: UsrCardType = UsrCardType(1003);
	pub const Quadalogdgtlmdmnic: UsrCardType = UsrCardType(1004);
	pub const Tokenringnic: UsrCardType = UsrCardType(1005);
	pub const Singlet1Nic: UsrCardType = UsrCardType(1006);
	pub const Ethernetnic: UsrCardType = UsrCardType(1007);
	pub const Shorthauldualt1Nic: UsrCardType = UsrCardType(1008);
	pub const Dualalogmgdintlmdmnic: UsrCardType = UsrCardType(1009);
	pub const X25Nic: UsrCardType = UsrCardType(1010);
	pub const Quadalognonmgdmdmnic: UsrCardType = UsrCardType(1011);
	pub const Quadalogmgdintlmdmnic: UsrCardType = UsrCardType(1012);
	pub const Quadalognonmgdintlmdmnic: UsrCardType = UsrCardType(1013);
	pub const Quadlsdlimgdmdmnic: UsrCardType = UsrCardType(1014);
	pub const Quadlsdlinonmgdmdmnic: UsrCardType = UsrCardType(1015);
	pub const Quadlsdlimgdintlmdmnic: UsrCardType = UsrCardType(1016);
	pub const Quadlsdlinonmgdintlmdmnic: UsrCardType = UsrCardType(1017);
	pub const Hsethernetwithv35Nic: UsrCardType = UsrCardType(1018);
	pub const Hsethernetwithoutv35Nic: UsrCardType = UsrCardType(1019);
	pub const Dualhighspeedv35Nic: UsrCardType = UsrCardType(1020);
	pub const Quadv35Rs232Lowspeednic: UsrCardType = UsrCardType(1021);
	pub const Duale1Nic: UsrCardType = UsrCardType(1022);
	pub const Shorthaulduale1Nic: UsrCardType = UsrCardType(1023);
	pub const Bellcorelonghauldualt1Nic: UsrCardType = UsrCardType(1025);
	pub const Bellcoreshrthauldualt1Nic: UsrCardType = UsrCardType(1026);
	pub const Scsiedgeservernic: UsrCardType = UsrCardType(1027);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrDefaultDteDataRate(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrDefaultDteDataRate {
	pub const One10Bps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(1);
	pub const Three00Bps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(2);
	pub const Six00Bps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(3);
	pub const One200Bps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(4);
	pub const Two400Bps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(5);
	pub const Four800Bps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(6);
	pub const Seven200Bps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(7);
	pub const Nine600Bps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(8);
	pub const One2KBps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(9);
	pub const One4dot4KBps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(10);
	pub const One6dot8Bps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(11);
	pub const One9dot2KBps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(12);
	pub const Three8dot4KBps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(13);
	pub const Seven5Bps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(14);
	pub const Four50Bps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(15);
	pub const UnknownBps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(16);
	pub const Five7dot6KBps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(17);
	pub const Two1dot6KBps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(18);
	pub const Two4KBps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(19);
	pub const Two6KBps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(20);
	pub const Two8KBps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(21);
	pub const One15KBps: UsrDefaultDteDataRate = UsrDefaultDteDataRate(22);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrInitialRxLinkDataRate(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrInitialRxLinkDataRate {
	pub const One10Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(1);
	pub const Three00Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(2);
	pub const Six00Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(3);
	pub const One200Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(4);
	pub const Two400Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(5);
	pub const Four800Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(6);
	pub const Seven200Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(7);
	pub const Nine600Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(8);
	pub const One2000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(9);
	pub const One4400Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(10);
	pub const One6800Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(11);
	pub const One9200Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(12);
	pub const Three8400Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(13);
	pub const Seven5Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(14);
	pub const Four50Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(15);
	pub const UnknownBps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(16);
	pub const Five7600Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(17);
	pub const Two1600Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(18);
	pub const Two4000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(19);
	pub const Two6400Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(20);
	pub const Two8800Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(21);
	pub const One15200Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(22);
	pub const Three1200Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(23);
	pub const Three3600Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(24);
	pub const Two5333Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(25);
	pub const Two6666Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(26);
	pub const Two8000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(27);
	pub const Two9333Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(28);
	pub const Three0666Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(29);
	pub const Three2000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(30);
	pub const Three3333Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(31);
	pub const Three4666Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(32);
	pub const Three6000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(33);
	pub const Three7333Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(34);
	pub const Three8666Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(35);
	pub const Four0000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(36);
	pub const Four1333Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(37);
	pub const Four2666Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(38);
	pub const Four4000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(39);
	pub const Four5333Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(40);
	pub const Four6666Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(41);
	pub const Four8000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(42);
	pub const Four9333Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(43);
	pub const Five0666Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(44);
	pub const Five2000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(45);
	pub const Five3333Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(46);
	pub const Five4666Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(47);
	pub const Five6000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(48);
	pub const Five7333Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(49);
	pub const Five8666Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(50);
	pub const Six0000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(51);
	pub const Six1333Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(52);
	pub const Six2666Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(53);
	pub const Six4000Bps: UsrInitialRxLinkDataRate = UsrInitialRxLinkDataRate(54);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrFinalRxLinkDataRate(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrFinalRxLinkDataRate {
	pub const One10Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(1);
	pub const Three00Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(2);
	pub const Six00Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(3);
	pub const One200Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(4);
	pub const Two400Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(5);
	pub const Four800Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(6);
	pub const Seven200Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(7);
	pub const Nine600Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(8);
	pub const One2000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(9);
	pub const One4400Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(10);
	pub const One6800Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(11);
	pub const One9200Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(12);
	pub const Three8400Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(13);
	pub const Seven5Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(14);
	pub const Four50Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(15);
	pub const UnknownBps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(16);
	pub const Five7600Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(17);
	pub const Two1600Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(18);
	pub const Two4000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(19);
	pub const Two6400Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(20);
	pub const Two8800Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(21);
	pub const One15200Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(22);
	pub const Three1200Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(23);
	pub const Three3600Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(24);
	pub const Two5333Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(25);
	pub const Two6666Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(26);
	pub const Two8000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(27);
	pub const Two9333Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(28);
	pub const Three0666Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(29);
	pub const Three2000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(30);
	pub const Three3333Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(31);
	pub const Three4666Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(32);
	pub const Three6000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(33);
	pub const Three7333Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(34);
	pub const Three8666Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(35);
	pub const Four0000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(36);
	pub const Four1333Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(37);
	pub const Four2666Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(38);
	pub const Four4000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(39);
	pub const Four5333Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(40);
	pub const Four6666Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(41);
	pub const Four8000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(42);
	pub const Four9333Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(43);
	pub const Five0666Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(44);
	pub const Five2000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(45);
	pub const Five3333Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(46);
	pub const Five4666Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(47);
	pub const Five6000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(48);
	pub const Five7333Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(49);
	pub const Five8666Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(50);
	pub const Six0000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(51);
	pub const Six1333Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(52);
	pub const Six2666Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(53);
	pub const Six4000Bps: UsrFinalRxLinkDataRate = UsrFinalRxLinkDataRate(54);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrInitialTxLinkDataRate(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrInitialTxLinkDataRate {
	pub const One10Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(1);
	pub const Three00Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(2);
	pub const Six00Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(3);
	pub const One200Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(4);
	pub const Two400Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(5);
	pub const Four800Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(6);
	pub const Seven200Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(7);
	pub const Nine600Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(8);
	pub const One2000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(9);
	pub const One4400Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(10);
	pub const One6800Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(11);
	pub const One9200Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(12);
	pub const Three8400Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(13);
	pub const Seven5Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(14);
	pub const Four50Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(15);
	pub const UnknownBps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(16);
	pub const Five7600Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(17);
	pub const Two1600Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(18);
	pub const Two4000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(19);
	pub const Two6400Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(20);
	pub const Two8800Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(21);
	pub const One15200Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(22);
	pub const Three1200Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(23);
	pub const Three3600Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(24);
	pub const Two5333Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(25);
	pub const Two6666Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(26);
	pub const Two8000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(27);
	pub const Two9333Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(28);
	pub const Three0666Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(29);
	pub const Three2000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(30);
	pub const Three3333Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(31);
	pub const Three4666Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(32);
	pub const Three6000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(33);
	pub const Three7333Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(34);
	pub const Three8666Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(35);
	pub const Four0000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(36);
	pub const Four1333Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(37);
	pub const Four2666Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(38);
	pub const Four4000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(39);
	pub const Four5333Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(40);
	pub const Four6666Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(41);
	pub const Four8000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(42);
	pub const Four9333Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(43);
	pub const Five0666Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(44);
	pub const Five2000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(45);
	pub const Five3333Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(46);
	pub const Five4666Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(47);
	pub const Five6000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(48);
	pub const Five7333Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(49);
	pub const Five8666Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(50);
	pub const Six0000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(51);
	pub const Six1333Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(52);
	pub const Six2666Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(53);
	pub const Six4000Bps: UsrInitialTxLinkDataRate = UsrInitialTxLinkDataRate(54);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrFinalTxLinkDataRate(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrFinalTxLinkDataRate {
	pub const One10Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(1);
	pub const Three00Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(2);
	pub const Six00Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(3);
	pub const One200Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(4);
	pub const Two400Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(5);
	pub const Four800Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(6);
	pub const Seven200Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(7);
	pub const Nine600Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(8);
	pub const One2000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(9);
	pub const One4400Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(10);
	pub const One6800Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(11);
	pub const One9200Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(12);
	pub const Three8400Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(13);
	pub const Seven5Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(14);
	pub const Four50Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(15);
	pub const UnknownBps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(16);
	pub const Five7600Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(17);
	pub const Two1600Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(18);
	pub const Two4000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(19);
	pub const Two6400Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(20);
	pub const Two8800Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(21);
	pub const One15200Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(22);
	pub const Three1200Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(23);
	pub const Three3600Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(24);
	pub const Two5333Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(25);
	pub const Two6666Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(26);
	pub const Two8000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(27);
	pub const Two9333Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(28);
	pub const Three0666Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(29);
	pub const Three2000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(30);
	pub const Three3333Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(31);
	pub const Three4666Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(32);
	pub const Three6000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(33);
	pub const Three7333Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(34);
	pub const Three8666Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(35);
	pub const Four0000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(36);
	pub const Four1333Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(37);
	pub const Four2666Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(38);
	pub const Four4000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(39);
	pub const Four5333Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(40);
	pub const Four6666Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(41);
	pub const Four8000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(42);
	pub const Four9333Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(43);
	pub const Five0666Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(44);
	pub const Five2000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(45);
	pub const Five3333Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(46);
	pub const Five4666Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(47);
	pub const Five6000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(48);
	pub const Five7333Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(49);
	pub const Five8666Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(50);
	pub const Six0000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(51);
	pub const Six1333Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(52);
	pub const Six2666Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(53);
	pub const Six4000Bps: UsrFinalTxLinkDataRate = UsrFinalTxLinkDataRate(54);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrConnectSpeed(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrConnectSpeed {
	pub const None: UsrConnectSpeed = UsrConnectSpeed(1);
	pub const Three00Bps: UsrConnectSpeed = UsrConnectSpeed(2);
	pub const One200Bps: UsrConnectSpeed = UsrConnectSpeed(3);
	pub const Two400Bps: UsrConnectSpeed = UsrConnectSpeed(4);
	pub const Four800Bps: UsrConnectSpeed = UsrConnectSpeed(5);
	pub const Seven200Bps: UsrConnectSpeed = UsrConnectSpeed(6);
	pub const Nine600Bps: UsrConnectSpeed = UsrConnectSpeed(7);
	pub const One2000Bps: UsrConnectSpeed = UsrConnectSpeed(8);
	pub const One4400Bps: UsrConnectSpeed = UsrConnectSpeed(9);
	pub const One6800Bps: UsrConnectSpeed = UsrConnectSpeed(10);
	pub const One9200Bps: UsrConnectSpeed = UsrConnectSpeed(11);
	pub const Two1600Bps: UsrConnectSpeed = UsrConnectSpeed(12);
	pub const Two8800Bps: UsrConnectSpeed = UsrConnectSpeed(13);
	pub const Three8400Bps: UsrConnectSpeed = UsrConnectSpeed(14);
	pub const Five7600Bps: UsrConnectSpeed = UsrConnectSpeed(15);
	pub const One15200Bps: UsrConnectSpeed = UsrConnectSpeed(16);
	pub const Two88000Bps: UsrConnectSpeed = UsrConnectSpeed(17);
	pub const Seven51200Bps: UsrConnectSpeed = UsrConnectSpeed(18);
	pub const One20075Bps: UsrConnectSpeed = UsrConnectSpeed(19);
	pub const Two4000Bps: UsrConnectSpeed = UsrConnectSpeed(20);
	pub const Two6400Bps: UsrConnectSpeed = UsrConnectSpeed(21);
	pub const Three1200Bps: UsrConnectSpeed = UsrConnectSpeed(22);
	pub const Three3600Bps: UsrConnectSpeed = UsrConnectSpeed(23);
	pub const Three3333Bps: UsrConnectSpeed = UsrConnectSpeed(24);
	pub const Three7333Bps: UsrConnectSpeed = UsrConnectSpeed(25);
	pub const Four1333Bps: UsrConnectSpeed = UsrConnectSpeed(26);
	pub const Four2666Bps: UsrConnectSpeed = UsrConnectSpeed(27);
	pub const Four4000Bps: UsrConnectSpeed = UsrConnectSpeed(28);
	pub const Four5333Bps: UsrConnectSpeed = UsrConnectSpeed(29);
	pub const Four6666Bps: UsrConnectSpeed = UsrConnectSpeed(30);
	pub const Four8000Bps: UsrConnectSpeed = UsrConnectSpeed(31);
	pub const Four9333Bps: UsrConnectSpeed = UsrConnectSpeed(32);
	pub const Five0666Bps: UsrConnectSpeed = UsrConnectSpeed(33);
	pub const Five2000Bps: UsrConnectSpeed = UsrConnectSpeed(34);
	pub const Five3333Bps: UsrConnectSpeed = UsrConnectSpeed(35);
	pub const Five4666Bps: UsrConnectSpeed = UsrConnectSpeed(36);
	pub const Five6000Bps: UsrConnectSpeed = UsrConnectSpeed(37);
	pub const Five7333Bps: UsrConnectSpeed = UsrConnectSpeed(38);
	pub const Six4000Bps: UsrConnectSpeed = UsrConnectSpeed(39);
	pub const Two5333Bps: UsrConnectSpeed = UsrConnectSpeed(40);
	pub const Two6666Bps: UsrConnectSpeed = UsrConnectSpeed(41);
	pub const Two8000Bps: UsrConnectSpeed = UsrConnectSpeed(42);
	pub const Two9333Bps: UsrConnectSpeed = UsrConnectSpeed(43);
	pub const Three0666Bps: UsrConnectSpeed = UsrConnectSpeed(44);
	pub const Three2000Bps: UsrConnectSpeed = UsrConnectSpeed(45);
	pub const Three4666Bps: UsrConnectSpeed = UsrConnectSpeed(46);
	pub const Three6000Bps: UsrConnectSpeed = UsrConnectSpeed(47);
	pub const Three8666Bps: UsrConnectSpeed = UsrConnectSpeed(48);
	pub const Four0000Bps: UsrConnectSpeed = UsrConnectSpeed(49);
	pub const Five8666Bps: UsrConnectSpeed = UsrConnectSpeed(50);
	pub const Six0000Bps: UsrConnectSpeed = UsrConnectSpeed(51);
	pub const Six1333Bps: UsrConnectSpeed = UsrConnectSpeed(52);
	pub const Six2666Bps: UsrConnectSpeed = UsrConnectSpeed(53);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrSyncAsyncMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrSyncAsyncMode {
	pub const Asynchronous: UsrSyncAsyncMode = UsrSyncAsyncMode(1);
	pub const Synchronous: UsrSyncAsyncMode = UsrSyncAsyncMode(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrOriginateAnswerMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrOriginateAnswerMode {
	pub const OriginateInOriginateMode: UsrOriginateAnswerMode = UsrOriginateAnswerMode(1);
	pub const OriginateInAnswerMode: UsrOriginateAnswerMode = UsrOriginateAnswerMode(2);
	pub const AnswerInOriginateMode: UsrOriginateAnswerMode = UsrOriginateAnswerMode(3);
	pub const AnswerInAnswerMode: UsrOriginateAnswerMode = UsrOriginateAnswerMode(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrModulationType(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrModulationType {
	pub const Usroboticshst: UsrModulationType = UsrModulationType(1);
	pub const Ccittv32: UsrModulationType = UsrModulationType(2);
	pub const Ccittv22Bis: UsrModulationType = UsrModulationType(3);
	pub const Bell103: UsrModulationType = UsrModulationType(4);
	pub const Ccittv21: UsrModulationType = UsrModulationType(5);
	pub const Bell212: UsrModulationType = UsrModulationType(6);
	pub const Ccittv32Bis: UsrModulationType = UsrModulationType(7);
	pub const Ccittv23: UsrModulationType = UsrModulationType(8);
	pub const Negotiationfailed: UsrModulationType = UsrModulationType(9);
	pub const Bell208B: UsrModulationType = UsrModulationType(10);
	pub const V21Faxclass1: UsrModulationType = UsrModulationType(11);
	pub const V27Faxclass1: UsrModulationType = UsrModulationType(12);
	pub const V29Faxclass1: UsrModulationType = UsrModulationType(13);
	pub const V17Faxclass1: UsrModulationType = UsrModulationType(14);
	pub const V21Faxclass2: UsrModulationType = UsrModulationType(15);
	pub const V27Faxclass2: UsrModulationType = UsrModulationType(16);
	pub const V29Faxclass2: UsrModulationType = UsrModulationType(17);
	pub const V17Faxclass2: UsrModulationType = UsrModulationType(18);
	pub const V32Terbo: UsrModulationType = UsrModulationType(19);
	pub const V34: UsrModulationType = UsrModulationType(20);
	pub const Vfc: UsrModulationType = UsrModulationType(21);
	pub const V34Plus: UsrModulationType = UsrModulationType(22);
	pub const X2: UsrModulationType = UsrModulationType(23);
	pub const V110: UsrModulationType = UsrModulationType(24);
	pub const V120: UsrModulationType = UsrModulationType(25);
	pub const X75: UsrModulationType = UsrModulationType(26);
	pub const Asyncsyncppp: UsrModulationType = UsrModulationType(27);
	pub const Clearchannel: UsrModulationType = UsrModulationType(28);
	pub const X2Client: UsrModulationType = UsrModulationType(29);
	pub const X2Symmetric: UsrModulationType = UsrModulationType(30);
	pub const Piafs: UsrModulationType = UsrModulationType(31);
	pub const X2Version2: UsrModulationType = UsrModulationType(32);
	pub const V90Analog: UsrModulationType = UsrModulationType(33);
	pub const V90Digital: UsrModulationType = UsrModulationType(34);
	pub const V90Alldigital: UsrModulationType = UsrModulationType(35);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitialModulationType(pub u32);
 
#[allow(non_upper_case_globals)]
impl InitialModulationType {
	pub const Usroboticshst: InitialModulationType = InitialModulationType(1);
	pub const Ccittv32: InitialModulationType = InitialModulationType(2);
	pub const Ccittv22Bis: InitialModulationType = InitialModulationType(3);
	pub const Bell103: InitialModulationType = InitialModulationType(4);
	pub const Ccittv21: InitialModulationType = InitialModulationType(5);
	pub const Bell212: InitialModulationType = InitialModulationType(6);
	pub const Ccittv32Bis: InitialModulationType = InitialModulationType(7);
	pub const Ccittv23: InitialModulationType = InitialModulationType(8);
	pub const Negotiationfailed: InitialModulationType = InitialModulationType(9);
	pub const Bell208B: InitialModulationType = InitialModulationType(10);
	pub const V21Faxclass1: InitialModulationType = InitialModulationType(11);
	pub const V27Faxclass1: InitialModulationType = InitialModulationType(12);
	pub const V29Faxclass1: InitialModulationType = InitialModulationType(13);
	pub const V17Faxclass1: InitialModulationType = InitialModulationType(14);
	pub const V21Faxclass2: InitialModulationType = InitialModulationType(15);
	pub const V27Faxclass2: InitialModulationType = InitialModulationType(16);
	pub const V29Faxclass2: InitialModulationType = InitialModulationType(17);
	pub const V17Faxclass2: InitialModulationType = InitialModulationType(18);
	pub const V32Terbo: InitialModulationType = InitialModulationType(19);
	pub const V34: InitialModulationType = InitialModulationType(20);
	pub const Vfc: InitialModulationType = InitialModulationType(21);
	pub const V34Plus: InitialModulationType = InitialModulationType(22);
	pub const X2: InitialModulationType = InitialModulationType(23);
	pub const V110: InitialModulationType = InitialModulationType(24);
	pub const V120: InitialModulationType = InitialModulationType(25);
	pub const X75: InitialModulationType = InitialModulationType(26);
	pub const Asyncsyncppp: InitialModulationType = InitialModulationType(27);
	pub const Clearchannel: InitialModulationType = InitialModulationType(28);
	pub const X2Client: InitialModulationType = InitialModulationType(29);
	pub const X2Symmetric: InitialModulationType = InitialModulationType(30);
	pub const Piafs: InitialModulationType = InitialModulationType(31);
	pub const X2Version2: InitialModulationType = InitialModulationType(32);
	pub const V90Analogue: InitialModulationType = InitialModulationType(33);
	pub const V90Digital: InitialModulationType = InitialModulationType(34);
	pub const V90Alldigital: InitialModulationType = InitialModulationType(35);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrConnectTermReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrConnectTermReason {
	pub const Dtrdrop: UsrConnectTermReason = UsrConnectTermReason(1);
	pub const Escapesequence: UsrConnectTermReason = UsrConnectTermReason(2);
	pub const Athcommand: UsrConnectTermReason = UsrConnectTermReason(3);
	pub const Carrierloss: UsrConnectTermReason = UsrConnectTermReason(4);
	pub const Inactivitytimout: UsrConnectTermReason = UsrConnectTermReason(5);
	pub const Mnpincompatible: UsrConnectTermReason = UsrConnectTermReason(6);
	pub const Undefined: UsrConnectTermReason = UsrConnectTermReason(7);
	pub const Remotepassword: UsrConnectTermReason = UsrConnectTermReason(8);
	pub const Linkpassword: UsrConnectTermReason = UsrConnectTermReason(9);
	pub const Retransmitlimit: UsrConnectTermReason = UsrConnectTermReason(10);
	pub const Linkdisconnectmsgreceived: UsrConnectTermReason = UsrConnectTermReason(11);
	pub const Noloopcurrent: UsrConnectTermReason = UsrConnectTermReason(12);
	pub const Invalidspeed: UsrConnectTermReason = UsrConnectTermReason(13);
	pub const Unabletoretrain: UsrConnectTermReason = UsrConnectTermReason(14);
	pub const Managementcommand: UsrConnectTermReason = UsrConnectTermReason(15);
	pub const Nodialtone: UsrConnectTermReason = UsrConnectTermReason(16);
	pub const Keyabort: UsrConnectTermReason = UsrConnectTermReason(17);
	pub const Linebusy: UsrConnectTermReason = UsrConnectTermReason(18);
	pub const Noanswer: UsrConnectTermReason = UsrConnectTermReason(19);
	pub const Voice: UsrConnectTermReason = UsrConnectTermReason(20);
	pub const Noanswertone: UsrConnectTermReason = UsrConnectTermReason(21);
	pub const Nocarrier: UsrConnectTermReason = UsrConnectTermReason(22);
	pub const Undetermined: UsrConnectTermReason = UsrConnectTermReason(23);
	pub const V42Sabmetimeout: UsrConnectTermReason = UsrConnectTermReason(24);
	pub const V42Breaktimeout: UsrConnectTermReason = UsrConnectTermReason(25);
	pub const V42Disconnectcmd: UsrConnectTermReason = UsrConnectTermReason(26);
	pub const V42Idexchangefail: UsrConnectTermReason = UsrConnectTermReason(27);
	pub const V42Badsetup: UsrConnectTermReason = UsrConnectTermReason(28);
	pub const V42Invalidcodeword: UsrConnectTermReason = UsrConnectTermReason(29);
	pub const V42Stringtolong: UsrConnectTermReason = UsrConnectTermReason(30);
	pub const V42Invalidcommand: UsrConnectTermReason = UsrConnectTermReason(31);
	pub const None: UsrConnectTermReason = UsrConnectTermReason(32);
	pub const V32Cleardown: UsrConnectTermReason = UsrConnectTermReason(33);
	pub const Dialsecurity: UsrConnectTermReason = UsrConnectTermReason(34);
	pub const Remoteaccessdenied: UsrConnectTermReason = UsrConnectTermReason(35);
	pub const Looploss: UsrConnectTermReason = UsrConnectTermReason(36);
	pub const Ds0Teardown: UsrConnectTermReason = UsrConnectTermReason(37);
	pub const Promptnotenabled: UsrConnectTermReason = UsrConnectTermReason(38);
	pub const Nopromptinginsync: UsrConnectTermReason = UsrConnectTermReason(39);
	pub const Nonarqmode: UsrConnectTermReason = UsrConnectTermReason(40);
	pub const Modeincompatible: UsrConnectTermReason = UsrConnectTermReason(41);
	pub const Nopromptinnonarq: UsrConnectTermReason = UsrConnectTermReason(42);
	pub const Dialbacklink: UsrConnectTermReason = UsrConnectTermReason(43);
	pub const Linkabort: UsrConnectTermReason = UsrConnectTermReason(44);
	pub const Autopassfailed: UsrConnectTermReason = UsrConnectTermReason(45);
	pub const Pbgenericerror: UsrConnectTermReason = UsrConnectTermReason(46);
	pub const Pblinkerrtxpreack: UsrConnectTermReason = UsrConnectTermReason(47);
	pub const Pblinkerrtxtardyack: UsrConnectTermReason = UsrConnectTermReason(48);
	pub const Pbtransmitbustimeout: UsrConnectTermReason = UsrConnectTermReason(49);
	pub const Pbreceivebustimeout: UsrConnectTermReason = UsrConnectTermReason(50);
	pub const Pblinkerrtxtal: UsrConnectTermReason = UsrConnectTermReason(51);
	pub const Pblinkerrrxtal: UsrConnectTermReason = UsrConnectTermReason(52);
	pub const Pbtransmitmastertimeout: UsrConnectTermReason = UsrConnectTermReason(53);
	pub const Pbclockmissing: UsrConnectTermReason = UsrConnectTermReason(54);
	pub const Pbreceivedlswhilelinkup: UsrConnectTermReason = UsrConnectTermReason(55);
	pub const Pboutofsequenceframe: UsrConnectTermReason = UsrConnectTermReason(56);
	pub const Pbbadframe: UsrConnectTermReason = UsrConnectTermReason(57);
	pub const Pbackwaittimeout: UsrConnectTermReason = UsrConnectTermReason(58);
	pub const Pbreceivedackseqerr: UsrConnectTermReason = UsrConnectTermReason(59);
	pub const Pbreceiveovrflwrnrfail: UsrConnectTermReason = UsrConnectTermReason(60);
	pub const Pbreceivemsgbufovrflw: UsrConnectTermReason = UsrConnectTermReason(61);
	pub const Rcvdgatewaydisccmd: UsrConnectTermReason = UsrConnectTermReason(62);
	pub const Tokenpassingtimeout: UsrConnectTermReason = UsrConnectTermReason(63);
	pub const Dspinterrupttimeout: UsrConnectTermReason = UsrConnectTermReason(64);
	pub const Mnpprotocolviolation: UsrConnectTermReason = UsrConnectTermReason(65);
	pub const Class2Faxhangupcmd: UsrConnectTermReason = UsrConnectTermReason(66);
	pub const Hstspeedswitchtimeout: UsrConnectTermReason = UsrConnectTermReason(67);
	pub const Toomanyunacked: UsrConnectTermReason = UsrConnectTermReason(68);
	pub const Timerexpired: UsrConnectTermReason = UsrConnectTermReason(69);
	pub const T1Glare: UsrConnectTermReason = UsrConnectTermReason(70);
	pub const Pridialoutrqtimeout: UsrConnectTermReason = UsrConnectTermReason(71);
	pub const Abortanlgdstovrisdn: UsrConnectTermReason = UsrConnectTermReason(72);
	pub const Normalusercallclear: UsrConnectTermReason = UsrConnectTermReason(73);
	pub const Normalunspecified: UsrConnectTermReason = UsrConnectTermReason(74);
	pub const Bearerincompatibility: UsrConnectTermReason = UsrConnectTermReason(75);
	pub const Protocolerrorevent: UsrConnectTermReason = UsrConnectTermReason(76);
	pub const Abnormaldisconnect: UsrConnectTermReason = UsrConnectTermReason(77);
	pub const Invalidcausevalue: UsrConnectTermReason = UsrConnectTermReason(78);
	pub const Resourceunavailable: UsrConnectTermReason = UsrConnectTermReason(79);
	pub const Remotehungupduringtraining: UsrConnectTermReason = UsrConnectTermReason(80);
	pub const Trainingtimeout: UsrConnectTermReason = UsrConnectTermReason(81);
	pub const Incomingmodemnotavailable: UsrConnectTermReason = UsrConnectTermReason(82);
	pub const Incominginvalidbearercap: UsrConnectTermReason = UsrConnectTermReason(83);
	pub const Incominginvalidchannelid: UsrConnectTermReason = UsrConnectTermReason(84);
	pub const Incominginvalidprogind: UsrConnectTermReason = UsrConnectTermReason(85);
	pub const Incominginvalidcallingpty: UsrConnectTermReason = UsrConnectTermReason(86);
	pub const Incominginvalidcalledpty: UsrConnectTermReason = UsrConnectTermReason(87);
	pub const Incomingcallblock: UsrConnectTermReason = UsrConnectTermReason(88);
	pub const Incomingloopstnoringoff: UsrConnectTermReason = UsrConnectTermReason(89);
	pub const Outgoingtelcodisconnect: UsrConnectTermReason = UsrConnectTermReason(90);
	pub const Outgoingemwinktimeout: UsrConnectTermReason = UsrConnectTermReason(91);
	pub const Outgoingemwinktooshort: UsrConnectTermReason = UsrConnectTermReason(92);
	pub const Outgoingnochannelavail: UsrConnectTermReason = UsrConnectTermReason(93);
	pub const Dspreboot: UsrConnectTermReason = UsrConnectTermReason(94);
	pub const Nodspresptoka: UsrConnectTermReason = UsrConnectTermReason(95);
	pub const Nodspresptodisc: UsrConnectTermReason = UsrConnectTermReason(96);
	pub const Dsptailptrinvalid: UsrConnectTermReason = UsrConnectTermReason(97);
	pub const Dspheadptrinvalid: UsrConnectTermReason = UsrConnectTermReason(98);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrFailureToConnectReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrFailureToConnectReason {
	pub const Dtrdrop: UsrFailureToConnectReason = UsrFailureToConnectReason(1);
	pub const Escapesequence: UsrFailureToConnectReason = UsrFailureToConnectReason(2);
	pub const Athcommand: UsrFailureToConnectReason = UsrFailureToConnectReason(3);
	pub const Carrierloss: UsrFailureToConnectReason = UsrFailureToConnectReason(4);
	pub const Inactivitytimout: UsrFailureToConnectReason = UsrFailureToConnectReason(5);
	pub const Mnpincompatible: UsrFailureToConnectReason = UsrFailureToConnectReason(6);
	pub const Undefined: UsrFailureToConnectReason = UsrFailureToConnectReason(7);
	pub const Remotepassword: UsrFailureToConnectReason = UsrFailureToConnectReason(8);
	pub const Linkpassword: UsrFailureToConnectReason = UsrFailureToConnectReason(9);
	pub const Retransmitlimit: UsrFailureToConnectReason = UsrFailureToConnectReason(10);
	pub const Linkdisconnectmsgrec: UsrFailureToConnectReason = UsrFailureToConnectReason(11);
	pub const Noloopcurrent: UsrFailureToConnectReason = UsrFailureToConnectReason(12);
	pub const Invalidspeed: UsrFailureToConnectReason = UsrFailureToConnectReason(13);
	pub const Unabletoretrain: UsrFailureToConnectReason = UsrFailureToConnectReason(14);
	pub const Managementcommand: UsrFailureToConnectReason = UsrFailureToConnectReason(15);
	pub const Nodialtone: UsrFailureToConnectReason = UsrFailureToConnectReason(16);
	pub const Keyabort: UsrFailureToConnectReason = UsrFailureToConnectReason(17);
	pub const Linebusy: UsrFailureToConnectReason = UsrFailureToConnectReason(18);
	pub const Noanswer: UsrFailureToConnectReason = UsrFailureToConnectReason(19);
	pub const Voice: UsrFailureToConnectReason = UsrFailureToConnectReason(20);
	pub const Noanswertone: UsrFailureToConnectReason = UsrFailureToConnectReason(21);
	pub const Nocarrier: UsrFailureToConnectReason = UsrFailureToConnectReason(22);
	pub const Undetermined: UsrFailureToConnectReason = UsrFailureToConnectReason(23);
	pub const V42Sabmetimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(24);
	pub const V42Breaktimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(25);
	pub const V42Disconnectcmd: UsrFailureToConnectReason = UsrFailureToConnectReason(26);
	pub const V42Idexchangefail: UsrFailureToConnectReason = UsrFailureToConnectReason(27);
	pub const V42Badsetup: UsrFailureToConnectReason = UsrFailureToConnectReason(28);
	pub const V42Invalidcodeword: UsrFailureToConnectReason = UsrFailureToConnectReason(29);
	pub const V42Stringtolong: UsrFailureToConnectReason = UsrFailureToConnectReason(30);
	pub const V42Invalidcommand: UsrFailureToConnectReason = UsrFailureToConnectReason(31);
	pub const None: UsrFailureToConnectReason = UsrFailureToConnectReason(32);
	pub const V32Cleardown: UsrFailureToConnectReason = UsrFailureToConnectReason(33);
	pub const Dialsecurity: UsrFailureToConnectReason = UsrFailureToConnectReason(34);
	pub const Remoteaccessdenied: UsrFailureToConnectReason = UsrFailureToConnectReason(35);
	pub const Looploss: UsrFailureToConnectReason = UsrFailureToConnectReason(36);
	pub const Ds0Teardown: UsrFailureToConnectReason = UsrFailureToConnectReason(37);
	pub const Promptnotenabled: UsrFailureToConnectReason = UsrFailureToConnectReason(38);
	pub const Nopromptinginsync: UsrFailureToConnectReason = UsrFailureToConnectReason(39);
	pub const Nonarqmode: UsrFailureToConnectReason = UsrFailureToConnectReason(40);
	pub const Modeincompatible: UsrFailureToConnectReason = UsrFailureToConnectReason(41);
	pub const Nopromptinnonarq: UsrFailureToConnectReason = UsrFailureToConnectReason(42);
	pub const Dialbacklink: UsrFailureToConnectReason = UsrFailureToConnectReason(43);
	pub const Linkabort: UsrFailureToConnectReason = UsrFailureToConnectReason(44);
	pub const Autopassfailed: UsrFailureToConnectReason = UsrFailureToConnectReason(45);
	pub const Pbgenericerror: UsrFailureToConnectReason = UsrFailureToConnectReason(46);
	pub const Pblinkerrtxpreack: UsrFailureToConnectReason = UsrFailureToConnectReason(47);
	pub const Pblinkerrtxtardyack: UsrFailureToConnectReason = UsrFailureToConnectReason(48);
	pub const Pbtransmitbustimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(49);
	pub const Pbreceivebustimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(50);
	pub const Pblinkerrtxtal: UsrFailureToConnectReason = UsrFailureToConnectReason(51);
	pub const Pblinkerrrxtal: UsrFailureToConnectReason = UsrFailureToConnectReason(52);
	pub const Pbtransmitmastertimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(53);
	pub const Pbclockmissing: UsrFailureToConnectReason = UsrFailureToConnectReason(54);
	pub const Pbreceivedlswhilelinkup: UsrFailureToConnectReason = UsrFailureToConnectReason(55);
	pub const Pboutofsequenceframe: UsrFailureToConnectReason = UsrFailureToConnectReason(56);
	pub const Pbbadframe: UsrFailureToConnectReason = UsrFailureToConnectReason(57);
	pub const Pbackwaittimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(58);
	pub const Pbreceivedackseqerr: UsrFailureToConnectReason = UsrFailureToConnectReason(59);
	pub const Pbreceiveovrflwrnrfail: UsrFailureToConnectReason = UsrFailureToConnectReason(60);
	pub const Pbreceivemsgbufovrflw: UsrFailureToConnectReason = UsrFailureToConnectReason(61);
	pub const Rcvdgatewaydisccmd: UsrFailureToConnectReason = UsrFailureToConnectReason(62);
	pub const Tokenpassingtimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(63);
	pub const Dspinterrupttimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(64);
	pub const Mnpprotocolviolation: UsrFailureToConnectReason = UsrFailureToConnectReason(65);
	pub const Class2Faxhangupcmd: UsrFailureToConnectReason = UsrFailureToConnectReason(66);
	pub const Hstspeedswitchtimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(67);
	pub const Toomanyunacked: UsrFailureToConnectReason = UsrFailureToConnectReason(68);
	pub const Timerexpired: UsrFailureToConnectReason = UsrFailureToConnectReason(69);
	pub const T1Glare: UsrFailureToConnectReason = UsrFailureToConnectReason(70);
	pub const Pridialoutrqtimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(71);
	pub const Abortanlgdstovrisdn: UsrFailureToConnectReason = UsrFailureToConnectReason(72);
	pub const Normalusercallclear: UsrFailureToConnectReason = UsrFailureToConnectReason(73);
	pub const Normalunspecified: UsrFailureToConnectReason = UsrFailureToConnectReason(74);
	pub const Bearerincompatibility: UsrFailureToConnectReason = UsrFailureToConnectReason(75);
	pub const Protocolerrorevent: UsrFailureToConnectReason = UsrFailureToConnectReason(76);
	pub const Abnormaldisconnect: UsrFailureToConnectReason = UsrFailureToConnectReason(77);
	pub const Invalidcausevalue: UsrFailureToConnectReason = UsrFailureToConnectReason(78);
	pub const Resourceunavailable: UsrFailureToConnectReason = UsrFailureToConnectReason(79);
	pub const Remotehungupduringtraining: UsrFailureToConnectReason = UsrFailureToConnectReason(80);
	pub const Trainingtimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(81);
	pub const Incomingmodemnotavailable: UsrFailureToConnectReason = UsrFailureToConnectReason(82);
	pub const Incominginvalidbearercap: UsrFailureToConnectReason = UsrFailureToConnectReason(83);
	pub const Incominginvalidchannelid: UsrFailureToConnectReason = UsrFailureToConnectReason(84);
	pub const Incominginvalidprogind: UsrFailureToConnectReason = UsrFailureToConnectReason(85);
	pub const Incominginvalidcallingpty: UsrFailureToConnectReason = UsrFailureToConnectReason(86);
	pub const Incominginvalidcalledpty: UsrFailureToConnectReason = UsrFailureToConnectReason(87);
	pub const Incomingcallblock: UsrFailureToConnectReason = UsrFailureToConnectReason(88);
	pub const Incomingloopstnoringoff: UsrFailureToConnectReason = UsrFailureToConnectReason(89);
	pub const Outgoingtelcodisconnect: UsrFailureToConnectReason = UsrFailureToConnectReason(90);
	pub const Outgoingemwinktimeout: UsrFailureToConnectReason = UsrFailureToConnectReason(91);
	pub const Outgoingemwinktooshort: UsrFailureToConnectReason = UsrFailureToConnectReason(92);
	pub const Outgoingnochannelavail: UsrFailureToConnectReason = UsrFailureToConnectReason(93);
	pub const Dspreboot: UsrFailureToConnectReason = UsrFailureToConnectReason(94);
	pub const Nodspresptoka: UsrFailureToConnectReason = UsrFailureToConnectReason(95);
	pub const Nodspresptodisc: UsrFailureToConnectReason = UsrFailureToConnectReason(96);
	pub const Dsptailptrinvalid: UsrFailureToConnectReason = UsrFailureToConnectReason(97);
	pub const Dspheadptrinvalid: UsrFailureToConnectReason = UsrFailureToConnectReason(98);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrSimplifiedMnpLevels(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrSimplifiedMnpLevels {
	pub const None: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(1);
	pub const Mnplevel3: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(2);
	pub const Mnplevel4: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(3);
	pub const Ccittv42: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(4);
	pub const Usroboticshst: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(5);
	pub const Synchronousnone: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(6);
	pub const Mnplevel2: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(7);
	pub const Mnp10: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(8);
	pub const V42Etc: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(9);
	pub const Mnp10Etc: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(10);
	pub const Lapmetc: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(11);
	pub const V42Etc2: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(12);
	pub const V42Srej: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(13);
	pub const Piafs: UsrSimplifiedMnpLevels = UsrSimplifiedMnpLevels(14);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrSimplifiedV42BisUsage(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrSimplifiedV42BisUsage {
	pub const None: UsrSimplifiedV42BisUsage = UsrSimplifiedV42BisUsage(1);
	pub const Ccittv42Bis: UsrSimplifiedV42BisUsage = UsrSimplifiedV42BisUsage(2);
	pub const Mnplevel5: UsrSimplifiedV42BisUsage = UsrSimplifiedV42BisUsage(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrEqualizationType(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrEqualizationType {
	pub const Long: UsrEqualizationType = UsrEqualizationType(1);
	pub const Short: UsrEqualizationType = UsrEqualizationType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrFallbackEnabled(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrFallbackEnabled {
	pub const Disabled: UsrFallbackEnabled = UsrFallbackEnabled(1);
	pub const Enabled: UsrFallbackEnabled = UsrFallbackEnabled(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrBackChannelDataRate(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrBackChannelDataRate {
	pub const Four50Bps: UsrBackChannelDataRate = UsrBackChannelDataRate(1);
	pub const Three00Bps: UsrBackChannelDataRate = UsrBackChannelDataRate(2);
	pub const None: UsrBackChannelDataRate = UsrBackChannelDataRate(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrDeviceConnectedTo(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrDeviceConnectedTo {
	pub const None: UsrDeviceConnectedTo = UsrDeviceConnectedTo(1);
	pub const Isdngateway: UsrDeviceConnectedTo = UsrDeviceConnectedTo(2);
	pub const Quadmodem: UsrDeviceConnectedTo = UsrDeviceConnectedTo(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrCallEventCode(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrCallEventCode {
	pub const Notsupported: UsrCallEventCode = UsrCallEventCode(1);
	pub const Setup: UsrCallEventCode = UsrCallEventCode(2);
	pub const Usrsetup: UsrCallEventCode = UsrCallEventCode(3);
	pub const Telcodisconnect: UsrCallEventCode = UsrCallEventCode(4);
	pub const Usrdisconnect: UsrCallEventCode = UsrCallEventCode(5);
	pub const Nofreemodem: UsrCallEventCode = UsrCallEventCode(6);
	pub const Modemsnotallowed: UsrCallEventCode = UsrCallEventCode(7);
	pub const Modemsrejectcall: UsrCallEventCode = UsrCallEventCode(8);
	pub const Modemsetuptimeout: UsrCallEventCode = UsrCallEventCode(9);
	pub const Nofreeigw: UsrCallEventCode = UsrCallEventCode(10);
	pub const Igwrejectcall: UsrCallEventCode = UsrCallEventCode(11);
	pub const Igwsetuptimeout: UsrCallEventCode = UsrCallEventCode(12);
	pub const Nofreetdmts: UsrCallEventCode = UsrCallEventCode(13);
	pub const Bcreject: UsrCallEventCode = UsrCallEventCode(14);
	pub const Iereject: UsrCallEventCode = UsrCallEventCode(15);
	pub const Chidreject: UsrCallEventCode = UsrCallEventCode(16);
	pub const Progreject: UsrCallEventCode = UsrCallEventCode(17);
	pub const Callingpartyreject: UsrCallEventCode = UsrCallEventCode(18);
	pub const Calledpartyreject: UsrCallEventCode = UsrCallEventCode(19);
	pub const Blocked: UsrCallEventCode = UsrCallEventCode(20);
	pub const Analogblocked: UsrCallEventCode = UsrCallEventCode(21);
	pub const Digitalblocked: UsrCallEventCode = UsrCallEventCode(22);
	pub const Outofservice: UsrCallEventCode = UsrCallEventCode(23);
	pub const Busy: UsrCallEventCode = UsrCallEventCode(24);
	pub const Congestion: UsrCallEventCode = UsrCallEventCode(25);
	pub const Protocolerror: UsrCallEventCode = UsrCallEventCode(26);
	pub const Nofreebchannel: UsrCallEventCode = UsrCallEventCode(27);
	pub const Inoutcallcollision: UsrCallEventCode = UsrCallEventCode(28);
	pub const Incallarrival: UsrCallEventCode = UsrCallEventCode(29);
	pub const Outcallarrival: UsrCallEventCode = UsrCallEventCode(30);
	pub const Incallconnect: UsrCallEventCode = UsrCallEventCode(31);
	pub const Outcallconnect: UsrCallEventCode = UsrCallEventCode(32);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrHarcDisconnectCode(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrHarcDisconnectCode {
	pub const NoError: UsrHarcDisconnectCode = UsrHarcDisconnectCode(0);
	pub const NoCarrier: UsrHarcDisconnectCode = UsrHarcDisconnectCode(1);
	pub const NoDsr: UsrHarcDisconnectCode = UsrHarcDisconnectCode(2);
	pub const Timeout: UsrHarcDisconnectCode = UsrHarcDisconnectCode(3);
	pub const Reset: UsrHarcDisconnectCode = UsrHarcDisconnectCode(4);
	pub const CallDropReq: UsrHarcDisconnectCode = UsrHarcDisconnectCode(5);
	pub const IdleTimeout: UsrHarcDisconnectCode = UsrHarcDisconnectCode(6);
	pub const SessionTimeout: UsrHarcDisconnectCode = UsrHarcDisconnectCode(7);
	pub const UserReqDrop: UsrHarcDisconnectCode = UsrHarcDisconnectCode(8);
	pub const HostReqDrop: UsrHarcDisconnectCode = UsrHarcDisconnectCode(9);
	pub const ServiceInterruption: UsrHarcDisconnectCode = UsrHarcDisconnectCode(10);
	pub const ServiceUnavailable: UsrHarcDisconnectCode = UsrHarcDisconnectCode(11);
	pub const UserInputError: UsrHarcDisconnectCode = UsrHarcDisconnectCode(12);
	pub const NasDropForCallback: UsrHarcDisconnectCode = UsrHarcDisconnectCode(13);
	pub const NasDropMiscNonError: UsrHarcDisconnectCode = UsrHarcDisconnectCode(14);
	pub const NasInternalError: UsrHarcDisconnectCode = UsrHarcDisconnectCode(15);
	pub const LineBusy: UsrHarcDisconnectCode = UsrHarcDisconnectCode(16);
	pub const TunnelTermUnreach: UsrHarcDisconnectCode = UsrHarcDisconnectCode(19);
	pub const TunnelRefused: UsrHarcDisconnectCode = UsrHarcDisconnectCode(20);
	pub const TunnelAuthFailed: UsrHarcDisconnectCode = UsrHarcDisconnectCode(21);
	pub const TunnelSessionTimeout: UsrHarcDisconnectCode = UsrHarcDisconnectCode(22);
	pub const TunnelTimeout: UsrHarcDisconnectCode = UsrHarcDisconnectCode(23);
	pub const RadiusResReclaim: UsrHarcDisconnectCode = UsrHarcDisconnectCode(25);
	pub const DnisAuthFailed: UsrHarcDisconnectCode = UsrHarcDisconnectCode(26);
	pub const PapAuthFailure: UsrHarcDisconnectCode = UsrHarcDisconnectCode(27);
	pub const ChapAuthFailure: UsrHarcDisconnectCode = UsrHarcDisconnectCode(28);
	pub const PppLcpFailed: UsrHarcDisconnectCode = UsrHarcDisconnectCode(29);
	pub const PppNcpFailed: UsrHarcDisconnectCode = UsrHarcDisconnectCode(30);
	pub const RadiusTimeout: UsrHarcDisconnectCode = UsrHarcDisconnectCode(31);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrCcpAlgorithm(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrCcpAlgorithm {
	pub const None: UsrCcpAlgorithm = UsrCcpAlgorithm(1);
	pub const Stac: UsrCcpAlgorithm = UsrCcpAlgorithm(2);
	pub const Ms: UsrCcpAlgorithm = UsrCcpAlgorithm(3);
	pub const Any: UsrCcpAlgorithm = UsrCcpAlgorithm(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrTunnelSecurity(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrTunnelSecurity {
	pub const None: UsrTunnelSecurity = UsrTunnelSecurity(0);
	pub const ControlOnly: UsrTunnelSecurity = UsrTunnelSecurity(1);
	pub const DataOnly: UsrTunnelSecurity = UsrTunnelSecurity(2);
	pub const BothDataAndControl: UsrTunnelSecurity = UsrTunnelSecurity(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrRmmieStatus(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrRmmieStatus {
	pub const Notenabledinlocalmodem: UsrRmmieStatus = UsrRmmieStatus(1);
	pub const Notdetectedinremotemodem: UsrRmmieStatus = UsrRmmieStatus(2);
	pub const Ok: UsrRmmieStatus = UsrRmmieStatus(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrRmmieX2Status(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrRmmieX2Status {
	pub const Notoperational: UsrRmmieX2Status = UsrRmmieX2Status(1);
	pub const Operational: UsrRmmieX2Status = UsrRmmieX2Status(2);
	pub const X2Disabled: UsrRmmieX2Status = UsrRmmieX2Status(3);
	pub const V8Disabled: UsrRmmieX2Status = UsrRmmieX2Status(4);
	pub const Remote3200Disabled: UsrRmmieX2Status = UsrRmmieX2Status(5);
	pub const Invalidspeedsetting: UsrRmmieX2Status = UsrRmmieX2Status(6);
	pub const V8Notdetected: UsrRmmieX2Status = UsrRmmieX2Status(7);
	pub const X2Notdetected: UsrRmmieX2Status = UsrRmmieX2Status(8);
	pub const Incompatibleversion: UsrRmmieX2Status = UsrRmmieX2Status(9);
	pub const Incompatiblemodes: UsrRmmieX2Status = UsrRmmieX2Status(10);
	pub const Local3200Disabled: UsrRmmieX2Status = UsrRmmieX2Status(11);
	pub const Excesshighfrequencyatten: UsrRmmieX2Status = UsrRmmieX2Status(12);
	pub const Connectnotsupport3200: UsrRmmieX2Status = UsrRmmieX2Status(13);
	pub const Retrainbeforeconnection: UsrRmmieX2Status = UsrRmmieX2Status(14);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrRmmiePlannedDisconnect(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrRmmiePlannedDisconnect {
	pub const None: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(1);
	pub const Dtenotready: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(2);
	pub const Dteinterfaceerror: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(3);
	pub const Dterequest: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(4);
	pub const Escapetoonlinecommandmode: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(5);
	pub const Athcommand: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(6);
	pub const Inactivitytimeout: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(7);
	pub const Arqprotocolerror: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(8);
	pub const Arqprotocolretransmitlim: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(9);
	pub const Invalidcomprdatacodeword: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(10);
	pub const Invalidcomprdatastringlen: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(11);
	pub const Invalidcomprdatacommand: UsrRmmiePlannedDisconnect = UsrRmmiePlannedDisconnect(12);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrRmmieLastUpdateEvent(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrRmmieLastUpdateEvent {
	pub const None: UsrRmmieLastUpdateEvent = UsrRmmieLastUpdateEvent(1);
	pub const Initialconnection: UsrRmmieLastUpdateEvent = UsrRmmieLastUpdateEvent(2);
	pub const Retrain: UsrRmmieLastUpdateEvent = UsrRmmieLastUpdateEvent(3);
	pub const Speedshift: UsrRmmieLastUpdateEvent = UsrRmmieLastUpdateEvent(4);
	pub const Planneddisconnect: UsrRmmieLastUpdateEvent = UsrRmmieLastUpdateEvent(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrRequestType(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrRequestType {
	pub const AccessRequest: UsrRequestType = UsrRequestType(1);
	pub const AccessAccept: UsrRequestType = UsrRequestType(2);
	pub const AccessReject: UsrRequestType = UsrRequestType(3);
	pub const AccountingRequest: UsrRequestType = UsrRequestType(4);
	pub const AccountingResponse: UsrRequestType = UsrRequestType(5);
	pub const AccessPasswordChange: UsrRequestType = UsrRequestType(7);
	pub const AccessPasswordAck: UsrRequestType = UsrRequestType(8);
	pub const AccessPasswordReject: UsrRequestType = UsrRequestType(9);
	pub const AccessChallenge: UsrRequestType = UsrRequestType(11);
	pub const StatusServer: UsrRequestType = UsrRequestType(12);
	pub const StatusClient: UsrRequestType = UsrRequestType(13);
	pub const ResourceFreeRequest: UsrRequestType = UsrRequestType(21);
	pub const ResourceFreeResponse: UsrRequestType = UsrRequestType(22);
	pub const ResourceQueryRequest: UsrRequestType = UsrRequestType(23);
	pub const ResourceQueryResponse: UsrRequestType = UsrRequestType(24);
	pub const DisconnectUser: UsrRequestType = UsrRequestType(25);
	pub const NasRebootRequest: UsrRequestType = UsrRequestType(26);
	pub const NasRebootResponse: UsrRequestType = UsrRequestType(27);
	pub const TacacsMessage: UsrRequestType = UsrRequestType(253);
	pub const Reserved: UsrRequestType = UsrRequestType(255);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrSpeedOfConnection(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrSpeedOfConnection {
	pub const Auto: UsrSpeedOfConnection = UsrSpeedOfConnection(0);
	pub const Five6: UsrSpeedOfConnection = UsrSpeedOfConnection(1);
	pub const Six4: UsrSpeedOfConnection = UsrSpeedOfConnection(2);
	pub const Voice: UsrSpeedOfConnection = UsrSpeedOfConnection(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrExpansionAlgorithm(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrExpansionAlgorithm {
	pub const Constant: UsrExpansionAlgorithm = UsrExpansionAlgorithm(1);
	pub const Linear: UsrExpansionAlgorithm = UsrExpansionAlgorithm(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrCompressionAlgorithm(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrCompressionAlgorithm {
	pub const None: UsrCompressionAlgorithm = UsrCompressionAlgorithm(0);
	pub const Stac: UsrCompressionAlgorithm = UsrCompressionAlgorithm(1);
	pub const Ascend: UsrCompressionAlgorithm = UsrCompressionAlgorithm(2);
	pub const Microsoft: UsrCompressionAlgorithm = UsrCompressionAlgorithm(3);
	pub const Auto: UsrCompressionAlgorithm = UsrCompressionAlgorithm(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrCompressionResetMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrCompressionResetMode {
	pub const Auto: UsrCompressionResetMode = UsrCompressionResetMode(0);
	pub const ResetEveryPacket: UsrCompressionResetMode = UsrCompressionResetMode(1);
	pub const ResetOnError: UsrCompressionResetMode = UsrCompressionResetMode(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrFilterZones(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrFilterZones {
	pub const Enabled: UsrFilterZones = UsrFilterZones(1);
	pub const Disabled: UsrFilterZones = UsrFilterZones(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrBridging(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrBridging {
	pub const Enabled: UsrBridging = UsrBridging(1);
	pub const Disabled: UsrBridging = UsrBridging(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrAppletalk(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrAppletalk {
	pub const Enabled: UsrAppletalk = UsrAppletalk(1);
	pub const Disabled: UsrAppletalk = UsrAppletalk(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrSpoofing(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrSpoofing {
	pub const Enabled: UsrSpoofing = UsrSpoofing(1);
	pub const Disabled: UsrSpoofing = UsrSpoofing(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrRoutingProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrRoutingProtocol {
	pub const Rip1: UsrRoutingProtocol = UsrRoutingProtocol(1);
	pub const Rip2: UsrRoutingProtocol = UsrRoutingProtocol(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrIpxRouting(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrIpxRouting {
	pub const None: UsrIpxRouting = UsrIpxRouting(0);
	pub const Send: UsrIpxRouting = UsrIpxRouting(1);
	pub const Listen: UsrIpxRouting = UsrIpxRouting(2);
	pub const Respond: UsrIpxRouting = UsrIpxRouting(3);
	pub const All: UsrIpxRouting = UsrIpxRouting(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrIpxWan(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrIpxWan {
	pub const Enabled: UsrIpxWan = UsrIpxWan(1);
	pub const Disabled: UsrIpxWan = UsrIpxWan(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrIpDefaultRouteOption(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrIpDefaultRouteOption {
	pub const Enabled: UsrIpDefaultRouteOption = UsrIpDefaultRouteOption(1);
	pub const Disabled: UsrIpDefaultRouteOption = UsrIpDefaultRouteOption(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrIpRipPolicies(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrIpRipPolicies {
	pub const Senddefault: UsrIpRipPolicies = UsrIpRipPolicies(0x0);
	pub const Sendroutes: UsrIpRipPolicies = UsrIpRipPolicies(0x2);
	pub const Sendsubnets: UsrIpRipPolicies = UsrIpRipPolicies(0x4);
	pub const Acceptdefault: UsrIpRipPolicies = UsrIpRipPolicies(0x8);
	pub const Splithorizon: UsrIpRipPolicies = UsrIpRipPolicies(0x10);
	pub const Poisonreserve: UsrIpRipPolicies = UsrIpRipPolicies(0x20);
	pub const Flashupdate: UsrIpRipPolicies = UsrIpRipPolicies(0x40);
	pub const Simpleauth: UsrIpRipPolicies = UsrIpRipPolicies(0x80);
	pub const V1Send: UsrIpRipPolicies = UsrIpRipPolicies(0x100);
	pub const V1Receive: UsrIpRipPolicies = UsrIpRipPolicies(0x200);
	pub const V2Receive: UsrIpRipPolicies = UsrIpRipPolicies(0x400);
	pub const Silent: UsrIpRipPolicies = UsrIpRipPolicies(0x80000000);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrCallbackType(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrCallbackType {
	pub const Normal: UsrCallbackType = UsrCallbackType(1);
	pub const Ani: UsrCallbackType = UsrCallbackType(2);
	pub const Static: UsrCallbackType = UsrCallbackType(3);
	pub const Dynamic: UsrCallbackType = UsrCallbackType(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrAgent(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrAgent {
	pub const Fa: UsrAgent = UsrAgent(1);
	pub const Ha: UsrAgent = UsrAgent(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrNasType(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrNasType {
	pub const ThreeComNmc: UsrNasType = UsrNasType(0);
	pub const ThreeComNetserver: UsrNasType = UsrNasType(1);
	pub const ThreeComHiperarc: UsrNasType = UsrNasType(2);
	pub const TacacsPlusServer: UsrNasType = UsrNasType(3);
	pub const ThreeComSaServer: UsrNasType = UsrNasType(4);
	pub const Ascend: UsrNasType = UsrNasType(5);
	pub const GenericRadius: UsrNasType = UsrNasType(6);
	pub const ThreeComNetbuilderIi: UsrNasType = UsrNasType(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrAuthMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrAuthMode {
	pub const Auth3Com: UsrAuthMode = UsrAuthMode(0);
	pub const AuthAce: UsrAuthMode = UsrAuthMode(1);
	pub const AuthSafeword: UsrAuthMode = UsrAuthMode(2);
	pub const AuthUnixPw: UsrAuthMode = UsrAuthMode(3);
	pub const AuthDefender: UsrAuthMode = UsrAuthMode(4);
	pub const AuthTacacsp: UsrAuthMode = UsrAuthMode(5);
	pub const AuthNetware: UsrAuthMode = UsrAuthMode(6);
	pub const AuthSkey: UsrAuthMode = UsrAuthMode(7);
	pub const AuthEapProxy: UsrAuthMode = UsrAuthMode(8);
	pub const AuthUnixCrypt: UsrAuthMode = UsrAuthMode(9);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwAcctType(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwAcctType {
	pub const ComsUnknownAcctType: CwAcctType = CwAcctType(0);
	pub const ComsPrepaidAcct: CwAcctType = CwAcctType(1);
	pub const ComsNewAcct: CwAcctType = CwAcctType(2);
	pub const ComsSuspendedAcct: CwAcctType = CwAcctType(3);
	pub const ComsAdministrativeAcct: CwAcctType = CwAcctType(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwSourceIdentifier(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwSourceIdentifier {
	pub const ComsUnknownSource: CwSourceIdentifier = CwSourceIdentifier(0);
	pub const ComsIngressOpen: CwSourceIdentifier = CwSourceIdentifier(257);
	pub const ComsEgressOpen: CwSourceIdentifier = CwSourceIdentifier(258);
	pub const ComsGtkprGenIngrOpen: CwSourceIdentifier = CwSourceIdentifier(259);
	pub const ComsGtkprGenEgrOpen: CwSourceIdentifier = CwSourceIdentifier(260);
	pub const ComsIngressClose: CwSourceIdentifier = CwSourceIdentifier(513);
	pub const ComsEgressClose: CwSourceIdentifier = CwSourceIdentifier(514);
	pub const ComsGtkprGenIngrClose: CwSourceIdentifier = CwSourceIdentifier(515);
	pub const ComsGtkprGenEgrClose: CwSourceIdentifier = CwSourceIdentifier(516);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwSessionSequenceEnd(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwSessionSequenceEnd {
	pub const NotTheLastCall: CwSessionSequenceEnd = CwSessionSequenceEnd(0);
	pub const LastCall: CwSessionSequenceEnd = CwSessionSequenceEnd(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwClgPartyE164Type(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwClgPartyE164Type {
	pub const Comsunknown: CwClgPartyE164Type = CwClgPartyE164Type(1);
	pub const Comsinternationalnumber: CwClgPartyE164Type = CwClgPartyE164Type(2);
	pub const Comsnationalnumber: CwClgPartyE164Type = CwClgPartyE164Type(3);
	pub const Comsnetworkspecificnumber: CwClgPartyE164Type = CwClgPartyE164Type(4);
	pub const Comssubscribernumber: CwClgPartyE164Type = CwClgPartyE164Type(5);
	pub const Comsabbreviatednumber: CwClgPartyE164Type = CwClgPartyE164Type(6);
	pub const Comsreserved: CwClgPartyE164Type = CwClgPartyE164Type(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwClgPartyTransProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwClgPartyTransProtocol {
	pub const Tcp: CwClgPartyTransProtocol = CwClgPartyTransProtocol(1);
	pub const Udp: CwClgPartyTransProtocol = CwClgPartyTransProtocol(2);
	pub const Sctp: CwClgPartyTransProtocol = CwClgPartyTransProtocol(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwCldPartyE164Type(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwCldPartyE164Type {
	pub const Comsunknown: CwCldPartyE164Type = CwCldPartyE164Type(1);
	pub const Comsinternationalnumber: CwCldPartyE164Type = CwCldPartyE164Type(2);
	pub const Comsnationalnumber: CwCldPartyE164Type = CwCldPartyE164Type(3);
	pub const Comsnetworkspecificnumber: CwCldPartyE164Type = CwCldPartyE164Type(4);
	pub const Comssubscribernumber: CwCldPartyE164Type = CwCldPartyE164Type(5);
	pub const Comsabbreviatednumber: CwCldPartyE164Type = CwCldPartyE164Type(6);
	pub const Comsreserved: CwCldPartyE164Type = CwCldPartyE164Type(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwCldPartyTransProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwCldPartyTransProtocol {
	pub const Tcp: CwCldPartyTransProtocol = CwCldPartyTransProtocol(1);
	pub const Udp: CwCldPartyTransProtocol = CwCldPartyTransProtocol(2);
	pub const Sctp: CwCldPartyTransProtocol = CwCldPartyTransProtocol(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwIngrGwayE164Type(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwIngrGwayE164Type {
	pub const Comsunknown: CwIngrGwayE164Type = CwIngrGwayE164Type(1);
	pub const Comsinternationalnumber: CwIngrGwayE164Type = CwIngrGwayE164Type(2);
	pub const Comsnationalnumber: CwIngrGwayE164Type = CwIngrGwayE164Type(3);
	pub const Comsnetworkspecificnumber: CwIngrGwayE164Type = CwIngrGwayE164Type(4);
	pub const Comssubscribernumber: CwIngrGwayE164Type = CwIngrGwayE164Type(5);
	pub const Comsabbreviatednumber: CwIngrGwayE164Type = CwIngrGwayE164Type(6);
	pub const Comsreserved: CwIngrGwayE164Type = CwIngrGwayE164Type(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwIngrGwayTransProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwIngrGwayTransProtocol {
	pub const Tcp: CwIngrGwayTransProtocol = CwIngrGwayTransProtocol(1);
	pub const Udp: CwIngrGwayTransProtocol = CwIngrGwayTransProtocol(2);
	pub const Sctp: CwIngrGwayTransProtocol = CwIngrGwayTransProtocol(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwEgrGwayTransProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwEgrGwayTransProtocol {
	pub const Tcp: CwEgrGwayTransProtocol = CwEgrGwayTransProtocol(1);
	pub const Udp: CwEgrGwayTransProtocol = CwEgrGwayTransProtocol(2);
	pub const Sctp: CwEgrGwayTransProtocol = CwEgrGwayTransProtocol(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwIngrGtkprTransProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwIngrGtkprTransProtocol {
	pub const Tcp: CwIngrGtkprTransProtocol = CwIngrGtkprTransProtocol(1);
	pub const Udp: CwIngrGtkprTransProtocol = CwIngrGtkprTransProtocol(2);
	pub const Sctp: CwIngrGtkprTransProtocol = CwIngrGtkprTransProtocol(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwEgrGtkprTransProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwEgrGtkprTransProtocol {
	pub const Tcp: CwEgrGtkprTransProtocol = CwEgrGtkprTransProtocol(1);
	pub const Udp: CwEgrGtkprTransProtocol = CwEgrGtkprTransProtocol(2);
	pub const Sctp: CwEgrGtkprTransProtocol = CwEgrGtkprTransProtocol(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwCallType(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwCallType {
	pub const ComsUnknownCalltype: CwCallType = CwCallType(0);
	pub const ComsPhoneToPhone: CwCallType = CwCallType(1);
	pub const ComsPhoneToPc: CwCallType = CwCallType(2);
	pub const ComsPcToPhone: CwCallType = CwCallType(3);
	pub const ComsPcToPc: CwCallType = CwCallType(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwCodecType(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwCodecType {
	pub const ComsUndefinedCodec: CwCodecType = CwCodecType(0);
	pub const ComsG7231: CwCodecType = CwCodecType(1);
	pub const ComsG729A: CwCodecType = CwCodecType(2);
	pub const ComsG710Alaw: CwCodecType = CwCodecType(3);
	pub const ComsG711Mulaw: CwCodecType = CwCodecType(4);
	pub const ComsFaxModulation: CwCodecType = CwCodecType(255);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwCallTerminationCause(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwCallTerminationCause {
	pub const CauseUnknown: CwCallTerminationCause = CwCallTerminationCause(0);
	pub const CauseCldPartyTerminate: CwCallTerminationCause = CwCallTerminationCause(1);
	pub const CauseClgPartyTerminate: CwCallTerminationCause = CwCallTerminationCause(2);
	pub const CauseAcctBalDepleted: CwCallTerminationCause = CwCallTerminationCause(3);
	pub const CauseNoEgrPortsAvail: CwCallTerminationCause = CwCallTerminationCause(4);
	pub const CauseH225UnableToCon: CwCallTerminationCause = CwCallTerminationCause(5);
	pub const CauseH245UnableToCon: CwCallTerminationCause = CwCallTerminationCause(6);
	pub const CauseIngrFacilityDisc: CwCallTerminationCause = CwCallTerminationCause(7);
	pub const CauseEgrFacilityDisc: CwCallTerminationCause = CwCallTerminationCause(8);
	pub const CauseDirServerDown: CwCallTerminationCause = CwCallTerminationCause(9);
	pub const CauseRatingServerDown: CwCallTerminationCause = CwCallTerminationCause(10);
	pub const CauseGatewayShutdown: CwCallTerminationCause = CwCallTerminationCause(11);
	pub const CauseGtkprTerminate: CwCallTerminationCause = CwCallTerminationCause(12);
	pub const CauseGtkprShutdownGtway: CwCallTerminationCause = CwCallTerminationCause(13);
	pub const CauseBusy: CwCallTerminationCause = CwCallTerminationCause(14);
	pub const CauseAbandon: CwCallTerminationCause = CwCallTerminationCause(15);
	pub const CauseInvalidLoginLimit: CwCallTerminationCause = CwCallTerminationCause(16);
	pub const CauseNoacctnumberEntry: CwCallTerminationCause = CwCallTerminationCause(17);
	pub const CauseSuspendedAcctLogin: CwCallTerminationCause = CwCallTerminationCause(18);
	pub const CauseAuthentServerDown: CwCallTerminationCause = CwCallTerminationCause(19);
	pub const CauseGatekeeperTimeout: CwCallTerminationCause = CwCallTerminationCause(20);
	pub const CauseGatewayNoResources: CwCallTerminationCause = CwCallTerminationCause(21);
	pub const CauseAcctInuse: CwCallTerminationCause = CwCallTerminationCause(22);
	pub const CauseDebitAcctBalZero: CwCallTerminationCause = CwCallTerminationCause(23);
	pub const CauseDebitAcctbalInsuff: CwCallTerminationCause = CwCallTerminationCause(24);
	pub const CauseInvalidDestnumberThresh: CwCallTerminationCause = CwCallTerminationCause(25);
	pub const CauseNoDestnumberEntry: CwCallTerminationCause = CwCallTerminationCause(26);
	pub const CauseSequenceDialingThresh: CwCallTerminationCause = CwCallTerminationCause(27);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwSignalingProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwSignalingProtocol {
	pub const SigUnknown: CwSignalingProtocol = CwSignalingProtocol(0);
	pub const SigSip: CwSignalingProtocol = CwSignalingProtocol(1);
	pub const SigH323: CwSignalingProtocol = CwSignalingProtocol(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwProtocolTransport(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwProtocolTransport {
	pub const Tcp: CwProtocolTransport = CwProtocolTransport(1);
	pub const Udp: CwProtocolTransport = CwProtocolTransport(2);
	pub const Sctp: CwProtocolTransport = CwProtocolTransport(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwLocalSigTransProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwLocalSigTransProtocol {
	pub const Tcp: CwLocalSigTransProtocol = CwLocalSigTransProtocol(1);
	pub const Udp: CwLocalSigTransProtocol = CwLocalSigTransProtocol(2);
	pub const Sctp: CwLocalSigTransProtocol = CwLocalSigTransProtocol(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwRemoteSigTransProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwRemoteSigTransProtocol {
	pub const Tcp: CwRemoteSigTransProtocol = CwRemoteSigTransProtocol(1);
	pub const Udp: CwRemoteSigTransProtocol = CwRemoteSigTransProtocol(2);
	pub const Sctp: CwRemoteSigTransProtocol = CwRemoteSigTransProtocol(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwLocalMgRtpProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwLocalMgRtpProtocol {
	pub const Tcp: CwLocalMgRtpProtocol = CwLocalMgRtpProtocol(1);
	pub const Udp: CwLocalMgRtpProtocol = CwLocalMgRtpProtocol(2);
	pub const Sctp: CwLocalMgRtpProtocol = CwLocalMgRtpProtocol(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwRemoteMgRtpProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwRemoteMgRtpProtocol {
	pub const Tcp: CwRemoteMgRtpProtocol = CwRemoteMgRtpProtocol(1);
	pub const Udp: CwRemoteMgRtpProtocol = CwRemoteMgRtpProtocol(2);
	pub const Sctp: CwRemoteMgRtpProtocol = CwRemoteMgRtpProtocol(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CwTransCldPartyE164Type(pub u32);
 
#[allow(non_upper_case_globals)]
impl CwTransCldPartyE164Type {
	pub const Unknown: CwTransCldPartyE164Type = CwTransCldPartyE164Type(1);
	pub const InternationalNumber: CwTransCldPartyE164Type = CwTransCldPartyE164Type(2);
	pub const NationalNumber: CwTransCldPartyE164Type = CwTransCldPartyE164Type(3);
	pub const NetworkSpecificNumber: CwTransCldPartyE164Type = CwTransCldPartyE164Type(4);
	pub const SubscriberNumber: CwTransCldPartyE164Type = CwTransCldPartyE164Type(5);
	pub const AbbreviatedNumber: CwTransCldPartyE164Type = CwTransCldPartyE164Type(6);
	pub const Reserved: CwTransCldPartyE164Type = CwTransCldPartyE164Type(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsrPrePaidEnabled(pub u32);
 
#[allow(non_upper_case_globals)]
impl UsrPrePaidEnabled {
	pub const Phase2ActiveTimeCounted: UsrPrePaidEnabled = UsrPrePaidEnabled(1);
	pub const Phase2TransferOrRecievPacketCounted: UsrPrePaidEnabled = UsrPrePaidEnabled(2);
	pub const Phase2TotalPacketCounted: UsrPrePaidEnabled = UsrPrePaidEnabled(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		102 => value!(i, Attribute::VsaUsrLastNumberDialedOut(i)),
		232 => value!(i, Attribute::VsaUsrLastNumberDialedInDnis(i)),
		233 => value!(i, Attribute::VsaUsrLastCallersNumberAni(i)),
		// out of u8 bounds 		48952 => map!{i, be_u32, |v| Attribute::VsaUsrChannel(v)},
		// out of u8 bounds 		49086 => map! {i, be_u32, |v| Attribute::VsaUsrEventId(UsrEventId(v))},
		// out of u8 bounds 		48943 => map!{i, be_u32, |v| Attribute::VsaUsrEventDateTime(v)},
		// out of u8 bounds 		49143 => map!{i, be_u32, |v| Attribute::VsaUsrCallStartDateTime(v)},
		// out of u8 bounds 		49142 => map!{i, be_u32, |v| Attribute::VsaUsrCallEndDateTime(v)},
		94 => map! {i, be_u32, |v| Attribute::VsaUsrDefaultDteDataRate(UsrDefaultDteDataRate(v))},
		// out of u8 bounds 		48941 => map! {i, be_u32, |v| Attribute::VsaUsrInitialRxLinkDataRate(UsrInitialRxLinkDataRate(v))},
		// out of u8 bounds 		48940 => map! {i, be_u32, |v| Attribute::VsaUsrFinalRxLinkDataRate(UsrFinalRxLinkDataRate(v))},
		106 => map! {i, be_u32, |v| Attribute::VsaUsrInitialTxLinkDataRate(UsrInitialTxLinkDataRate(v))},
		107 => map! {i, be_u32, |v| Attribute::VsaUsrFinalTxLinkDataRate(UsrFinalTxLinkDataRate(v))},
		// out of u8 bounds 		48945 => map!{i, be_u32, |v| Attribute::VsaUsrChassisTemperature(v)},
		// out of u8 bounds 		48772 => map!{i, be_u32, |v| Attribute::VsaUsrChassisTempThreshold(v)},
		// out of u8 bounds 		48946 => map!{i, be_u32, |v| Attribute::VsaUsrActualVoltage(v)},
		// out of u8 bounds 		48947 => map!{i, be_u32, |v| Attribute::VsaUsrExpectedVoltage(v)},
		// out of u8 bounds 		48948 => map!{i, be_u32, |v| Attribute::VsaUsrPowerSupplyNumber(v)},
		// out of u8 bounds 		48773 => map! {i, be_u32, |v| Attribute::VsaUsrCardType(UsrCardType(v))},
		// out of u8 bounds 		48953 => map!{i, be_u32, |v| Attribute::VsaUsrChassisSlot(v)},
		103 => map! {i, be_u32, |v| Attribute::VsaUsrSyncAsyncMode(UsrSyncAsyncMode(v))},
		104 => map! {i, be_u32, |v| Attribute::VsaUsrOriginateAnswerMode(UsrOriginateAnswerMode(v))},
		108 => map! {i, be_u32, |v| Attribute::VsaUsrModulationType(UsrModulationType(v))},
		155 => map! {i, be_u32, |v| Attribute::VsaUsrConnectTermReason(UsrConnectTermReason(v))},
		105 => map! {i, be_u32, |v| Attribute::VsaUsrFailureToConnectReason(UsrFailureToConnectReason(v))},
		111 => map! {i, be_u32, |v| Attribute::VsaUsrEqualizationType(UsrEqualizationType(v))},
		112 => map! {i, be_u32, |v| Attribute::VsaUsrFallbackEnabled(UsrFallbackEnabled(v))},
		// out of u8 bounds 		49127 => map!{i, be_u32, |v| Attribute::VsaUsrConnectTimeLimit(v)},
		// out of u8 bounds 		49126 => map!{i, be_u32, |v| Attribute::VsaUsrNumberOfRingsLimit(v)},
		72 => map!{i, be_u32, |v| Attribute::VsaUsrDteDataIdleTimout(v)},
		113 => map!{i, be_u32, |v| Attribute::VsaUsrCharactersSent(v)},
		114 => map!{i, be_u32, |v| Attribute::VsaUsrCharactersReceived(v)},
		117 => map!{i, be_u32, |v| Attribute::VsaUsrBlocksSent(v)},
		118 => map!{i, be_u32, |v| Attribute::VsaUsrBlocksReceived(v)},
		119 => map!{i, be_u32, |v| Attribute::VsaUsrBlocksResent(v)},
		120 => map!{i, be_u32, |v| Attribute::VsaUsrRetrainsRequested(v)},
		121 => map!{i, be_u32, |v| Attribute::VsaUsrRetrainsGranted(v)},
		122 => map!{i, be_u32, |v| Attribute::VsaUsrLineReversals(v)},
		123 => map!{i, be_u32, |v| Attribute::VsaUsrNumberOfCharactersLost(v)},
		125 => map!{i, be_u32, |v| Attribute::VsaUsrNumberOfBlers(v)},
		126 => map!{i, be_u32, |v| Attribute::VsaUsrNumberOfLinkTimeouts(v)},
		127 => map!{i, be_u32, |v| Attribute::VsaUsrNumberOfFallbacks(v)},
		128 => map!{i, be_u32, |v| Attribute::VsaUsrNumberOfUpshifts(v)},
		129 => map!{i, be_u32, |v| Attribute::VsaUsrNumberOfLinkNaks(v)},
		190 => map!{i, be_u32, |v| Attribute::VsaUsrDtrFalseTimeout(v)},
		191 => map!{i, be_u32, |v| Attribute::VsaUsrFallbackLimit(v)},
		192 => map!{i, be_u32, |v| Attribute::VsaUsrBlockErrorCountLimit(v)},
		218 => map!{i, be_u32, |v| Attribute::VsaUsrDtrTrueTimeout(v)},
		// out of u8 bounds 		48862 => map!{i, be_u32, |v| Attribute::VsaUsrSecurityLoginLimit(v)},
		// out of u8 bounds 		48890 => map!{i, be_u32, |v| Attribute::VsaUsrSecurityRespLimit(v)},
		// out of u8 bounds 		48919 => map!{i, be_u32, |v| Attribute::VsaUsrDteRingNoAnswerLimit(v)},
		124 => map! {i, be_u32, |v| Attribute::VsaUsrBackChannelDataRate(UsrBackChannelDataRate(v))},
		153 => map! {i, be_u32, |v| Attribute::VsaUsrSimplifiedMnpLevels(UsrSimplifiedMnpLevels(v))},
		199 => map! {i, be_u32, |v| Attribute::VsaUsrSimplifiedV42BisUsage(UsrSimplifiedV42BisUsage(v))},
		// out of u8 bounds 		388 => map!{i, be_u32, |v| Attribute::VsaUsrMbiCtPriCardSlot(v)},
		// out of u8 bounds 		389 => map!{i, be_u32, |v| Attribute::VsaUsrMbiCtTdmTimeSlot(v)},
		// out of u8 bounds 		390 => map!{i, be_u32, |v| Attribute::VsaUsrMbiCtPriCardSpanLine(v)},
		// out of u8 bounds 		391 => map!{i, be_u32, |v| Attribute::VsaUsrMbiCtBchannelUsed(v)},
		// out of u8 bounds 		48759 => map!{i, be_u32, |v| Attribute::VsaUsrPhysicalState(v)},
		// out of u8 bounds 		48916 => map!{i, be_u32, |v| Attribute::VsaUsrPacketBusSession(v)},
		// out of u8 bounds 		61440 => map!{i, be_u32, |v| Attribute::VsaUsrServerTime(v)},
		// out of u8 bounds 		48733 => map!{i, be_u32, |v| Attribute::VsaUsrChannelConnectedTo(v)},
		// out of u8 bounds 		48734 => map!{i, be_u32, |v| Attribute::VsaUsrSlotConnectedTo(v)},
		// out of u8 bounds 		48735 => map! {i, be_u32, |v| Attribute::VsaUsrDeviceConnectedTo(UsrDeviceConnectedTo(v))},
		// out of u8 bounds 		48736 => map!{i, be_u32, |v| Attribute::VsaUsrNfasId(v)},
		// out of u8 bounds 		48737 => map!{i, be_u32, |v| Attribute::VsaUsrQ931CallReferenceValue(v)},
		// out of u8 bounds 		48738 => map! {i, be_u32, |v| Attribute::VsaUsrCallEventCode(UsrCallEventCode(v))},
		// out of u8 bounds 		48739 => map!{i, be_u32, |v| Attribute::VsaUsrDs0(v)},
		// out of u8 bounds 		48740 => value!(i, Attribute::VsaUsrDs0S(i)),
		// out of u8 bounds 		48742 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrGatewayIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		32768 => map!{i, be_u32, |v| Attribute::VsaCwVersionId(v)},
		// out of u8 bounds 		32769 => value!(i, Attribute::VsaCwAccountId(i)),
		// out of u8 bounds 		32770 => map! {i, be_u32, |v| Attribute::VsaCwAcctType(CwAcctType(v))},
		// out of u8 bounds 		32771 => map!{i, be_u32, |v| Attribute::VsaCwAcctIdentificationCode(v)},
		// out of u8 bounds 		32772 => map!{i, be_u32, |v| Attribute::VsaCwServiceType(v)},
		// out of u8 bounds 		32773 => map!{i, be_u32, |v| Attribute::VsaCwRatePlanId(v)},
		// out of u8 bounds 		32774 => map! {i, be_u32, |v| Attribute::VsaCwSourceIdentifier(CwSourceIdentifier(v))},
		// out of u8 bounds 		32775 => value!(i, Attribute::VsaCwSessionId(i)),
		// out of u8 bounds 		32776 => map!{i, be_u32, |v| Attribute::VsaCwNumCallAttemptSession(v)},
		// out of u8 bounds 		32777 => map!{i, be_u32, |v| Attribute::VsaCwSessionSequenceNum(v)},
		// out of u8 bounds 		32778 => map! {i, be_u32, |v| Attribute::VsaCwSessionSequenceEnd(CwSessionSequenceEnd(v))},
		// out of u8 bounds 		32779 => map!{i, be_u32, |v| Attribute::VsaCwAuthenticationFailCnt(v)},
		// out of u8 bounds 		32780 => map! {i, be_u32, |v| Attribute::VsaCwClgPartyE164Type(CwClgPartyE164Type(v))},
		// out of u8 bounds 		32781 => value!(i, Attribute::VsaCwClgPartyE164Number(i)),
		// out of u8 bounds 		32782 => map! {i, be_u32, |v| Attribute::VsaCwClgPartyTransProtocol(CwClgPartyTransProtocol(v))},
		// out of u8 bounds 		32783 => map!{i, be_u32, |v| Attribute::VsaCwClgPartyTransPort(v)},
		// out of u8 bounds 		32784 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCwClgPartyTransIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		32785 => value!(i, Attribute::VsaCwClgPartyTransDns(i)),
		// out of u8 bounds 		32786 => map! {i, be_u32, |v| Attribute::VsaCwCldPartyE164Type(CwCldPartyE164Type(v))},
		// out of u8 bounds 		32787 => value!(i, Attribute::VsaCwCldPartyE164Number(i)),
		// out of u8 bounds 		32788 => map! {i, be_u32, |v| Attribute::VsaCwCldPartyTransProtocol(CwCldPartyTransProtocol(v))},
		// out of u8 bounds 		32789 => map!{i, be_u32, |v| Attribute::VsaCwCldPartyTransPort(v)},
		// out of u8 bounds 		32790 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCwCldPartyTransIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		32791 => value!(i, Attribute::VsaCwCldPartyTransDns(i)),
		// out of u8 bounds 		32792 => map!{i, be_u32, |v| Attribute::VsaCwOrigLineIdentifier(v)},
		// out of u8 bounds 		32793 => map!{i, be_u32, |v| Attribute::VsaCwPstnInterfaceNumber(v)},
		// out of u8 bounds 		32794 => map! {i, be_u32, |v| Attribute::VsaCwIngrGwayE164Type(CwIngrGwayE164Type(v))},
		// out of u8 bounds 		32795 => value!(i, Attribute::VsaCwIngrGwayE164Number(i)),
		// out of u8 bounds 		32796 => map! {i, be_u32, |v| Attribute::VsaCwIngrGwayTransProtocol(CwIngrGwayTransProtocol(v))},
		// out of u8 bounds 		32797 => map!{i, be_u32, |v| Attribute::VsaCwIngrGwayTransPort(v)},
		// out of u8 bounds 		32798 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCwIngrGwayTransIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		32799 => value!(i, Attribute::VsaCwIngrGwayTransDns(i)),
		// out of u8 bounds 		32800 => map! {i, be_u32, |v| Attribute::VsaCwEgrGwayTransProtocol(CwEgrGwayTransProtocol(v))},
		// out of u8 bounds 		32801 => map!{i, be_u32, |v| Attribute::VsaCwEgrGwayTransPort(v)},
		// out of u8 bounds 		32802 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCwEgrGwayTransIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		32803 => value!(i, Attribute::VsaCwEgrGwayTransDns(i)),
		// out of u8 bounds 		32804 => map! {i, be_u32, |v| Attribute::VsaCwIngrGtkprTransProtocol(CwIngrGtkprTransProtocol(v))},
		// out of u8 bounds 		32805 => map!{i, be_u32, |v| Attribute::VsaCwIngrGtkprTransPort(v)},
		// out of u8 bounds 		32806 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCwIngrGtkprTransIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		32807 => value!(i, Attribute::VsaCwIngrGtkprTransDns(i)),
		// out of u8 bounds 		32808 => map! {i, be_u32, |v| Attribute::VsaCwEgrGtkprTransProtocol(CwEgrGtkprTransProtocol(v))},
		// out of u8 bounds 		32809 => map!{i, be_u32, |v| Attribute::VsaCwEgrGtkprTransPort(v)},
		// out of u8 bounds 		32810 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCwEgrGtkprTransIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		32811 => value!(i, Attribute::VsaCwEgrGtkprTransDns(i)),
		// out of u8 bounds 		32812 => value!(i, Attribute::VsaCwCallIdentifier(i)),
		// out of u8 bounds 		32813 => map! {i, be_u32, |v| Attribute::VsaCwCallType(CwCallType(v))},
		// out of u8 bounds 		32814 => value!(i, Attribute::VsaCwCallStartIngrGwSec(i)),
		// out of u8 bounds 		32815 => map!{i, be_u32, |v| Attribute::VsaCwCallStartIngrGwMsec(v)},
		// out of u8 bounds 		32816 => value!(i, Attribute::VsaCwCallStartTimeAnsSec(i)),
		// out of u8 bounds 		32817 => map!{i, be_u32, |v| Attribute::VsaCwCallStartTimeAnsMsec(v)},
		// out of u8 bounds 		32818 => value!(i, Attribute::VsaCwCallEndTimeSec(i)),
		// out of u8 bounds 		32819 => map!{i, be_u32, |v| Attribute::VsaCwCallEndTimeMsec(v)},
		// out of u8 bounds 		32820 => map!{i, be_u32, |v| Attribute::VsaCwCallDurnConnectDisc(v)},
		// out of u8 bounds 		32821 => map! {i, be_u32, |v| Attribute::VsaCwCodecType(CwCodecType(v))},
		// out of u8 bounds 		32822 => map! {i, be_u32, |v| Attribute::VsaCwCallTerminationCause(CwCallTerminationCause(v))},
		// out of u8 bounds 		32823 => map!{i, be_u32, |v| Attribute::VsaCwAudioPacketsSent(v)},
		// out of u8 bounds 		32824 => map!{i, be_u32, |v| Attribute::VsaCwAudioPacketsReceived(v)},
		// out of u8 bounds 		32825 => map!{i, be_u32, |v| Attribute::VsaCwAudioPacketsLost(v)},
		// out of u8 bounds 		32826 => map!{i, be_u32, |v| Attribute::VsaCwAudioPacketsInFrame(v)},
		// out of u8 bounds 		32827 => map!{i, be_u32, |v| Attribute::VsaCwAudioBytesInFrame(v)},
		// out of u8 bounds 		32828 => map!{i, be_u32, |v| Attribute::VsaCwAudioSignalInPacket(v)},
		// out of u8 bounds 		32829 => map!{i, be_u32, |v| Attribute::VsaCwPortIdForCall(v)},
		// out of u8 bounds 		32830 => map!{i, be_u32, |v| Attribute::VsaCwSlotIdForCall(v)},
		// out of u8 bounds 		32831 => map!{i, be_u32, |v| Attribute::VsaCwAcctBalanceStartCurr(v)},
		// out of u8 bounds 		32832 => map!{i, be_u32, |v| Attribute::VsaCwAcctBalanceStartAmt(v)},
		// out of u8 bounds 		32833 => map!{i, be_u32, |v| Attribute::VsaCwAcctBalanceStartDec(v)},
		// out of u8 bounds 		32834 => map!{i, be_u32, |v| Attribute::VsaCwAcctBalanceDecrCurr(v)},
		// out of u8 bounds 		32835 => value!(i, Attribute::VsaCwLrqToken(i)),
		// out of u8 bounds 		32836 => value!(i, Attribute::VsaCwArqToken(i)),
		// out of u8 bounds 		32837 => map!{i, be_u32, |v| Attribute::VsaCwTokenStatus(v)},
		// out of u8 bounds 		32838 => map!{i, be_u32, |v| Attribute::VsaCwSs7DestnPtcodeType(v)},
		// out of u8 bounds 		32839 => map!{i, be_u32, |v| Attribute::VsaCwSs7DestnPtcodeAddress(v)},
		// out of u8 bounds 		32840 => map!{i, be_u32, |v| Attribute::VsaCwSs7OrigPtcodeType(v)},
		// out of u8 bounds 		32841 => map!{i, be_u32, |v| Attribute::VsaCwSs7OrigPtcodeAddress(v)},
		// out of u8 bounds 		32842 => map!{i, be_u32, |v| Attribute::VsaCwSs7Cic(v)},
		// out of u8 bounds 		32843 => map!{i, be_u32, |v| Attribute::VsaCwMgcId(v)},
		// out of u8 bounds 		32844 => map!{i, be_u32, |v| Attribute::VsaCwMgId(v)},
		// out of u8 bounds 		32845 => map! {i, be_u32, |v| Attribute::VsaCwSignalingProtocol(CwSignalingProtocol(v))},
		// out of u8 bounds 		32846 => map! {i, be_u32, |v| Attribute::VsaCwProtocolTransport(CwProtocolTransport(v))},
		// out of u8 bounds 		32847 => map! {i, be_u32, |v| Attribute::VsaCwLocalSigTransProtocol(CwLocalSigTransProtocol(v))},
		// out of u8 bounds 		32848 => map!{i, be_u32, |v| Attribute::VsaCwLocalSigTransPort(v)},
		// out of u8 bounds 		32849 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCwLocalSigTransIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		32850 => value!(i, Attribute::VsaCwLocalSigTransDns(i)),
		// out of u8 bounds 		32851 => map! {i, be_u32, |v| Attribute::VsaCwRemoteSigTransProtocol(CwRemoteSigTransProtocol(v))},
		// out of u8 bounds 		32852 => map!{i, be_u32, |v| Attribute::VsaCwRemoteSigTransPort(v)},
		// out of u8 bounds 		32853 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCwRemoteSigTransIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		32854 => value!(i, Attribute::VsaCwRemoteSigTransDns(i)),
		// out of u8 bounds 		32855 => map! {i, be_u32, |v| Attribute::VsaCwLocalMgRtpProtocol(CwLocalMgRtpProtocol(v))},
		// out of u8 bounds 		32856 => map!{i, be_u32, |v| Attribute::VsaCwLocalMgRtpPort(v)},
		// out of u8 bounds 		32857 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCwLocalMgRtpIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		32858 => value!(i, Attribute::VsaCwLocalMgRtpDns(i)),
		// out of u8 bounds 		32859 => map! {i, be_u32, |v| Attribute::VsaCwRemoteMgRtpProtocol(CwRemoteMgRtpProtocol(v))},
		// out of u8 bounds 		32860 => map!{i, be_u32, |v| Attribute::VsaCwRemoteMgRtpPort(v)},
		// out of u8 bounds 		32861 => map!{i, take!(4), |v:&[u8]| Attribute::VsaCwRemoteMgRtpIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		32862 => value!(i, Attribute::VsaCwRemoteMgRtpDns(i)),
		// out of u8 bounds 		32863 => map!{i, be_u32, |v| Attribute::VsaCwCallModel(v)},
		// out of u8 bounds 		32864 => map!{i, be_u32, |v| Attribute::VsaCwCallPlanId(v)},
		// out of u8 bounds 		32865 => map! {i, be_u32, |v| Attribute::VsaCwTransCldPartyE164Type(CwTransCldPartyE164Type(v))},
		// out of u8 bounds 		32866 => value!(i, Attribute::VsaCwTransCldPartyE164Num(i)),
		// out of u8 bounds 		32867 => value!(i, Attribute::VsaCwOspSourceDevice(i)),
		// out of u8 bounds 		36864 => value!(i, Attribute::VsaUsrPwUsrIfilterIp(i)),
		// out of u8 bounds 		36865 => value!(i, Attribute::VsaUsrPwUsrIfilterIpx(i)),
		// out of u8 bounds 		36867 => value!(i, Attribute::VsaUsrPwUsrOfilterIp(i)),
		// out of u8 bounds 		36868 => value!(i, Attribute::VsaUsrPwUsrOfilterIpx(i)),
		// out of u8 bounds 		36869 => value!(i, Attribute::VsaUsrPwUsrOfilterSap(i)),
		// out of u8 bounds 		36870 => value!(i, Attribute::VsaUsrPwVpnId(i)),
		// out of u8 bounds 		36871 => value!(i, Attribute::VsaUsrPwVpnName(i)),
		// out of u8 bounds 		36872 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrPwVpnNeighbor(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		36873 => value!(i, Attribute::VsaUsrPwFramedRoutingV2(i)),
		// out of u8 bounds 		36874 => value!(i, Attribute::VsaUsrPwVpnGateway(i)),
		// out of u8 bounds 		36875 => value!(i, Attribute::VsaUsrPwTunnelAuthentication(i)),
		// out of u8 bounds 		36876 => value!(i, Attribute::VsaUsrPwIndex(i)),
		// out of u8 bounds 		36877 => value!(i, Attribute::VsaUsrPwCutoff(i)),
		// out of u8 bounds 		36878 => value!(i, Attribute::VsaUsrPwPacket(i)),
		// out of u8 bounds 		36879 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrPrimaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		36880 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrSecondaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		36881 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrPrimaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		36882 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrSecondaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		36883 => map! {i, be_u32, |v| Attribute::VsaUsrSyslogTap(UsrSyslogTap(v))},
		// out of u8 bounds 		36889 => map!{i, be_u32, |v| Attribute::VsaUsrChassisCallSlot(v)},
		// out of u8 bounds 		36890 => map!{i, be_u32, |v| Attribute::VsaUsrChassisCallSpan(v)},
		// out of u8 bounds 		36891 => map!{i, be_u32, |v| Attribute::VsaUsrChassisCallChannel(v)},
		// out of u8 bounds 		36892 => map!{i, be_u32, |v| Attribute::VsaUsrKeypressTimeout(v)},
		// out of u8 bounds 		36893 => map!{i, be_u32, |v| Attribute::VsaUsrUnauthenticatedTime(v)},
		// out of u8 bounds 		36899 => map! {i, be_u32, |v| Attribute::VsaUsrConnectSpeed(UsrConnectSpeed(v))},
		// out of u8 bounds 		36900 => value!(i, Attribute::VsaUsrFramedIpAddressPoolName(i)),
		// out of u8 bounds 		36901 => value!(i, Attribute::VsaUsrMpEdo(i)),
		// out of u8 bounds 		38912 => map!{i, be_u32, |v| Attribute::VsaUsrBearerCapabilities(v)},
		// out of u8 bounds 		38913 => map! {i, be_u32, |v| Attribute::VsaUsrSpeedOfConnection(UsrSpeedOfConnection(v))},
		// out of u8 bounds 		38914 => map!{i, be_u32, |v| Attribute::VsaUsrMaxChannels(v)},
		// out of u8 bounds 		38915 => map!{i, be_u32, |v| Attribute::VsaUsrChannelExpansion(v)},
		// out of u8 bounds 		38916 => map!{i, be_u32, |v| Attribute::VsaUsrChannelDecrement(v)},
		// out of u8 bounds 		38917 => map! {i, be_u32, |v| Attribute::VsaUsrExpansionAlgorithm(UsrExpansionAlgorithm(v))},
		// out of u8 bounds 		38918 => map! {i, be_u32, |v| Attribute::VsaUsrCompressionAlgorithm(UsrCompressionAlgorithm(v))},
		// out of u8 bounds 		38919 => map!{i, be_u32, |v| Attribute::VsaUsrReceiveAccMap(v)},
		// out of u8 bounds 		38920 => map!{i, be_u32, |v| Attribute::VsaUsrTransmitAccMap(v)},
		// out of u8 bounds 		38922 => map! {i, be_u32, |v| Attribute::VsaUsrCompressionResetMode(UsrCompressionResetMode(v))},
		// out of u8 bounds 		38923 => map!{i, be_u32, |v| Attribute::VsaUsrMinCompressionSize(v)},
		// out of u8 bounds 		38924 => map!{i, be_u32, |v| Attribute::VsaUsrIp(v)},
		// out of u8 bounds 		38925 => map!{i, be_u32, |v| Attribute::VsaUsrIpx(v)},
		// out of u8 bounds 		38926 => map! {i, be_u32, |v| Attribute::VsaUsrFilterZones(UsrFilterZones(v))},
		// out of u8 bounds 		38927 => map! {i, be_u32, |v| Attribute::VsaUsrAppletalk(UsrAppletalk(v))},
		// out of u8 bounds 		38928 => map! {i, be_u32, |v| Attribute::VsaUsrBridging(UsrBridging(v))},
		// out of u8 bounds 		38929 => map! {i, be_u32, |v| Attribute::VsaUsrSpoofing(UsrSpoofing(v))},
		// out of u8 bounds 		38930 => map!{i, be_u32, |v| Attribute::VsaUsrHostType(v)},
		// out of u8 bounds 		38931 => value!(i, Attribute::VsaUsrSendName(i)),
		// out of u8 bounds 		38932 => value!(i, Attribute::VsaUsrSendPassword(i)),
		// out of u8 bounds 		38933 => map!{i, be_u32, |v| Attribute::VsaUsrStartTime(v)},
		// out of u8 bounds 		38934 => map!{i, be_u32, |v| Attribute::VsaUsrEndTime(v)},
		// out of u8 bounds 		38935 => value!(i, Attribute::VsaUsrSendScript1(i)),
		// out of u8 bounds 		38936 => value!(i, Attribute::VsaUsrReplyScript1(i)),
		// out of u8 bounds 		38937 => value!(i, Attribute::VsaUsrSendScript2(i)),
		// out of u8 bounds 		38938 => value!(i, Attribute::VsaUsrReplyScript2(i)),
		// out of u8 bounds 		38939 => value!(i, Attribute::VsaUsrSendScript3(i)),
		// out of u8 bounds 		38940 => value!(i, Attribute::VsaUsrReplyScript3(i)),
		// out of u8 bounds 		38941 => value!(i, Attribute::VsaUsrSendScript4(i)),
		// out of u8 bounds 		38942 => value!(i, Attribute::VsaUsrReplyScript4(i)),
		// out of u8 bounds 		38943 => value!(i, Attribute::VsaUsrSendScript5(i)),
		// out of u8 bounds 		38944 => value!(i, Attribute::VsaUsrReplyScript5(i)),
		// out of u8 bounds 		38945 => value!(i, Attribute::VsaUsrSendScript6(i)),
		// out of u8 bounds 		38946 => value!(i, Attribute::VsaUsrReplyScript6(i)),
		// out of u8 bounds 		38947 => value!(i, Attribute::VsaUsrTerminalType(i)),
		// out of u8 bounds 		38948 => map!{i, be_u32, |v| Attribute::VsaUsrAppletalkNetworkRange(v)},
		// out of u8 bounds 		38949 => value!(i, Attribute::VsaUsrLocalIpAddress(i)),
		// out of u8 bounds 		38950 => map! {i, be_u32, |v| Attribute::VsaUsrRoutingProtocol(UsrRoutingProtocol(v))},
		// out of u8 bounds 		38951 => map!{i, be_u32, |v| Attribute::VsaUsrModemGroup(v)},
		// out of u8 bounds 		38978 => map!{i, be_u32, |v| Attribute::VsaUsrModemTrainingTime(v)},
		// out of u8 bounds 		38979 => map!{i, be_u32, |v| Attribute::VsaUsrInterfaceIndex(v)},
		// out of u8 bounds 		38989 => map!{i, be_u32, |v| Attribute::VsaUsrMulticastProxy(v)},
		// out of u8 bounds 		38992 => map!{i, be_u32, |v| Attribute::VsaUsrMulticastForwarding(v)},
		// out of u8 bounds 		38959 => map!{i, be_u32, |v| Attribute::VsaUsrMpMrru(v)},
		// out of u8 bounds 		36866 => value!(i, Attribute::VsaUsrSapFilterIn(i)),
		// out of u8 bounds 		36884 => value!(i, Attribute::VsaUsrMic(i)),
		// out of u8 bounds 		36887 => value!(i, Attribute::VsaUsrLogFilterPackets(i)),
		// out of u8 bounds 		36894 => map!{i, be_u32, |v| Attribute::VsaUsrVpnEncrypter(v)},
		// out of u8 bounds 		36896 => map!{i, be_u32, |v| Attribute::VsaUsrReChapTimeout(v)},
		// out of u8 bounds 		39016 => value!(i, Attribute::VsaUsrTunnelSwitchEndpoint(i)),
		// out of u8 bounds 		39024 => map!{i, be_u32, |v| Attribute::VsaUsrIpSaaFilter(v)},
		// out of u8 bounds 		2339 => map! {i, be_u32, |v| Attribute::VsaInitialModulationType(InitialModulationType(v))},
		// out of u8 bounds 		38998 => value!(i, Attribute::VsaUsrVtsSessionKey(i)),
		// out of u8 bounds 		38999 => value!(i, Attribute::VsaUsrOrigNasType(i)),
		// out of u8 bounds 		39000 => map!{i, be_u32, |v| Attribute::VsaUsrCallArrivalTime(v)},
		// out of u8 bounds 		39001 => map!{i, be_u32, |v| Attribute::VsaUsrCallEndTime(v)},
		// out of u8 bounds 		39019 => value!(i, Attribute::VsaUsrTunnelAuthHostname(i)),
		// out of u8 bounds 		39020 => map!{i, be_u32, |v| Attribute::VsaUsrAcctReasonCode(v)},
		// out of u8 bounds 		39049 => map!{i, be_u32, |v| Attribute::VsaUsrSupportsTags(v)},
		// out of u8 bounds 		39051 => map! {i, be_u32, |v| Attribute::VsaUsrHarcDisconnectCode(UsrHarcDisconnectCode(v))},
		// out of u8 bounds 		461 => map! {i, be_u32, |v| Attribute::VsaUsrRmmieStatus(UsrRmmieStatus(v))},
		// out of u8 bounds 		2305 => map! {i, be_u32, |v| Attribute::VsaUsrRmmieLastUpdateEvent(UsrRmmieLastUpdateEvent(v))},
		// out of u8 bounds 		2313 => map! {i, be_u32, |v| Attribute::VsaUsrRmmieX2Status(UsrRmmieX2Status(v))},
		// out of u8 bounds 		2314 => map! {i, be_u32, |v| Attribute::VsaUsrRmmiePlannedDisconnect(UsrRmmiePlannedDisconnect(v))},
		// out of u8 bounds 		36895 => value!(i, Attribute::VsaUsrVpnGwLocationId(i)),
		// out of u8 bounds 		36897 => map! {i, be_u32, |v| Attribute::VsaUsrCcpAlgorithm(UsrCcpAlgorithm(v))},
		// out of u8 bounds 		36898 => map!{i, be_u32, |v| Attribute::VsaUsrAccmType(v)},
		// out of u8 bounds 		36902 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrLocalFramedIpAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		38952 => map! {i, be_u32, |v| Attribute::VsaUsrIpxRouting(UsrIpxRouting(v))},
		// out of u8 bounds 		38953 => map! {i, be_u32, |v| Attribute::VsaUsrIpxWan(UsrIpxWan(v))},
		// out of u8 bounds 		38954 => map! {i, be_u32, |v| Attribute::VsaUsrIpRipPolicies(UsrIpRipPolicies(v))},
		// out of u8 bounds 		38955 => value!(i, Attribute::VsaUsrIpRipSimpleAuthPassword(i)),
		// out of u8 bounds 		38956 => value!(i, Attribute::VsaUsrIpRipInputFilter(i)),
		// out of u8 bounds 		38957 => value!(i, Attribute::VsaUsrIpCallInputFilter(i)),
		// out of u8 bounds 		38958 => value!(i, Attribute::VsaUsrIpxRipInputFilter(i)),
		// out of u8 bounds 		38960 => value!(i, Attribute::VsaUsrIpxCallInputFilter(i)),
		// out of u8 bounds 		38961 => value!(i, Attribute::VsaUsrAtInputFilter(i)),
		// out of u8 bounds 		38962 => value!(i, Attribute::VsaUsrAtRtmpInputFilter(i)),
		// out of u8 bounds 		38963 => value!(i, Attribute::VsaUsrAtZipInputFilter(i)),
		// out of u8 bounds 		38964 => value!(i, Attribute::VsaUsrAtCallInputFilter(i)),
		// out of u8 bounds 		38965 => value!(i, Attribute::VsaUsrEtBridgeInputFilter(i)),
		// out of u8 bounds 		38966 => value!(i, Attribute::VsaUsrIpRipOutputFilter(i)),
		// out of u8 bounds 		38967 => value!(i, Attribute::VsaUsrIpCallOutputFilter(i)),
		// out of u8 bounds 		38968 => value!(i, Attribute::VsaUsrIpxRipOutputFilter(i)),
		// out of u8 bounds 		38969 => value!(i, Attribute::VsaUsrIpxCallOutputFilter(i)),
		// out of u8 bounds 		38970 => value!(i, Attribute::VsaUsrAtOutputFilter(i)),
		// out of u8 bounds 		38971 => value!(i, Attribute::VsaUsrAtRtmpOutputFilter(i)),
		// out of u8 bounds 		38972 => value!(i, Attribute::VsaUsrAtZipOutputFilter(i)),
		// out of u8 bounds 		38973 => value!(i, Attribute::VsaUsrAtCallOutputFilter(i)),
		// out of u8 bounds 		38974 => value!(i, Attribute::VsaUsrEtBridgeOutputFilter(i)),
		// out of u8 bounds 		38975 => value!(i, Attribute::VsaUsrEtBridgeCallOutputFilte(i)),
		// out of u8 bounds 		38976 => map! {i, be_u32, |v| Attribute::VsaUsrIpDefaultRouteOption(UsrIpDefaultRouteOption(v))},
		// out of u8 bounds 		38977 => value!(i, Attribute::VsaUsrMpEdoHiper(i)),
		// out of u8 bounds 		38980 => map! {i, be_u32, |v| Attribute::VsaUsrTunnelSecurity(UsrTunnelSecurity(v))},
		// out of u8 bounds 		38981 => map!{i, be_u32, |v| Attribute::VsaUsrPortTap(v)},
		// out of u8 bounds 		38982 => map!{i, be_u32, |v| Attribute::VsaUsrPortTapFormat(v)},
		// out of u8 bounds 		38983 => map!{i, be_u32, |v| Attribute::VsaUsrPortTapOutput(v)},
		// out of u8 bounds 		38984 => map!{i, be_u32, |v| Attribute::VsaUsrPortTapFacility(v)},
		// out of u8 bounds 		38985 => map!{i, be_u32, |v| Attribute::VsaUsrPortTapPriority(v)},
		// out of u8 bounds 		38986 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrPortTapAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		38987 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrMobileipHomeAgentAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		38988 => map!{i, be_u32, |v| Attribute::VsaUsrTunneledMlpp(v)},
		// out of u8 bounds 		38990 => map!{i, be_u32, |v| Attribute::VsaUsrMulticastReceive(v)},
		// out of u8 bounds 		38993 => map!{i, be_u32, |v| Attribute::VsaUsrIgmpQueryInterval(v)},
		// out of u8 bounds 		38994 => map!{i, be_u32, |v| Attribute::VsaUsrIgmpMaximumResponseTime(v)},
		// out of u8 bounds 		38995 => map!{i, be_u32, |v| Attribute::VsaUsrIgmpRobustness(v)},
		// out of u8 bounds 		38996 => map!{i, be_u32, |v| Attribute::VsaUsrIgmpVersion(v)},
		// out of u8 bounds 		39018 => map! {i, be_u32, |v| Attribute::VsaUsrCallbackType(UsrCallbackType(v))},
		// out of u8 bounds 		61441 => map! {i, be_u32, |v| Attribute::VsaUsrRequestType(UsrRequestType(v))},
		// out of u8 bounds 		462 => map!{i, be_u32, |v| Attribute::VsaUsrRmmieNumOfUpdates(v)},
		// out of u8 bounds 		479 => map!{i, be_u32, |v| Attribute::VsaUsrRmmieManufacturerId(v)},
		// out of u8 bounds 		480 => value!(i, Attribute::VsaUsrRmmieProductCode(i)),
		// out of u8 bounds 		481 => value!(i, Attribute::VsaUsrRmmieSerialNumber(i)),
		// out of u8 bounds 		482 => value!(i, Attribute::VsaUsrRmmieFirmwareVersion(i)),
		// out of u8 bounds 		483 => value!(i, Attribute::VsaUsrRmmieFirmwareBuildDate(i)),
		// out of u8 bounds 		48722 => map!{i, be_u32, |v| Attribute::VsaUsrCallArrivalInGmt(v)},
		// out of u8 bounds 		48721 => map!{i, be_u32, |v| Attribute::VsaUsrCallConnectInGmt(v)},
		// out of u8 bounds 		48720 => map!{i, be_u32, |v| Attribute::VsaUsrCallTerminateInGmt(v)},
		// out of u8 bounds 		48719 => map!{i, be_u32, |v| Attribute::VsaUsrIds0CallType(v)},
		// out of u8 bounds 		48765 => map!{i, be_u32, |v| Attribute::VsaUsrCallReferenceNumber(v)},
		// out of u8 bounds 		387 => map!{i, be_u32, |v| Attribute::VsaUsrCdmaCallReferenceNumber(v)},
		// out of u8 bounds 		2190 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrMobileIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		2292 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrQnc1ServiceDestination(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		1012 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrIwfIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		2192 => value!(i, Attribute::VsaUsrCalledPartyNumber(i)),
		// out of u8 bounds 		2191 => value!(i, Attribute::VsaUsrCallingPartyNumber(i)),
		// out of u8 bounds 		2193 => map!{i, be_u32, |v| Attribute::VsaUsrCallType(v)},
		// out of u8 bounds 		2194 => value!(i, Attribute::VsaUsrEsn(i)),
		// out of u8 bounds 		2195 => map!{i, be_u32, |v| Attribute::VsaUsrIwfCallIdentifier(v)},
		// out of u8 bounds 		2196 => value!(i, Attribute::VsaUsrImsi(i)),
		// out of u8 bounds 		2197 => map!{i, be_u32, |v| Attribute::VsaUsrServiceOption(v)},
		// out of u8 bounds 		2198 => map!{i, be_u32, |v| Attribute::VsaUsrDisconnectCauseIndicator(v)},
		// out of u8 bounds 		2199 => map!{i, be_u32, |v| Attribute::VsaUsrMobileNumbytesTxed(v)},
		// out of u8 bounds 		2200 => map!{i, be_u32, |v| Attribute::VsaUsrMobileNumbytesRxed(v)},
		// out of u8 bounds 		2201 => map!{i, be_u32, |v| Attribute::VsaUsrNumFaxPagesProcessed(v)},
		// out of u8 bounds 		2202 => map!{i, be_u32, |v| Attribute::VsaUsrCompressionType(v)},
		// out of u8 bounds 		2203 => map!{i, be_u32, |v| Attribute::VsaUsrCallErrorCode(v)},
		// out of u8 bounds 		2204 => map!{i, be_u32, |v| Attribute::VsaUsrModemSetupTime(v)},
		// out of u8 bounds 		2205 => map!{i, be_u32, |v| Attribute::VsaUsrCallConnectingTime(v)},
		// out of u8 bounds 		2206 => map!{i, be_u32, |v| Attribute::VsaUsrConnectTime(v)},
		// out of u8 bounds 		2304 => map!{i, be_u32, |v| Attribute::VsaUsrRmmieLastUpdateTime(v)},
		// out of u8 bounds 		2306 => map!{i, be_u32, |v| Attribute::VsaUsrRmmieRcvTotPwrlvl(v)},
		// out of u8 bounds 		2307 => map!{i, be_u32, |v| Attribute::VsaUsrRmmieRcvPwrlvl3300Hz(v)},
		// out of u8 bounds 		2308 => map!{i, be_u32, |v| Attribute::VsaUsrRmmieRcvPwrlvl3750Hz(v)},
		// out of u8 bounds 		2309 => map!{i, be_u32, |v| Attribute::VsaUsrRmmiePwrlvlNearechoCanc(v)},
		// out of u8 bounds 		2310 => map!{i, be_u32, |v| Attribute::VsaUsrRmmiePwrlvlFarechoCanc(v)},
		// out of u8 bounds 		2311 => map!{i, be_u32, |v| Attribute::VsaUsrRmmiePwrlvlNoiseLvl(v)},
		// out of u8 bounds 		2312 => map!{i, be_u32, |v| Attribute::VsaUsrRmmiePwrlvlXmitLvl(v)},
		// out of u8 bounds 		36903 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrFramedIpxRoute(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		36904 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrMpipTunnelOriginator(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		38997 => map!{i, be_u32, |v| Attribute::VsaUsrIgmpRouting(v)},
		// out of u8 bounds 		39008 => map!{i, be_u32, |v| Attribute::VsaUsrRadMulticastRoutingTtl(v)},
		// out of u8 bounds 		39009 => map!{i, be_u32, |v| Attribute::VsaUsrRadMulticastRoutingRtlim(v)},
		// out of u8 bounds 		39010 => map!{i, be_u32, |v| Attribute::VsaUsrRadMulticastRoutingProto(v)},
		// out of u8 bounds 		39011 => value!(i, Attribute::VsaUsrRadMulticastRoutingBound(i)),
		// out of u8 bounds 		39012 => map!{i, be_u32, |v| Attribute::VsaUsrRadDvmrpMetric(v)},
		// out of u8 bounds 		39013 => value!(i, Attribute::VsaUsrChatScriptName(i)),
		// out of u8 bounds 		39014 => value!(i, Attribute::VsaUsrCusrHatScriptRules(i)),
		// out of u8 bounds 		39015 => map!{i, be_u32, |v| Attribute::VsaUsrRadLocationType(v)},
		// out of u8 bounds 		39017 => map!{i, be_u32, |v| Attribute::VsaUsrOspfAddresslessIndex(v)},
		// out of u8 bounds 		39021 => map!{i, be_u32, |v| Attribute::VsaUsrQosQueuingMehtod(v)},
		// out of u8 bounds 		39022 => map!{i, be_u32, |v| Attribute::VsaUsrPqDefaultPriority(v)},
		// out of u8 bounds 		39025 => map!{i, be_u32, |v| Attribute::VsaUsrFqDefaultPriority(v)},
		// out of u8 bounds 		39026 => map!{i, be_u32, |v| Attribute::VsaUsrIppEnable(v)},
		// out of u8 bounds 		39027 => value!(i, Attribute::VsaUsrPreSharedMnKey(i)),
		// out of u8 bounds 		39028 => map!{i, be_u32, |v| Attribute::VsaUsrMipNai(v)},
		// out of u8 bounds 		39029 => map!{i, be_u32, |v| Attribute::VsaUsrDnisReauthentication(v)},
		// out of u8 bounds 		39030 => map! {i, be_u32, |v| Attribute::VsaUsrAgent(UsrAgent(v))},
		// out of u8 bounds 		39031 => map!{i, be_u32, |v| Attribute::VsaUsrPqParameters(v)},
		// out of u8 bounds 		39032 => map!{i, be_u32, |v| Attribute::VsaUsrDvmrpPruneLifetime(v)},
		// out of u8 bounds 		39033 => map!{i, be_u32, |v| Attribute::VsaUsrSpecialXonXoffFlow(v)},
		// out of u8 bounds 		39034 => map!{i, be_u32, |v| Attribute::VsaUsrDvmrpAdvertisedMetric(v)},
		// out of u8 bounds 		39035 => map!{i, be_u32, |v| Attribute::VsaUsrDvmrpRetransmitPrunes(v)},
		// out of u8 bounds 		39036 => map!{i, be_u32, |v| Attribute::VsaUsrDvmrpNonPruners(v)},
		// out of u8 bounds 		39037 => map!{i, be_u32, |v| Attribute::VsaUsrDvmrpRouteTransit(v)},
		// out of u8 bounds 		39038 => value!(i, Attribute::VsaUsrDvmrpInputFilter(i)),
		// out of u8 bounds 		39040 => value!(i, Attribute::VsaUsrDvmrpOutputFilter(i)),
		// out of u8 bounds 		39041 => map!{i, be_u32, |v| Attribute::VsaUsrPolicyAccess(v)},
		// out of u8 bounds 		39042 => map!{i, be_u32, |v| Attribute::VsaUsrPolicyConfiguration(v)},
		// out of u8 bounds 		39043 => value!(i, Attribute::VsaUsrPolicyFilename(i)),
		// out of u8 bounds 		39044 => map!{i, be_u32, |v| Attribute::VsaUsrPolicyType(v)},
		// out of u8 bounds 		39045 => map!{i, be_u32, |v| Attribute::VsaUsrMobileSessionId(v)},
		// out of u8 bounds 		39046 => map!{i, be_u32, |v| Attribute::VsaUsrMobileAccountingType(v)},
		// out of u8 bounds 		39047 => map!{i, be_u32, |v| Attribute::VsaUsrMobileServiceOption(v)},
		// out of u8 bounds 		39048 => map!{i, be_u32, |v| Attribute::VsaUsrWallclockTimestamp(v)},
		// out of u8 bounds 		39050 => map!{i, be_u32, |v| Attribute::VsaUsrDvmrpInitialFlooding(v)},
		// out of u8 bounds 		39052 => map!{i, be_u32, |v| Attribute::VsaUsrTelnetOptions(v)},
		// out of u8 bounds 		39053 => map!{i, be_u32, |v| Attribute::VsaUsrCdmaPktdataNetworkId(v)},
		// out of u8 bounds 		39054 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrAuthNextServerAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		39055 => map!{i, be_u32, |v| Attribute::VsaUsrUserPppAodiType(v)},
		// out of u8 bounds 		39056 => map!{i, be_u32, |v| Attribute::VsaUsrMlpppFragmentationThreshld(v)},
		// out of u8 bounds 		39057 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrUnnumberedLocalIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		39058 => map!{i, be_u32, |v| Attribute::VsaUsrTrafficThreshold(v)},
		// out of u8 bounds 		39059 => map!{i, be_u32, |v| Attribute::VsaUsrKeepAliveInterval(v)},
		// out of u8 bounds 		39060 => map!{i, be_u32, |v| Attribute::VsaVirtualServerId(v)},
		// out of u8 bounds 		39061 => value!(i, Attribute::VsaUsrX25TrunkProfile(i)),
		// out of u8 bounds 		39062 => map!{i, be_u32, |v| Attribute::VsaUsrX25AcctInputSegmentCount(v)},
		// out of u8 bounds 		39063 => map!{i, be_u32, |v| Attribute::VsaUsrX25AcctOutputSegmentCoun(v)},
		// out of u8 bounds 		39064 => map!{i, be_u32, |v| Attribute::VsaUsrX25AcctSegmentSize(v)},
		// out of u8 bounds 		39065 => map!{i, be_u32, |v| Attribute::VsaUsrX25AcctTerminationCode(v)},
		// out of u8 bounds 		39066 => map!{i, be_u32, |v| Attribute::VsaUsrX25SvcLogicalChannelNumb(v)},
		// out of u8 bounds 		39067 => map!{i, be_u32, |v| Attribute::VsaUsrNailedBChannelIndicator(v)},
		// out of u8 bounds 		39068 => map!{i, be_u32, |v| Attribute::VsaUsrX25SvcCallAttributes(v)},
		// out of u8 bounds 		39069 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrInitRegServerAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		39070 => map!{i, take!(4), |v:&[u8]| Attribute::VsaUsrReRegServerAddr(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		39071 => map!{i, be_u32, |v| Attribute::VsaUsrBytesTxRemain(v)},
		// out of u8 bounds 		39072 => map!{i, be_u32, |v| Attribute::VsaUsrBytesRxRemain(v)},
		// out of u8 bounds 		39073 => map!{i, be_u32, |v| Attribute::VsaUsrSessionTimeRemain(v)},
		// out of u8 bounds 		39074 => map! {i, be_u32, |v| Attribute::VsaUsrPrePaidEnabled(UsrPrePaidEnabled(v))},
		// out of u8 bounds 		39075 => map!{i, be_u32, |v| Attribute::VsaUsrRegServerProvTimeout(v)},
		// out of u8 bounds 		39076 => map!{i, be_u32, |v| Attribute::VsaUsrRedirect(v)},
		// out of u8 bounds 		39077 => map!{i, be_u32, |v| Attribute::VsaUsrVlanTag(v)},
		// out of u8 bounds 		39078 => value!(i, Attribute::VsaUsrRadIpPoolDefinition(i)),
		// out of u8 bounds 		39079 => map!{i, be_u32, |v| Attribute::VsaUsrRadNmcCallProgressStatus(v)},
		// out of u8 bounds 		39080 => map!{i, be_u32, |v| Attribute::VsaUsrRadNmcBlocksRx(v)},
		// out of u8 bounds 		39096 => map!{i, be_u32, |v| Attribute::VsaUsrTotalBytesRemain(v)},
		// out of u8 bounds 		39097 => map!{i, be_u32, |v| Attribute::VsaUsrForwardRateLimit(v)},
		// out of u8 bounds 		39100 => map!{i, be_u32, |v| Attribute::VsaUsrReverseRateLimit(v)},
		// out of u8 bounds 		61442 => map! {i, be_u32, |v| Attribute::VsaUsrNasType(UsrNasType(v))},
		// out of u8 bounds 		61443 => map! {i, be_u32, |v| Attribute::VsaUsrAuthMode(UsrAuthMode(v))},
        _ => value!(i, Attribute::VsaUnknown(429, typ, i)),
    }
}
