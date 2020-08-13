/// Definitions for vendor 3GPP2, vendor value 5535
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThreeGpp2AirlinkRecordType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ThreeGpp2AirlinkRecordType {
	pub const ConnectionSetup: ThreeGpp2AirlinkRecordType = ThreeGpp2AirlinkRecordType(1);
	pub const ActiveStart: ThreeGpp2AirlinkRecordType = ThreeGpp2AirlinkRecordType(2);
	pub const ActiveStop: ThreeGpp2AirlinkRecordType = ThreeGpp2AirlinkRecordType(3);
	pub const ShortDataBurst: ThreeGpp2AirlinkRecordType = ThreeGpp2AirlinkRecordType(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthorizationRequiredFlag(pub u32);
 
#[allow(non_upper_case_globals)]
impl AuthorizationRequiredFlag {
	pub const AuthorizationIsNotRequiredForThisFlow: AuthorizationRequiredFlag = AuthorizationRequiredFlag(0);
	pub const AuthorizationIsRequiredForThisFlow: AuthorizationRequiredFlag = AuthorizationRequiredFlag(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReasonCode(pub u32);
 
#[allow(non_upper_case_globals)]
impl ReasonCode {
	pub const Reserved: ReasonCode = ReasonCode(0);
	pub const AuthorizationOnly: ReasonCode = ReasonCode(1);
	pub const SessionDiscoveryOnly: ReasonCode = ReasonCode(2);
	pub const AuthorizationAndSessionDiscovery: ReasonCode = ReasonCode(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeaderCompressionAlgorithm(pub u32);
 
#[allow(non_upper_case_globals)]
impl HeaderCompressionAlgorithm {
	pub const NoHeaderCompression: HeaderCompressionAlgorithm = HeaderCompressionAlgorithm(0);
	pub const RohcUMode: HeaderCompressionAlgorithm = HeaderCompressionAlgorithm(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cid(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cid {
	pub const SmallCid: Cid = Cid(0);
	pub const LargeCid: Cid = Cid(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EncryptionMechanism(pub u32);
 
#[allow(non_upper_case_globals)]
impl EncryptionMechanism {
	pub const HighLayerEncryptionInTheContentServer: EncryptionMechanism = EncryptionMechanism(0);
	pub const LinkLayerEncryptionInTheRan: EncryptionMechanism = EncryptionMechanism(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThreeGpp2PmipBasedMobilityCapability(pub u32);
 
#[allow(non_upper_case_globals)]
impl ThreeGpp2PmipBasedMobilityCapability {
	pub const AgwSupportsTheNetworkPmip4Only: ThreeGpp2PmipBasedMobilityCapability = ThreeGpp2PmipBasedMobilityCapability(1);
	pub const AgwSupportsTheNetworkPmip6Only: ThreeGpp2PmipBasedMobilityCapability = ThreeGpp2PmipBasedMobilityCapability(2);
	pub const AgwSupportsTheBothNetworkPmip4AndPmip6: ThreeGpp2PmipBasedMobilityCapability = ThreeGpp2PmipBasedMobilityCapability(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpdateReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl UpdateReason {
	pub const PreInitializatio: UpdateReason = UpdateReason(1);
	pub const InitialRequest: UpdateReason = UpdateReason(2);
	pub const ThresholdReached: UpdateReason = UpdateReason(3);
	pub const QuotaReached: UpdateReason = UpdateReason(4);
	pub const RemoteForcedDisconnect: UpdateReason = UpdateReason(5);
	pub const ClientServiceTermination: UpdateReason = UpdateReason(6);
	pub const MainSiReleased: UpdateReason = UpdateReason(7);
	pub const ServiceInstanceNotEstablished: UpdateReason = UpdateReason(8);
	pub const TariffSwitchUpdate: UpdateReason = UpdateReason(9);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Availableinclient(pub u32);
 
#[allow(non_upper_case_globals)]
impl Availableinclient {
	pub const PrepaidAccountingForVolumeSupported: Availableinclient = Availableinclient(1);
	pub const PrepaidAccountingForDurationSupported: Availableinclient = Availableinclient(2);
	pub const PrepaidAccountingForVolumeAndDurationSupported: Availableinclient = Availableinclient(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selectedforsession(pub u32);
 
#[allow(non_upper_case_globals)]
impl Selectedforsession {
	pub const UsageOfPrepaidAccountingForVolume: Selectedforsession = Selectedforsession(1);
	pub const UsageOfPrepaidAccountingForDuration: Selectedforsession = Selectedforsession(2);
	pub const UsageOfPrepaidAccountingForVolumeAndDuration: Selectedforsession = Selectedforsession(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flag(pub u32);
 
#[allow(non_upper_case_globals)]
impl Flag {
	pub const DnsIpAddressesProvidedByHaaaForcibly: Flag = Flag(1);
	pub const DnsIpAddressesProvidedByHaaaUnobtrusively: Flag = Flag(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityType(pub u32);
 
#[allow(non_upper_case_globals)]
impl EntityType {
	pub const Haaa: EntityType = EntityType(1);
	pub const Vaaa: EntityType = EntityType(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2IkePresharedSecretRequest(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2SecurityLevel(v)},
		3 => value!(i, Attribute::VsaThreeGpp2PreSharedSecret(i)),
		4 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ReverseTunnelSpec(v)},
		5 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2DiffservClassOption(v)},
		6 => value!(i, Attribute::VsaThreeGpp2AccountingContainer(i)),
		7 => value!(i, Attribute::VsaThreeGpp2HomeAgentIpAddress(i)),
		8 => value!(i, Attribute::VsaThreeGpp2Keyid(i)),
		9 => map!{i, take!(4), |v:&[u8]| Attribute::VsaThreeGpp2PcfIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		10 => value!(i, Attribute::VsaThreeGpp2Bsid(i)),
		11 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2UserId(v)},
		12 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ForwardFchMuxOption(v)},
		13 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ReverseFchMuxOption(v)},
		16 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ServiceOption(v)},
		17 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ForwardTrafficType(v)},
		18 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ReverseTrafficType(v)},
		19 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2FchFrameSize(v)},
		20 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ForwardFchRc(v)},
		21 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ReverseFchRc(v)},
		22 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2IpTechnology(v)},
		23 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2CompulsoryTunnelIndicator(v)},
		24 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ReleaseIndicator(v)},
		25 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2BadPppFrameCount(v)},
		30 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2NumberActiveTransitions(v)},
		31 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2TerminatingSdbOctetCount(v)},
		32 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2OriginatingSdbOctetCount(v)},
		33 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2TerminatingNumberSdbs(v)},
		34 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2OriginatingNumberSdbs(v)},
		36 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2IpQos(v)},
		39 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2AirlinkPriority(v)},
		40 => map! {i, be_u32, |v| Attribute::VsaThreeGpp2AirlinkRecordType(ThreeGpp2AirlinkRecordType(v))},
		41 => value!(i, Attribute::VsaThreeGpp2RPSessionId(i)),
		42 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2AirlinkSequenceNumber(v)},
		43 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ReceivedHdlcOctets(v)},
		44 => value!(i, Attribute::VsaThreeGpp2CorrelationId(i)),
		45 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ModuleOrigTermIndicator(v)},
		46 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2InboundMobileIpSigOctets(v)},
		47 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2OutboundMobileIpSigOctets(v)},
		48 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2SessionContinue(v)},
		49 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ActiveTime(v)},
		50 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2DcchFrameSize(v)},
		51 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2BeginSession(v)},
		52 => value!(i, Attribute::VsaThreeGpp2Esn(i)),
		54 => value!(i, Attribute::VsaThreeGpp2SKey(i)),
		55 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2SRequest(v)},
		56 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2SLifetime(v)},
		57 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2MnHaSpi(v)},
		58 => value!(i, Attribute::VsaThreeGpp2MnHaSharedKey(i)),
		59 => value!(i, Attribute::VsaThreeGpp2RemoteIpAddress(i)),
		60 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2HrpdAccessOrTerminalAuthenticationAnd1XAccessAuthorization(v)},
		61 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2AtHardwareIdentifier(v)},
		70 => value!(i, Attribute::VsaThreeGpp2RemoteIpv6Address(i)),
		71 => value!(i, Attribute::VsaThreeGpp2RemoteAddressTableIndex(i)),
		72 => value!(i, Attribute::VsaThreeGpp2RemoteIpv4AddrOctetCount(i)),
		73 => value!(i, Attribute::VsaThreeGpp2AllowedDiffservMarking(i)),
		74 => value!(i, Attribute::VsaThreeGpp2ServiceOptionProfile(i)),
		75 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2DnsUpdateRequired(v)},
		78 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2AlwaysOn(v)},
		79 => map!{i, take!(4), |v:&[u8]| Attribute::VsaThreeGpp2ForeignAgentAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		80 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2LastUserActivityTime(v)},
		81 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2MnAaaRemovalIndication(v)},
		82 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2RnPacketDataInactivityTimer(v)},
		83 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ForwardPdchRc(v)},
		84 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ForwardDcchMuxOption(v)},
		85 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ReverseDcchMuxOption(v)},
		86 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ForwardDcchRc(v)},
		87 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ReverseDhhcRc(v)},
		88 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2SessionTerminationCapability(v)},
		89 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2AllowedPersistentTfts(v)},
		90 => value!(i, Attribute::VsaThreeGpp2PrepaidAcctQuota(i)),
		91 => value!(i, Attribute::VsaThreeGpp2PrepaidAcctCapability(i)),
		92 => value!(i, Attribute::VsaThreeGpp2MipLifetime(i)),
		93 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2AcctStopTrigger(v)},
		94 => value!(i, Attribute::VsaThreeGpp2ServiceReferenceId(i)),
		95 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2DnsUpdateCapability(v)},
		96 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2DisconnectReason(v)},
		97 => value!(i, Attribute::VsaThreeGpp2RemoteIpv6OctetCount(i)),
		98 => value!(i, Attribute::VsaThreeGpp2PrepaidTariffSwitching(i)),
		99 => value!(i, Attribute::VsaThreeGpp2AuthorizationParameters(i)),
		100 => value!(i, Attribute::VsaThreeGpp2BcmcsFlowId(i)),
		101 => value!(i, Attribute::VsaThreeGpp2BcmcsCapability(i)),
		102 => value!(i, Attribute::VsaThreeGpp2CommonSessionInfo(i)),
		103 => value!(i, Attribute::VsaBsnSessionInfo(i)),
		104 => value!(i, Attribute::VsaRanSessionInfo(i)),
		105 => map! {i, be_u32, |v| Attribute::VsaReasonCode(ReasonCode(v))},
		107 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2BcmcsFlowTransmissionTime(v)},
		108 => value!(i, Attribute::VsaThreeGpp2Subnet(i)),
		109 => map!{i, take!(4), |v:&[u8]| Attribute::VsaThreeGpp2MulticastIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		110 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2Port(v)},
		112 => value!(i, Attribute::VsaThreeGpp2TkInfo(i)),
		113 => value!(i, Attribute::VsaThreeGpp2BakId(i)),
		114 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2ReversePdchRc(v)},
		116 => value!(i, Attribute::VsaThreeGpp2Meid(i)),
		117 => value!(i, Attribute::VsaThreeGpp2DnsServerIpAddress(i)),
		118 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaThreeGpp2Mip6HomeAgentAddressFromBu(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		119 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaThreeGpp2Mip6CareOfAddress(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		120 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2HomeAgentNotAuthorized(v)},
		121 => value!(i, Attribute::VsaThreeGpp2Mip6SessionKey(i)),
		122 => value!(i, Attribute::VsaThreeGpp2HotLineAccountingInformation(i)),
		123 => value!(i, Attribute::VsaThreeGpp2Mip6MesgId(i)),
		124 => value!(i, Attribute::VsaThreeGpp2FilterRule(i)),
		125 => value!(i, Attribute::VsaThreeGpp2HttpRedirectionRule(i)),
		126 => value!(i, Attribute::VsaThreeGpp2IpRedirectionRule(i)),
		127 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2HotLineCapability(v)},
		128 => value!(i, Attribute::VsaThreeGpp2Mip6HomeLinkPrefixAttrA(i)),
		130 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2MaxAuthorizedAggrBandwidth(v)},
		131 => value!(i, Attribute::VsaThreeGpp2AuthorizedFlowProfileIds(i)),
		132 => value!(i, Attribute::VsaThreeGpp2GrantedQosParameters(i)),
		133 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2MaximumPerFlowPriority(v)},
		134 => value!(i, Attribute::VsaThreeGpp2Mip6Authenticator(i)),
		138 => value!(i, Attribute::VsaThreeGpp2Mip6MacMobilityData(i)),
		139 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2InterUserPriority(v)},
		140 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaThreeGpp2Mip6HomeAgentAddressAttrB(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		141 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaThreeGpp2Mip6HoaReceivedFromBu(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		142 => value!(i, Attribute::VsaThreeGpp2CarrierId(i)),
		143 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2GmtTimeZoneOffset(v)},
		144 => value!(i, Attribute::VsaThreeGpp2FlowIdParameter(i)),
		145 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2FlowStatus(v)},
		146 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2FilteredOctetCountTerminating(v)},
		147 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2FilteredOctetCountOriginating(v)},
		162 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2RsvpInboundOctetCount(v)},
		163 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2RsvpOutboundOctetCount(v)},
		164 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2RsvpInboundPacketCount(v)},
		165 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2RsvpOutboundPacketCount(v)},
		168 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2HaRequest(v)},
		169 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2HaAuthorised(v)},
		172 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2IpVerAuthorised(v)},
		173 => value!(i, Attribute::VsaThreeGpp2Mipv4MesgId(i)),
		179 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2Mip6HaLocalAssignmentCapblty(v)},
		192 => value!(i, Attribute::VsaThreeGpp2NetworkPmipNai(i)),
		193 => map! {i, be_u32, |v| Attribute::VsaThreeGpp2PmipBasedMobilityCapability(ThreeGpp2PmipBasedMobilityCapability(v))},
		198 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2AccountingMode(v)},
		203 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2HaaaMip6HaProtocolCapbltyInd(v)},
		205 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaThreeGpp2VaaaAssignedMip6Ha(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		206 => value!(i, Attribute::VsaThreeGpp2VaaaAssignedMip6Hl(i)),
		207 => map!{i, be_u32, |v| Attribute::VsaThreeGpp2VaaaMip6HaProtocolCapbltyInd(v)},
		214 => value!(i, Attribute::VsaThreeGpp2DnsServerIpv6Address(i)),
        _ => value!(i, Attribute::VsaUnknown(5535, typ, i)),
    }
}
