/// Definitions for vendor Bay-Networks, vendor value 1584
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassportAccessPriority(pub u32);
 
#[allow(non_upper_case_globals)]
impl PassportAccessPriority {
	pub const NoneAccess: PassportAccessPriority = PassportAccessPriority(0);
	pub const ReadOnlyAccess: PassportAccessPriority = PassportAccessPriority(1);
	pub const L1ReadWriteAccess: PassportAccessPriority = PassportAccessPriority(2);
	pub const L2ReadWriteAccess: PassportAccessPriority = PassportAccessPriority(3);
	pub const L3ReadWriteAccess: PassportAccessPriority = PassportAccessPriority(4);
	pub const ReadWriteAccess: PassportAccessPriority = PassportAccessPriority(5);
	pub const ReadWriteAllAccess: PassportAccessPriority = PassportAccessPriority(6);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnnexCommandAccess(pub u32);
 
#[allow(non_upper_case_globals)]
impl AnnexCommandAccess {
	pub const False: AnnexCommandAccess = AnnexCommandAccess(0);
	pub const True: AnnexCommandAccess = AnnexCommandAccess(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnnexTunnelAuthenType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AnnexTunnelAuthenType {
	pub const None: AnnexTunnelAuthenType = AnnexTunnelAuthenType(0);
	pub const Kmd5128: AnnexTunnelAuthenType = AnnexTunnelAuthenType(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnnexTunnelAuthenMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AnnexTunnelAuthenMode {
	pub const None: AnnexTunnelAuthenMode = AnnexTunnelAuthenMode(0);
	pub const PrefixSuffix: AnnexTunnelAuthenMode = AnnexTunnelAuthenMode(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnnexUserServerLocation(pub u32);
 
#[allow(non_upper_case_globals)]
impl AnnexUserServerLocation {
	pub const Local: AnnexUserServerLocation = AnnexUserServerLocation(1);
	pub const Remote: AnnexUserServerLocation = AnnexUserServerLocation(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnnexAddrResolutionProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl AnnexAddrResolutionProtocol {
	pub const None: AnnexAddrResolutionProtocol = AnnexAddrResolutionProtocol(0);
	pub const Dhcp: AnnexAddrResolutionProtocol = AnnexAddrResolutionProtocol(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnnexSystemDiscReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl AnnexSystemDiscReason {
	pub const Unknown: AnnexSystemDiscReason = AnnexSystemDiscReason(0);
	pub const LineDisconnected: AnnexSystemDiscReason = AnnexSystemDiscReason(1);
	pub const DialFailed: AnnexSystemDiscReason = AnnexSystemDiscReason(2);
	pub const WanManagerError: AnnexSystemDiscReason = AnnexSystemDiscReason(3);
	pub const DisconnectReset: AnnexSystemDiscReason = AnnexSystemDiscReason(4);
	pub const ErrorFromAdmNotify: AnnexSystemDiscReason = AnnexSystemDiscReason(5);
	pub const ModemDownAdmNotify: AnnexSystemDiscReason = AnnexSystemDiscReason(6);
	pub const PppProtocolDisconnect: AnnexSystemDiscReason = AnnexSystemDiscReason(7);
	pub const InactivityTimer: AnnexSystemDiscReason = AnnexSystemDiscReason(8);
	pub const CliHangupCommand: AnnexSystemDiscReason = AnnexSystemDiscReason(9);
	pub const CliLastJob: AnnexSystemDiscReason = AnnexSystemDiscReason(10);
	pub const SessionTimeout: AnnexSystemDiscReason = AnnexSystemDiscReason(11);
	pub const SlaveTermination: AnnexSystemDiscReason = AnnexSystemDiscReason(12);
	pub const AbnormalTermination: AnnexSystemDiscReason = AnnexSystemDiscReason(13);
	pub const DcdWaitFailed: AnnexSystemDiscReason = AnnexSystemDiscReason(14);
	pub const CliInactivity: AnnexSystemDiscReason = AnnexSystemDiscReason(15);
	pub const AdminPortReset: AnnexSystemDiscReason = AnnexSystemDiscReason(16);
	pub const CliAuthFailed: AnnexSystemDiscReason = AnnexSystemDiscReason(17);
	pub const SlaveAuthFailed: AnnexSystemDiscReason = AnnexSystemDiscReason(18);
	pub const PapAuthFailed: AnnexSystemDiscReason = AnnexSystemDiscReason(19);
	pub const ChapAuthFailed: AnnexSystemDiscReason = AnnexSystemDiscReason(20);
	pub const LocalModemReset: AnnexSystemDiscReason = AnnexSystemDiscReason(21);
	pub const ModemDead: AnnexSystemDiscReason = AnnexSystemDiscReason(22);
	pub const PppLcpFailure: AnnexSystemDiscReason = AnnexSystemDiscReason(23);
	pub const PppIpcpFailure: AnnexSystemDiscReason = AnnexSystemDiscReason(24);
	pub const PppIpxcpFailure: AnnexSystemDiscReason = AnnexSystemDiscReason(25);
	pub const PppAtcpFailure: AnnexSystemDiscReason = AnnexSystemDiscReason(26);
	pub const PppCcpFailure: AnnexSystemDiscReason = AnnexSystemDiscReason(27);
	pub const PppMpFailure: AnnexSystemDiscReason = AnnexSystemDiscReason(28);
	pub const PppIpcpTimeout: AnnexSystemDiscReason = AnnexSystemDiscReason(29);
	pub const PppIpxcpTimeout: AnnexSystemDiscReason = AnnexSystemDiscReason(30);
	pub const PppAtcpTimeout: AnnexSystemDiscReason = AnnexSystemDiscReason(31);
	pub const PppCcpTimeout: AnnexSystemDiscReason = AnnexSystemDiscReason(32);
	pub const PppMpTimeout: AnnexSystemDiscReason = AnnexSystemDiscReason(33);
	pub const PppInitFailure: AnnexSystemDiscReason = AnnexSystemDiscReason(34);
	pub const PppUnknown: AnnexSystemDiscReason = AnnexSystemDiscReason(35);
	pub const PppDialbackFailed: AnnexSystemDiscReason = AnnexSystemDiscReason(36);
	pub const PppAddressInUse: AnnexSystemDiscReason = AnnexSystemDiscReason(37);
	pub const PppNoDevice: AnnexSystemDiscReason = AnnexSystemDiscReason(38);
	pub const PppModemHangupRcvd: AnnexSystemDiscReason = AnnexSystemDiscReason(39);
	pub const PppHangupRcvd: AnnexSystemDiscReason = AnnexSystemDiscReason(40);
	pub const PppTerminationRcvd: AnnexSystemDiscReason = AnnexSystemDiscReason(41);
	pub const PppKillRcvd: AnnexSystemDiscReason = AnnexSystemDiscReason(42);
	pub const PppTimeRcvd: AnnexSystemDiscReason = AnnexSystemDiscReason(43);
	pub const PppNoMemory: AnnexSystemDiscReason = AnnexSystemDiscReason(44);
	pub const PppConnectionAbort: AnnexSystemDiscReason = AnnexSystemDiscReason(45);
	pub const PppVpnLcpFailure: AnnexSystemDiscReason = AnnexSystemDiscReason(46);
	pub const PppVpnAuthFailure: AnnexSystemDiscReason = AnnexSystemDiscReason(47);
	pub const PppMpInvalidPort: AnnexSystemDiscReason = AnnexSystemDiscReason(48);
	pub const PppInvalidDevice: AnnexSystemDiscReason = AnnexSystemDiscReason(49);
	pub const PppMmpBundleFailure: AnnexSystemDiscReason = AnnexSystemDiscReason(50);
	pub const DvsRegistrationFailure: AnnexSystemDiscReason = AnnexSystemDiscReason(51);
	pub const DvsHomeAgentDereg: AnnexSystemDiscReason = AnnexSystemDiscReason(52);
	pub const DvsTunnelNoRenew: AnnexSystemDiscReason = AnnexSystemDiscReason(53);
	pub const DvsTunnelExpired: AnnexSystemDiscReason = AnnexSystemDiscReason(54);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnnexModemDiscReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl AnnexModemDiscReason {
	pub const Unknown: AnnexModemDiscReason = AnnexModemDiscReason(0);
	pub const LocalDisconnect: AnnexModemDiscReason = AnnexModemDiscReason(1);
	pub const CdTimerExpired: AnnexModemDiscReason = AnnexModemDiscReason(2);
	pub const RemoteProtocolDisc: AnnexModemDiscReason = AnnexModemDiscReason(4);
	pub const ClearDown: AnnexModemDiscReason = AnnexModemDiscReason(5);
	pub const LongSpaceDisconnect: AnnexModemDiscReason = AnnexModemDiscReason(6);
	pub const CarrierLost: AnnexModemDiscReason = AnnexModemDiscReason(7);
	pub const ModemRetrainTimeout: AnnexModemDiscReason = AnnexModemDiscReason(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnnexUserLevel(pub u32);
 
#[allow(non_upper_case_globals)]
impl AnnexUserLevel {
	pub const Manager: AnnexUserLevel = AnnexUserLevel(2);
	pub const User: AnnexUserLevel = AnnexUserLevel(4);
	pub const Operator: AnnexUserLevel = AnnexUserLevel(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnnexAuditLevel(pub u32);
 
#[allow(non_upper_case_globals)]
impl AnnexAuditLevel {
	pub const Manager: AnnexAuditLevel = AnnexAuditLevel(2);
	pub const User: AnnexAuditLevel = AnnexAuditLevel(4);
	pub const Operator: AnnexAuditLevel = AnnexAuditLevel(8);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		28 => value!(i, Attribute::VsaAnnexFilter(i)),
		29 => value!(i, Attribute::VsaAnnexCliCommand(i)),
		30 => value!(i, Attribute::VsaAnnexCliFilter(i)),
		31 => value!(i, Attribute::VsaAnnexHostRestrict(i)),
		32 => value!(i, Attribute::VsaAnnexHostAllow(i)),
		33 => value!(i, Attribute::VsaAnnexProductName(i)),
		34 => value!(i, Attribute::VsaAnnexSwVersion(i)),
		35 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAnnexLocalIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		36 => map!{i, be_u32, |v| Attribute::VsaAnnexCallbackPortlist(v)},
		37 => map!{i, be_u32, |v| Attribute::VsaAnnexSecProfileIndex(v)},
		38 => map! {i, be_u32, |v| Attribute::VsaAnnexTunnelAuthenType(AnnexTunnelAuthenType(v))},
		39 => map! {i, be_u32, |v| Attribute::VsaAnnexTunnelAuthenMode(AnnexTunnelAuthenMode(v))},
		40 => value!(i, Attribute::VsaAnnexAuthenServers(i)),
		41 => value!(i, Attribute::VsaAnnexAcctServers(i)),
		42 => map! {i, be_u32, |v| Attribute::VsaAnnexUserServerLocation(AnnexUserServerLocation(v))},
		43 => value!(i, Attribute::VsaAnnexLocalUsername(i)),
		44 => map! {i, be_u32, |v| Attribute::VsaAnnexSystemDiscReason(AnnexSystemDiscReason(v))},
		45 => map! {i, be_u32, |v| Attribute::VsaAnnexModemDiscReason(AnnexModemDiscReason(v))},
		46 => map!{i, be_u32, |v| Attribute::VsaAnnexDisconnectReason(v)},
		47 => map! {i, be_u32, |v| Attribute::VsaAnnexAddrResolutionProtocol(AnnexAddrResolutionProtocol(v))},
		48 => value!(i, Attribute::VsaAnnexAddrResolutionServers(i)),
		49 => value!(i, Attribute::VsaAnnexDomainName(i)),
		50 => map!{i, be_u32, |v| Attribute::VsaAnnexTransmitSpeed(v)},
		51 => map!{i, be_u32, |v| Attribute::VsaAnnexReceiveSpeed(v)},
		52 => value!(i, Attribute::VsaAnnexInputFilter(i)),
		53 => value!(i, Attribute::VsaAnnexOutputFilter(i)),
		54 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAnnexPrimaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		55 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAnnexSecondaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		56 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAnnexPrimaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		57 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAnnexSecondaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		58 => map!{i, be_u32, |v| Attribute::VsaAnnexSyslogTap(v)},
		59 => map!{i, be_u32, |v| Attribute::VsaAnnexKeypressTimeout(v)},
		60 => map!{i, be_u32, |v| Attribute::VsaAnnexUnauthenticatedTime(v)},
		61 => map!{i, be_u32, |v| Attribute::VsaAnnexReChapTimeout(v)},
		62 => map!{i, be_u32, |v| Attribute::VsaAnnexMrru(v)},
		63 => value!(i, Attribute::VsaAnnexEdo(i)),
		64 => map!{i, be_u32, |v| Attribute::VsaAnnexPppTraceLevel(v)},
		65 => map!{i, be_u32, |v| Attribute::VsaAnnexPreInputOctets(v)},
		66 => map!{i, be_u32, |v| Attribute::VsaAnnexPreOutputOctets(v)},
		67 => map!{i, be_u32, |v| Attribute::VsaAnnexPreInputPackets(v)},
		68 => map!{i, be_u32, |v| Attribute::VsaAnnexPreOutputPackets(v)},
		69 => map!{i, be_u32, |v| Attribute::VsaAnnexConnectProgress(v)},
		73 => map!{i, be_u32, |v| Attribute::VsaAnnexMulticastRateLimit(v)},
		74 => map!{i, be_u32, |v| Attribute::VsaAnnexMaximumCallDuration(v)},
		75 => map!{i, be_u32, |v| Attribute::VsaAnnexMultilinkId(v)},
		76 => map!{i, be_u32, |v| Attribute::VsaAnnexNumInMultilink(v)},
		79 => value!(i, Attribute::VsaAnnexSecondarySrvEndpoint(i)),
		80 => map!{i, be_u32, |v| Attribute::VsaAnnexGwySelectionMode(v)},
		81 => map!{i, be_u32, |v| Attribute::VsaAnnexLogicalChannelNumber(v)},
		82 => map!{i, be_u32, |v| Attribute::VsaAnnexWanNumber(v)},
		83 => map!{i, be_u32, |v| Attribute::VsaAnnexPort(v)},
		85 => map!{i, be_u32, |v| Attribute::VsaAnnexPoolId(v)},
		86 => value!(i, Attribute::VsaAnnexCompressionProtocol(i)),
		87 => map!{i, be_u32, |v| Attribute::VsaAnnexTransmittedPackets(v)},
		88 => map!{i, be_u32, |v| Attribute::VsaAnnexRetransmittedPackets(v)},
		89 => map!{i, be_u32, |v| Attribute::VsaAnnexSignalToNoiseRatio(v)},
		90 => map!{i, be_u32, |v| Attribute::VsaAnnexRetrainRequestsSent(v)},
		91 => map!{i, be_u32, |v| Attribute::VsaAnnexRetrainRequestsRcvd(v)},
		92 => map!{i, be_u32, |v| Attribute::VsaAnnexRateRenegReqSent(v)},
		93 => map!{i, be_u32, |v| Attribute::VsaAnnexRateRenegReqRcvd(v)},
		94 => map!{i, be_u32, |v| Attribute::VsaAnnexBeginReceiveLineLevel(v)},
		95 => map!{i, be_u32, |v| Attribute::VsaAnnexEndReceiveLineLevel(v)},
		96 => value!(i, Attribute::VsaAnnexBeginModulation(i)),
		97 => value!(i, Attribute::VsaAnnexErrorCorrectionProt(i)),
		98 => value!(i, Attribute::VsaAnnexEndModulation(i)),
		100 => map! {i, be_u32, |v| Attribute::VsaAnnexUserLevel(AnnexUserLevel(v))},
		101 => map! {i, be_u32, |v| Attribute::VsaAnnexAuditLevel(AnnexAuditLevel(v))},
		102 => value!(i, Attribute::VsaCesGroup(i)),
		192 => map! {i, be_u32, |v| Attribute::VsaPassportAccessPriority(PassportAccessPriority(v))},
		193 => value!(i, Attribute::VsaAnnexCliCommands(i)),
		194 => map! {i, be_u32, |v| Attribute::VsaAnnexCommandAccess(AnnexCommandAccess(v))},
		195 => value!(i, Attribute::VsaCommands(i)),
        _ => value!(i, Attribute::VsaUnknown(1584, typ, i)),
    }
}
