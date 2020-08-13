/// Definitions for vendor Acc, vendor value 5
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccReasonCode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccReasonCode {
	pub const NoReasonNoFailure: AccReasonCode = AccReasonCode(0);
	pub const ResourceShortage: AccReasonCode = AccReasonCode(1);
	pub const SessionAlreadyOpen: AccReasonCode = AccReasonCode(2);
	pub const TooManyRadiusUsers: AccReasonCode = AccReasonCode(3);
	pub const NoAuthenticationServer: AccReasonCode = AccReasonCode(4);
	pub const NoAuthenticationResponse: AccReasonCode = AccReasonCode(5);
	pub const NoAccountingServer: AccReasonCode = AccReasonCode(6);
	pub const NoAccountingResponse: AccReasonCode = AccReasonCode(7);
	pub const AccessDenied: AccReasonCode = AccReasonCode(8);
	pub const TemporaryBufferShortage: AccReasonCode = AccReasonCode(9);
	pub const ProtocolError: AccReasonCode = AccReasonCode(10);
	pub const InvalidAttribute: AccReasonCode = AccReasonCode(11);
	pub const InvalidServiceType: AccReasonCode = AccReasonCode(12);
	pub const InvalidFramedProtocol: AccReasonCode = AccReasonCode(13);
	pub const InvalidAttributeValue: AccReasonCode = AccReasonCode(14);
	pub const InvalidUserInformation: AccReasonCode = AccReasonCode(15);
	pub const InvalidIpAddress: AccReasonCode = AccReasonCode(16);
	pub const InvalidIntegerSyntax: AccReasonCode = AccReasonCode(17);
	pub const InvalidNasPort: AccReasonCode = AccReasonCode(18);
	pub const RequestedByUser: AccReasonCode = AccReasonCode(19);
	pub const NetworkDisconnect: AccReasonCode = AccReasonCode(20);
	pub const ServiceInterruption: AccReasonCode = AccReasonCode(21);
	pub const PhysicalPortError: AccReasonCode = AccReasonCode(22);
	pub const IdleTimeout: AccReasonCode = AccReasonCode(23);
	pub const SessionTimeout: AccReasonCode = AccReasonCode(24);
	pub const AdministrativeReset: AccReasonCode = AccReasonCode(25);
	pub const NasReloadOrReset: AccReasonCode = AccReasonCode(26);
	pub const NasError: AccReasonCode = AccReasonCode(27);
	pub const NasRequest: AccReasonCode = AccReasonCode(28);
	pub const UndefinedReasonGiven: AccReasonCode = AccReasonCode(29);
	pub const ConflictingAttributes: AccReasonCode = AccReasonCode(30);
	pub const PortLimitExceeded: AccReasonCode = AccReasonCode(31);
	pub const FacilityNotAvailable: AccReasonCode = AccReasonCode(32);
	pub const InternalConfigError: AccReasonCode = AccReasonCode(33);
	pub const BadRouteSpecification: AccReasonCode = AccReasonCode(34);
	pub const AccessPartitionBindFailure: AccReasonCode = AccReasonCode(35);
	pub const SecurityViolation: AccReasonCode = AccReasonCode(36);
	pub const RequestTypeConflict: AccReasonCode = AccReasonCode(37);
	pub const ConfigurationDisallowed: AccReasonCode = AccReasonCode(38);
	pub const MissingAttribute: AccReasonCode = AccReasonCode(39);
	pub const InvalidRequest: AccReasonCode = AccReasonCode(40);
	pub const MissingParameter: AccReasonCode = AccReasonCode(41);
	pub const InvalidParameter: AccReasonCode = AccReasonCode(42);
	pub const CallClearedWithCause: AccReasonCode = AccReasonCode(43);
	pub const InopportuneConfigRequest: AccReasonCode = AccReasonCode(44);
	pub const InvalidConfigParameter: AccReasonCode = AccReasonCode(45);
	pub const MissingConfigParameter: AccReasonCode = AccReasonCode(46);
	pub const IncompatibleServiceProfile: AccReasonCode = AccReasonCode(47);
	pub const AdministrativeReset2: AccReasonCode = AccReasonCode(48);
	pub const AdministrativeReload: AccReasonCode = AccReasonCode(49);
	pub const PortUnneeded: AccReasonCode = AccReasonCode(50);
	pub const PortPreempted: AccReasonCode = AccReasonCode(51);
	pub const PortSuspended: AccReasonCode = AccReasonCode(52);
	pub const ServiceUnavailable: AccReasonCode = AccReasonCode(53);
	pub const Callback: AccReasonCode = AccReasonCode(54);
	pub const UserError: AccReasonCode = AccReasonCode(55);
	pub const HostRequest: AccReasonCode = AccReasonCode(56);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccCcpOption(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccCcpOption {
	pub const Disabled: AccCcpOption = AccCcpOption(1);
	pub const Enabled: AccCcpOption = AccCcpOption(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccRoutePolicy(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccRoutePolicy {
	pub const Funnel: AccRoutePolicy = AccRoutePolicy(1);
	pub const Direct: AccRoutePolicy = AccRoutePolicy(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccMlMlxAdminState(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccMlMlxAdminState {
	pub const Enabled: AccMlMlxAdminState = AccMlMlxAdminState(1);
	pub const Disabled: AccMlMlxAdminState = AccMlMlxAdminState(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccClearingCause(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccClearingCause {
	pub const CauseUnspecified: AccClearingCause = AccClearingCause(0);
	pub const UnassignedNumber: AccClearingCause = AccClearingCause(1);
	pub const NoRouteToTransitNetwork: AccClearingCause = AccClearingCause(2);
	pub const NoRouteToDestination: AccClearingCause = AccClearingCause(3);
	pub const ChannelUnacceptable: AccClearingCause = AccClearingCause(6);
	pub const CallAwardedBeingDelivered: AccClearingCause = AccClearingCause(7);
	pub const NormalClearing: AccClearingCause = AccClearingCause(16);
	pub const UserBusy: AccClearingCause = AccClearingCause(17);
	pub const NoUserResponding: AccClearingCause = AccClearingCause(18);
	pub const UserAlertedNoAnswer: AccClearingCause = AccClearingCause(19);
	pub const CallRejected: AccClearingCause = AccClearingCause(21);
	pub const NumberChanged: AccClearingCause = AccClearingCause(22);
	pub const NonSelectedUserClearing: AccClearingCause = AccClearingCause(26);
	pub const DestinationOutOfOrder: AccClearingCause = AccClearingCause(27);
	pub const InvalidOrIncompleteNumber: AccClearingCause = AccClearingCause(28);
	pub const FacilityRejected: AccClearingCause = AccClearingCause(29);
	pub const ResponseToStatusInquiry: AccClearingCause = AccClearingCause(30);
	pub const NormalUnspecifiedCause: AccClearingCause = AccClearingCause(31);
	pub const NoCircuitOrChannelAvailable: AccClearingCause = AccClearingCause(34);
	pub const NetworkOutOfOrder: AccClearingCause = AccClearingCause(38);
	pub const TemporaryFailure: AccClearingCause = AccClearingCause(41);
	pub const SwitchingEquipmentCongestion: AccClearingCause = AccClearingCause(42);
	pub const AccessInformationDiscarded: AccClearingCause = AccClearingCause(43);
	pub const CircuitOrChannelUnavailable: AccClearingCause = AccClearingCause(44);
	pub const CircuitOrChannedPreempted: AccClearingCause = AccClearingCause(45);
	pub const ResourcesUnavailable: AccClearingCause = AccClearingCause(47);
	pub const QualityOfServiceUnavailable: AccClearingCause = AccClearingCause(49);
	pub const FacilityNotSubscribed: AccClearingCause = AccClearingCause(50);
	pub const OutgoingCallsBarred: AccClearingCause = AccClearingCause(52);
	pub const IncomingCallsBarred: AccClearingCause = AccClearingCause(54);
	pub const BearerCapabilityUnauthorized: AccClearingCause = AccClearingCause(57);
	pub const BearerCapabilityNotAvailable: AccClearingCause = AccClearingCause(58);
	pub const ServiceNotAvailable: AccClearingCause = AccClearingCause(63);
	pub const BearerCapablityNotImplmented: AccClearingCause = AccClearingCause(65);
	pub const ChannelTypeNotImplemented: AccClearingCause = AccClearingCause(66);
	pub const FacilityNotImplemented: AccClearingCause = AccClearingCause(69);
	pub const RestrctedDigtalInfrmtionOnly: AccClearingCause = AccClearingCause(70);
	pub const ServiceNotImplemented: AccClearingCause = AccClearingCause(79);
	pub const InvalidCallReference: AccClearingCause = AccClearingCause(81);
	pub const IdentifiedChannelDoesntExist: AccClearingCause = AccClearingCause(82);
	pub const CallIdentifyInUse: AccClearingCause = AccClearingCause(84);
	pub const NoCallSuspended: AccClearingCause = AccClearingCause(85);
	pub const SuspendedCallCleared: AccClearingCause = AccClearingCause(86);
	pub const IncompatibleDestination: AccClearingCause = AccClearingCause(88);
	pub const InvalidTransitNetworkSelctin: AccClearingCause = AccClearingCause(91);
	pub const InvalidMessage: AccClearingCause = AccClearingCause(95);
	pub const MandtoryInfrmtionElmentMiss: AccClearingCause = AccClearingCause(96);
	pub const MessageNotImplemented: AccClearingCause = AccClearingCause(97);
	pub const InopportuneMessage: AccClearingCause = AccClearingCause(98);
	pub const InfrmtionElemntNotImplmented: AccClearingCause = AccClearingCause(99);
	pub const InvlidInfrmtionElementContnt: AccClearingCause = AccClearingCause(100);
	pub const MessageIncompatibleWithState: AccClearingCause = AccClearingCause(101);
	pub const RecoveryOnTimerExpiration: AccClearingCause = AccClearingCause(102);
	pub const MndtryInfrmtionElmntLngtErr: AccClearingCause = AccClearingCause(103);
	pub const ProtocolError: AccClearingCause = AccClearingCause(111);
	pub const Interworking: AccClearingCause = AccClearingCause(127);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccClearingLocation(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccClearingLocation {
	pub const LocalOrRemoteUser: AccClearingLocation = AccClearingLocation(0);
	pub const PrvteNtworkServingLocalUser: AccClearingLocation = AccClearingLocation(1);
	pub const PblicNtworkServingLocalUser: AccClearingLocation = AccClearingLocation(2);
	pub const TransitNetwork: AccClearingLocation = AccClearingLocation(3);
	pub const PrvteNtworkServRemoteUser: AccClearingLocation = AccClearingLocation(4);
	pub const PblicNtworkServRemoteUser: AccClearingLocation = AccClearingLocation(5);
	pub const InternationalNetwork: AccClearingLocation = AccClearingLocation(6);
	pub const BeyondInterworkingPoint: AccClearingLocation = AccClearingLocation(10);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccRequestType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccRequestType {
	pub const RingIndication: AccRequestType = AccRequestType(1);
	pub const DialRequest: AccRequestType = AccRequestType(2);
	pub const UserAuthentication: AccRequestType = AccRequestType(3);
	pub const TunnelAuthentication: AccRequestType = AccRequestType(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccBridgingSupport(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccBridgingSupport {
	pub const Disabled: AccBridgingSupport = AccBridgingSupport(1);
	pub const Enabled: AccBridgingSupport = AccBridgingSupport(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccApsmOversubscribed(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccApsmOversubscribed {
	pub const False: AccApsmOversubscribed = AccApsmOversubscribed(1);
	pub const True: AccApsmOversubscribed = AccApsmOversubscribed(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccAcctOnOffReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccAcctOnOffReason {
	pub const NasReset: AccAcctOnOffReason = AccAcctOnOffReason(0);
	pub const NasReload: AccAcctOnOffReason = AccAcctOnOffReason(1);
	pub const ConfigurationReset: AccAcctOnOffReason = AccAcctOnOffReason(2);
	pub const ConfigurationReload: AccAcctOnOffReason = AccAcctOnOffReason(3);
	pub const Enabled: AccAcctOnOffReason = AccAcctOnOffReason(4);
	pub const Disabled: AccAcctOnOffReason = AccAcctOnOffReason(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccIpCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccIpCompression {
	pub const Disabled: AccIpCompression = AccIpCompression(1);
	pub const Enabled: AccIpCompression = AccIpCompression(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccIpxCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccIpxCompression {
	pub const Disabled: AccIpxCompression = AccIpxCompression(1);
	pub const Enabled: AccIpxCompression = AccIpxCompression(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccCallbackMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccCallbackMode {
	pub const UserAuth: AccCallbackMode = AccCallbackMode(0);
	pub const UserSpecifiedE164: AccCallbackMode = AccCallbackMode(3);
	pub const CbcpCallback: AccCallbackMode = AccCallbackMode(6);
	pub const CliCallback: AccCallbackMode = AccCallbackMode(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccCallbackCbcpType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccCallbackCbcpType {
	pub const CbcpNone: AccCallbackCbcpType = AccCallbackCbcpType(1);
	pub const CbcpUserSpecified: AccCallbackCbcpType = AccCallbackCbcpType(2);
	pub const CbcpPreSpecified: AccCallbackCbcpType = AccCallbackCbcpType(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccDialoutAuthMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccDialoutAuthMode {
	pub const Pap: AccDialoutAuthMode = AccDialoutAuthMode(1);
	pub const Chap: AccDialoutAuthMode = AccDialoutAuthMode(2);
	pub const ChapPap: AccDialoutAuthMode = AccDialoutAuthMode(3);
	pub const None: AccDialoutAuthMode = AccDialoutAuthMode(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccAccessCommunity(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccAccessCommunity {
	pub const Public: AccAccessCommunity = AccAccessCommunity(1);
	pub const Netman: AccAccessCommunity = AccAccessCommunity(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccVpsmRejectCause(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccVpsmRejectCause {
	pub const NoAccessPartition: AccVpsmRejectCause = AccVpsmRejectCause(1);
	pub const AccessPartitionDisabled: AccVpsmRejectCause = AccVpsmRejectCause(2);
	pub const PartitionPortlimitExceeded: AccVpsmRejectCause = AccVpsmRejectCause(3);
	pub const LicensePortlimitExceeded: AccVpsmRejectCause = AccVpsmRejectCause(4);
	pub const HomeServerDown: AccVpsmRejectCause = AccVpsmRejectCause(5);
	pub const RejectedByHomeServer: AccVpsmRejectCause = AccVpsmRejectCause(6);
	pub const NasAdministrativelyDisabled: AccVpsmRejectCause = AccVpsmRejectCause(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccIgmpAdminState(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccIgmpAdminState {
	pub const Enabled: AccIgmpAdminState = AccIgmpAdminState(1);
	pub const Disabled: AccIgmpAdminState = AccIgmpAdminState(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccIgmpVersion(pub u32);
 
#[allow(non_upper_case_globals)]
impl AccIgmpVersion {
	pub const V1: AccIgmpVersion = AccIgmpVersion(1);
	pub const V2: AccIgmpVersion = AccIgmpVersion(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaAccReasonCode(AccReasonCode(v))},
		2 => map! {i, be_u32, |v| Attribute::VsaAccCcpOption(AccCcpOption(v))},
		3 => map!{i, be_u32, |v| Attribute::VsaAccInputErrors(v)},
		4 => map!{i, be_u32, |v| Attribute::VsaAccOutputErrors(v)},
		5 => value!(i, Attribute::VsaAccAccessPartition(i)),
		6 => value!(i, Attribute::VsaAccCustomerId(i)),
		7 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAccIpGatewayPri(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		8 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAccIpGatewaySec(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		9 => map! {i, be_u32, |v| Attribute::VsaAccRoutePolicy(AccRoutePolicy(v))},
		10 => map! {i, be_u32, |v| Attribute::VsaAccMlMlxAdminState(AccMlMlxAdminState(v))},
		11 => map!{i, be_u32, |v| Attribute::VsaAccMlCallThreshold(v)},
		12 => map!{i, be_u32, |v| Attribute::VsaAccMlClearThreshold(v)},
		13 => map!{i, be_u32, |v| Attribute::VsaAccMlDampingFactor(v)},
		14 => value!(i, Attribute::VsaAccTunnelSecret(i)),
		15 => map! {i, be_u32, |v| Attribute::VsaAccClearingCause(AccClearingCause(v))},
		16 => map! {i, be_u32, |v| Attribute::VsaAccClearingLocation(AccClearingLocation(v))},
		17 => value!(i, Attribute::VsaAccServiceProfile(i)),
		18 => map! {i, be_u32, |v| Attribute::VsaAccRequestType(AccRequestType(v))},
		19 => map! {i, be_u32, |v| Attribute::VsaAccBridgingSupport(AccBridgingSupport(v))},
		20 => map! {i, be_u32, |v| Attribute::VsaAccApsmOversubscribed(AccApsmOversubscribed(v))},
		21 => map! {i, be_u32, |v| Attribute::VsaAccAcctOnOffReason(AccAcctOnOffReason(v))},
		22 => map!{i, be_u32, |v| Attribute::VsaAccTunnelPort(v)},
		23 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAccDnsServerPri(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		24 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAccDnsServerSec(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		25 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAccNbnsServerPri(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		26 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAccNbnsServerSec(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		27 => map!{i, be_u32, |v| Attribute::VsaAccDialPortIndex(v)},
		28 => map! {i, be_u32, |v| Attribute::VsaAccIpCompression(AccIpCompression(v))},
		29 => map! {i, be_u32, |v| Attribute::VsaAccIpxCompression(AccIpxCompression(v))},
		30 => map!{i, be_u32, |v| Attribute::VsaAccConnectTxSpeed(v)},
		31 => map!{i, be_u32, |v| Attribute::VsaAccConnectRxSpeed(v)},
		32 => value!(i, Attribute::VsaAccModemModulationType(i)),
		33 => value!(i, Attribute::VsaAccModemErrorProtocol(i)),
		34 => map!{i, be_u32, |v| Attribute::VsaAccCallbackDelay(v)},
		35 => value!(i, Attribute::VsaAccCallbackNumValid(i)),
		36 => map! {i, be_u32, |v| Attribute::VsaAccCallbackMode(AccCallbackMode(v))},
		37 => map! {i, be_u32, |v| Attribute::VsaAccCallbackCbcpType(AccCallbackCbcpType(v))},
		38 => map! {i, be_u32, |v| Attribute::VsaAccDialoutAuthMode(AccDialoutAuthMode(v))},
		39 => value!(i, Attribute::VsaAccDialoutAuthPassword(i)),
		40 => value!(i, Attribute::VsaAccDialoutAuthUsername(i)),
		42 => map! {i, be_u32, |v| Attribute::VsaAccAccessCommunity(AccAccessCommunity(v))},
		43 => map! {i, be_u32, |v| Attribute::VsaAccVpsmRejectCause(AccVpsmRejectCause(v))},
		44 => value!(i, Attribute::VsaAccAceToken(i)),
		45 => map!{i, be_u32, |v| Attribute::VsaAccAceTokenTtl(v)},
		46 => value!(i, Attribute::VsaAccIpPoolName(i)),
		47 => map! {i, be_u32, |v| Attribute::VsaAccIgmpAdminState(AccIgmpAdminState(v))},
		48 => map! {i, be_u32, |v| Attribute::VsaAccIgmpVersion(AccIgmpVersion(v))},
		73 => value!(i, Attribute::VsaAccMnHaSecret(i)),
		98 => value!(i, Attribute::VsaAccLocationId(i)),
		99 => map!{i, be_u32, |v| Attribute::VsaAccCallingStationCategory(v)},
        _ => value!(i, Attribute::VsaUnknown(5, typ, i)),
    }
}
