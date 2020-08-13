/// Definitions for vendor CableLabs, vendor value 4491
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsQueryType(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsQueryType {
	pub const Reserved: CablelabsQueryType = CablelabsQueryType(0);
	pub const TollFreeNumberLookup: CablelabsQueryType = CablelabsQueryType(1);
	pub const LnpNumberLookup: CablelabsQueryType = CablelabsQueryType(2);
	pub const CallingNameDeliveryLookup: CablelabsQueryType = CablelabsQueryType(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsChannelState(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsChannelState {
	pub const Reserved: CablelabsChannelState = CablelabsChannelState(0);
	pub const Open: CablelabsChannelState = CablelabsChannelState(1);
	pub const Change: CablelabsChannelState = CablelabsChannelState(2);
	pub const Close: CablelabsChannelState = CablelabsChannelState(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsDirectionIndicator(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsDirectionIndicator {
	pub const Undefined: CablelabsDirectionIndicator = CablelabsDirectionIndicator(0);
	pub const Originating: CablelabsDirectionIndicator = CablelabsDirectionIndicator(1);
	pub const Terminating: CablelabsDirectionIndicator = CablelabsDirectionIndicator(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsFlowDirection(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsFlowDirection {
	pub const Reserved: CablelabsFlowDirection = CablelabsFlowDirection(0);
	pub const Upstream: CablelabsFlowDirection = CablelabsFlowDirection(1);
	pub const Downstream: CablelabsFlowDirection = CablelabsFlowDirection(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsSignalType(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsSignalType {
	pub const Reserved: CablelabsSignalType = CablelabsSignalType(0);
	pub const NetworkSignal: CablelabsSignalType = CablelabsSignalType(1);
	pub const SubjectSignal: CablelabsSignalType = CablelabsSignalType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsAlertingSignal(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsAlertingSignal {
	pub const Reserved0: CablelabsAlertingSignal = CablelabsAlertingSignal(0);
	pub const Ringing: CablelabsAlertingSignal = CablelabsAlertingSignal(1);
	pub const DistinctiveRinging2: CablelabsAlertingSignal = CablelabsAlertingSignal(2);
	pub const DistinctiveRinging3: CablelabsAlertingSignal = CablelabsAlertingSignal(3);
	pub const DistinctiveRinging4: CablelabsAlertingSignal = CablelabsAlertingSignal(4);
	pub const Ringsplash: CablelabsAlertingSignal = CablelabsAlertingSignal(5);
	pub const CallWaitingTone1: CablelabsAlertingSignal = CablelabsAlertingSignal(6);
	pub const CallWaitingTone2: CablelabsAlertingSignal = CablelabsAlertingSignal(7);
	pub const CallWaitingTone3: CablelabsAlertingSignal = CablelabsAlertingSignal(8);
	pub const CallWaitingTone4: CablelabsAlertingSignal = CablelabsAlertingSignal(9);
	pub const Reserved10: CablelabsAlertingSignal = CablelabsAlertingSignal(10);
	pub const DistinctiveRinging0: CablelabsAlertingSignal = CablelabsAlertingSignal(11);
	pub const DistinctiveRinging1: CablelabsAlertingSignal = CablelabsAlertingSignal(12);
	pub const DistinctiveRinging5: CablelabsAlertingSignal = CablelabsAlertingSignal(13);
	pub const DistinctiveRinging6: CablelabsAlertingSignal = CablelabsAlertingSignal(14);
	pub const DistinctiveRinging7: CablelabsAlertingSignal = CablelabsAlertingSignal(15);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsAmOpaqueData(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsAmOpaqueData {
	pub const Reserved0: CablelabsAmOpaqueData = CablelabsAmOpaqueData(0);
	pub const DialTone: CablelabsAmOpaqueData = CablelabsAmOpaqueData(1);
	pub const StutterDialTone: CablelabsAmOpaqueData = CablelabsAmOpaqueData(2);
	pub const RingBackTone: CablelabsAmOpaqueData = CablelabsAmOpaqueData(3);
	pub const ReorderTone: CablelabsAmOpaqueData = CablelabsAmOpaqueData(4);
	pub const BusyTone: CablelabsAmOpaqueData = CablelabsAmOpaqueData(5);
	pub const ConfirmationTone: CablelabsAmOpaqueData = CablelabsAmOpaqueData(6);
	pub const Reserved7: CablelabsAmOpaqueData = CablelabsAmOpaqueData(7);
	pub const MessageWaitingIndicator: CablelabsAmOpaqueData = CablelabsAmOpaqueData(8);
	pub const OffHookWarningTone: CablelabsAmOpaqueData = CablelabsAmOpaqueData(9);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsElementRequestingQos(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsElementRequestingQos {
	pub const Client: CablelabsElementRequestingQos = CablelabsElementRequestingQos(0);
	pub const PolicyServer: CablelabsElementRequestingQos = CablelabsElementRequestingQos(1);
	pub const EmbeddedClient: CablelabsElementRequestingQos = CablelabsElementRequestingQos(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsQosReleaseReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsQosReleaseReason {
	pub const GateClosedByPs: CablelabsQosReleaseReason = CablelabsQosReleaseReason(1);
	pub const InactivityResourceRecoveryTimerExpiration: CablelabsQosReleaseReason = CablelabsQosReleaseReason(2);
	pub const CmFailure: CablelabsQosReleaseReason = CablelabsQosReleaseReason(3);
	pub const PreEmpted: CablelabsQosReleaseReason = CablelabsQosReleaseReason(4);
	pub const RsvpPathtearRequest: CablelabsQosReleaseReason = CablelabsQosReleaseReason(5);
	pub const CmRequest: CablelabsQosReleaseReason = CablelabsQosReleaseReason(6);
	pub const AdmittedTimerExpiration: CablelabsQosReleaseReason = CablelabsQosReleaseReason(7);
	pub const Other: CablelabsQosReleaseReason = CablelabsQosReleaseReason(127);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsPolicyDeniedReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsPolicyDeniedReason {
	pub const PolicyServerAdmissionControlFailure: CablelabsPolicyDeniedReason = CablelabsPolicyDeniedReason(1);
	pub const InsufficientResources: CablelabsPolicyDeniedReason = CablelabsPolicyDeniedReason(2);
	pub const UnknownSubscriber: CablelabsPolicyDeniedReason = CablelabsPolicyDeniedReason(3);
	pub const UnauthorizedAmid: CablelabsPolicyDeniedReason = CablelabsPolicyDeniedReason(4);
	pub const UndefinedServiceClassName: CablelabsPolicyDeniedReason = CablelabsPolicyDeniedReason(5);
	pub const IncompatibleEnvelope: CablelabsPolicyDeniedReason = CablelabsPolicyDeniedReason(6);
	pub const Other: CablelabsPolicyDeniedReason = CablelabsPolicyDeniedReason(127);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsPolicyDeletedReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsPolicyDeletedReason {
	pub const ApplicationManagerRequest: CablelabsPolicyDeletedReason = CablelabsPolicyDeletedReason(1);
	pub const CmtsDecistion: CablelabsPolicyDeletedReason = CablelabsPolicyDeletedReason(2);
	pub const Other: CablelabsPolicyDeletedReason = CablelabsPolicyDeletedReason(127);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsPolicyUpdateReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsPolicyUpdateReason {
	pub const TrafficProfile: CablelabsPolicyUpdateReason = CablelabsPolicyUpdateReason(1);
	pub const Classifier: CablelabsPolicyUpdateReason = CablelabsPolicyUpdateReason(2);
	pub const VolumeLimit: CablelabsPolicyUpdateReason = CablelabsPolicyUpdateReason(3);
	pub const TimeLimit: CablelabsPolicyUpdateReason = CablelabsPolicyUpdateReason(4);
	pub const OpaqueData: CablelabsPolicyUpdateReason = CablelabsPolicyUpdateReason(5);
	pub const MultipleUpdates: CablelabsPolicyUpdateReason = CablelabsPolicyUpdateReason(6);
	pub const Other: CablelabsPolicyUpdateReason = CablelabsPolicyUpdateReason(127);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CablelabsPolicyDecisionStatus(pub u32);
 
#[allow(non_upper_case_globals)]
impl CablelabsPolicyDecisionStatus {
	pub const PolicyApproved: CablelabsPolicyDecisionStatus = CablelabsPolicyDecisionStatus(1);
	pub const PolicyDenied: CablelabsPolicyDecisionStatus = CablelabsPolicyDecisionStatus(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		0 => value!(i, Attribute::VsaCablelabsReserved(i)),
		1 => value!(i, Attribute::VsaCablelabsEventMessage(i)),
		3 => value!(i, Attribute::VsaCablelabsMtaEndpointName(i)),
		4 => value!(i, Attribute::VsaCablelabsCallingPartyNumber(i)),
		5 => value!(i, Attribute::VsaCablelabsCalledPartyNumber(i)),
		6 => value!(i, Attribute::VsaCablelabsDatabaseId(i)),
		7 => map! {i, be_u32, |v| Attribute::VsaCablelabsQueryType(CablelabsQueryType(v))},
		9 => value!(i, Attribute::VsaCablelabsReturnedNumber(i)),
		11 => value!(i, Attribute::VsaCablelabsCallTerminationCause(i)),
		13 => value!(i, Attribute::VsaCablelabsRelatedCallBillingCrlId(i)),
		14 => value!(i, Attribute::VsaCablelabsFirstCallCallingPartyNum(i)),
		15 => value!(i, Attribute::VsaCablelabsSecondCallCallingPartyNum(i)),
		16 => value!(i, Attribute::VsaCablelabsChargeNumber(i)),
		17 => value!(i, Attribute::VsaCablelabsForwardedNumber(i)),
		18 => value!(i, Attribute::VsaCablelabsServiceName(i)),
		20 => value!(i, Attribute::VsaCablelabsIntlCode(i)),
		21 => value!(i, Attribute::VsaCablelabsDialAroundCode(i)),
		22 => value!(i, Attribute::VsaCablelabsLocationRoutingNumber(i)),
		23 => value!(i, Attribute::VsaCablelabsCarrierIdentificationCode(i)),
		24 => value!(i, Attribute::VsaCablelabsTrunkGroupId(i)),
		25 => value!(i, Attribute::VsaCablelabsRoutingNumber(i)),
		26 => map!{i, be_u32, |v| Attribute::VsaCablelabsMtaUdpPortnum(v)},
		29 => map! {i, be_u32, |v| Attribute::VsaCablelabsChannelState(CablelabsChannelState(v))},
		30 => map!{i, be_u32, |v| Attribute::VsaCablelabsSfId(v)},
		31 => value!(i, Attribute::VsaCablelabsErrorDescription(i)),
		32 => value!(i, Attribute::VsaCablelabsQosDescriptor(i)),
		37 => map! {i, be_u32, |v| Attribute::VsaCablelabsDirectionIndicator(CablelabsDirectionIndicator(v))},
		38 => value!(i, Attribute::VsaCablelabsTimeAdjustment(i)),
		39 => value!(i, Attribute::VsaCablelabsSdpUpstream(i)),
		40 => value!(i, Attribute::VsaCablelabsSdpDownstream(i)),
		41 => value!(i, Attribute::VsaCablelabsUserInput(i)),
		42 => value!(i, Attribute::VsaCablelabsTranslationInput(i)),
		43 => value!(i, Attribute::VsaCablelabsRedirectedFromInfo(i)),
		44 => value!(i, Attribute::VsaCablelabsElectronicSurveillanceInd(i)),
		45 => value!(i, Attribute::VsaCablelabsRedirectedFromPartyNumber(i)),
		46 => value!(i, Attribute::VsaCablelabsRedirectedToPartyNumber(i)),
		47 => value!(i, Attribute::VsaCablelabsElSurveillanceDfSecurity(i)),
		48 => value!(i, Attribute::VsaCablelabsCccId(i)),
		49 => value!(i, Attribute::VsaCablelabsFinancialEntityId(i)),
		50 => map! {i, be_u32, |v| Attribute::VsaCablelabsFlowDirection(CablelabsFlowDirection(v))},
		51 => map! {i, be_u32, |v| Attribute::VsaCablelabsSignalType(CablelabsSignalType(v))},
		52 => map! {i, be_u32, |v| Attribute::VsaCablelabsAlertingSignal(CablelabsAlertingSignal(v))},
		53 => map!{i, be_u32, |v| Attribute::VsaCablelabsSubjectAudibleSignal(v)},
		54 => value!(i, Attribute::VsaCablelabsTerminalDisplayInfo(i)),
		55 => value!(i, Attribute::VsaCablelabsSwitchHookFlash(i)),
		56 => value!(i, Attribute::VsaCablelabsDialedDigits(i)),
		57 => value!(i, Attribute::VsaCablelabsMiscSignalingInformation(i)),
		61 => map! {i, be_u32, |v| Attribute::VsaCablelabsAmOpaqueData(CablelabsAmOpaqueData(v))},
		62 => map!{i, be_u32, |v| Attribute::VsaCablelabsSubscriberId(v)},
		63 => map!{i, be_u32, |v| Attribute::VsaCablelabsVolumeUsageLimit(v)},
		64 => map!{i, be_u32, |v| Attribute::VsaCablelabsGateUsageInfo(v)},
		65 => map! {i, be_u32, |v| Attribute::VsaCablelabsElementRequestingQos(CablelabsElementRequestingQos(v))},
		66 => map! {i, be_u32, |v| Attribute::VsaCablelabsQosReleaseReason(CablelabsQosReleaseReason(v))},
		67 => map! {i, be_u32, |v| Attribute::VsaCablelabsPolicyDeniedReason(CablelabsPolicyDeniedReason(v))},
		68 => map! {i, be_u32, |v| Attribute::VsaCablelabsPolicyDeletedReason(CablelabsPolicyDeletedReason(v))},
		69 => map! {i, be_u32, |v| Attribute::VsaCablelabsPolicyUpdateReason(CablelabsPolicyUpdateReason(v))},
		70 => map! {i, be_u32, |v| Attribute::VsaCablelabsPolicyDecisionStatus(CablelabsPolicyDecisionStatus(v))},
		71 => map!{i, be_u32, |v| Attribute::VsaCablelabsApplicationManagerId(v)},
		72 => map!{i, be_u32, |v| Attribute::VsaCablelabsTimeUsageLimit(v)},
		73 => map!{i, be_u32, |v| Attribute::VsaCablelabsGateTimeInfo(v)},
		74 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaCablelabsIpv6SubscriberId(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		75 => value!(i, Attribute::VsaCablelabsUserId(i)),
		80 => value!(i, Attribute::VsaCablelabsAccountCode(i)),
		81 => value!(i, Attribute::VsaCablelabsAuthorizationCode(i)),
		82 => value!(i, Attribute::VsaCablelabsJurisdictionInfoParameter(i)),
		83 => map!{i, be_u32, |v| Attribute::VsaCablelabsCalledPartyNpSource(v)},
		84 => map!{i, be_u32, |v| Attribute::VsaCablelabsCallingPartyNpSource(v)},
		85 => map!{i, be_u32, |v| Attribute::VsaCablelabsPortedInCallingNumber(v)},
		86 => map!{i, be_u32, |v| Attribute::VsaCablelabsPortedInCalledNumber(v)},
		87 => map!{i, be_u32, |v| Attribute::VsaCablelabsBillingType(v)},
		88 => value!(i, Attribute::VsaCablelabsSignaledToNumber(i)),
		89 => value!(i, Attribute::VsaCablelabsSignaledFromNumber(i)),
		90 => value!(i, Attribute::VsaCablelabsCommunicatingParty(i)),
		91 => value!(i, Attribute::VsaCablelabsJoinedParty(i)),
		92 => value!(i, Attribute::VsaCablelabsRemovedParty(i)),
		93 => value!(i, Attribute::VsaCablelabsRtcpData(i)),
		94 => value!(i, Attribute::VsaCablelabsLocalXrBlock(i)),
		95 => value!(i, Attribute::VsaCablelabsRemoteXrBlock(i)),
		96 => map!{i, be_u32, |v| Attribute::VsaSurveillanceStopType(v)},
		97 => map!{i, be_u32, |v| Attribute::VsaSurveillanceStopDestination(v)},
		98 => value!(i, Attribute::VsaRelatedIcid(i)),
        _ => value!(i, Attribute::VsaUnknown(4491, typ, i)),
    }
}
