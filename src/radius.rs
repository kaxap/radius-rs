use crate::vendors::threecom;
use crate::vendors::threegpp;
use crate::vendors::threegpp2;
use crate::vendors::acc;
use crate::vendors::airespace;
use crate::vendors::alcatel;
use crate::vendors::alcatel_lucent_service_router;
use crate::vendors::alu_aaa;
use crate::vendors::alteon;
use crate::vendors::apc;
use crate::vendors::aptilo;
use crate::vendors::aruba;
use crate::vendors::azaire;
use crate::vendors::ascend;
use crate::vendors::bay_networks;
use crate::vendors::bluecoat;
use crate::vendors::cablelabs;
use crate::vendors::cabletron;
use crate::vendors::camiant;
use crate::vendors::chillispot;
use crate::vendors::cisco;
use crate::vendors::cisco_asa;
use crate::vendors::cnergee;
use crate::vendors::dlink;
use crate::vendors::dragonwave;
use crate::vendors::eltex;
use crate::vendors::epygi;
use crate::vendors::equallogic;
use crate::vendors::ericsson_ab;
use crate::vendors::extreme;
use crate::vendors::f5;
use crate::vendors::freeradius;
use crate::vendors::freeswitch;
use crate::vendors::foundry;
use crate::vendors::gandalf;
use crate::vendors::h3c;
use crate::vendors::hp;
use crate::vendors::huawei;
use crate::vendors::iea_software;
use crate::vendors::issanni;
use crate::vendors::juniper;
use crate::vendors::karlnet;
use crate::vendors::kineto;
use crate::vendors::livingston;
use crate::vendors::manzara;
use crate::vendors::meinberg;
use crate::vendors::microsoft;
use crate::vendors::mikrotik;
use crate::vendors::motorola;
use crate::vendors::netscreen;
use crate::vendors::ntua;
use crate::vendors::nomadix;
use crate::vendors::nortel;
use crate::vendors::patton;
use crate::vendors::perle;
use crate::vendors::prosoft;
use crate::vendors::purewave;
use crate::vendors::ruckus;
use crate::vendors::netborder;
use crate::vendors::shasta;
use crate::vendors::sg;
use crate::vendors::shiva;
use crate::vendors::sonicwall;
use crate::vendors::starent;
use crate::vendors::symbol;
use crate::vendors::travelping;
use crate::vendors::unisphere;
use crate::vendors::usr;
use crate::vendors::valemountnetworks;
use crate::vendors::versanet;
use crate::vendors::waverider;
use crate::vendors::wimax;
use crate::vendors::xylan;
use crate::vendors::zyxel;
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::parser_match::parse_attribute_content;
use nom::number::streaming::{be_u16, be_u8};
use nom::IResult;
use std::io::{Error, ErrorKind};
use std::result::Result;

// #[derive(Clone, Debug, PartialEq)]
pub struct Data<'a> {
    pub code: Code,
    pub packet_id: u8,
    pub length: u16,
    pub authenticator: &'a [u8],
    offset: usize,
    data: &'a [u8],
    pub result: Result<(), Error>,
}

impl<'a> Iterator for Data<'a> {
    type Item = Attribute<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == self.data.len() {
            return None
        }
    
        let res = do_parse! {&self.data,
            take!(self.offset) >>
            t: be_u8 >>
            l: verify!(be_u8, |val:&u8| *val >= 2) >>
            v: flat_map!(take!(l-2),call!(parse_attribute_content,t)) >>
            ( v, l )
        };

        match res {
            Ok(a) => {
                let (attr, offset) = a.1;
                self.offset += offset as usize;
                Some(attr)
            }
            Err(_) => {
                self.result = Err(Error::new(ErrorKind::InvalidData, "parse error"));
                None
            }
        }
    }
}

pub fn new(data: &[u8]) -> Result<Data<'_>, Error> {
    let res: IResult<&[u8], Data> = do_parse!(
        &data,
        c: be_u8
            >> id: be_u8
            >> len: be_u16
            >> auth: take!(16)
            >> (Data {
                code: Code(c),
                packet_id: id,
                length: len,
                authenticator: auth,
                offset: 20,
                data: data,
                result: Ok(()),
            })
    );

    match res {
        Ok(d) => Ok(d.1),
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "parse error")),
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Code(pub u8);

#[allow(non_upper_case_globals)]
impl Code {
	pub const AccessRequest: Code = Code(1);
	pub const AccessAccept: Code = Code(2);
	pub const AccessReject: Code = Code(3);
	pub const AccountingRequest: Code = Code(4);
	pub const AccountingResponse: Code = Code(5);
	pub const AccessChallenge: Code = Code(11);
	pub const StatusServer: Code = Code(12);
	pub const StatusClient: Code = Code(13);
	pub const Reserved: Code = Code(255);
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ServiceType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ServiceType {
	pub const LoginUser: ServiceType = ServiceType(1);
	pub const FramedUser: ServiceType = ServiceType(2);
	pub const CallbackLoginUser: ServiceType = ServiceType(3);
	pub const CallbackFramedUser: ServiceType = ServiceType(4);
	pub const OutboundUser: ServiceType = ServiceType(5);
	pub const AdministrativeUser: ServiceType = ServiceType(6);
	pub const NasPromptUser: ServiceType = ServiceType(7);
	pub const AuthenticateOnly: ServiceType = ServiceType(8);
	pub const CallbackNasPrompt: ServiceType = ServiceType(9);
	pub const CallCheck: ServiceType = ServiceType(10);
	pub const CallbackAdministrative: ServiceType = ServiceType(11);
	pub const AuthorizeOnly: ServiceType = ServiceType(17);
	pub const FramedManagement: ServiceType = ServiceType(18);
	pub const Voice: ServiceType = ServiceType(12);
	pub const Fax: ServiceType = ServiceType(13);
	pub const ModemRelay: ServiceType = ServiceType(14);
	pub const IappRegister: ServiceType = ServiceType(15);
	pub const IappApCheck: ServiceType = ServiceType(16);
	pub const AnnexAuthorizeOnly: ServiceType = ServiceType(0x06300001);
	pub const AnnexFramedTunnel: ServiceType = ServiceType(0x06300002);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FramedProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl FramedProtocol {
	pub const Ppp: FramedProtocol = FramedProtocol(1);
	pub const Slip: FramedProtocol = FramedProtocol(2);
	pub const Arap: FramedProtocol = FramedProtocol(3);
	pub const GandalfSlml: FramedProtocol = FramedProtocol(4);
	pub const XylogicsIpxSlip: FramedProtocol = FramedProtocol(5);
	pub const Xdot75Synchronous: FramedProtocol = FramedProtocol(6);
	pub const GprsPdpContext: FramedProtocol = FramedProtocol(7);
	pub const Pptp: FramedProtocol = FramedProtocol(9);
	pub const BintecX25: FramedProtocol = FramedProtocol(0x01100002);
	pub const BintecX25Ppp: FramedProtocol = FramedProtocol(0x01100003);
	pub const BintecIpLapb: FramedProtocol = FramedProtocol(0x01100004);
	pub const BintecIpHdlc: FramedProtocol = FramedProtocol(0x01100006);
	pub const BintecMprLapb: FramedProtocol = FramedProtocol(0x01100007);
	pub const BintecMprHdlc: FramedProtocol = FramedProtocol(0x01100008);
	pub const BintecFrameRelay: FramedProtocol = FramedProtocol(0x01100009);
	pub const BintecX31Bchan: FramedProtocol = FramedProtocol(0x0110000a);
	pub const BintecX75Ppp: FramedProtocol = FramedProtocol(0x0110000b);
	pub const BintecX75BtxPpp: FramedProtocol = FramedProtocol(0x0110000c);
	pub const BintecX25Nosig: FramedProtocol = FramedProtocol(0x0110000d);
	pub const BintecX25PppOpt: FramedProtocol = FramedProtocol(0x0110000e);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FramedRouting(pub u32);
 
#[allow(non_upper_case_globals)]
impl FramedRouting {
	pub const None: FramedRouting = FramedRouting(0);
	pub const Broadcast: FramedRouting = FramedRouting(1);
	pub const Listen: FramedRouting = FramedRouting(2);
	pub const BroadcastListen: FramedRouting = FramedRouting(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FramedCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl FramedCompression {
	pub const None: FramedCompression = FramedCompression(0);
	pub const VanJacobsonTcpIp: FramedCompression = FramedCompression(1);
	pub const IpxHeaderCompression: FramedCompression = FramedCompression(2);
	pub const StacLzs: FramedCompression = FramedCompression(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoginService(pub u32);
 
#[allow(non_upper_case_globals)]
impl LoginService {
	pub const Telnet: LoginService = LoginService(0);
	pub const Rlogin: LoginService = LoginService(1);
	pub const TcpClear: LoginService = LoginService(2);
	pub const Portmaster: LoginService = LoginService(3);
	pub const Lat: LoginService = LoginService(4);
	pub const X25Pad: LoginService = LoginService(5);
	pub const X25T3Pos: LoginService = LoginService(6);
	pub const TcpClearQuiet: LoginService = LoginService(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoginTcpPort(pub u32);
 
#[allow(non_upper_case_globals)]
impl LoginTcpPort {
	pub const Telnet: LoginTcpPort = LoginTcpPort(23);
	pub const Rlogin: LoginTcpPort = LoginTcpPort(513);
	pub const Rsh: LoginTcpPort = LoginTcpPort(514);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerminationAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl TerminationAction {
	pub const Default: TerminationAction = TerminationAction(0);
	pub const RadiusRequest: TerminationAction = TerminationAction(1);
	pub const ManageResources: TerminationAction = TerminationAction(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NasPortType(pub u32);
 
#[allow(non_upper_case_globals)]
impl NasPortType {
	pub const Async: NasPortType = NasPortType(0);
	pub const Sync: NasPortType = NasPortType(1);
	pub const Isdn: NasPortType = NasPortType(2);
	pub const IsdnV120: NasPortType = NasPortType(3);
	pub const IsdnV110: NasPortType = NasPortType(4);
	pub const Virtual: NasPortType = NasPortType(5);
	pub const Piafs: NasPortType = NasPortType(6);
	pub const HdlcClearChannel: NasPortType = NasPortType(7);
	pub const Xdot25: NasPortType = NasPortType(8);
	pub const Xdot75: NasPortType = NasPortType(9);
	pub const Gdot3Fax: NasPortType = NasPortType(10);
	pub const Sdsl: NasPortType = NasPortType(11);
	pub const AdslCap: NasPortType = NasPortType(12);
	pub const AdslDmt: NasPortType = NasPortType(13);
	pub const Idsl: NasPortType = NasPortType(14);
	pub const Ethernet: NasPortType = NasPortType(15);
	pub const Xdsl: NasPortType = NasPortType(16);
	pub const Cable: NasPortType = NasPortType(17);
	pub const WirelessOther: NasPortType = NasPortType(18);
	pub const Wireless802dot11: NasPortType = NasPortType(19);
	pub const TokenRing: NasPortType = NasPortType(20);
	pub const Fddi: NasPortType = NasPortType(21);
	pub const Pppoa: NasPortType = NasPortType(30);
	pub const Pppoeoa: NasPortType = NasPortType(31);
	pub const Pppoeoe: NasPortType = NasPortType(32);
	pub const Pppoeovlan: NasPortType = NasPortType(33);
	pub const Pppoeoqinq: NasPortType = NasPortType(34);
	pub const WirelessCdma2000: NasPortType = NasPortType(22);
	pub const WirelessUmts: NasPortType = NasPortType(23);
	pub const Wireless1XEv: NasPortType = NasPortType(24);
	pub const Iapp: NasPortType = NasPortType(25);
	pub const Fttp: NasPortType = NasPortType(26);
	pub const Wireless802dot16: NasPortType = NasPortType(27);
	pub const Wireless802dot20: NasPortType = NasPortType(28);
	pub const Wireless802dot22: NasPortType = NasPortType(29);
	pub const Xpon: NasPortType = NasPortType(35);
	pub const WirelessXgp: NasPortType = NasPortType(36);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AcctStatusType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AcctStatusType {
	pub const Start: AcctStatusType = AcctStatusType(1);
	pub const Stop: AcctStatusType = AcctStatusType(2);
	pub const InterimUpdate: AcctStatusType = AcctStatusType(3);
	pub const AccountingOn: AcctStatusType = AcctStatusType(7);
	pub const AccountingOff: AcctStatusType = AcctStatusType(8);
	pub const Failed: AcctStatusType = AcctStatusType(15);
	pub const TunnelStart: AcctStatusType = AcctStatusType(9);
	pub const TunnelStop: AcctStatusType = AcctStatusType(10);
	pub const TunnelReject: AcctStatusType = AcctStatusType(11);
	pub const TunnelLinkStart: AcctStatusType = AcctStatusType(12);
	pub const TunnelLinkStop: AcctStatusType = AcctStatusType(13);
	pub const TunnelLinkReject: AcctStatusType = AcctStatusType(14);
	pub const AnnexUserReject: AcctStatusType = AcctStatusType(0x06300001);
	pub const AnnexCallReject: AcctStatusType = AcctStatusType(0x06300002);
	pub const AnnexIpcpStart: AcctStatusType = AcctStatusType(0x06300003);
	pub const AnnexIpxcpStart: AcctStatusType = AcctStatusType(0x06300004);
	pub const AnnexAtcpStart: AcctStatusType = AcctStatusType(0x06300005);
	pub const AnnexAccountingRestart: AcctStatusType = AcctStatusType(0x06300006);
	pub const AnnexAccountingShutoff: AcctStatusType = AcctStatusType(0x06300007);
	pub const AnnexTunnelStart: AcctStatusType = AcctStatusType(0x06300008);
	pub const AnnexTunnelStop: AcctStatusType = AcctStatusType(0x06300009);
	pub const AnnexTunnelReject: AcctStatusType = AcctStatusType(0x0630000a);
	pub const AnnexTunnelLinkStart: AcctStatusType = AcctStatusType(0x0630000b);
	pub const AnnexTunnelLinkStop: AcctStatusType = AcctStatusType(0x0630000c);
	pub const AnnexMpStart: AcctStatusType = AcctStatusType(0x0630000d);
	pub const AnnexMpStop: AcctStatusType = AcctStatusType(0x0630000e);
	pub const AnnexLineSeizure: AcctStatusType = AcctStatusType(0x0630000f);
	pub const AnnexRloginStart: AcctStatusType = AcctStatusType(0x06300010);
	pub const AnnexRloginStop: AcctStatusType = AcctStatusType(0x06300011);
	pub const ModemStart: AcctStatusType = AcctStatusType(4);
	pub const ModemStop: AcctStatusType = AcctStatusType(5);
	pub const Cancel: AcctStatusType = AcctStatusType(6);
	pub const WbLogin: AcctStatusType = AcctStatusType(217);
	pub const WbLogout: AcctStatusType = AcctStatusType(218);
	pub const WbWritelog: AcctStatusType = AcctStatusType(219);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AcctAuthentic(pub u32);
 
#[allow(non_upper_case_globals)]
impl AcctAuthentic {
	pub const Radius: AcctAuthentic = AcctAuthentic(1);
	pub const Local: AcctAuthentic = AcctAuthentic(2);
	pub const Remote: AcctAuthentic = AcctAuthentic(3);
	pub const Diameter: AcctAuthentic = AcctAuthentic(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AcctTerminateCause(pub u32);
 
#[allow(non_upper_case_globals)]
impl AcctTerminateCause {
	pub const UserRequest: AcctTerminateCause = AcctTerminateCause(1);
	pub const LostCarrier: AcctTerminateCause = AcctTerminateCause(2);
	pub const LostService: AcctTerminateCause = AcctTerminateCause(3);
	pub const IdleTimeout: AcctTerminateCause = AcctTerminateCause(4);
	pub const SessionTimeout: AcctTerminateCause = AcctTerminateCause(5);
	pub const AdminReset: AcctTerminateCause = AcctTerminateCause(6);
	pub const AdminReboot: AcctTerminateCause = AcctTerminateCause(7);
	pub const PortError: AcctTerminateCause = AcctTerminateCause(8);
	pub const NasError: AcctTerminateCause = AcctTerminateCause(9);
	pub const NasRequest: AcctTerminateCause = AcctTerminateCause(10);
	pub const NasReboot: AcctTerminateCause = AcctTerminateCause(11);
	pub const PortUnneeded: AcctTerminateCause = AcctTerminateCause(12);
	pub const PortPreempted: AcctTerminateCause = AcctTerminateCause(13);
	pub const PortSuspended: AcctTerminateCause = AcctTerminateCause(14);
	pub const ServiceUnavailable: AcctTerminateCause = AcctTerminateCause(15);
	pub const Callback: AcctTerminateCause = AcctTerminateCause(16);
	pub const UserError: AcctTerminateCause = AcctTerminateCause(17);
	pub const HostRequest: AcctTerminateCause = AcctTerminateCause(18);
	pub const SupplicantRestart: AcctTerminateCause = AcctTerminateCause(19);
	pub const ReauthenticationFailure: AcctTerminateCause = AcctTerminateCause(20);
	pub const PortReinit: AcctTerminateCause = AcctTerminateCause(21);
	pub const PortDisabled: AcctTerminateCause = AcctTerminateCause(22);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TunnelType(pub u32);
 
#[allow(non_upper_case_globals)]
impl TunnelType {
	pub const Pptp: TunnelType = TunnelType(1);
	pub const L2F: TunnelType = TunnelType(2);
	pub const L2Tp: TunnelType = TunnelType(3);
	pub const Atmp: TunnelType = TunnelType(4);
	pub const Vtp: TunnelType = TunnelType(5);
	pub const Ah: TunnelType = TunnelType(6);
	pub const Ip: TunnelType = TunnelType(7);
	pub const MinIp: TunnelType = TunnelType(8);
	pub const Esp: TunnelType = TunnelType(9);
	pub const Gre: TunnelType = TunnelType(10);
	pub const Dvs: TunnelType = TunnelType(11);
	pub const IpInIp: TunnelType = TunnelType(12);
	pub const Vlan: TunnelType = TunnelType(13);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TunnelMediumType(pub u32);
 
#[allow(non_upper_case_globals)]
impl TunnelMediumType {
	pub const Ip: TunnelMediumType = TunnelMediumType(1);
	pub const Ipv6: TunnelMediumType = TunnelMediumType(2);
	pub const Nsap: TunnelMediumType = TunnelMediumType(3);
	pub const Hdlc: TunnelMediumType = TunnelMediumType(4);
	pub const Bbn1822: TunnelMediumType = TunnelMediumType(5);
	pub const Ieee802: TunnelMediumType = TunnelMediumType(6);
	pub const Edot163: TunnelMediumType = TunnelMediumType(7);
	pub const Edot164: TunnelMediumType = TunnelMediumType(8);
	pub const Fdot69: TunnelMediumType = TunnelMediumType(9);
	pub const Xdot121: TunnelMediumType = TunnelMediumType(10);
	pub const Ipx: TunnelMediumType = TunnelMediumType(11);
	pub const Appletalk: TunnelMediumType = TunnelMediumType(12);
	pub const DecnetIv: TunnelMediumType = TunnelMediumType(13);
	pub const BanyanVines: TunnelMediumType = TunnelMediumType(14);
	pub const Edot164Nsap: TunnelMediumType = TunnelMediumType(15);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArapZoneAccess(pub u32);
 
#[allow(non_upper_case_globals)]
impl ArapZoneAccess {
	pub const DefaultZone: ArapZoneAccess = ArapZoneAccess(1);
	pub const ZoneFilterInclusive: ArapZoneAccess = ArapZoneAccess(2);
	pub const ZoneFilterExclusive: ArapZoneAccess = ArapZoneAccess(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Prompt(pub u32);
 
#[allow(non_upper_case_globals)]
impl Prompt {
	pub const NoEcho: Prompt = Prompt(0);
	pub const Echo: Prompt = Prompt(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorCause(pub u32);
 
#[allow(non_upper_case_globals)]
impl ErrorCause {
	pub const ResidualContextRemoved: ErrorCause = ErrorCause(201);
	pub const InvalidEapPacket: ErrorCause = ErrorCause(202);
	pub const UnsupportedAttribute: ErrorCause = ErrorCause(401);
	pub const MissingAttribute: ErrorCause = ErrorCause(402);
	pub const NasIdentificationMismatch: ErrorCause = ErrorCause(403);
	pub const InvalidRequest: ErrorCause = ErrorCause(404);
	pub const UnsupportedService: ErrorCause = ErrorCause(405);
	pub const UnsupportedExtension: ErrorCause = ErrorCause(406);
	pub const AdministrativelyProhibited: ErrorCause = ErrorCause(501);
	pub const ProxyRequestNotRoutable: ErrorCause = ErrorCause(502);
	pub const SessionContextNotFound: ErrorCause = ErrorCause(503);
	pub const SessionContextNotRemovable: ErrorCause = ErrorCause(504);
	pub const ProxyProcessingError: ErrorCause = ErrorCause(505);
	pub const ResourcesUnavailable: ErrorCause = ErrorCause(506);
	pub const RequestInitiated: ErrorCause = ErrorCause(507);
	pub const InvalidAttributeValue: ErrorCause = ErrorCause(407);
	pub const MultipleSessionSelectionUnsupported: ErrorCause = ErrorCause(508);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IngressFilters(pub u32);
 
#[allow(non_upper_case_globals)]
impl IngressFilters {
	pub const Enabled: IngressFilters = IngressFilters(1);
	pub const Disabled: IngressFilters = IngressFilters(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocationCapable(pub u32);
 
#[allow(non_upper_case_globals)]
impl LocationCapable {
	pub const CivicLocation: LocationCapable = LocationCapable(1);
	pub const GeoLocation: LocationCapable = LocationCapable(2);
	pub const UsersLocation: LocationCapable = LocationCapable(4);
	pub const NasLocation: LocationCapable = LocationCapable(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RequestedLocationInfo(pub u32);
 
#[allow(non_upper_case_globals)]
impl RequestedLocationInfo {
	pub const CivicLocation: RequestedLocationInfo = RequestedLocationInfo(1);
	pub const GeoLocation: RequestedLocationInfo = RequestedLocationInfo(2);
	pub const UsersLocation: RequestedLocationInfo = RequestedLocationInfo(4);
	pub const NasLocation: RequestedLocationInfo = RequestedLocationInfo(8);
	pub const FutureRequests: RequestedLocationInfo = RequestedLocationInfo(16);
	pub const None: RequestedLocationInfo = RequestedLocationInfo(32);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FramedManagement(pub u32);
 
#[allow(non_upper_case_globals)]
impl FramedManagement {
	pub const Snmp: FramedManagement = FramedManagement(1);
	pub const WebBased: FramedManagement = FramedManagement(2);
	pub const Netconf: FramedManagement = FramedManagement(3);
	pub const Ftp: FramedManagement = FramedManagement(4);
	pub const Tftp: FramedManagement = FramedManagement(5);
	pub const Sftp: FramedManagement = FramedManagement(6);
	pub const Rcp: FramedManagement = FramedManagement(7);
	pub const Scp: FramedManagement = FramedManagement(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ManagementTransportProtection(pub u32);
 
#[allow(non_upper_case_globals)]
impl ManagementTransportProtection {
	pub const NoProtection: ManagementTransportProtection = ManagementTransportProtection(1);
	pub const IntegrityProtection: ManagementTransportProtection = ManagementTransportProtection(2);
	pub const IntegrityConfidentialityProtection: ManagementTransportProtection = ManagementTransportProtection(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EapLowerLayer(pub u32);
 
#[allow(non_upper_case_globals)]
impl EapLowerLayer {
	pub const WiredIeee802dot1X: EapLowerLayer = EapLowerLayer(1);
	pub const Ieee802dot1XNoPreauth: EapLowerLayer = EapLowerLayer(2);
	pub const Ieee802dot1XPreauth: EapLowerLayer = EapLowerLayer(3);
	pub const Ieee802dot16E: EapLowerLayer = EapLowerLayer(4);
	pub const Ikev2: EapLowerLayer = EapLowerLayer(5);
	pub const Ppp: EapLowerLayer = EapLowerLayer(6);
	pub const PanaNoPreauth: EapLowerLayer = EapLowerLayer(7);
	pub const GssApi: EapLowerLayer = EapLowerLayer(8);
	pub const PanaPreauth: EapLowerLayer = EapLowerLayer(9);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FragStatus(pub u32);
 
#[allow(non_upper_case_globals)]
impl FragStatus {
	pub const Reserved: FragStatus = FragStatus(0);
	pub const FragmentationSupported: FragStatus = FragStatus(1);
	pub const MoreDataPending: FragStatus = FragStatus(2);
	pub const MoreDataRequest: FragStatus = FragStatus(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MultiLinkFlag(pub u32);
 
#[allow(non_upper_case_globals)]
impl MultiLinkFlag {
	pub const True: MultiLinkFlag = MultiLinkFlag(1);
	pub const False: MultiLinkFlag = MultiLinkFlag(0);
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute<'a> {
	UserName(&'a [u8]), // AVP with plain value, tag=1, type=string
	UserPassword(&'a [u8]), // AVP with plain value, tag=2, type=string
	ChapPassword(&'a [u8]), // AVP with plain value, tag=3, type=octets
	NasIpAddress(Ipv4Addr), // AVP with plain value, tag=4, type=ipaddr
	NasPort(u32), // AVP with plain value, tag=5, type=integer
	ServiceType(ServiceType), // AVP with Enum value, tag=6, type=integer
	FramedProtocol(FramedProtocol), // AVP with Enum value, tag=7, type=integer
	FramedIpAddress(Ipv4Addr), // AVP with plain value, tag=8, type=ipaddr
	FramedIpNetmask(Ipv4Addr), // AVP with plain value, tag=9, type=ipaddr
	FramedRouting(FramedRouting), // AVP with Enum value, tag=10, type=integer
	FilterId(&'a [u8]), // AVP with plain value, tag=11, type=string
	FramedMtu(u32), // AVP with plain value, tag=12, type=integer
	FramedCompression(FramedCompression), // AVP with Enum value, tag=13, type=integer
	LoginIpHost(Ipv4Addr), // AVP with plain value, tag=14, type=ipaddr
	LoginService(LoginService), // AVP with Enum value, tag=15, type=integer
	LoginTcpPort(LoginTcpPort), // AVP with Enum value, tag=16, type=integer
	ReplyMessage(&'a [u8]), // AVP with plain value, tag=18, type=string
	CallbackNumber(&'a [u8]), // AVP with plain value, tag=19, type=string
	CallbackId(&'a [u8]), // AVP with plain value, tag=20, type=string
	FramedRoute(&'a [u8]), // AVP with plain value, tag=22, type=string
	FramedIpxNetwork(Ipv4Addr), // AVP with plain value, tag=23, type=ipaddr
	State(&'a [u8]), // AVP with plain value, tag=24, type=octets
	Class(&'a [u8]), // AVP with plain value, tag=25, type=octets
	VendorSpecific(&'a [u8]), // AVP with plain value, tag=26, type=octets
	SessionTimeout(u32), // AVP with plain value, tag=27, type=integer
	IdleTimeout(u32), // AVP with plain value, tag=28, type=integer
	TerminationAction(TerminationAction), // AVP with Enum value, tag=29, type=integer
	CalledStationId(&'a [u8]), // AVP with plain value, tag=30, type=string
	CallingStationId(&'a [u8]), // AVP with plain value, tag=31, type=string
	NasIdentifier(&'a [u8]), // AVP with plain value, tag=32, type=string
	ProxyState(&'a [u8]), // AVP with plain value, tag=33, type=octets
	LoginLatService(&'a [u8]), // AVP with plain value, tag=34, type=string
	LoginLatNode(&'a [u8]), // AVP with plain value, tag=35, type=string
	LoginLatGroup(&'a [u8]), // AVP with plain value, tag=36, type=octets
	FramedAppletalkLink(u32), // AVP with plain value, tag=37, type=integer
	FramedAppletalkNetwork(u32), // AVP with plain value, tag=38, type=integer
	FramedAppletalkZone(&'a [u8]), // AVP with plain value, tag=39, type=string
	ChapChallenge(&'a [u8]), // AVP with plain value, tag=60, type=octets
	NasPortType(NasPortType), // AVP with Enum value, tag=61, type=integer
	PortLimit(u32), // AVP with plain value, tag=62, type=integer
	LoginLatPort(&'a [u8]), // AVP with plain value, tag=63, type=string
	AcctStatusType(AcctStatusType), // AVP with Enum value, tag=40, type=integer
	AcctDelayTime(u32), // AVP with plain value, tag=41, type=integer
	AcctInputOctets(u32), // AVP with plain value, tag=42, type=integer
	AcctOutputOctets(u32), // AVP with plain value, tag=43, type=integer
	AcctSessionId(&'a [u8]), // AVP with plain value, tag=44, type=string
	AcctAuthentic(AcctAuthentic), // AVP with Enum value, tag=45, type=integer
	AcctSessionTime(u32), // AVP with plain value, tag=46, type=integer
	AcctInputPackets(u32), // AVP with plain value, tag=47, type=integer
	AcctOutputPackets(u32), // AVP with plain value, tag=48, type=integer
	AcctTerminateCause(AcctTerminateCause), // AVP with Enum value, tag=49, type=integer
	AcctMultiSessionId(&'a [u8]), // AVP with plain value, tag=50, type=string
	AcctLinkCount(u32), // AVP with plain value, tag=51, type=integer
	AcctTunnelConnection(&'a [u8]), // AVP with plain value, tag=68, type=string
	AcctTunnelPacketsLost(u32), // AVP with plain value, tag=86, type=integer
	TunnelType(TunnelType), // AVP with Enum value, tag=64, type=integer
	TunnelMediumType(TunnelMediumType), // AVP with Enum value, tag=65, type=integer
	TunnelClientEndpoint(&'a [u8]), // AVP with plain value, tag=66, type=string
	TunnelServerEndpoint(&'a [u8]), // AVP with plain value, tag=67, type=string
	TunnelPassword(&'a [u8]), // AVP with plain value, tag=69, type=string
	TunnelPrivateGroupId(&'a [u8]), // AVP with plain value, tag=81, type=string
	TunnelAssignmentId(&'a [u8]), // AVP with plain value, tag=82, type=string
	TunnelPreference(u32), // AVP with plain value, tag=83, type=integer
	TunnelClientAuthId(&'a [u8]), // AVP with plain value, tag=90, type=string
	TunnelServerAuthId(&'a [u8]), // AVP with plain value, tag=91, type=string
	AcctInputGigawords(u32), // AVP with plain value, tag=52, type=integer
	AcctOutputGigawords(u32), // AVP with plain value, tag=53, type=integer
	EventTimestamp(u32), // AVP with plain value, tag=55, type=date
	ArapPassword(&'a [u8]), // AVP with plain value, tag=70, type=octets
	ArapFeatures(&'a [u8]), // AVP with plain value, tag=71, type=octets
	ArapZoneAccess(ArapZoneAccess), // AVP with Enum value, tag=72, type=integer
	ArapSecurity(u32), // AVP with plain value, tag=73, type=integer
	ArapSecurityData(&'a [u8]), // AVP with plain value, tag=74, type=string
	PasswordRetry(u32), // AVP with plain value, tag=75, type=integer
	Prompt(Prompt), // AVP with Enum value, tag=76, type=integer
	ConnectInfo(&'a [u8]), // AVP with plain value, tag=77, type=string
	ConfigurationToken(&'a [u8]), // AVP with plain value, tag=78, type=string
	EapMessage(&'a [u8]), // AVP with plain value, tag=79, type=octets
	MessageAuthenticator(&'a [u8]), // AVP with plain value, tag=80, type=octets
	ArapChallengeResponse(&'a [u8]), // AVP with plain value, tag=84, type=octets
	AcctInterimInterval(u32), // AVP with plain value, tag=85, type=integer
	NasPortId(&'a [u8]), // AVP with plain value, tag=87, type=string
	FramedPool(&'a [u8]), // AVP with plain value, tag=88, type=string
	NasIpv6Address(Ipv6Addr), // AVP with plain value, tag=95, type=ipv6addr
	FramedInterfaceId(&'a [u8]), // AVP with plain value, tag=96, type=ifid
	FramedIpv6Prefix(&'a [u8]), // AVP with plain value, tag=97, type=ipv6prefix
	LoginIpv6Host(Ipv6Addr), // AVP with plain value, tag=98, type=ipv6addr
	FramedIpv6Route(&'a [u8]), // AVP with plain value, tag=99, type=string
	FramedIpv6Pool(&'a [u8]), // AVP with plain value, tag=100, type=string
	ErrorCause(ErrorCause), // AVP with Enum value, tag=101, type=integer
	EapKeyName(&'a [u8]), // AVP with plain value, tag=102, type=string
	ChargeableUserIdentity(&'a [u8]), // AVP with plain value, tag=89, type=string
	EgressVlanid(u32), // AVP with plain value, tag=56, type=integer
	IngressFilters(IngressFilters), // AVP with Enum value, tag=57, type=integer
	EgressVlanName(&'a [u8]), // AVP with plain value, tag=58, type=string
	UserPriorityTable(&'a [u8]), // AVP with plain value, tag=59, type=octets
	DelegatedIpv6Prefix(&'a [u8]), // AVP with plain value, tag=123, type=ipv6prefix
	NasFilterRule(&'a [u8]), // AVP with plain value, tag=92, type=string
	Mip6FeatureVector(u64), // AVP with plain value, tag=124, type=integer64
	Mip6HomeLinkPrefix(&'a [u8]), // AVP with plain value, tag=125, type=octets
	OperatorName(&'a [u8]), // AVP with plain value, tag=126, type=string
	LocationInformation(&'a [u8]), // AVP with plain value, tag=127, type=octets
	LocationData(&'a [u8]), // AVP with plain value, tag=128, type=octets
	BasicLocationPolicyRules(&'a [u8]), // AVP with plain value, tag=129, type=octets
	ExtendedLocationPolicyRules(&'a [u8]), // AVP with plain value, tag=130, type=string
	LocationCapable(LocationCapable), // AVP with Enum value, tag=131, type=integer
	RequestedLocationInfo(RequestedLocationInfo), // AVP with Enum value, tag=132, type=integer
	FramedManagement(FramedManagement), // AVP with Enum value, tag=133, type=integer
	ManagementTransportProtection(ManagementTransportProtection), // AVP with Enum value, tag=134, type=integer
	ManagementPolicyId(&'a [u8]), // AVP with plain value, tag=135, type=string
	ManagementPrivilegeLevel(u32), // AVP with plain value, tag=136, type=integer
	PkmSsCert(&'a [u8]), // AVP with plain value, tag=137, type=octets
	PkmCaCert(&'a [u8]), // AVP with plain value, tag=138, type=octets
	PkmConfigSettings(&'a [u8]), // AVP with plain value, tag=139, type=octets
	PkmCryptosuiteList(&'a [u8]), // AVP with plain value, tag=140, type=octets
	PkmSaid(u16), // AVP with plain value, tag=141, type=short
	PkmSaDescriptor(&'a [u8]), // AVP with plain value, tag=142, type=octets
	PkmAuthKey(&'a [u8]), // AVP with plain value, tag=143, type=octets
	DsLiteTunnelName(&'a [u8]), // AVP with plain value, tag=144, type=string
	MobileNodeIdentifier(&'a [u8]), // AVP with plain value, tag=145, type=octets
	ServiceSelection(&'a [u8]), // AVP with plain value, tag=146, type=string
	Pmip6HomeLmaIpv6Address(Ipv6Addr), // AVP with plain value, tag=147, type=ipv6addr
	Pmip6VisitedLmaIpv6Address(Ipv6Addr), // AVP with plain value, tag=148, type=ipv6addr
	Pmip6HomeLmaIpv4Address(Ipv4Addr), // AVP with plain value, tag=149, type=ipaddr
	Pmip6VisitedLmaIpv4Address(Ipv4Addr), // AVP with plain value, tag=150, type=ipaddr
	Pmip6HomeHnPrefix(&'a [u8]), // AVP with plain value, tag=151, type=ipv6prefix
	Pmip6VisitedHnPrefix(&'a [u8]), // AVP with plain value, tag=152, type=ipv6prefix
	Pmip6HomeInterfaceId(&'a [u8]), // AVP with plain value, tag=153, type=ifid
	Pmip6VisitedInterfaceId(&'a [u8]), // AVP with plain value, tag=154, type=ifid
	Pmip6HomeIpv4Hoa(&'a [u8]), // AVP with plain value, tag=155, type=ipv4prefix
	Pmip6VisitedIpv4Hoa(&'a [u8]), // AVP with plain value, tag=156, type=ipv4prefix
	Pmip6HomeDhcp4ServerAddress(Ipv4Addr), // AVP with plain value, tag=157, type=ipaddr
	Pmip6VisitedDhcp4ServerAddress(Ipv4Addr), // AVP with plain value, tag=158, type=ipaddr
	Pmip6HomeDhcp6ServerAddress(Ipv6Addr), // AVP with plain value, tag=159, type=ipv6addr
	Pmip6VisitedDhcp6ServerAddress(Ipv6Addr), // AVP with plain value, tag=160, type=ipv6addr
	Pmip6HomeIpv4Gateway(Ipv4Addr), // AVP with plain value, tag=161, type=ipaddr
	Pmip6VisitedIpv4Gateway(Ipv4Addr), // AVP with plain value, tag=162, type=ipaddr
	EapLowerLayer(EapLowerLayer), // AVP with Enum value, tag=163, type=integer
	FramedIpv6Address(Ipv6Addr), // AVP with plain value, tag=168, type=ipv6addr
	DnsServerIpv6Address(Ipv6Addr), // AVP with plain value, tag=169, type=ipv6addr
	RouteIpv6Information(&'a [u8]), // AVP with plain value, tag=170, type=ipv6prefix
	DelegatedIpv6PrefixPool(&'a [u8]), // AVP with plain value, tag=171, type=string
	StatefulIpv6AddressPool(&'a [u8]), // AVP with plain value, tag=172, type=string
	ExtendedAttribute1(&'a [u8]), // AVP with plain value, tag=241, type=extended
	ExtendedAttribute2(&'a [u8]), // AVP with plain value, tag=242, type=extended
	ExtendedAttribute3(&'a [u8]), // AVP with plain value, tag=243, type=extended
	ExtendedAttribute4(&'a [u8]), // AVP with plain value, tag=244, type=extended
	ExtendedAttribute5(&'a [u8]), // AVP with plain value, tag=245, type=long-extended
	ExtendedAttribute6(&'a [u8]), // AVP with plain value, tag=246, type=long-extended
	Ipv66RdConfiguration(&'a [u8]), // AVP with plain value, tag=173, type=tlv
	GssAcceptorServiceName(&'a [u8]), // AVP with plain value, tag=164, type=string
	GssAcceptorHostName(&'a [u8]), // AVP with plain value, tag=165, type=string
	GssAcceptorServiceSpecifics(&'a [u8]), // AVP with plain value, tag=166, type=string
	GssAcceptorRealmName(&'a [u8]), // AVP with plain value, tag=167, type=string
	CharNoecho(u32), // AVP with plain value, tag=250, type=integer
	OriginatingLineInfo(&'a [u8]), // AVP with plain value, tag=94, type=string
	DigestResponse(&'a [u8]), // AVP with plain value, tag=206, type=string
	DigestAttributes(&'a [u8]), // AVP with plain value, tag=207, type=octets
	VsaAdslAgentCircuitId(&'a [u8]), // from ADSL-Forum with plain value, tag=1, type=string
	VsaAdslAgentRemoteId(&'a [u8]), // from ADSL-Forum with plain value, tag=2, type=string
	VsaActualDataRateUpstream(u32), // from ADSL-Forum with plain value, tag=129, type=integer
	VsaActualDataRateDownstream(u32), // from ADSL-Forum with plain value, tag=130, type=integer
	VsaMinimumDataRateUpstream(u32), // from ADSL-Forum with plain value, tag=131, type=integer
	VsaMinimumDataRateDownstream(u32), // from ADSL-Forum with plain value, tag=132, type=integer
	VsaAttainableDataRateUpstream(u32), // from ADSL-Forum with plain value, tag=133, type=integer
	VsaAttainableDataRateDownstream(u32), // from ADSL-Forum with plain value, tag=134, type=integer
	VsaMaximumDataRateUpstream(u32), // from ADSL-Forum with plain value, tag=135, type=integer
	VsaMaximumDataRateDownstream(u32), // from ADSL-Forum with plain value, tag=136, type=integer
	VsaMinimumDataRateUpstreamLowPower(u32), // from ADSL-Forum with plain value, tag=137, type=integer
	VsaMinimumDataRateDownstreamLowPower(u32), // from ADSL-Forum with plain value, tag=138, type=integer
	VsaMaximumInterleavingDelayUpstream(u32), // from ADSL-Forum with plain value, tag=139, type=integer
	VsaActualInterleavingDelayUpstream(u32), // from ADSL-Forum with plain value, tag=140, type=integer
	VsaMaximumInterleavingDelayDownstream(u32), // from ADSL-Forum with plain value, tag=141, type=integer
	VsaActualInterleavingDelayDownstream(u32), // from ADSL-Forum with plain value, tag=142, type=integer
	VsaAccessLoopEncapsulation(&'a [u8]), // from ADSL-Forum with plain value, tag=144, type=octets
	VsaIwfSession(&'a [u8]), // from ADSL-Forum with plain value, tag=252, type=octets
	VsaThreeComUserAccessLevel(threecom::ThreeComUserAccessLevel), // from 3com with Enum value, tag=1, type=integer
	VsaThreeComVlanName(&'a [u8]), // from 3com with plain value, tag=2, type=string
	VsaThreeComMobilityProfile(&'a [u8]), // from 3com with plain value, tag=3, type=string
	VsaThreeComEncryptionType(&'a [u8]), // from 3com with plain value, tag=4, type=string
	VsaThreeComTimeOfDay(&'a [u8]), // from 3com with plain value, tag=5, type=string
	VsaThreeComSsid(&'a [u8]), // from 3com with plain value, tag=6, type=string
	VsaThreeComEndDate(&'a [u8]), // from 3com with plain value, tag=7, type=string
	VsaThreeComUrl(&'a [u8]), // from 3com with plain value, tag=8, type=string
	VsaThreeComConnectId(u32), // from 3com with plain value, tag=26, type=integer
	VsaThreeComNasStartupTimestamp(u32), // from 3com with plain value, tag=59, type=integer
	VsaThreeComIpHostAddr(&'a [u8]), // from 3com with plain value, tag=60, type=string
	VsaThreeComProductId(&'a [u8]), // from 3com with plain value, tag=255, type=string
	VsaThreeGppImsi(&'a [u8]), // from 3GPP with plain value, tag=1, type=string
	VsaThreeGppChargingId(u32), // from 3GPP with plain value, tag=2, type=integer
	VsaThreeGppPdpType(threegpp::ThreeGppPdpType), // from 3GPP with Enum value, tag=3, type=integer
	VsaThreeGppChargingGatewayAddress(Ipv4Addr), // from 3GPP with plain value, tag=4, type=ipaddr
	VsaThreeGppGprsNegotiatedQosProfile(&'a [u8]), // from 3GPP with plain value, tag=5, type=string
	VsaThreeGppSgsnAddress(Ipv4Addr), // from 3GPP with plain value, tag=6, type=ipaddr
	VsaThreeGppGgsnAddress(Ipv4Addr), // from 3GPP with plain value, tag=7, type=ipaddr
	VsaThreeGppImsiMccMnc(&'a [u8]), // from 3GPP with plain value, tag=8, type=string
	VsaThreeGppGgsnMccMnc(&'a [u8]), // from 3GPP with plain value, tag=9, type=string
	VsaThreeGppNsapi(&'a [u8]), // from 3GPP with plain value, tag=10, type=string
	VsaThreeGppSessionStopIndicator(&'a [u8]), // from 3GPP with plain value, tag=11, type=octets
	VsaThreeGppSelectionMode(&'a [u8]), // from 3GPP with plain value, tag=12, type=string
	VsaThreeGppChargingCharacteristics(&'a [u8]), // from 3GPP with plain value, tag=13, type=string
	VsaThreeGppChargingGatewayIpv6Address(Ipv6Addr), // from 3GPP with plain value, tag=14, type=ipv6addr
	VsaThreeGppSgsnIpv6Address(Ipv6Addr), // from 3GPP with plain value, tag=15, type=ipv6addr
	VsaThreeGppGgsnIpv6Address(Ipv6Addr), // from 3GPP with plain value, tag=16, type=ipv6addr
	VsaThreeGppIpv6DnsServers(&'a [u8]), // from 3GPP with plain value, tag=17, type=octets
	VsaThreeGppSgsnMccMnc(&'a [u8]), // from 3GPP with plain value, tag=18, type=string
	VsaThreeGppTeardownIndicator(&'a [u8]), // from 3GPP with plain value, tag=19, type=octets
	VsaThreeGppImeisv(&'a [u8]), // from 3GPP with plain value, tag=20, type=string
	VsaThreeGppRatType(threegpp::ThreeGppRatType), // from 3GPP with Enum value, tag=21, type=byte
	VsaThreeGppUserLocationInfo(&'a [u8]), // from 3GPP with plain value, tag=22, type=octets
	VsaThreeGppMsTimezone(&'a [u8]), // from 3GPP with plain value, tag=23, type=octets
	VsaThreeGppCamelChargingInfo(&'a [u8]), // from 3GPP with plain value, tag=24, type=octets
	VsaThreeGppPacketFilter(&'a [u8]), // from 3GPP with plain value, tag=25, type=octets
	VsaThreeGppNegotiatedDscp(u8), // from 3GPP with plain value, tag=26, type=byte
	VsaThreeGppAllocateIpType(threegpp::ThreeGppAllocateIpType), // from 3GPP with Enum value, tag=27, type=integer
	VsaThreeGpp2IkePresharedSecretRequest(u32), // from 3GPP2 with plain value, tag=1, type=integer
	VsaThreeGpp2SecurityLevel(u32), // from 3GPP2 with plain value, tag=2, type=integer
	VsaThreeGpp2PreSharedSecret(&'a [u8]), // from 3GPP2 with plain value, tag=3, type=string
	VsaThreeGpp2ReverseTunnelSpec(u32), // from 3GPP2 with plain value, tag=4, type=integer
	VsaThreeGpp2DiffservClassOption(u32), // from 3GPP2 with plain value, tag=5, type=integer
	VsaThreeGpp2AccountingContainer(&'a [u8]), // from 3GPP2 with plain value, tag=6, type=octets
	VsaThreeGpp2HomeAgentIpAddress(&'a [u8]), // from 3GPP2 with plain value, tag=7, type=octets
	VsaThreeGpp2Keyid(&'a [u8]), // from 3GPP2 with plain value, tag=8, type=string
	VsaThreeGpp2PcfIpAddress(Ipv4Addr), // from 3GPP2 with plain value, tag=9, type=ipaddr
	VsaThreeGpp2Bsid(&'a [u8]), // from 3GPP2 with plain value, tag=10, type=string
	VsaThreeGpp2UserId(u32), // from 3GPP2 with plain value, tag=11, type=integer
	VsaThreeGpp2ForwardFchMuxOption(u32), // from 3GPP2 with plain value, tag=12, type=integer
	VsaThreeGpp2ReverseFchMuxOption(u32), // from 3GPP2 with plain value, tag=13, type=integer
	VsaThreeGpp2ServiceOption(u32), // from 3GPP2 with plain value, tag=16, type=integer
	VsaThreeGpp2ForwardTrafficType(u32), // from 3GPP2 with plain value, tag=17, type=integer
	VsaThreeGpp2ReverseTrafficType(u32), // from 3GPP2 with plain value, tag=18, type=integer
	VsaThreeGpp2FchFrameSize(u32), // from 3GPP2 with plain value, tag=19, type=integer
	VsaThreeGpp2ForwardFchRc(u32), // from 3GPP2 with plain value, tag=20, type=integer
	VsaThreeGpp2ReverseFchRc(u32), // from 3GPP2 with plain value, tag=21, type=integer
	VsaThreeGpp2IpTechnology(u32), // from 3GPP2 with plain value, tag=22, type=integer
	VsaThreeGpp2CompulsoryTunnelIndicator(u32), // from 3GPP2 with plain value, tag=23, type=integer
	VsaThreeGpp2ReleaseIndicator(u32), // from 3GPP2 with plain value, tag=24, type=integer
	VsaThreeGpp2BadPppFrameCount(u32), // from 3GPP2 with plain value, tag=25, type=integer
	VsaThreeGpp2NumberActiveTransitions(u32), // from 3GPP2 with plain value, tag=30, type=integer
	VsaThreeGpp2TerminatingSdbOctetCount(u32), // from 3GPP2 with plain value, tag=31, type=integer
	VsaThreeGpp2OriginatingSdbOctetCount(u32), // from 3GPP2 with plain value, tag=32, type=integer
	VsaThreeGpp2TerminatingNumberSdbs(u32), // from 3GPP2 with plain value, tag=33, type=integer
	VsaThreeGpp2OriginatingNumberSdbs(u32), // from 3GPP2 with plain value, tag=34, type=integer
	VsaThreeGpp2IpQos(u32), // from 3GPP2 with plain value, tag=36, type=integer
	VsaThreeGpp2AirlinkPriority(u32), // from 3GPP2 with plain value, tag=39, type=integer
	VsaThreeGpp2AirlinkRecordType(threegpp2::ThreeGpp2AirlinkRecordType), // from 3GPP2 with Enum value, tag=40, type=integer
	VsaThreeGpp2RPSessionId(&'a [u8]), // from 3GPP2 with plain value, tag=41, type=octets
	VsaThreeGpp2AirlinkSequenceNumber(u32), // from 3GPP2 with plain value, tag=42, type=integer
	VsaThreeGpp2ReceivedHdlcOctets(u32), // from 3GPP2 with plain value, tag=43, type=integer
	VsaThreeGpp2CorrelationId(&'a [u8]), // from 3GPP2 with plain value, tag=44, type=string
	VsaThreeGpp2ModuleOrigTermIndicator(u32), // from 3GPP2 with plain value, tag=45, type=integer
	VsaThreeGpp2InboundMobileIpSigOctets(u32), // from 3GPP2 with plain value, tag=46, type=integer
	VsaThreeGpp2OutboundMobileIpSigOctets(u32), // from 3GPP2 with plain value, tag=47, type=integer
	VsaThreeGpp2SessionContinue(u32), // from 3GPP2 with plain value, tag=48, type=integer
	VsaThreeGpp2ActiveTime(u32), // from 3GPP2 with plain value, tag=49, type=integer
	VsaThreeGpp2DcchFrameSize(u32), // from 3GPP2 with plain value, tag=50, type=integer
	VsaThreeGpp2BeginSession(u32), // from 3GPP2 with plain value, tag=51, type=integer
	VsaThreeGpp2Esn(&'a [u8]), // from 3GPP2 with plain value, tag=52, type=string
	VsaThreeGpp2SKey(&'a [u8]), // from 3GPP2 with plain value, tag=54, type=octets
	VsaThreeGpp2SRequest(u32), // from 3GPP2 with plain value, tag=55, type=integer
	VsaThreeGpp2SLifetime(u32), // from 3GPP2 with plain value, tag=56, type=date
	VsaThreeGpp2MnHaSpi(u32), // from 3GPP2 with plain value, tag=57, type=integer
	VsaThreeGpp2MnHaSharedKey(&'a [u8]), // from 3GPP2 with plain value, tag=58, type=string
	VsaThreeGpp2RemoteIpAddress(&'a [u8]), // from 3GPP2 with plain value, tag=59, type=octets
	VsaThreeGpp2HrpdAccessOrTerminalAuthenticationAnd1XAccessAuthorization(u32), // from 3GPP2 with plain value, tag=60, type=integer
	VsaThreeGpp2AtHardwareIdentifier(u32), // from 3GPP2 with plain value, tag=61, type=integer
	VsaThreeGpp2RemoteIpv6Address(&'a [u8]), // from 3GPP2 with plain value, tag=70, type=octets
	VsaThreeGpp2RemoteAddressTableIndex(&'a [u8]), // from 3GPP2 with plain value, tag=71, type=octets
	VsaThreeGpp2RemoteIpv4AddrOctetCount(&'a [u8]), // from 3GPP2 with plain value, tag=72, type=octets
	VsaThreeGpp2AllowedDiffservMarking(&'a [u8]), // from 3GPP2 with plain value, tag=73, type=tlv
	VsaThreeGpp2ServiceOptionProfile(&'a [u8]), // from 3GPP2 with plain value, tag=74, type=tlv
	VsaThreeGpp2DnsUpdateRequired(u32), // from 3GPP2 with plain value, tag=75, type=integer
	VsaThreeGpp2AlwaysOn(u32), // from 3GPP2 with plain value, tag=78, type=integer
	VsaThreeGpp2ForeignAgentAddress(Ipv4Addr), // from 3GPP2 with plain value, tag=79, type=ipaddr
	VsaThreeGpp2LastUserActivityTime(u32), // from 3GPP2 with plain value, tag=80, type=integer
	VsaThreeGpp2MnAaaRemovalIndication(u32), // from 3GPP2 with plain value, tag=81, type=integer
	VsaThreeGpp2RnPacketDataInactivityTimer(u32), // from 3GPP2 with plain value, tag=82, type=integer
	VsaThreeGpp2ForwardPdchRc(u32), // from 3GPP2 with plain value, tag=83, type=integer
	VsaThreeGpp2ForwardDcchMuxOption(u32), // from 3GPP2 with plain value, tag=84, type=integer
	VsaThreeGpp2ReverseDcchMuxOption(u32), // from 3GPP2 with plain value, tag=85, type=integer
	VsaThreeGpp2ForwardDcchRc(u32), // from 3GPP2 with plain value, tag=86, type=integer
	VsaThreeGpp2ReverseDhhcRc(u32), // from 3GPP2 with plain value, tag=87, type=integer
	VsaThreeGpp2SessionTerminationCapability(u32), // from 3GPP2 with plain value, tag=88, type=integer
	VsaThreeGpp2AllowedPersistentTfts(u32), // from 3GPP2 with plain value, tag=89, type=integer
	VsaThreeGpp2PrepaidAcctQuota(&'a [u8]), // from 3GPP2 with plain value, tag=90, type=tlv
	VsaThreeGpp2PrepaidAcctCapability(&'a [u8]), // from 3GPP2 with plain value, tag=91, type=tlv
	VsaThreeGpp2MipLifetime(&'a [u8]), // from 3GPP2 with plain value, tag=92, type=octets
	VsaThreeGpp2AcctStopTrigger(u32), // from 3GPP2 with plain value, tag=93, type=integer
	VsaThreeGpp2ServiceReferenceId(&'a [u8]), // from 3GPP2 with plain value, tag=94, type=octets
	VsaThreeGpp2DnsUpdateCapability(u32), // from 3GPP2 with plain value, tag=95, type=integer
	VsaThreeGpp2DisconnectReason(u32), // from 3GPP2 with plain value, tag=96, type=integer
	VsaThreeGpp2RemoteIpv6OctetCount(&'a [u8]), // from 3GPP2 with plain value, tag=97, type=octets
	VsaThreeGpp2PrepaidTariffSwitching(&'a [u8]), // from 3GPP2 with plain value, tag=98, type=tlv
	VsaThreeGpp2AuthorizationParameters(&'a [u8]), // from 3GPP2 with plain value, tag=99, type=tlv
	VsaThreeGpp2BcmcsFlowId(&'a [u8]), // from 3GPP2 with plain value, tag=100, type=bytes
	VsaThreeGpp2BcmcsCapability(&'a [u8]), // from 3GPP2 with plain value, tag=101, type=tlv
	VsaThreeGpp2CommonSessionInfo(&'a [u8]), // from 3GPP2 with plain value, tag=102, type=tlv
	VsaBsnSessionInfo(&'a [u8]), // from 3GPP2 with plain value, tag=103, type=tlv
	VsaRanSessionInfo(&'a [u8]), // from 3GPP2 with plain value, tag=104, type=tlv
	VsaReasonCode(threegpp2::ReasonCode), // from 3GPP2 with Enum value, tag=105, type=integer
	VsaThreeGpp2BcmcsFlowTransmissionTime(u32), // from 3GPP2 with plain value, tag=107, type=date
	VsaThreeGpp2Subnet(&'a [u8]), // from 3GPP2 with plain value, tag=108, type=tlv
	VsaThreeGpp2MulticastIpAddress(Ipv4Addr), // from 3GPP2 with plain value, tag=109, type=ipaddr
	VsaThreeGpp2Port(u32), // from 3GPP2 with plain value, tag=110, type=integer
	VsaThreeGpp2TkInfo(&'a [u8]), // from 3GPP2 with plain value, tag=112, type=tlv
	VsaThreeGpp2BakId(&'a [u8]), // from 3GPP2 with plain value, tag=113, type=tlv
	VsaThreeGpp2ReversePdchRc(u32), // from 3GPP2 with plain value, tag=114, type=integer
	VsaThreeGpp2Meid(&'a [u8]), // from 3GPP2 with plain value, tag=116, type=string
	VsaThreeGpp2DnsServerIpAddress(&'a [u8]), // from 3GPP2 with plain value, tag=117, type=tlv
	VsaThreeGpp2Mip6HomeAgentAddressFromBu(Ipv6Addr), // from 3GPP2 with plain value, tag=118, type=ipv6addr
	VsaThreeGpp2Mip6CareOfAddress(Ipv6Addr), // from 3GPP2 with plain value, tag=119, type=ipv6addr
	VsaThreeGpp2HomeAgentNotAuthorized(u32), // from 3GPP2 with plain value, tag=120, type=integer
	VsaThreeGpp2Mip6SessionKey(&'a [u8]), // from 3GPP2 with plain value, tag=121, type=octets
	VsaThreeGpp2HotLineAccountingInformation(&'a [u8]), // from 3GPP2 with plain value, tag=122, type=string
	VsaThreeGpp2Mip6MesgId(&'a [u8]), // from 3GPP2 with plain value, tag=123, type=octets
	VsaThreeGpp2FilterRule(&'a [u8]), // from 3GPP2 with plain value, tag=124, type=string
	VsaThreeGpp2HttpRedirectionRule(&'a [u8]), // from 3GPP2 with plain value, tag=125, type=string
	VsaThreeGpp2IpRedirectionRule(&'a [u8]), // from 3GPP2 with plain value, tag=126, type=string
	VsaThreeGpp2HotLineCapability(u32), // from 3GPP2 with plain value, tag=127, type=integer
	VsaThreeGpp2Mip6HomeLinkPrefixAttrA(&'a [u8]), // from 3GPP2 with plain value, tag=128, type=octets
	VsaThreeGpp2MaxAuthorizedAggrBandwidth(u32), // from 3GPP2 with plain value, tag=130, type=integer
	VsaThreeGpp2AuthorizedFlowProfileIds(&'a [u8]), // from 3GPP2 with plain value, tag=131, type=tlv
	VsaThreeGpp2GrantedQosParameters(&'a [u8]), // from 3GPP2 with plain value, tag=132, type=tlv
	VsaThreeGpp2MaximumPerFlowPriority(u32), // from 3GPP2 with plain value, tag=133, type=integer
	VsaThreeGpp2Mip6Authenticator(&'a [u8]), // from 3GPP2 with plain value, tag=134, type=octets
	VsaThreeGpp2Mip6MacMobilityData(&'a [u8]), // from 3GPP2 with plain value, tag=138, type=octets
	VsaThreeGpp2InterUserPriority(u32), // from 3GPP2 with plain value, tag=139, type=integer
	VsaThreeGpp2Mip6HomeAgentAddressAttrB(Ipv6Addr), // from 3GPP2 with plain value, tag=140, type=ipv6addr
	VsaThreeGpp2Mip6HoaReceivedFromBu(Ipv6Addr), // from 3GPP2 with plain value, tag=141, type=ipv6addr
	VsaThreeGpp2CarrierId(&'a [u8]), // from 3GPP2 with plain value, tag=142, type=octets
	VsaThreeGpp2GmtTimeZoneOffset(u32), // from 3GPP2 with plain value, tag=143, type=integer
	VsaThreeGpp2FlowIdParameter(&'a [u8]), // from 3GPP2 with plain value, tag=144, type=octets
	VsaThreeGpp2FlowStatus(u32), // from 3GPP2 with plain value, tag=145, type=integer
	VsaThreeGpp2FilteredOctetCountTerminating(u32), // from 3GPP2 with plain value, tag=146, type=integer
	VsaThreeGpp2FilteredOctetCountOriginating(u32), // from 3GPP2 with plain value, tag=147, type=integer
	VsaThreeGpp2RsvpInboundOctetCount(u32), // from 3GPP2 with plain value, tag=162, type=integer
	VsaThreeGpp2RsvpOutboundOctetCount(u32), // from 3GPP2 with plain value, tag=163, type=integer
	VsaThreeGpp2RsvpInboundPacketCount(u32), // from 3GPP2 with plain value, tag=164, type=integer
	VsaThreeGpp2RsvpOutboundPacketCount(u32), // from 3GPP2 with plain value, tag=165, type=integer
	VsaThreeGpp2HaRequest(u32), // from 3GPP2 with plain value, tag=168, type=integer
	VsaThreeGpp2HaAuthorised(u32), // from 3GPP2 with plain value, tag=169, type=integer
	VsaThreeGpp2IpVerAuthorised(u32), // from 3GPP2 with plain value, tag=172, type=integer
	VsaThreeGpp2Mipv4MesgId(&'a [u8]), // from 3GPP2 with plain value, tag=173, type=string
	VsaThreeGpp2Mip6HaLocalAssignmentCapblty(u32), // from 3GPP2 with plain value, tag=179, type=integer
	VsaThreeGpp2NetworkPmipNai(&'a [u8]), // from 3GPP2 with plain value, tag=192, type=string
	VsaThreeGpp2PmipBasedMobilityCapability(threegpp2::ThreeGpp2PmipBasedMobilityCapability), // from 3GPP2 with Enum value, tag=193, type=integer
	VsaThreeGpp2AccountingMode(u32), // from 3GPP2 with plain value, tag=198, type=integer
	VsaThreeGpp2HaaaMip6HaProtocolCapbltyInd(u32), // from 3GPP2 with plain value, tag=203, type=integer
	VsaThreeGpp2VaaaAssignedMip6Ha(Ipv6Addr), // from 3GPP2 with plain value, tag=205, type=ipv6addr
	VsaThreeGpp2VaaaAssignedMip6Hl(&'a [u8]), // from 3GPP2 with plain value, tag=206, type=octets
	VsaThreeGpp2VaaaMip6HaProtocolCapbltyInd(u32), // from 3GPP2 with plain value, tag=207, type=integer
	VsaThreeGpp2DnsServerIpv6Address(&'a [u8]), // from 3GPP2 with plain value, tag=214, type=tlv
	VsaAccReasonCode(acc::AccReasonCode), // from Acc with Enum value, tag=1, type=integer
	VsaAccCcpOption(acc::AccCcpOption), // from Acc with Enum value, tag=2, type=integer
	VsaAccInputErrors(u32), // from Acc with plain value, tag=3, type=integer
	VsaAccOutputErrors(u32), // from Acc with plain value, tag=4, type=integer
	VsaAccAccessPartition(&'a [u8]), // from Acc with plain value, tag=5, type=string
	VsaAccCustomerId(&'a [u8]), // from Acc with plain value, tag=6, type=string
	VsaAccIpGatewayPri(Ipv4Addr), // from Acc with plain value, tag=7, type=ipaddr
	VsaAccIpGatewaySec(Ipv4Addr), // from Acc with plain value, tag=8, type=ipaddr
	VsaAccRoutePolicy(acc::AccRoutePolicy), // from Acc with Enum value, tag=9, type=integer
	VsaAccMlMlxAdminState(acc::AccMlMlxAdminState), // from Acc with Enum value, tag=10, type=integer
	VsaAccMlCallThreshold(u32), // from Acc with plain value, tag=11, type=integer
	VsaAccMlClearThreshold(u32), // from Acc with plain value, tag=12, type=integer
	VsaAccMlDampingFactor(u32), // from Acc with plain value, tag=13, type=integer
	VsaAccTunnelSecret(&'a [u8]), // from Acc with plain value, tag=14, type=string
	VsaAccClearingCause(acc::AccClearingCause), // from Acc with Enum value, tag=15, type=integer
	VsaAccClearingLocation(acc::AccClearingLocation), // from Acc with Enum value, tag=16, type=integer
	VsaAccServiceProfile(&'a [u8]), // from Acc with plain value, tag=17, type=string
	VsaAccRequestType(acc::AccRequestType), // from Acc with Enum value, tag=18, type=integer
	VsaAccBridgingSupport(acc::AccBridgingSupport), // from Acc with Enum value, tag=19, type=integer
	VsaAccApsmOversubscribed(acc::AccApsmOversubscribed), // from Acc with Enum value, tag=20, type=integer
	VsaAccAcctOnOffReason(acc::AccAcctOnOffReason), // from Acc with Enum value, tag=21, type=integer
	VsaAccTunnelPort(u32), // from Acc with plain value, tag=22, type=integer
	VsaAccDnsServerPri(Ipv4Addr), // from Acc with plain value, tag=23, type=ipaddr
	VsaAccDnsServerSec(Ipv4Addr), // from Acc with plain value, tag=24, type=ipaddr
	VsaAccNbnsServerPri(Ipv4Addr), // from Acc with plain value, tag=25, type=ipaddr
	VsaAccNbnsServerSec(Ipv4Addr), // from Acc with plain value, tag=26, type=ipaddr
	VsaAccDialPortIndex(u32), // from Acc with plain value, tag=27, type=integer
	VsaAccIpCompression(acc::AccIpCompression), // from Acc with Enum value, tag=28, type=integer
	VsaAccIpxCompression(acc::AccIpxCompression), // from Acc with Enum value, tag=29, type=integer
	VsaAccConnectTxSpeed(u32), // from Acc with plain value, tag=30, type=integer
	VsaAccConnectRxSpeed(u32), // from Acc with plain value, tag=31, type=integer
	VsaAccModemModulationType(&'a [u8]), // from Acc with plain value, tag=32, type=string
	VsaAccModemErrorProtocol(&'a [u8]), // from Acc with plain value, tag=33, type=string
	VsaAccCallbackDelay(u32), // from Acc with plain value, tag=34, type=integer
	VsaAccCallbackNumValid(&'a [u8]), // from Acc with plain value, tag=35, type=string
	VsaAccCallbackMode(acc::AccCallbackMode), // from Acc with Enum value, tag=36, type=integer
	VsaAccCallbackCbcpType(acc::AccCallbackCbcpType), // from Acc with Enum value, tag=37, type=integer
	VsaAccDialoutAuthMode(acc::AccDialoutAuthMode), // from Acc with Enum value, tag=38, type=integer
	VsaAccDialoutAuthPassword(&'a [u8]), // from Acc with plain value, tag=39, type=string
	VsaAccDialoutAuthUsername(&'a [u8]), // from Acc with plain value, tag=40, type=string
	VsaAccAccessCommunity(acc::AccAccessCommunity), // from Acc with Enum value, tag=42, type=integer
	VsaAccVpsmRejectCause(acc::AccVpsmRejectCause), // from Acc with Enum value, tag=43, type=integer
	VsaAccAceToken(&'a [u8]), // from Acc with plain value, tag=44, type=string
	VsaAccAceTokenTtl(u32), // from Acc with plain value, tag=45, type=integer
	VsaAccIpPoolName(&'a [u8]), // from Acc with plain value, tag=46, type=string
	VsaAccIgmpAdminState(acc::AccIgmpAdminState), // from Acc with Enum value, tag=47, type=integer
	VsaAccIgmpVersion(acc::AccIgmpVersion), // from Acc with Enum value, tag=48, type=integer
	VsaAccMnHaSecret(&'a [u8]), // from Acc with plain value, tag=73, type=string
	VsaAccLocationId(&'a [u8]), // from Acc with plain value, tag=98, type=string
	VsaAccCallingStationCategory(u32), // from Acc with plain value, tag=99, type=integer
	VsaAcmeFlowidFs1F(&'a [u8]), // from Acme with plain value, tag=1, type=string
	VsaAcmeFlowtypeFs1F(&'a [u8]), // from Acme with plain value, tag=2, type=string
	VsaAcmeSessionIngressCallid(&'a [u8]), // from Acme with plain value, tag=3, type=string
	VsaAcmeSessionEgressCallid(&'a [u8]), // from Acme with plain value, tag=4, type=string
	VsaAcmeFlowInRealmFs1F(&'a [u8]), // from Acme with plain value, tag=10, type=string
	VsaAcmeFlowInSrcAddrFs1F(Ipv4Addr), // from Acme with plain value, tag=11, type=ipaddr
	VsaAcmeFlowInSrcPortFs1F(u32), // from Acme with plain value, tag=12, type=integer
	VsaAcmeFlowInDstAddrFs1F(Ipv4Addr), // from Acme with plain value, tag=13, type=ipaddr
	VsaAcmeFlowInDstPortFs1F(u32), // from Acme with plain value, tag=14, type=integer
	VsaAcmeFlowOutRealmFs1F(&'a [u8]), // from Acme with plain value, tag=20, type=string
	VsaAcmeFlowOutSrcAddrFs1F(Ipv4Addr), // from Acme with plain value, tag=21, type=ipaddr
	VsaAcmeFlowOutSrcPortFs1F(u32), // from Acme with plain value, tag=22, type=integer
	VsaAcmeFlowOutDstAddrFs1F(Ipv4Addr), // from Acme with plain value, tag=23, type=ipaddr
	VsaAcmeFlowOutDstPortFs1F(u32), // from Acme with plain value, tag=24, type=integer
	VsaAcmeCallingOctetsFs1(u32), // from Acme with plain value, tag=28, type=integer
	VsaAcmeCallingPacketsFs1(u32), // from Acme with plain value, tag=29, type=integer
	VsaAcmeCallingRtcpPacketsLostFs1(u32), // from Acme with plain value, tag=32, type=integer
	VsaAcmeCallingRtcpAvgJitterFs1(u32), // from Acme with plain value, tag=33, type=integer
	VsaAcmeCallingRtcpAvgLatencyFs1(u32), // from Acme with plain value, tag=34, type=integer
	VsaAcmeCallingRtcpMaxjitterFs1(u32), // from Acme with plain value, tag=35, type=integer
	VsaAcmeCallingRtcpMaxlatencyFs1(u32), // from Acme with plain value, tag=36, type=integer
	VsaAcmeCallingRtpPacketsLostFs1(u32), // from Acme with plain value, tag=37, type=integer
	VsaAcmeCallingRtpAvgJitterFs1(u32), // from Acme with plain value, tag=38, type=integer
	VsaAcmeCallingRtpMaxjitterFs1(u32), // from Acme with plain value, tag=39, type=integer
	VsaAcmeSessionGenericId(&'a [u8]), // from Acme with plain value, tag=40, type=string
	VsaAcmeSessionIngressRealm(&'a [u8]), // from Acme with plain value, tag=41, type=string
	VsaAcmeSessionEgressRealm(&'a [u8]), // from Acme with plain value, tag=42, type=string
	VsaAcmeSessionProtocolType(&'a [u8]), // from Acme with plain value, tag=43, type=string
	VsaAcmeCalledOctetsFs1(u32), // from Acme with plain value, tag=44, type=integer
	VsaAcmeCalledPacketsFs1(u32), // from Acme with plain value, tag=45, type=integer
	VsaAcmeCalledRtcpPacketsLostFs1(u32), // from Acme with plain value, tag=46, type=integer
	VsaAcmeCalledRtcpAvgJitterFs1(u32), // from Acme with plain value, tag=47, type=integer
	VsaAcmeCalledRtcpAvgLatencyFs1(u32), // from Acme with plain value, tag=48, type=integer
	VsaAcmeCalledRtcpMaxjitterFs1(u32), // from Acme with plain value, tag=49, type=integer
	VsaAcmeCalledRtcpMaxlatencyFs1(u32), // from Acme with plain value, tag=50, type=integer
	VsaAcmeCalledRtpPacketsLostFs1(u32), // from Acme with plain value, tag=51, type=integer
	VsaAcmeCalledRtpAvgJitterFs1(u32), // from Acme with plain value, tag=52, type=integer
	VsaAcmeCalledRtpMaxjitterFs1(u32), // from Acme with plain value, tag=53, type=integer
	VsaAcmeSessionChargingVector(&'a [u8]), // from Acme with plain value, tag=54, type=string
	VsaAcmeSessionChargingFunctionAddress(&'a [u8]), // from Acme with plain value, tag=55, type=string
	VsaAcmeFirmwareVersion(&'a [u8]), // from Acme with plain value, tag=56, type=string
	VsaAcmeLocalTimeZone(&'a [u8]), // from Acme with plain value, tag=57, type=string
	VsaAcmePostDialDelay(u32), // from Acme with plain value, tag=58, type=integer
	VsaAcmeCdrSequenceNumber(u32), // from Acme with plain value, tag=59, type=integer
	VsaAcmeSessionDisposition(u32), // from Acme with plain value, tag=60, type=integer
	VsaAcmeDisconnectInitiator(u32), // from Acme with plain value, tag=61, type=integer
	VsaAcmeDisconnectCause(u32), // from Acme with plain value, tag=62, type=integer
	VsaAcmeIntermediateTime(&'a [u8]), // from Acme with plain value, tag=63, type=string
	VsaAcmePrimaryRoutingNumber(&'a [u8]), // from Acme with plain value, tag=64, type=string
	VsaAcmeOriginatingTrunkGroup(&'a [u8]), // from Acme with plain value, tag=65, type=string
	VsaAcmeTerminatingTrunkGroup(&'a [u8]), // from Acme with plain value, tag=66, type=string
	VsaAcmeOriginatingTrunkContext(&'a [u8]), // from Acme with plain value, tag=67, type=string
	VsaAcmeTerminatingTrunkContext(&'a [u8]), // from Acme with plain value, tag=68, type=string
	VsaAcmePAssertedId(&'a [u8]), // from Acme with plain value, tag=69, type=string
	VsaAcmeSipDiversion(&'a [u8]), // from Acme with plain value, tag=70, type=string
	VsaAcmeSipStatus(u32), // from Acme with plain value, tag=71, type=integer
	VsaAcmeIngressLocalAddr(&'a [u8]), // from Acme with plain value, tag=74, type=string
	VsaAcmeIngressRemoteAddr(&'a [u8]), // from Acme with plain value, tag=75, type=string
	VsaAcmeEgressLocalAddr(&'a [u8]), // from Acme with plain value, tag=76, type=string
	VsaAcmeEgressRemoteAddr(&'a [u8]), // from Acme with plain value, tag=77, type=string
	VsaAcmeFlowidFs1R(&'a [u8]), // from Acme with plain value, tag=78, type=string
	VsaAcmeFlowtypeFs1R(&'a [u8]), // from Acme with plain value, tag=79, type=string
	VsaAcmeFlowInRealmFs1R(&'a [u8]), // from Acme with plain value, tag=80, type=string
	VsaAcmeFlowInSrcAddrFs1R(Ipv4Addr), // from Acme with plain value, tag=81, type=ipaddr
	VsaAcmeFlowInSrcPortFs1R(u32), // from Acme with plain value, tag=82, type=integer
	VsaAcmeFlowInDstAddrFs1R(Ipv4Addr), // from Acme with plain value, tag=83, type=ipaddr
	VsaAcmeFlowInDstPortFs1R(u32), // from Acme with plain value, tag=84, type=integer
	VsaAcmeFlowOutRealmFs1R(&'a [u8]), // from Acme with plain value, tag=85, type=string
	VsaAcmeFlowOutSrcAddrFs1R(Ipv4Addr), // from Acme with plain value, tag=86, type=ipaddr
	VsaAcmeFlowOutSrcPortFs1R(u32), // from Acme with plain value, tag=87, type=integer
	VsaAcmeFlowOutDstAddrFs1R(Ipv4Addr), // from Acme with plain value, tag=88, type=ipaddr
	VsaAcmeFlowOutDstPortFs1R(u32), // from Acme with plain value, tag=89, type=integer
	VsaAcmeFlowidFs2F(&'a [u8]), // from Acme with plain value, tag=90, type=string
	VsaAcmeFlowtypeFs2F(&'a [u8]), // from Acme with plain value, tag=91, type=string
	VsaAcmeFlowInRealmFs2F(&'a [u8]), // from Acme with plain value, tag=92, type=string
	VsaAcmeFlowInSrcAddrFs2F(Ipv4Addr), // from Acme with plain value, tag=93, type=ipaddr
	VsaAcmeFlowInSrcPortFs2F(u32), // from Acme with plain value, tag=94, type=integer
	VsaAcmeFlowInDstAddrFs2F(Ipv4Addr), // from Acme with plain value, tag=95, type=ipaddr
	VsaAcmeFlowInDstPortFs2F(u32), // from Acme with plain value, tag=96, type=integer
	VsaAcmeFlowOutRealmFs2F(&'a [u8]), // from Acme with plain value, tag=97, type=string
	VsaAcmeFlowOutSrcAddrFs2F(Ipv4Addr), // from Acme with plain value, tag=98, type=ipaddr
	VsaAcmeFlowOutSrcPortFs2F(u32), // from Acme with plain value, tag=99, type=integer
	VsaAcmeFlowOutDstAddrFs2F(Ipv4Addr), // from Acme with plain value, tag=100, type=ipaddr
	VsaAcmeFlowOutDstPortFs2F(u32), // from Acme with plain value, tag=101, type=integer
	VsaAcmeCallingOctetsFs2(u32), // from Acme with plain value, tag=102, type=integer
	VsaAcmeCallingPacketsFs2(u32), // from Acme with plain value, tag=103, type=integer
	VsaAcmeCallingRtcpPacketsLostFs2(u32), // from Acme with plain value, tag=104, type=integer
	VsaAcmeCallingRtcpAvgJitterFs2(u32), // from Acme with plain value, tag=105, type=integer
	VsaAcmeCallingRtcpAvgLatencyFs2(u32), // from Acme with plain value, tag=106, type=integer
	VsaAcmeCallingRtcpMaxjitterFs2(u32), // from Acme with plain value, tag=107, type=integer
	VsaAcmeCallingRtcpMaxlatencyFs2(u32), // from Acme with plain value, tag=108, type=integer
	VsaAcmeCallingRtpPacketsLostFs2(u32), // from Acme with plain value, tag=109, type=integer
	VsaAcmeCallingRtpAvgJitterFs2(u32), // from Acme with plain value, tag=110, type=integer
	VsaAcmeCallingRtpMaxjitterFs2(u32), // from Acme with plain value, tag=111, type=integer
	VsaAcmeFlowidFs2R(&'a [u8]), // from Acme with plain value, tag=112, type=string
	VsaAcmeFlowtypeFs2R(&'a [u8]), // from Acme with plain value, tag=113, type=string
	VsaAcmeFlowInRealmFs2R(&'a [u8]), // from Acme with plain value, tag=114, type=string
	VsaAcmeFlowInSrcAddrFs2R(Ipv4Addr), // from Acme with plain value, tag=115, type=ipaddr
	VsaAcmeFlowInSrcPortFs2R(u32), // from Acme with plain value, tag=116, type=integer
	VsaAcmeFlowInDstAddrFs2R(Ipv4Addr), // from Acme with plain value, tag=117, type=ipaddr
	VsaAcmeFlowInDstPortFs2R(u32), // from Acme with plain value, tag=118, type=integer
	VsaAcmeFlowOutRealmFs2R(&'a [u8]), // from Acme with plain value, tag=119, type=string
	VsaAcmeFlowOutSrcAddrFs2R(Ipv4Addr), // from Acme with plain value, tag=120, type=ipaddr
	VsaAcmeFlowOutSrcPortFs2R(u32), // from Acme with plain value, tag=121, type=integer
	VsaAcmeFlowOutDstAddrFs2R(Ipv4Addr), // from Acme with plain value, tag=122, type=ipaddr
	VsaAcmeFlowOutDstPortFs2R(u32), // from Acme with plain value, tag=123, type=integer
	VsaAcmeCalledOctetsFs2(u32), // from Acme with plain value, tag=124, type=integer
	VsaAcmeCalledPacketsFs2(u32), // from Acme with plain value, tag=125, type=integer
	VsaAcmeCalledRtcpPacketsLostFs2(u32), // from Acme with plain value, tag=126, type=integer
	VsaAcmeCalledRtcpAvgJitterFs2(u32), // from Acme with plain value, tag=127, type=integer
	VsaAcmeCalledRtcpAvgLatencyFs2(u32), // from Acme with plain value, tag=128, type=integer
	VsaAcmeCalledRtcpMaxjitterFs2(u32), // from Acme with plain value, tag=129, type=integer
	VsaAcmeCalledRtcpMaxlatencyFs2(u32), // from Acme with plain value, tag=130, type=integer
	VsaAcmeCalledRtpPacketsLostFs2(u32), // from Acme with plain value, tag=131, type=integer
	VsaAcmeCalledRtpAvgJitterFs2(u32), // from Acme with plain value, tag=132, type=integer
	VsaAcmeCalledRtpMaxjitterFs2(u32), // from Acme with plain value, tag=133, type=integer
	VsaAcmeEgressFinalRoutingNumber(&'a [u8]), // from Acme with plain value, tag=134, type=string
	VsaAcmeSessionIngressRph(&'a [u8]), // from Acme with plain value, tag=135, type=string
	VsaAcmeSessionEgressRph(&'a [u8]), // from Acme with plain value, tag=136, type=string
	VsaAcmeIngressNetworkInterfaceId(&'a [u8]), // from Acme with plain value, tag=137, type=string
	VsaAcmeIngressVlanTagValue(u32), // from Acme with plain value, tag=138, type=integer
	VsaAcmeEgressNetworkInterfaceId(&'a [u8]), // from Acme with plain value, tag=139, type=string
	VsaAcmeEgressVlanTagValue(u32), // from Acme with plain value, tag=140, type=integer
	VsaAcmeReferCallTransferId(&'a [u8]), // from Acme with plain value, tag=141, type=string
	VsaAcmeFlowmediatypeFs1F(&'a [u8]), // from Acme with plain value, tag=142, type=string
	VsaAcmeFlowmediatypeFs1R(&'a [u8]), // from Acme with plain value, tag=143, type=string
	VsaAcmeFlowmediatypeFs2F(&'a [u8]), // from Acme with plain value, tag=144, type=string
	VsaAcmeFlowmediatypeFs2R(&'a [u8]), // from Acme with plain value, tag=145, type=string
	VsaAcmeFlowPtimeFs1F(u32), // from Acme with plain value, tag=146, type=integer
	VsaAcmeFlowPtimeFs1R(u32), // from Acme with plain value, tag=147, type=integer
	VsaAcmeFlowPtimeFs2F(u32), // from Acme with plain value, tag=148, type=integer
	VsaAcmeFlowPtimeFs2R(u32), // from Acme with plain value, tag=149, type=integer
	VsaAcmeSessionMediaProcess(&'a [u8]), // from Acme with plain value, tag=150, type=string
	VsaAcmeCallingRFactor(u32), // from Acme with plain value, tag=151, type=integer
	VsaAcmeCallingMos(u32), // from Acme with plain value, tag=152, type=integer
	VsaAcmeCalledRFactor(u32), // from Acme with plain value, tag=153, type=integer
	VsaAcmeCalledMos(u32), // from Acme with plain value, tag=154, type=integer
	VsaAcmeFlowInSrcIpv6AddrFs1F(Ipv6Addr), // from Acme with plain value, tag=155, type=ipv6addr
	VsaAcmeFlowInDstIpv6AddrFs1F(Ipv6Addr), // from Acme with plain value, tag=156, type=ipv6addr
	VsaAcmeFlowOutSrcIpv6AddrFs1F(Ipv6Addr), // from Acme with plain value, tag=157, type=ipv6addr
	VsaAcmeFlowOutDstIpv6AddrFs1F(Ipv6Addr), // from Acme with plain value, tag=158, type=ipv6addr
	VsaAcmeFlowInSrcIpv6AddrFs1R(Ipv6Addr), // from Acme with plain value, tag=159, type=ipv6addr
	VsaAcmeFlowInDstIpv6AddrFs1R(Ipv6Addr), // from Acme with plain value, tag=160, type=ipv6addr
	VsaAcmeFlowOutSrcIpv6AddrFs1R(Ipv6Addr), // from Acme with plain value, tag=161, type=ipv6addr
	VsaAcmeFlowOutDstIpv6AddrFs1R(Ipv6Addr), // from Acme with plain value, tag=162, type=ipv6addr
	VsaAcmeFlowInSrcIpv6AddrFs2F(Ipv6Addr), // from Acme with plain value, tag=163, type=ipv6addr
	VsaAcmeFlowInDstIpv6AddrFs2F(Ipv6Addr), // from Acme with plain value, tag=164, type=ipv6addr
	VsaAcmeFlowOutSrcIpv6AddrFs2F(Ipv6Addr), // from Acme with plain value, tag=165, type=ipv6addr
	VsaAcmeFlowOutDstIpv6AddrFs2F(Ipv6Addr), // from Acme with plain value, tag=166, type=ipv6addr
	VsaAcmeFlowInSrcIpv6AddrFs2R(Ipv6Addr), // from Acme with plain value, tag=167, type=ipv6addr
	VsaAcmeFlowInDstIpv6AddrFs2R(Ipv6Addr), // from Acme with plain value, tag=168, type=ipv6addr
	VsaAcmeFlowOutSrcIpv6AddrFs2R(Ipv6Addr), // from Acme with plain value, tag=169, type=ipv6addr
	VsaAcmeFlowOutDstIpv6AddrFs2R(Ipv6Addr), // from Acme with plain value, tag=170, type=ipv6addr
	VsaAcmeSessionForkedCallId(&'a [u8]), // from Acme with plain value, tag=171, type=string
	VsaAcmeCustomVsa200(&'a [u8]), // from Acme with plain value, tag=200, type=string
	VsaAcmeCustomVsa201(&'a [u8]), // from Acme with plain value, tag=201, type=string
	VsaAcmeCustomVsa202(&'a [u8]), // from Acme with plain value, tag=202, type=string
	VsaAcmeCustomVsa203(&'a [u8]), // from Acme with plain value, tag=203, type=string
	VsaAcmeCustomVsa204(&'a [u8]), // from Acme with plain value, tag=204, type=string
	VsaAcmeCustomVsa205(&'a [u8]), // from Acme with plain value, tag=205, type=string
	VsaAcmeCustomVsa206(&'a [u8]), // from Acme with plain value, tag=206, type=string
	VsaAcmeCustomVsa207(&'a [u8]), // from Acme with plain value, tag=207, type=string
	VsaAcmeCustomVsa208(&'a [u8]), // from Acme with plain value, tag=208, type=string
	VsaAcmeCustomVsa209(&'a [u8]), // from Acme with plain value, tag=209, type=string
	VsaAcmeCustomVsa210(&'a [u8]), // from Acme with plain value, tag=210, type=string
	VsaAcmeCustomVsa211(&'a [u8]), // from Acme with plain value, tag=211, type=string
	VsaAcmeCustomVsa212(&'a [u8]), // from Acme with plain value, tag=212, type=string
	VsaAcmeCustomVsa213(&'a [u8]), // from Acme with plain value, tag=213, type=string
	VsaAcmeCustomVsa214(&'a [u8]), // from Acme with plain value, tag=214, type=string
	VsaAcmeCustomVsa215(&'a [u8]), // from Acme with plain value, tag=215, type=string
	VsaAcmeCustomVsa216(&'a [u8]), // from Acme with plain value, tag=216, type=string
	VsaAcmeCustomVsa217(&'a [u8]), // from Acme with plain value, tag=217, type=string
	VsaAcmeCustomVsa218(&'a [u8]), // from Acme with plain value, tag=218, type=string
	VsaAcmeCustomVsa219(&'a [u8]), // from Acme with plain value, tag=219, type=string
	VsaAcmeCustomVsa220(&'a [u8]), // from Acme with plain value, tag=220, type=string
	VsaAcmeCustomVsa221(&'a [u8]), // from Acme with plain value, tag=221, type=string
	VsaAcmeCustomVsa222(&'a [u8]), // from Acme with plain value, tag=222, type=string
	VsaAcmeCustomVsa223(&'a [u8]), // from Acme with plain value, tag=223, type=string
	VsaAcmeCustomVsa224(&'a [u8]), // from Acme with plain value, tag=224, type=string
	VsaAcmeCustomVsa225(&'a [u8]), // from Acme with plain value, tag=225, type=string
	VsaAcmeCustomVsa226(&'a [u8]), // from Acme with plain value, tag=226, type=string
	VsaAcmeCustomVsa227(&'a [u8]), // from Acme with plain value, tag=227, type=string
	VsaAcmeCustomVsa228(&'a [u8]), // from Acme with plain value, tag=228, type=string
	VsaAcmeCustomVsa229(&'a [u8]), // from Acme with plain value, tag=229, type=string
	VsaAcmeCustomVsa230(&'a [u8]), // from Acme with plain value, tag=230, type=string
	VsaAcmeUserClass(&'a [u8]), // from Acme with plain value, tag=254, type=string
	VsaActelisPrivilege(&'a [u8]), // from Actelis with plain value, tag=1, type=string
	VsaAirespaceWlanId(u32), // from Airespace with plain value, tag=1, type=integer
	VsaAirespaceQosLevel(airespace::AirespaceQosLevel), // from Airespace with Enum value, tag=2, type=integer
	VsaAirespaceDscp(u32), // from Airespace with plain value, tag=3, type=integer
	VsaAirespace8021PTag(u32), // from Airespace with plain value, tag=4, type=integer
	VsaAirespaceInterfaceName(&'a [u8]), // from Airespace with plain value, tag=5, type=string
	VsaAirespaceAclName(&'a [u8]), // from Airespace with plain value, tag=6, type=string
	VsaAirespaceDataBandwidthAverageContract(u32), // from Airespace with plain value, tag=7, type=integer
	VsaAirespaceRealTimeBandwidthAverageContract(u32), // from Airespace with plain value, tag=8, type=integer
	VsaAirespaceDataBandwidthBurstContract(u32), // from Airespace with plain value, tag=9, type=integer
	VsaAirespaceRealTimeBandwidthBurstContract(u32), // from Airespace with plain value, tag=10, type=integer
	VsaAirespaceGuestRoleName(&'a [u8]), // from Airespace with plain value, tag=11, type=string
	VsaAirespaceDataBandwidthAverageContractUpstream(u32), // from Airespace with plain value, tag=13, type=integer
	VsaAirespaceRealTimeBandwidthAverageContractUpstream(u32), // from Airespace with plain value, tag=14, type=integer
	VsaAirespaceDataBandwidthBurstContractUpstream(u32), // from Airespace with plain value, tag=15, type=integer
	VsaAirespaceRealTimeBandwidthBurstContractUpstream(u32), // from Airespace with plain value, tag=16, type=integer
	VsaAatClientPrimaryDns(Ipv4Addr), // from Alcatel with plain value, tag=5, type=ipaddr
	VsaAatClientPrimaryWinsNbns(Ipv4Addr), // from Alcatel with plain value, tag=6, type=ipaddr
	VsaAatClientSecondaryWinsNbns(Ipv4Addr), // from Alcatel with plain value, tag=7, type=ipaddr
	VsaAatClientSecondaryDns(Ipv4Addr), // from Alcatel with plain value, tag=8, type=ipaddr
	VsaAatPppAddress(Ipv4Addr), // from Alcatel with plain value, tag=9, type=ipaddr
	VsaAatPppNetmask(Ipv4Addr), // from Alcatel with plain value, tag=10, type=ipaddr
	VsaAatPrimaryHomeAgent(&'a [u8]), // from Alcatel with plain value, tag=12, type=string
	VsaAatSecondaryHomeAgent(&'a [u8]), // from Alcatel with plain value, tag=13, type=string
	VsaAatHomeAgentPassword(&'a [u8]), // from Alcatel with plain value, tag=14, type=string
	VsaAatHomeNetworkName(&'a [u8]), // from Alcatel with plain value, tag=15, type=string
	VsaAatHomeAgentUdpPort(u32), // from Alcatel with plain value, tag=16, type=integer
	VsaAatIpDirect(Ipv4Addr), // from Alcatel with plain value, tag=17, type=ipaddr
	VsaAatFrDirect(alcatel::AatFrDirect), // from Alcatel with Enum value, tag=18, type=integer
	VsaAatFrDirectProfile(&'a [u8]), // from Alcatel with plain value, tag=19, type=string
	VsaAatFrDirectDlci(u32), // from Alcatel with plain value, tag=20, type=integer
	VsaAatAtmDirect(&'a [u8]), // from Alcatel with plain value, tag=21, type=string
	VsaAatIpTos(alcatel::AatIpTos), // from Alcatel with Enum value, tag=22, type=integer
	VsaAatIpTosPrecedence(alcatel::AatIpTosPrecedence), // from Alcatel with Enum value, tag=23, type=integer
	VsaAatIpTosApplyTo(alcatel::AatIpTosApplyTo), // from Alcatel with Enum value, tag=24, type=integer
	VsaAatMcastClient(alcatel::AatMcastClient), // from Alcatel with Enum value, tag=27, type=integer
	VsaAatModemPortNo(u32), // from Alcatel with plain value, tag=28, type=integer
	VsaAatModemSlotNo(u32), // from Alcatel with plain value, tag=29, type=integer
	VsaAatModemShelfNo(u32), // from Alcatel with plain value, tag=30, type=integer
	VsaAatFilter(&'a [u8]), // from Alcatel with plain value, tag=60, type=string
	VsaAatVrouterName(&'a [u8]), // from Alcatel with plain value, tag=61, type=string
	VsaAatRequireAuth(alcatel::AatRequireAuth), // from Alcatel with Enum value, tag=62, type=integer
	VsaAatIpPoolDefinition(&'a [u8]), // from Alcatel with plain value, tag=63, type=string
	VsaAatAssignIpPool(u32), // from Alcatel with plain value, tag=64, type=integer
	VsaAatDataFilter(&'a [u8]), // from Alcatel with plain value, tag=65, type=string
	VsaAatSourceIpCheck(alcatel::AatSourceIpCheck), // from Alcatel with Enum value, tag=66, type=integer
	VsaAatModemAnswerString(&'a [u8]), // from Alcatel with plain value, tag=67, type=string
	VsaAatAuthType(alcatel::AatAuthType), // from Alcatel with Enum value, tag=68, type=integer
	VsaAatQos(u32), // from Alcatel with plain value, tag=70, type=integer
	VsaAatQoa(u32), // from Alcatel with plain value, tag=71, type=integer
	VsaAatClientAssignDns(alcatel::AatClientAssignDns), // from Alcatel with Enum value, tag=72, type=integer
	VsaAatAtmVpi(u32), // from Alcatel with plain value, tag=128, type=integer
	VsaAatAtmVci(u32), // from Alcatel with plain value, tag=129, type=integer
	VsaAatInputOctetsDiff(u32), // from Alcatel with plain value, tag=130, type=integer
	VsaAatOutputOctetsDiff(u32), // from Alcatel with plain value, tag=131, type=integer
	VsaAatUserMacAddress(&'a [u8]), // from Alcatel with plain value, tag=132, type=string
	VsaAatAtmTrafficProfile(&'a [u8]), // from Alcatel with plain value, tag=133, type=string
	VsaTimetraAccess(alcatel_lucent_service_router::TimetraAccess), // from Alcatel-Lucent-Service-Router with Enum value, tag=1, type=integer
	VsaTimetraHomeDirectory(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=2, type=string
	VsaTimetraRestrictToHome(alcatel_lucent_service_router::TimetraRestrictToHome), // from Alcatel-Lucent-Service-Router with Enum value, tag=3, type=integer
	VsaTimetraProfile(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=4, type=string
	VsaTimetraDefaultAction(alcatel_lucent_service_router::TimetraDefaultAction), // from Alcatel-Lucent-Service-Router with Enum value, tag=5, type=integer
	VsaTimetraCmd(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=6, type=string
	VsaTimetraAction(alcatel_lucent_service_router::TimetraAction), // from Alcatel-Lucent-Service-Router with Enum value, tag=7, type=integer
	VsaTimetraExecFile(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=8, type=string
	VsaAlcPrimaryDns(Ipv4Addr), // from Alcatel-Lucent-Service-Router with plain value, tag=9, type=ipaddr
	VsaAlcSecondaryDns(Ipv4Addr), // from Alcatel-Lucent-Service-Router with plain value, tag=10, type=ipaddr
	VsaAlcSubscIdStr(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=11, type=string
	VsaAlcSubscProfStr(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=12, type=string
	VsaAlcSlaProfStr(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=13, type=string
	VsaAlcForceRenew(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=14, type=string
	VsaAlcCreateHost(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=15, type=string
	VsaAlcAncpStr(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=16, type=string
	VsaAlcRetailServId(u32), // from Alcatel-Lucent-Service-Router with plain value, tag=17, type=integer
	VsaAlcDefaultRouter(Ipv4Addr), // from Alcatel-Lucent-Service-Router with plain value, tag=18, type=ipaddr
	VsaAlcClientHardwareAddr(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=27, type=string
	VsaAlcAcctIInprofOctets64(u32), // from Alcatel-Lucent-Service-Router with plain value, tag=19, type=integer
	VsaAlcAcctIOutprofOctets64(u32), // from Alcatel-Lucent-Service-Router with plain value, tag=20, type=integer
	VsaAlcAcctOInprofOctets64(u32), // from Alcatel-Lucent-Service-Router with plain value, tag=21, type=integer
	VsaAlcAcctOOutprofOctets64(u32), // from Alcatel-Lucent-Service-Router with plain value, tag=22, type=integer
	VsaAlcAcctIInprofPkts64(u32), // from Alcatel-Lucent-Service-Router with plain value, tag=23, type=integer
	VsaAlcAcctIOutprofPkts64(u32), // from Alcatel-Lucent-Service-Router with plain value, tag=24, type=integer
	VsaAlcAcctOInprofPkts64(u32), // from Alcatel-Lucent-Service-Router with plain value, tag=25, type=integer
	VsaAlcAcctOOutprofPkts64(u32), // from Alcatel-Lucent-Service-Router with plain value, tag=26, type=integer
	VsaAlcWlanPortalRedirect(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=172, type=string
	VsaAlcWlanPortalUrl(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=173, type=string
	VsaAlcLeaseTime(u32), // from Alcatel-Lucent-Service-Router with plain value, tag=174, type=integer
	VsaAlcPortalUrl(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=177, type=string
	VsaAlcSlaacIpv6Pool(&'a [u8]), // from Alcatel-Lucent-Service-Router with plain value, tag=181, type=string
	VsaAluAaaAccessRule(&'a [u8]), // from ALU-AAA with plain value, tag=1, type=string
	VsaAluAaaAvPair(&'a [u8]), // from ALU-AAA with plain value, tag=2, type=string
	VsaAluAaaGsmTripletsNeeded(u32), // from ALU-AAA with plain value, tag=3, type=integer
	VsaAluAaaGsmTriplet(&'a [u8]), // from ALU-AAA with plain value, tag=4, type=octets
	VsaAluAaaAkaQuintetsNeeded(u32), // from ALU-AAA with plain value, tag=5, type=integer
	VsaAluAaaAkaQuintet(&'a [u8]), // from ALU-AAA with plain value, tag=6, type=octets
	VsaAluAaaAkaRand(&'a [u8]), // from ALU-AAA with plain value, tag=7, type=octets
	VsaAluAaaAkaAuts(&'a [u8]), // from ALU-AAA with plain value, tag=8, type=octets
	VsaAluAaaServiceProfile(&'a [u8]), // from ALU-AAA with plain value, tag=9, type=string
	VsaAluAaaLawfulInterceptStatus(u8), // from ALU-AAA with plain value, tag=10, type=byte
	VsaAluAaaDfCcAddress(Ipv4Addr), // from ALU-AAA with plain value, tag=11, type=ipaddr
	VsaAluAaaDfCcPort(u16), // from ALU-AAA with plain value, tag=12, type=short
	VsaAluAaaClientProgram(&'a [u8]), // from ALU-AAA with plain value, tag=13, type=string
	VsaAluAaaClientErrorAction(alu_aaa::AluAaaClientErrorAction), // from ALU-AAA with Enum value, tag=14, type=integer
	VsaAluAaaClientOs(&'a [u8]), // from ALU-AAA with plain value, tag=15, type=string
	VsaAluAaaClientVersion(&'a [u8]), // from ALU-AAA with plain value, tag=16, type=string
	VsaAluAaaNonce(&'a [u8]), // from ALU-AAA with plain value, tag=17, type=octets
	VsaAluAaaFemtoPublicKeyHash(&'a [u8]), // from ALU-AAA with plain value, tag=18, type=octets
	VsaAluAaaFemtoAssociatedUserName(&'a [u8]), // from ALU-AAA with plain value, tag=19, type=string
	VsaAluAaaString0(&'a [u8]), // from ALU-AAA with plain value, tag=100, type=string
	VsaAluAaaString1(&'a [u8]), // from ALU-AAA with plain value, tag=101, type=string
	VsaAluAaaString2(&'a [u8]), // from ALU-AAA with plain value, tag=102, type=string
	VsaAluAaaString3(&'a [u8]), // from ALU-AAA with plain value, tag=103, type=string
	VsaAluAaaInteger0(u32), // from ALU-AAA with plain value, tag=104, type=integer
	VsaAluAaaInteger1(u32), // from ALU-AAA with plain value, tag=105, type=integer
	VsaAluAaaInteger2(u32), // from ALU-AAA with plain value, tag=106, type=integer
	VsaAluAaaInteger3(u32), // from ALU-AAA with plain value, tag=107, type=integer
	VsaAluAaaAddress0(&'a [u8]), // from ALU-AAA with plain value, tag=108, type=combo-ip
	VsaAluAaaAddress1(&'a [u8]), // from ALU-AAA with plain value, tag=109, type=combo-ip
	VsaAluAaaAddress2(&'a [u8]), // from ALU-AAA with plain value, tag=110, type=combo-ip
	VsaAluAaaAddress3(&'a [u8]), // from ALU-AAA with plain value, tag=111, type=combo-ip
	VsaAluAaaValue0(&'a [u8]), // from ALU-AAA with plain value, tag=112, type=octets
	VsaAluAaaValue1(&'a [u8]), // from ALU-AAA with plain value, tag=113, type=octets
	VsaAluAaaValue2(&'a [u8]), // from ALU-AAA with plain value, tag=114, type=octets
	VsaAluAaaValue3(&'a [u8]), // from ALU-AAA with plain value, tag=115, type=octets
	VsaAluAaaKey0(&'a [u8]), // from ALU-AAA with plain value, tag=116, type=octets
	VsaAluAaaKey1(&'a [u8]), // from ALU-AAA with plain value, tag=117, type=octets
	VsaAluAaaKey2(&'a [u8]), // from ALU-AAA with plain value, tag=118, type=octets
	VsaAluAaaKey3(&'a [u8]), // from ALU-AAA with plain value, tag=119, type=octets
	VsaAluAaaOpaque0(&'a [u8]), // from ALU-AAA with plain value, tag=120, type=octets
	VsaAluAaaOpaque1(&'a [u8]), // from ALU-AAA with plain value, tag=121, type=octets
	VsaAluAaaOpaque2(&'a [u8]), // from ALU-AAA with plain value, tag=122, type=octets
	VsaAluAaaOpaque3(&'a [u8]), // from ALU-AAA with plain value, tag=123, type=octets
	VsaAluAaaEval0(&'a [u8]), // from ALU-AAA with plain value, tag=124, type=string
	VsaAluAaaEval1(&'a [u8]), // from ALU-AAA with plain value, tag=125, type=string
	VsaAluAaaEval2(&'a [u8]), // from ALU-AAA with plain value, tag=126, type=string
	VsaAluAaaEval3(&'a [u8]), // from ALU-AAA with plain value, tag=127, type=string
	VsaAluAaaExec0(&'a [u8]), // from ALU-AAA with plain value, tag=128, type=string
	VsaAluAaaExec1(&'a [u8]), // from ALU-AAA with plain value, tag=129, type=string
	VsaAluAaaExec2(&'a [u8]), // from ALU-AAA with plain value, tag=130, type=string
	VsaAluAaaExec3(&'a [u8]), // from ALU-AAA with plain value, tag=131, type=string
	VsaAluAaaOriginalReceiptTime(&'a [u8]), // from ALU-AAA with plain value, tag=199, type=octets
	VsaAluAaaReplyMessage(&'a [u8]), // from ALU-AAA with plain value, tag=201, type=string
	VsaAluAaaCalledStationId(&'a [u8]), // from ALU-AAA with plain value, tag=202, type=string
	VsaAluAaaNasIpAddress(Ipv4Addr), // from ALU-AAA with plain value, tag=203, type=ipaddr
	VsaAluAaaNasPort(u32), // from ALU-AAA with plain value, tag=204, type=integer
	VsaAluAaaOldState(&'a [u8]), // from ALU-AAA with plain value, tag=205, type=string
	VsaAluAaaNewState(&'a [u8]), // from ALU-AAA with plain value, tag=206, type=string
	VsaAluAaaEvent(&'a [u8]), // from ALU-AAA with plain value, tag=207, type=string
	VsaAluAaaOldTimestamp(u32), // from ALU-AAA with plain value, tag=208, type=date
	VsaAluAaaNewTimestamp(u32), // from ALU-AAA with plain value, tag=209, type=date
	VsaAluAaaDeltaSession(alu_aaa::AluAaaDeltaSession), // from ALU-AAA with Enum value, tag=210, type=integer
	VsaAluAaaCivicLocation(&'a [u8]), // from ALU-AAA with plain value, tag=211, type=octets
	VsaAluAaaGeospatialLocation(&'a [u8]), // from ALU-AAA with plain value, tag=212, type=octets
	VsaAlteonGroupMapping(&'a [u8]), // from Alteon with plain value, tag=1, type=string
	VsaAlteonVpnId(u32), // from Alteon with plain value, tag=3, type=integer
	VsaAlteonClientIpAddress(Ipv4Addr), // from Alteon with plain value, tag=4, type=ipaddr
	VsaAlteonClientNetmask(Ipv4Addr), // from Alteon with plain value, tag=5, type=ipaddr
	VsaAlteonPrimaryNbnsServer(Ipv4Addr), // from Alteon with plain value, tag=6, type=ipaddr
	VsaAlteonSecondaryNbnsServer(Ipv4Addr), // from Alteon with plain value, tag=7, type=ipaddr
	VsaAlteonPrimaryDnsServer(Ipv4Addr), // from Alteon with plain value, tag=8, type=ipaddr
	VsaAlteonSecondaryDnsServer(Ipv4Addr), // from Alteon with plain value, tag=9, type=ipaddr
	VsaAlteonDomainName(&'a [u8]), // from Alteon with plain value, tag=10, type=string
	VsaAlteonServiceType(alteon::AlteonServiceType), // from Alteon with Enum value, tag=26, type=integer
	VsaAlvarionVsa1(&'a [u8]), // from Alvarion with plain value, tag=1, type=string
	VsaAlvarionVsa2(&'a [u8]), // from Alvarion with plain value, tag=2, type=string
	VsaAlvarionVsa3(&'a [u8]), // from Alvarion with plain value, tag=3, type=string
	VsaAlvarionVsa4(&'a [u8]), // from Alvarion with plain value, tag=4, type=string
	VsaAlvarionVsa5(&'a [u8]), // from Alvarion with plain value, tag=5, type=string
	VsaAlvarionVsa6(&'a [u8]), // from Alvarion with plain value, tag=6, type=string
	VsaAlvarionVsa7(&'a [u8]), // from Alvarion with plain value, tag=7, type=string
	VsaAlvarionVsa8(&'a [u8]), // from Alvarion with plain value, tag=8, type=string
	VsaAlvarionVsa9(&'a [u8]), // from Alvarion with plain value, tag=9, type=string
	VsaAlvarionVsa10(&'a [u8]), // from Alvarion with plain value, tag=10, type=string
	VsaAlvarionVsa11(&'a [u8]), // from Alvarion with plain value, tag=11, type=string
	VsaAlvarionVsa12(&'a [u8]), // from Alvarion with plain value, tag=12, type=string
	VsaAlvarionVsa13(&'a [u8]), // from Alvarion with plain value, tag=13, type=string
	VsaAlvarionVsa14(&'a [u8]), // from Alvarion with plain value, tag=14, type=string
	VsaAlvarionVsa15(&'a [u8]), // from Alvarion with plain value, tag=15, type=string
	VsaAlvarionVsa16(&'a [u8]), // from Alvarion with plain value, tag=16, type=string
	VsaAlvarionVsa17(&'a [u8]), // from Alvarion with plain value, tag=17, type=string
	VsaAlvarionVsa18(&'a [u8]), // from Alvarion with plain value, tag=18, type=string
	VsaAlvarionVsa19(&'a [u8]), // from Alvarion with plain value, tag=19, type=string
	VsaAlvarionVsa20(&'a [u8]), // from Alvarion with plain value, tag=20, type=string
	VsaAlvarionVsa21(&'a [u8]), // from Alvarion with plain value, tag=21, type=string
	VsaAlvarionVsa22(&'a [u8]), // from Alvarion with plain value, tag=22, type=string
	VsaAlvarionVsa23(&'a [u8]), // from Alvarion with plain value, tag=23, type=string
	VsaAlvarionVsa24(&'a [u8]), // from Alvarion with plain value, tag=24, type=string
	VsaAlvarionVsa25(&'a [u8]), // from Alvarion with plain value, tag=25, type=string
	VsaAlvarionVsa26(&'a [u8]), // from Alvarion with plain value, tag=26, type=string
	VsaAlvarionVsa27(&'a [u8]), // from Alvarion with plain value, tag=27, type=string
	VsaAlvarionVsa28(&'a [u8]), // from Alvarion with plain value, tag=28, type=string
	VsaAlvarionVsa29(&'a [u8]), // from Alvarion with plain value, tag=29, type=string
	VsaAlvarionVsa30(&'a [u8]), // from Alvarion with plain value, tag=30, type=string
	VsaAlvarionVsa31(&'a [u8]), // from Alvarion with plain value, tag=31, type=string
	VsaAlvarionVsa32(&'a [u8]), // from Alvarion with plain value, tag=32, type=string
	VsaAlvarionVsa33(&'a [u8]), // from Alvarion with plain value, tag=33, type=string
	VsaAlvarionVsa34(&'a [u8]), // from Alvarion with plain value, tag=34, type=string
	VsaAlvarionVsa35(&'a [u8]), // from Alvarion with plain value, tag=35, type=string
	VsaAlvarionVsa36(&'a [u8]), // from Alvarion with plain value, tag=36, type=string
	VsaAlvarionVsa37(&'a [u8]), // from Alvarion with plain value, tag=37, type=string
	VsaAlvarionVsa38(&'a [u8]), // from Alvarion with plain value, tag=38, type=string
	VsaAlvarionVsa39(&'a [u8]), // from Alvarion with plain value, tag=39, type=string
	VsaAlvarionVsa40(&'a [u8]), // from Alvarion with plain value, tag=40, type=string
	VsaAlvarionVsa41(&'a [u8]), // from Alvarion with plain value, tag=41, type=string
	VsaAlvarionVsa42(&'a [u8]), // from Alvarion with plain value, tag=42, type=string
	VsaAlvarionVsa43(&'a [u8]), // from Alvarion with plain value, tag=43, type=string
	VsaAlvarionVsa44(&'a [u8]), // from Alvarion with plain value, tag=44, type=string
	VsaAlvarionVsa45(&'a [u8]), // from Alvarion with plain value, tag=45, type=string
	VsaAlvarionVsa46(&'a [u8]), // from Alvarion with plain value, tag=46, type=string
	VsaAlvarionVsa47(&'a [u8]), // from Alvarion with plain value, tag=47, type=string
	VsaAlvarionVsa48(&'a [u8]), // from Alvarion with plain value, tag=48, type=string
	VsaAlvarionVsa49(&'a [u8]), // from Alvarion with plain value, tag=49, type=string
	VsaAlvarionVsa50(&'a [u8]), // from Alvarion with plain value, tag=50, type=string
	VsaAlvarionVsa51(&'a [u8]), // from Alvarion with plain value, tag=51, type=string
	VsaAlvarionVsa52(&'a [u8]), // from Alvarion with plain value, tag=52, type=string
	VsaAlvarionVsa53(&'a [u8]), // from Alvarion with plain value, tag=53, type=string
	VsaAlvarionVsa54(&'a [u8]), // from Alvarion with plain value, tag=54, type=string
	VsaAlvarionVsa55(&'a [u8]), // from Alvarion with plain value, tag=55, type=string
	VsaAlvarionVsa56(&'a [u8]), // from Alvarion with plain value, tag=56, type=string
	VsaAlvarionVsa57(&'a [u8]), // from Alvarion with plain value, tag=57, type=string
	VsaAlvarionVsa58(&'a [u8]), // from Alvarion with plain value, tag=58, type=string
	VsaAlvarionVsa59(&'a [u8]), // from Alvarion with plain value, tag=59, type=string
	VsaAlvarionVsa60(&'a [u8]), // from Alvarion with plain value, tag=60, type=string
	VsaAlvarionVsa61(&'a [u8]), // from Alvarion with plain value, tag=61, type=string
	VsaAlvarionVsa62(&'a [u8]), // from Alvarion with plain value, tag=62, type=string
	VsaAlvarionVsa63(&'a [u8]), // from Alvarion with plain value, tag=63, type=string
	VsaAlvarionVsa64(&'a [u8]), // from Alvarion with plain value, tag=64, type=string
	VsaAlvarionVsa65(&'a [u8]), // from Alvarion with plain value, tag=65, type=string
	VsaAlvarionVsa66(&'a [u8]), // from Alvarion with plain value, tag=66, type=string
	VsaAlvarionVsa67(&'a [u8]), // from Alvarion with plain value, tag=67, type=string
	VsaAlvarionVsa68(&'a [u8]), // from Alvarion with plain value, tag=68, type=string
	VsaAlvarionVsa69(&'a [u8]), // from Alvarion with plain value, tag=69, type=string
	VsaAlvarionVsa70(&'a [u8]), // from Alvarion with plain value, tag=70, type=string
	VsaAlvarionVsa71(&'a [u8]), // from Alvarion with plain value, tag=71, type=string
	VsaAlvarionVsa72(&'a [u8]), // from Alvarion with plain value, tag=72, type=string
	VsaAlvarionVsa73(&'a [u8]), // from Alvarion with plain value, tag=73, type=string
	VsaAlvarionVsa74(&'a [u8]), // from Alvarion with plain value, tag=74, type=string
	VsaAlvarionVsa75(&'a [u8]), // from Alvarion with plain value, tag=75, type=string
	VsaAlvarionVsa76(&'a [u8]), // from Alvarion with plain value, tag=76, type=string
	VsaAlvarionVsa77(&'a [u8]), // from Alvarion with plain value, tag=77, type=string
	VsaAlvarionVsa78(&'a [u8]), // from Alvarion with plain value, tag=78, type=string
	VsaAlvarionVsa79(&'a [u8]), // from Alvarion with plain value, tag=79, type=string
	VsaAlvarionVsa80(&'a [u8]), // from Alvarion with plain value, tag=80, type=string
	VsaAlvarionVsa81(&'a [u8]), // from Alvarion with plain value, tag=81, type=string
	VsaAlvarionVsa82(&'a [u8]), // from Alvarion with plain value, tag=82, type=string
	VsaAlvarionVsa83(&'a [u8]), // from Alvarion with plain value, tag=83, type=string
	VsaAlvarionVsa84(&'a [u8]), // from Alvarion with plain value, tag=84, type=string
	VsaAlvarionVsa85(&'a [u8]), // from Alvarion with plain value, tag=85, type=string
	VsaAlvarionVsa86(&'a [u8]), // from Alvarion with plain value, tag=86, type=string
	VsaAlvarionVsa87(&'a [u8]), // from Alvarion with plain value, tag=87, type=string
	VsaAlvarionVsa88(&'a [u8]), // from Alvarion with plain value, tag=88, type=string
	VsaAlvarionVsa89(&'a [u8]), // from Alvarion with plain value, tag=89, type=string
	VsaAlvarionVsa90(&'a [u8]), // from Alvarion with plain value, tag=90, type=string
	VsaAlvarionVsa91(&'a [u8]), // from Alvarion with plain value, tag=91, type=string
	VsaAlvarionVsa92(&'a [u8]), // from Alvarion with plain value, tag=92, type=string
	VsaAlvarionVsa93(&'a [u8]), // from Alvarion with plain value, tag=93, type=string
	VsaAlvarionVsa94(&'a [u8]), // from Alvarion with plain value, tag=94, type=string
	VsaAlvarionVsa95(&'a [u8]), // from Alvarion with plain value, tag=95, type=string
	VsaAlvarionVsa96(&'a [u8]), // from Alvarion with plain value, tag=96, type=string
	VsaAlvarionVsa97(&'a [u8]), // from Alvarion with plain value, tag=97, type=string
	VsaAlvarionVsa98(&'a [u8]), // from Alvarion with plain value, tag=98, type=string
	VsaAlvarionVsa99(&'a [u8]), // from Alvarion with plain value, tag=99, type=string
	VsaAlvarionVsa100(&'a [u8]), // from Alvarion with plain value, tag=100, type=string
	VsaAlvarionVsa101(&'a [u8]), // from Alvarion with plain value, tag=101, type=string
	VsaAlvarionVsa102(&'a [u8]), // from Alvarion with plain value, tag=102, type=string
	VsaAlvarionVsa103(&'a [u8]), // from Alvarion with plain value, tag=103, type=string
	VsaAlvarionVsa104(&'a [u8]), // from Alvarion with plain value, tag=104, type=string
	VsaAlvarionVsa105(&'a [u8]), // from Alvarion with plain value, tag=105, type=string
	VsaAlvarionVsa106(&'a [u8]), // from Alvarion with plain value, tag=106, type=string
	VsaAlvarionVsa107(&'a [u8]), // from Alvarion with plain value, tag=107, type=string
	VsaAlvarionVsa108(&'a [u8]), // from Alvarion with plain value, tag=108, type=string
	VsaAlvarionVsa109(&'a [u8]), // from Alvarion with plain value, tag=109, type=string
	VsaAlvarionVsa110(&'a [u8]), // from Alvarion with plain value, tag=110, type=string
	VsaAlvarionVsa111(&'a [u8]), // from Alvarion with plain value, tag=111, type=string
	VsaAlvarionVsa112(&'a [u8]), // from Alvarion with plain value, tag=112, type=string
	VsaAlvarionVsa113(&'a [u8]), // from Alvarion with plain value, tag=113, type=string
	VsaAlvarionVsa114(&'a [u8]), // from Alvarion with plain value, tag=114, type=string
	VsaAlvarionVsa115(&'a [u8]), // from Alvarion with plain value, tag=115, type=string
	VsaAlvarionVsa116(&'a [u8]), // from Alvarion with plain value, tag=116, type=string
	VsaAlvarionVsa117(&'a [u8]), // from Alvarion with plain value, tag=117, type=string
	VsaAlvarionVsa118(&'a [u8]), // from Alvarion with plain value, tag=118, type=string
	VsaAlvarionVsa119(&'a [u8]), // from Alvarion with plain value, tag=119, type=string
	VsaAlvarionVsa120(&'a [u8]), // from Alvarion with plain value, tag=120, type=string
	VsaAlvarionVsa121(&'a [u8]), // from Alvarion with plain value, tag=121, type=string
	VsaAlvarionVsa122(&'a [u8]), // from Alvarion with plain value, tag=122, type=string
	VsaAlvarionVsa123(&'a [u8]), // from Alvarion with plain value, tag=123, type=string
	VsaAlvarionVsa124(&'a [u8]), // from Alvarion with plain value, tag=124, type=string
	VsaAlvarionVsa125(&'a [u8]), // from Alvarion with plain value, tag=125, type=string
	VsaAlvarionVsa126(&'a [u8]), // from Alvarion with plain value, tag=126, type=string
	VsaAlvarionVsa127(&'a [u8]), // from Alvarion with plain value, tag=127, type=string
	VsaAlvarionVsa128(&'a [u8]), // from Alvarion with plain value, tag=128, type=string
	VsaAlvarionVsa129(&'a [u8]), // from Alvarion with plain value, tag=129, type=string
	VsaAlvarionVsa130(&'a [u8]), // from Alvarion with plain value, tag=130, type=string
	VsaAlvarionVsa131(&'a [u8]), // from Alvarion with plain value, tag=131, type=string
	VsaAlvarionVsa132(&'a [u8]), // from Alvarion with plain value, tag=132, type=string
	VsaAlvarionVsa133(&'a [u8]), // from Alvarion with plain value, tag=133, type=string
	VsaAlvarionVsa134(&'a [u8]), // from Alvarion with plain value, tag=134, type=string
	VsaAlvarionVsa135(&'a [u8]), // from Alvarion with plain value, tag=135, type=string
	VsaAlvarionVsa136(&'a [u8]), // from Alvarion with plain value, tag=136, type=string
	VsaAlvarionVsa137(&'a [u8]), // from Alvarion with plain value, tag=137, type=string
	VsaAlvarionVsa138(&'a [u8]), // from Alvarion with plain value, tag=138, type=string
	VsaAlvarionVsa139(&'a [u8]), // from Alvarion with plain value, tag=139, type=string
	VsaAlvarionVsa140(&'a [u8]), // from Alvarion with plain value, tag=140, type=string
	VsaAlvarionVsa141(&'a [u8]), // from Alvarion with plain value, tag=141, type=string
	VsaAlvarionVsa142(&'a [u8]), // from Alvarion with plain value, tag=142, type=string
	VsaAlvarionVsa143(&'a [u8]), // from Alvarion with plain value, tag=143, type=string
	VsaAlvarionVsa144(&'a [u8]), // from Alvarion with plain value, tag=144, type=string
	VsaAlvarionVsa145(&'a [u8]), // from Alvarion with plain value, tag=145, type=string
	VsaAlvarionVsa146(&'a [u8]), // from Alvarion with plain value, tag=146, type=string
	VsaAlvarionVsa147(&'a [u8]), // from Alvarion with plain value, tag=147, type=string
	VsaAlvarionVsa148(&'a [u8]), // from Alvarion with plain value, tag=148, type=string
	VsaAlvarionVsa149(&'a [u8]), // from Alvarion with plain value, tag=149, type=string
	VsaAlvarionVsa150(&'a [u8]), // from Alvarion with plain value, tag=150, type=string
	VsaAlvarionVsa151(&'a [u8]), // from Alvarion with plain value, tag=151, type=string
	VsaAlvarionVsa152(&'a [u8]), // from Alvarion with plain value, tag=152, type=string
	VsaAlvarionVsa153(&'a [u8]), // from Alvarion with plain value, tag=153, type=string
	VsaAlvarionVsa154(&'a [u8]), // from Alvarion with plain value, tag=154, type=string
	VsaAlvarionVsa155(&'a [u8]), // from Alvarion with plain value, tag=155, type=string
	VsaAlvarionVsa156(&'a [u8]), // from Alvarion with plain value, tag=156, type=string
	VsaAlvarionVsa157(&'a [u8]), // from Alvarion with plain value, tag=157, type=string
	VsaAlvarionVsa158(&'a [u8]), // from Alvarion with plain value, tag=158, type=string
	VsaAlvarionVsa159(&'a [u8]), // from Alvarion with plain value, tag=159, type=string
	VsaAlvarionVsa160(&'a [u8]), // from Alvarion with plain value, tag=160, type=string
	VsaAlvarionVsa161(&'a [u8]), // from Alvarion with plain value, tag=161, type=string
	VsaAlvarionVsa162(&'a [u8]), // from Alvarion with plain value, tag=162, type=string
	VsaAlvarionVsa163(&'a [u8]), // from Alvarion with plain value, tag=163, type=string
	VsaAlvarionVsa164(&'a [u8]), // from Alvarion with plain value, tag=164, type=string
	VsaAlvarionVsa165(&'a [u8]), // from Alvarion with plain value, tag=165, type=string
	VsaAlvarionVsa166(&'a [u8]), // from Alvarion with plain value, tag=166, type=string
	VsaAlvarionVsa167(&'a [u8]), // from Alvarion with plain value, tag=167, type=string
	VsaAlvarionVsa168(&'a [u8]), // from Alvarion with plain value, tag=168, type=string
	VsaAlvarionVsa169(&'a [u8]), // from Alvarion with plain value, tag=169, type=string
	VsaAlvarionVsa170(&'a [u8]), // from Alvarion with plain value, tag=170, type=string
	VsaAlvarionVsa171(&'a [u8]), // from Alvarion with plain value, tag=171, type=string
	VsaAlvarionVsa172(&'a [u8]), // from Alvarion with plain value, tag=172, type=string
	VsaAlvarionVsa173(&'a [u8]), // from Alvarion with plain value, tag=173, type=string
	VsaAlvarionVsa174(&'a [u8]), // from Alvarion with plain value, tag=174, type=string
	VsaAlvarionVsa175(&'a [u8]), // from Alvarion with plain value, tag=175, type=string
	VsaAlvarionVsa176(&'a [u8]), // from Alvarion with plain value, tag=176, type=string
	VsaAlvarionVsa177(&'a [u8]), // from Alvarion with plain value, tag=177, type=string
	VsaAlvarionVsa178(&'a [u8]), // from Alvarion with plain value, tag=178, type=string
	VsaAlvarionVsa179(&'a [u8]), // from Alvarion with plain value, tag=179, type=string
	VsaAlvarionVsa180(&'a [u8]), // from Alvarion with plain value, tag=180, type=string
	VsaAlvarionVsa181(&'a [u8]), // from Alvarion with plain value, tag=181, type=string
	VsaAlvarionVsa182(&'a [u8]), // from Alvarion with plain value, tag=182, type=string
	VsaAlvarionVsa183(&'a [u8]), // from Alvarion with plain value, tag=183, type=string
	VsaAlvarionVsa184(&'a [u8]), // from Alvarion with plain value, tag=184, type=string
	VsaAlvarionVsa185(&'a [u8]), // from Alvarion with plain value, tag=185, type=string
	VsaAlvarionVsa186(&'a [u8]), // from Alvarion with plain value, tag=186, type=string
	VsaAlvarionVsa187(&'a [u8]), // from Alvarion with plain value, tag=187, type=string
	VsaAlvarionVsa188(&'a [u8]), // from Alvarion with plain value, tag=188, type=string
	VsaAlvarionVsa189(&'a [u8]), // from Alvarion with plain value, tag=189, type=string
	VsaAlvarionVsa190(&'a [u8]), // from Alvarion with plain value, tag=190, type=string
	VsaAlvarionVsa191(&'a [u8]), // from Alvarion with plain value, tag=191, type=string
	VsaAlvarionVsa192(&'a [u8]), // from Alvarion with plain value, tag=192, type=string
	VsaAlvarionVsa193(&'a [u8]), // from Alvarion with plain value, tag=193, type=string
	VsaAlvarionVsa194(&'a [u8]), // from Alvarion with plain value, tag=194, type=string
	VsaAlvarionVsa195(&'a [u8]), // from Alvarion with plain value, tag=195, type=string
	VsaAlvarionVsa196(&'a [u8]), // from Alvarion with plain value, tag=196, type=string
	VsaAlvarionVsa197(&'a [u8]), // from Alvarion with plain value, tag=197, type=string
	VsaAlvarionVsa198(&'a [u8]), // from Alvarion with plain value, tag=198, type=string
	VsaAlvarionVsa199(&'a [u8]), // from Alvarion with plain value, tag=199, type=string
	VsaAlvarionVsa200(&'a [u8]), // from Alvarion with plain value, tag=200, type=string
	VsaAlvarionVsa201(&'a [u8]), // from Alvarion with plain value, tag=201, type=string
	VsaAlvarionVsa202(&'a [u8]), // from Alvarion with plain value, tag=202, type=string
	VsaAlvarionVsa203(&'a [u8]), // from Alvarion with plain value, tag=203, type=string
	VsaAlvarionVsa204(&'a [u8]), // from Alvarion with plain value, tag=204, type=string
	VsaAlvarionVsa205(&'a [u8]), // from Alvarion with plain value, tag=205, type=string
	VsaAlvarionVsa206(&'a [u8]), // from Alvarion with plain value, tag=206, type=string
	VsaAlvarionVsa207(&'a [u8]), // from Alvarion with plain value, tag=207, type=string
	VsaAlvarionVsa208(&'a [u8]), // from Alvarion with plain value, tag=208, type=string
	VsaAlvarionVsa209(&'a [u8]), // from Alvarion with plain value, tag=209, type=string
	VsaAlvarionVsa210(&'a [u8]), // from Alvarion with plain value, tag=210, type=string
	VsaAlvarionVsa211(&'a [u8]), // from Alvarion with plain value, tag=211, type=string
	VsaAlvarionVsa212(&'a [u8]), // from Alvarion with plain value, tag=212, type=string
	VsaAlvarionVsa213(&'a [u8]), // from Alvarion with plain value, tag=213, type=string
	VsaAlvarionVsa214(&'a [u8]), // from Alvarion with plain value, tag=214, type=string
	VsaAlvarionVsa215(&'a [u8]), // from Alvarion with plain value, tag=215, type=string
	VsaAlvarionVsa216(&'a [u8]), // from Alvarion with plain value, tag=216, type=string
	VsaAlvarionVsa217(&'a [u8]), // from Alvarion with plain value, tag=217, type=string
	VsaAlvarionVsa218(&'a [u8]), // from Alvarion with plain value, tag=218, type=string
	VsaAlvarionVsa219(&'a [u8]), // from Alvarion with plain value, tag=219, type=string
	VsaAlvarionVsa220(&'a [u8]), // from Alvarion with plain value, tag=220, type=string
	VsaAlvarionVsa221(&'a [u8]), // from Alvarion with plain value, tag=221, type=string
	VsaAlvarionVsa222(&'a [u8]), // from Alvarion with plain value, tag=222, type=string
	VsaAlvarionVsa223(&'a [u8]), // from Alvarion with plain value, tag=223, type=string
	VsaAlvarionVsa224(&'a [u8]), // from Alvarion with plain value, tag=224, type=string
	VsaAlvarionVsa225(&'a [u8]), // from Alvarion with plain value, tag=225, type=string
	VsaAlvarionVsa226(&'a [u8]), // from Alvarion with plain value, tag=226, type=string
	VsaAlvarionVsa227(&'a [u8]), // from Alvarion with plain value, tag=227, type=string
	VsaAlvarionVsa228(&'a [u8]), // from Alvarion with plain value, tag=228, type=string
	VsaAlvarionVsa229(&'a [u8]), // from Alvarion with plain value, tag=229, type=string
	VsaAlvarionVsa230(&'a [u8]), // from Alvarion with plain value, tag=230, type=string
	VsaAlvarionVsa231(&'a [u8]), // from Alvarion with plain value, tag=231, type=string
	VsaAlvarionVsa232(&'a [u8]), // from Alvarion with plain value, tag=232, type=string
	VsaAlvarionVsa233(&'a [u8]), // from Alvarion with plain value, tag=233, type=string
	VsaAlvarionVsa234(&'a [u8]), // from Alvarion with plain value, tag=234, type=string
	VsaAlvarionVsa235(&'a [u8]), // from Alvarion with plain value, tag=235, type=string
	VsaAlvarionVsa236(&'a [u8]), // from Alvarion with plain value, tag=236, type=string
	VsaAlvarionVsa237(&'a [u8]), // from Alvarion with plain value, tag=237, type=string
	VsaAlvarionVsa238(&'a [u8]), // from Alvarion with plain value, tag=238, type=string
	VsaAlvarionVsa239(&'a [u8]), // from Alvarion with plain value, tag=239, type=string
	VsaAlvarionVsa240(&'a [u8]), // from Alvarion with plain value, tag=240, type=string
	VsaAlvarionVsa241(&'a [u8]), // from Alvarion with plain value, tag=241, type=string
	VsaAlvarionVsa242(&'a [u8]), // from Alvarion with plain value, tag=242, type=string
	VsaAlvarionVsa243(&'a [u8]), // from Alvarion with plain value, tag=243, type=string
	VsaAlvarionVsa244(&'a [u8]), // from Alvarion with plain value, tag=244, type=string
	VsaAlvarionVsa245(&'a [u8]), // from Alvarion with plain value, tag=245, type=string
	VsaAlvarionVsa246(&'a [u8]), // from Alvarion with plain value, tag=246, type=string
	VsaAlvarionVsa247(&'a [u8]), // from Alvarion with plain value, tag=247, type=string
	VsaAlvarionVsa248(&'a [u8]), // from Alvarion with plain value, tag=248, type=string
	VsaAlvarionVsa249(&'a [u8]), // from Alvarion with plain value, tag=249, type=string
	VsaAlvarionVsa250(&'a [u8]), // from Alvarion with plain value, tag=250, type=string
	VsaAlvarionVsa251(&'a [u8]), // from Alvarion with plain value, tag=251, type=string
	VsaAlvarionVsa252(&'a [u8]), // from Alvarion with plain value, tag=252, type=string
	VsaAlvarionVsa253(&'a [u8]), // from Alvarion with plain value, tag=253, type=string
	VsaAlvarionVsa254(&'a [u8]), // from Alvarion with plain value, tag=254, type=string
	VsaAlvarionVsa255(&'a [u8]), // from Alvarion with plain value, tag=255, type=string
	VsaApcServiceType(apc::ApcServiceType), // from APC with Enum value, tag=1, type=integer
	VsaApcOutlets(&'a [u8]), // from APC with plain value, tag=2, type=string
	VsaApcPerms(&'a [u8]), // from APC with plain value, tag=3, type=string
	VsaApcUsername(&'a [u8]), // from APC with plain value, tag=4, type=string
	VsaApcContact(&'a [u8]), // from APC with plain value, tag=5, type=string
	VsaApcAccpxDoors(&'a [u8]), // from APC with plain value, tag=6, type=string
	VsaApcAccpxStatus(&'a [u8]), // from APC with plain value, tag=7, type=string
	VsaApcAccpxAccess1(&'a [u8]), // from APC with plain value, tag=8, type=string
	VsaApcAccpxAccess2(&'a [u8]), // from APC with plain value, tag=9, type=string
	VsaApcAccpxAccess3(&'a [u8]), // from APC with plain value, tag=10, type=string
	VsaApcAccpxAccess4(&'a [u8]), // from APC with plain value, tag=11, type=string
	VsaApcAccpxAccess5(&'a [u8]), // from APC with plain value, tag=12, type=string
	VsaApcAccpxAccess6(&'a [u8]), // from APC with plain value, tag=13, type=string
	VsaApcAccpxAccess7(&'a [u8]), // from APC with plain value, tag=14, type=string
	VsaAptiloSubnetName(&'a [u8]), // from Aptilo with plain value, tag=1, type=string
	VsaAptiloOctetsLimit(u32), // from Aptilo with plain value, tag=2, type=integer
	VsaAptiloGigawordsLimit(u32), // from Aptilo with plain value, tag=3, type=integer
	VsaAptiloInputOctetsLimit(u32), // from Aptilo with plain value, tag=4, type=integer
	VsaAptiloInputGigawordsLimit(u32), // from Aptilo with plain value, tag=5, type=integer
	VsaAptiloOutputOctetsLimit(u32), // from Aptilo with plain value, tag=6, type=integer
	VsaAptiloOutputGigawordsLimit(u32), // from Aptilo with plain value, tag=7, type=integer
	VsaAptiloLimitMode(aptilo::AptiloLimitMode), // from Aptilo with Enum value, tag=8, type=integer
	VsaAptiloApcId(&'a [u8]), // from Aptilo with plain value, tag=9, type=string
	VsaAptiloOpaqueKey(&'a [u8]), // from Aptilo with plain value, tag=10, type=string
	VsaAptiloDeniedCause(aptilo::AptiloDeniedCause), // from Aptilo with Enum value, tag=11, type=integer
	VsaAptiloRealmId(u32), // from Aptilo with plain value, tag=12, type=integer
	VsaAptiloApId(u32), // from Aptilo with plain value, tag=13, type=integer
	VsaAptiloUserId(u32), // from Aptilo with plain value, tag=14, type=integer
	VsaAptiloZone(&'a [u8]), // from Aptilo with plain value, tag=15, type=string
	VsaAptiloFirstName(&'a [u8]), // from Aptilo with plain value, tag=16, type=string
	VsaAptiloLastName(&'a [u8]), // from Aptilo with plain value, tag=17, type=string
	VsaAptiloPhone(&'a [u8]), // from Aptilo with plain value, tag=18, type=string
	VsaAptiloEmail(&'a [u8]), // from Aptilo with plain value, tag=19, type=string
	VsaAptiloOrganization(&'a [u8]), // from Aptilo with plain value, tag=20, type=string
	VsaAptiloAccessProfile(&'a [u8]), // from Aptilo with plain value, tag=21, type=string
	VsaAptiloRealmConcurrentLogin(u32), // from Aptilo with plain value, tag=22, type=integer
	VsaAptiloAuthResult(u32), // from Aptilo with plain value, tag=23, type=integer
	VsaAptiloHotlineIndicator(&'a [u8]), // from Aptilo with plain value, tag=24, type=string
	VsaAptiloUserType(u32), // from Aptilo with plain value, tag=25, type=integer
	VsaAptiloExclusiveCount(u32), // from Aptilo with plain value, tag=26, type=integer
	VsaAptiloDurationQuota(u32), // from Aptilo with plain value, tag=27, type=integer
	VsaAptiloVolumeQuota(&'a [u8]), // from Aptilo with plain value, tag=28, type=string
	VsaAptiloRxVolumeQuota(&'a [u8]), // from Aptilo with plain value, tag=29, type=string
	VsaAptiloTxVolumeQuota(&'a [u8]), // from Aptilo with plain value, tag=30, type=string
	VsaAptiloResourceQuota(u32), // from Aptilo with plain value, tag=31, type=integer
	VsaAptiloQuotaId(&'a [u8]), // from Aptilo with plain value, tag=32, type=string
	VsaAptiloRxLimit(u32), // from Aptilo with plain value, tag=33, type=integer
	VsaAptiloTxLimit(u32), // from Aptilo with plain value, tag=34, type=integer
	VsaAptiloTrxLimit(u32), // from Aptilo with plain value, tag=35, type=integer
	VsaAptiloBwMinUp(u32), // from Aptilo with plain value, tag=36, type=integer
	VsaAptiloBwMaxUp(u32), // from Aptilo with plain value, tag=37, type=integer
	VsaAptiloBwMinDown(u32), // from Aptilo with plain value, tag=38, type=integer
	VsaAptiloBwMaxDown(u32), // from Aptilo with plain value, tag=39, type=integer
	VsaAptiloServiceProfile(&'a [u8]), // from Aptilo with plain value, tag=40, type=string
	VsaAptiloAutomaticService(&'a [u8]), // from Aptilo with plain value, tag=41, type=string
	VsaAptiloAuthType(aptilo::AptiloAuthType), // from Aptilo with Enum value, tag=42, type=integer
	VsaAptiloNasCapabilities(aptilo::AptiloNasCapabilities), // from Aptilo with Enum value, tag=43, type=integer
	VsaAptiloService(&'a [u8]), // from Aptilo with plain value, tag=44, type=string
	VsaAptiloServiceProfileId(u32), // from Aptilo with plain value, tag=45, type=integer
	VsaAptiloAuthParam(u32), // from Aptilo with plain value, tag=50, type=integer
	VsaAptiloAccessProfileId(u32), // from Aptilo with plain value, tag=53, type=integer
	VsaAptiloNasModel(&'a [u8]), // from Aptilo with plain value, tag=56, type=string
	VsaAptiloDebugOption(aptilo::AptiloDebugOption), // from Aptilo with Enum value, tag=57, type=integer
	VsaAptiloSessionId(&'a [u8]), // from Aptilo with plain value, tag=58, type=string
	VsaAptiloPrepaidCapabilities(&'a [u8]), // from Aptilo with plain value, tag=59, type=octets
	VsaAptiloOctetsQuota(&'a [u8]), // from Aptilo with plain value, tag=60, type=octets
	VsaAptiloOctetsThreshold(&'a [u8]), // from Aptilo with plain value, tag=61, type=octets
	VsaAptiloResourceThreshold(u32), // from Aptilo with plain value, tag=62, type=integer
	VsaAptiloDurationThreshold(u32), // from Aptilo with plain value, tag=63, type=integer
	VsaAptiloOctetsBalance(&'a [u8]), // from Aptilo with plain value, tag=64, type=octets
	VsaAptiloResourceBalance(u32), // from Aptilo with plain value, tag=65, type=integer
	VsaAptiloDurationBalance(u32), // from Aptilo with plain value, tag=66, type=integer
	VsaAptiloOctetsUsed(&'a [u8]), // from Aptilo with plain value, tag=67, type=octets
	VsaAptiloResourceUsed(u32), // from Aptilo with plain value, tag=68, type=integer
	VsaAptiloDurationUsed(u32), // from Aptilo with plain value, tag=69, type=integer
	VsaAptiloOctetsRequest(&'a [u8]), // from Aptilo with plain value, tag=70, type=octets
	VsaAptiloResourceRequest(u32), // from Aptilo with plain value, tag=71, type=integer
	VsaAptiloDurationRequest(u32), // from Aptilo with plain value, tag=72, type=integer
	VsaAptiloQosIndicator(&'a [u8]), // from Aptilo with plain value, tag=73, type=string
	VsaAptiloCircuitId(&'a [u8]), // from Aptilo with plain value, tag=74, type=octets
	VsaAptiloRemoteId(&'a [u8]), // from Aptilo with plain value, tag=75, type=octets
	VsaAptiloLocationName(&'a [u8]), // from Aptilo with plain value, tag=76, type=string
	VsaAptiloKeyIpv61(Ipv6Addr), // from Aptilo with plain value, tag=231, type=ipv6addr
	VsaAptiloKeyIpv62(Ipv6Addr), // from Aptilo with plain value, tag=232, type=ipv6addr
	VsaAptiloKeyIpv63(Ipv6Addr), // from Aptilo with plain value, tag=233, type=ipv6addr
	VsaAptiloKeyIpv64(Ipv6Addr), // from Aptilo with plain value, tag=234, type=ipv6addr
	VsaAptiloKeyIpv65(Ipv6Addr), // from Aptilo with plain value, tag=235, type=ipv6addr
	VsaAptiloKeyOctets1(&'a [u8]), // from Aptilo with plain value, tag=236, type=octets
	VsaAptiloKeyOctets2(&'a [u8]), // from Aptilo with plain value, tag=237, type=octets
	VsaAptiloKeyOctets3(&'a [u8]), // from Aptilo with plain value, tag=238, type=octets
	VsaAptiloKeyOctets4(&'a [u8]), // from Aptilo with plain value, tag=239, type=octets
	VsaAptiloKeyOctets5(&'a [u8]), // from Aptilo with plain value, tag=240, type=octets
	VsaAptiloKeyString1(&'a [u8]), // from Aptilo with plain value, tag=241, type=string
	VsaAptiloKeyString2(&'a [u8]), // from Aptilo with plain value, tag=242, type=string
	VsaAptiloKeyString3(&'a [u8]), // from Aptilo with plain value, tag=243, type=string
	VsaAptiloKeyString4(&'a [u8]), // from Aptilo with plain value, tag=244, type=string
	VsaAptiloKeyString5(&'a [u8]), // from Aptilo with plain value, tag=245, type=string
	VsaAptiloKeyIp1(Ipv4Addr), // from Aptilo with plain value, tag=246, type=ipaddr
	VsaAptiloKeyIp2(Ipv4Addr), // from Aptilo with plain value, tag=247, type=ipaddr
	VsaAptiloKeyIp3(Ipv4Addr), // from Aptilo with plain value, tag=248, type=ipaddr
	VsaAptiloKeyIp4(Ipv4Addr), // from Aptilo with plain value, tag=249, type=ipaddr
	VsaAptiloKeyIp5(Ipv4Addr), // from Aptilo with plain value, tag=250, type=ipaddr
	VsaAptiloKeyInteger1(u32), // from Aptilo with plain value, tag=251, type=integer
	VsaAptiloKeyInteger2(u32), // from Aptilo with plain value, tag=252, type=integer
	VsaAptiloKeyInteger3(u32), // from Aptilo with plain value, tag=253, type=integer
	VsaAptiloKeyInteger4(u32), // from Aptilo with plain value, tag=254, type=integer
	VsaAptiloKeyInteger5(u32), // from Aptilo with plain value, tag=255, type=integer
	VsaArborPrivilegeLevel(&'a [u8]), // from Arbor with plain value, tag=1, type=string
	VsaArubaUserRole(&'a [u8]), // from Aruba with plain value, tag=1, type=string
	VsaArubaUserVlan(u32), // from Aruba with plain value, tag=2, type=integer
	VsaArubaPrivAdminUser(u32), // from Aruba with plain value, tag=3, type=integer
	VsaArubaAdminRole(&'a [u8]), // from Aruba with plain value, tag=4, type=string
	VsaArubaEssidName(&'a [u8]), // from Aruba with plain value, tag=5, type=string
	VsaArubaLocationId(&'a [u8]), // from Aruba with plain value, tag=6, type=string
	VsaArubaPortIdentifier(&'a [u8]), // from Aruba with plain value, tag=7, type=string
	VsaArubaMmsUserTemplate(&'a [u8]), // from Aruba with plain value, tag=8, type=string
	VsaArubaNamedUserVlan(&'a [u8]), // from Aruba with plain value, tag=9, type=string
	VsaArubaApGroup(&'a [u8]), // from Aruba with plain value, tag=10, type=string
	VsaArubaFramedIpv6Address(&'a [u8]), // from Aruba with plain value, tag=11, type=string
	VsaArubaDeviceType(&'a [u8]), // from Aruba with plain value, tag=12, type=string
	VsaArubaApName(&'a [u8]), // from Aruba with plain value, tag=13, type=string
	VsaArubaNoDhcpFingerprint(u32), // from Aruba with plain value, tag=14, type=integer
	VsaArubaMdpsDeviceUdid(&'a [u8]), // from Aruba with plain value, tag=15, type=string
	VsaArubaMdpsDeviceImei(&'a [u8]), // from Aruba with plain value, tag=16, type=string
	VsaArubaMdpsDeviceIccid(&'a [u8]), // from Aruba with plain value, tag=17, type=string
	VsaArubaMdpsMaxDevices(u32), // from Aruba with plain value, tag=18, type=integer
	VsaArubaMdpsDeviceName(&'a [u8]), // from Aruba with plain value, tag=19, type=string
	VsaArubaMdpsDeviceProduct(&'a [u8]), // from Aruba with plain value, tag=20, type=string
	VsaArubaMdpsDeviceVersion(&'a [u8]), // from Aruba with plain value, tag=21, type=string
	VsaArubaMdpsDeviceSerial(&'a [u8]), // from Aruba with plain value, tag=22, type=string
	VsaArubaCppmRole(&'a [u8]), // from Aruba with plain value, tag=23, type=string
	VsaArubaAirgroupUserName(&'a [u8]), // from Aruba with plain value, tag=24, type=string
	VsaArubaAirgroupSharedUser(&'a [u8]), // from Aruba with plain value, tag=25, type=string
	VsaArubaAirgroupSharedRole(&'a [u8]), // from Aruba with plain value, tag=26, type=string
	VsaArubaAirgroupDeviceType(aruba::ArubaAirgroupDeviceType), // from Aruba with Enum value, tag=27, type=integer
	VsaArubaAuthSurvivability(&'a [u8]), // from Aruba with plain value, tag=28, type=string
	VsaArubaAsUserName(&'a [u8]), // from Aruba with plain value, tag=29, type=string
	VsaArubaAsCredentialHash(&'a [u8]), // from Aruba with plain value, tag=30, type=string
	VsaArubaWorkspaceAppName(&'a [u8]), // from Aruba with plain value, tag=31, type=string
	VsaArubaMdpsProvisioningSettings(&'a [u8]), // from Aruba with plain value, tag=32, type=string
	VsaArubaMdpsDeviceProfile(&'a [u8]), // from Aruba with plain value, tag=33, type=string
	VsaArubaApIpAddress(Ipv4Addr), // from Aruba with plain value, tag=34, type=ipaddr
	VsaArubaAirgroupSharedGroup(&'a [u8]), // from Aruba with plain value, tag=35, type=string
	VsaArubaUserGroup(&'a [u8]), // from Aruba with plain value, tag=36, type=string
	VsaArubaNetworkSsoToken(&'a [u8]), // from Aruba with plain value, tag=37, type=string
	VsaArubaAirgroupVersion(aruba::ArubaAirgroupVersion), // from Aruba with Enum value, tag=38, type=integer
	VsaArubaAuthSurvmethod(u32), // from Aruba with plain value, tag=39, type=integer
	VsaArubaPortBounceHost(u32), // from Aruba with plain value, tag=40, type=integer
	VsaArubaCaleaServerIp(Ipv4Addr), // from Aruba with plain value, tag=41, type=ipaddr
	VsaArubaAdminPath(&'a [u8]), // from Aruba with plain value, tag=42, type=string
	VsaAzaireTriplets(&'a [u8]), // from Azaire with plain value, tag=1, type=octets
	VsaAzaireImsi(&'a [u8]), // from Azaire with plain value, tag=2, type=octets
	VsaAzaireMsisdn(&'a [u8]), // from Azaire with plain value, tag=3, type=octets
	VsaAzaireApn(&'a [u8]), // from Azaire with plain value, tag=4, type=string
	VsaAzaireQos(&'a [u8]), // from Azaire with plain value, tag=5, type=octets
	VsaAzaireSelectionMode(azaire::AzaireSelectionMode), // from Azaire with Enum value, tag=6, type=integer
	VsaAzaireApnResolutionReq(azaire::AzaireApnResolutionReq), // from Azaire with Enum value, tag=7, type=integer
	VsaAzaireStartTime(&'a [u8]), // from Azaire with plain value, tag=8, type=octets
	VsaAzaireNasType(u32), // from Azaire with plain value, tag=9, type=integer
	VsaAzaireStatus(azaire::AzaireStatus), // from Azaire with Enum value, tag=10, type=integer
	VsaAzaireApnOi(&'a [u8]), // from Azaire with plain value, tag=11, type=string
	VsaAzaireAuthType(azaire::AzaireAuthType), // from Azaire with Enum value, tag=12, type=integer
	VsaAzaireGnUserName(&'a [u8]), // from Azaire with plain value, tag=13, type=string
	VsaAzaireBrandCode(&'a [u8]), // from Azaire with plain value, tag=14, type=string
	VsaAzairePolicyName(&'a [u8]), // from Azaire with plain value, tag=15, type=string
	VsaAzaireClientLocalIp(Ipv4Addr), // from Azaire with plain value, tag=16, type=ipaddr
	VsaAscendMaxSharedUsers(u32), // from Ascend with plain value, tag=2, type=integer
	VsaAscendUuInfo(&'a [u8]), // from Ascend with plain value, tag=7, type=string
	VsaAscendCirTimer(u32), // from Ascend with plain value, tag=9, type=integer
	VsaAscendFr08Mode(u32), // from Ascend with plain value, tag=10, type=integer
	VsaAscendDestinationNasPort(u32), // from Ascend with plain value, tag=11, type=integer
	VsaAscendFrSvcAddr(&'a [u8]), // from Ascend with plain value, tag=12, type=string
	VsaAscendNasPortFormat(ascend::AscendNasPortFormat), // from Ascend with Enum value, tag=13, type=integer
	VsaAscendAtmFaultManagement(ascend::AscendAtmFaultManagement), // from Ascend with Enum value, tag=14, type=integer
	VsaAscendAtmLoopbackCellLoss(u32), // from Ascend with plain value, tag=15, type=integer
	VsaAscendCktType(ascend::AscendCktType), // from Ascend with Enum value, tag=16, type=integer
	VsaAscendSvcEnabled(ascend::AscendSvcEnabled), // from Ascend with Enum value, tag=17, type=integer
	VsaAscendSessionType(ascend::AscendSessionType), // from Ascend with Enum value, tag=18, type=integer
	VsaAscendH323Gatekeeper(Ipv4Addr), // from Ascend with plain value, tag=19, type=ipaddr
	VsaAscendGlobalCallId(&'a [u8]), // from Ascend with plain value, tag=20, type=string
	VsaAscendH323ConferenceId(u32), // from Ascend with plain value, tag=21, type=integer
	VsaAscendH323FegwAddress(Ipv4Addr), // from Ascend with plain value, tag=22, type=ipaddr
	VsaAscendH323DialedTime(u32), // from Ascend with plain value, tag=23, type=integer
	VsaAscendDialedNumber(&'a [u8]), // from Ascend with plain value, tag=24, type=string
	VsaAscendInterArrivalJitter(u32), // from Ascend with plain value, tag=25, type=integer
	VsaAscendDroppedOctets(u32), // from Ascend with plain value, tag=26, type=integer
	VsaAscendDroppedPackets(u32), // from Ascend with plain value, tag=27, type=integer
	VsaAscendAuthDelay(u32), // from Ascend with plain value, tag=28, type=integer
	VsaAscendX25PadX3Profile(ascend::AscendX25PadX3Profile), // from Ascend with Enum value, tag=29, type=integer
	VsaAscendX25PadX3Parameters(&'a [u8]), // from Ascend with plain value, tag=30, type=string
	VsaAscendTunnelVrouterName(&'a [u8]), // from Ascend with plain value, tag=31, type=string
	VsaAscendX25ReverseCharging(ascend::AscendX25ReverseCharging), // from Ascend with Enum value, tag=32, type=integer
	VsaAscendX25NuiPrompt(&'a [u8]), // from Ascend with plain value, tag=33, type=string
	VsaAscendX25NuiPasswordPrompt(&'a [u8]), // from Ascend with plain value, tag=34, type=string
	VsaAscendX25Cug(&'a [u8]), // from Ascend with plain value, tag=35, type=string
	VsaAscendX25PadAlias1(&'a [u8]), // from Ascend with plain value, tag=36, type=string
	VsaAscendX25PadAlias2(&'a [u8]), // from Ascend with plain value, tag=37, type=string
	VsaAscendX25PadAlias3(&'a [u8]), // from Ascend with plain value, tag=38, type=string
	VsaAscendX25X121Address(&'a [u8]), // from Ascend with plain value, tag=39, type=string
	VsaAscendX25Nui(&'a [u8]), // from Ascend with plain value, tag=40, type=string
	VsaAscendX25Rpoa(&'a [u8]), // from Ascend with plain value, tag=41, type=string
	VsaAscendX25PadPrompt(&'a [u8]), // from Ascend with plain value, tag=42, type=string
	VsaAscendX25PadBanner(&'a [u8]), // from Ascend with plain value, tag=43, type=string
	VsaAscendX25ProfileName(&'a [u8]), // from Ascend with plain value, tag=44, type=string
	VsaAscendRecvName(&'a [u8]), // from Ascend with plain value, tag=45, type=string
	VsaAscendBiDirectionalAuth(ascend::AscendBiDirectionalAuth), // from Ascend with Enum value, tag=46, type=integer
	VsaAscendMtu(u32), // from Ascend with plain value, tag=47, type=integer
	VsaAscendCallDirection(ascend::AscendCallDirection), // from Ascend with Enum value, tag=48, type=integer
	VsaAscendServiceType(ascend::AscendServiceType), // from Ascend with Enum value, tag=49, type=integer
	VsaAscendFilterRequired(ascend::AscendFilterRequired), // from Ascend with Enum value, tag=50, type=integer
	VsaAscendTrafficShaper(u32), // from Ascend with plain value, tag=51, type=integer
	VsaAscendAccessInterceptLea(&'a [u8]), // from Ascend with plain value, tag=52, type=string
	VsaAscendAccessInterceptLog(&'a [u8]), // from Ascend with plain value, tag=53, type=string
	VsaAscendPrivateRouteTableId(&'a [u8]), // from Ascend with plain value, tag=54, type=string
	VsaAscendPrivateRouteRequired(ascend::AscendPrivateRouteRequired), // from Ascend with Enum value, tag=55, type=integer
	VsaAscendCacheRefresh(ascend::AscendCacheRefresh), // from Ascend with Enum value, tag=56, type=integer
	VsaAscendCacheTime(u32), // from Ascend with plain value, tag=57, type=integer
	VsaAscendEgressEnabled(u32), // from Ascend with plain value, tag=58, type=integer
	VsaAscendQosUpstream(&'a [u8]), // from Ascend with plain value, tag=59, type=string
	VsaAscendQosDownstream(&'a [u8]), // from Ascend with plain value, tag=60, type=string
	VsaAscendAtmConnectVpi(u32), // from Ascend with plain value, tag=61, type=integer
	VsaAscendAtmConnectVci(u32), // from Ascend with plain value, tag=62, type=integer
	VsaAscendAtmConnectGroup(u32), // from Ascend with plain value, tag=63, type=integer
	VsaAscendAtmGroup(u32), // from Ascend with plain value, tag=64, type=integer
	VsaAscendIpxHeaderCompression(ascend::AscendIpxHeaderCompression), // from Ascend with Enum value, tag=65, type=integer
	VsaAscendCallingIdTypeOfNum(ascend::AscendCallingIdTypeOfNum), // from Ascend with Enum value, tag=66, type=integer
	VsaAscendCallingIdNumberPlan(ascend::AscendCallingIdNumberPlan), // from Ascend with Enum value, tag=67, type=integer
	VsaAscendCallingIdPresentatn(ascend::AscendCallingIdPresentatn), // from Ascend with Enum value, tag=68, type=integer
	VsaAscendCallingIdScreening(ascend::AscendCallingIdScreening), // from Ascend with Enum value, tag=69, type=integer
	VsaAscendBirEnable(ascend::AscendBirEnable), // from Ascend with Enum value, tag=70, type=integer
	VsaAscendBirProxy(ascend::AscendBirProxy), // from Ascend with Enum value, tag=71, type=integer
	VsaAscendBirBridgeGroup(u32), // from Ascend with plain value, tag=72, type=integer
	VsaAscendIpsecProfile(&'a [u8]), // from Ascend with plain value, tag=73, type=string
	VsaAscendPppoeEnable(ascend::AscendPppoeEnable), // from Ascend with Enum value, tag=74, type=integer
	VsaAscendBridgeNonPppoe(ascend::AscendBridgeNonPppoe), // from Ascend with Enum value, tag=75, type=integer
	VsaAscendAtmDirect(ascend::AscendAtmDirect), // from Ascend with Enum value, tag=76, type=integer
	VsaAscendAtmDirectProfile(&'a [u8]), // from Ascend with plain value, tag=77, type=string
	VsaAscendClientPrimaryWins(Ipv4Addr), // from Ascend with plain value, tag=78, type=ipaddr
	VsaAscendClientSecondaryWins(Ipv4Addr), // from Ascend with plain value, tag=79, type=ipaddr
	VsaAscendClientAssignWins(ascend::AscendClientAssignWins), // from Ascend with Enum value, tag=80, type=integer
	VsaAscendAuthType(ascend::AscendAuthType), // from Ascend with Enum value, tag=81, type=integer
	VsaAscendPortRedirProtocol(ascend::AscendPortRedirProtocol), // from Ascend with Enum value, tag=82, type=integer
	VsaAscendPortRedirPortnum(u32), // from Ascend with plain value, tag=83, type=integer
	VsaAscendPortRedirServer(Ipv4Addr), // from Ascend with plain value, tag=84, type=ipaddr
	VsaAscendIpPoolChaining(ascend::AscendIpPoolChaining), // from Ascend with Enum value, tag=85, type=integer
	VsaAscendOwnerIpAddr(Ipv4Addr), // from Ascend with plain value, tag=86, type=ipaddr
	VsaAscendIpTos(ascend::AscendIpTos), // from Ascend with Enum value, tag=87, type=integer
	VsaAscendIpTosPrecedence(ascend::AscendIpTosPrecedence), // from Ascend with Enum value, tag=88, type=integer
	VsaAscendIpTosApplyTo(ascend::AscendIpTosApplyTo), // from Ascend with Enum value, tag=89, type=integer
	VsaAscendFilter(&'a [u8]), // from Ascend with plain value, tag=90, type=string
	VsaAscendTelnetProfile(&'a [u8]), // from Ascend with plain value, tag=91, type=string
	VsaAscendDslRateType(ascend::AscendDslRateType), // from Ascend with Enum value, tag=92, type=integer
	VsaAscendRedirectNumber(&'a [u8]), // from Ascend with plain value, tag=93, type=string
	VsaAscendAtmVpi(u32), // from Ascend with plain value, tag=94, type=integer
	VsaAscendAtmVci(u32), // from Ascend with plain value, tag=95, type=integer
	VsaAscendSourceIpCheck(ascend::AscendSourceIpCheck), // from Ascend with Enum value, tag=96, type=integer
	VsaAscendDslRateMode(ascend::AscendDslRateMode), // from Ascend with Enum value, tag=97, type=integer
	VsaAscendDslUpstreamLimit(ascend::AscendDslUpstreamLimit), // from Ascend with Enum value, tag=98, type=integer
	VsaAscendDslDownstreamLimit(ascend::AscendDslDownstreamLimit), // from Ascend with Enum value, tag=99, type=integer
	VsaAscendDslCirRecvLimit(u32), // from Ascend with plain value, tag=100, type=integer
	VsaAscendDslCirXmitLimit(u32), // from Ascend with plain value, tag=101, type=integer
	VsaAscendVrouterName(&'a [u8]), // from Ascend with plain value, tag=102, type=string
	VsaAscendSourceAuth(&'a [u8]), // from Ascend with plain value, tag=103, type=string
	VsaAscendPrivateRoute(&'a [u8]), // from Ascend with plain value, tag=104, type=string
	VsaAscendNumberingPlanId(ascend::AscendNumberingPlanId), // from Ascend with Enum value, tag=105, type=integer
	VsaAscendFrLinkStatusDlci(ascend::AscendFrLinkStatusDlci), // from Ascend with Enum value, tag=106, type=integer
	VsaAscendCallingSubaddress(&'a [u8]), // from Ascend with plain value, tag=107, type=string
	VsaAscendCallbackDelay(u32), // from Ascend with plain value, tag=108, type=integer
	VsaAscendEndpointDisc(&'a [u8]), // from Ascend with plain value, tag=109, type=string
	VsaAscendRemoteFw(&'a [u8]), // from Ascend with plain value, tag=110, type=string
	VsaAscendMulticastGleaveDelay(u32), // from Ascend with plain value, tag=111, type=integer
	VsaAscendCbcpEnable(ascend::AscendCbcpEnable), // from Ascend with Enum value, tag=112, type=integer
	VsaAscendCbcpMode(ascend::AscendCbcpMode), // from Ascend with Enum value, tag=113, type=integer
	VsaAscendCbcpDelay(u32), // from Ascend with plain value, tag=114, type=integer
	VsaAscendCbcpTrunkGroup(u32), // from Ascend with plain value, tag=115, type=integer
	VsaAscendAppletalkRoute(&'a [u8]), // from Ascend with plain value, tag=116, type=string
	VsaAscendAppletalkPeerMode(ascend::AscendAppletalkPeerMode), // from Ascend with Enum value, tag=117, type=integer
	VsaAscendRouteAppletalk(ascend::AscendRouteAppletalk), // from Ascend with Enum value, tag=118, type=integer
	VsaAscendFcpParameter(&'a [u8]), // from Ascend with plain value, tag=119, type=string
	VsaAscendModemPortno(u32), // from Ascend with plain value, tag=120, type=integer
	VsaAscendModemSlotno(u32), // from Ascend with plain value, tag=121, type=integer
	VsaAscendModemShelfno(u32), // from Ascend with plain value, tag=122, type=integer
	VsaAscendCallAttemptLimit(u32), // from Ascend with plain value, tag=123, type=integer
	VsaAscendCallBlockDuration(u32), // from Ascend with plain value, tag=124, type=integer
	VsaAscendMaximumCallDuration(u32), // from Ascend with plain value, tag=125, type=integer
	VsaAscendTemporaryRtes(ascend::AscendTemporaryRtes), // from Ascend with Enum value, tag=126, type=integer
	VsaAscendTunnelingProtocol(ascend::AscendTunnelingProtocol), // from Ascend with Enum value, tag=127, type=integer
	VsaAscendSharedProfileEnable(ascend::AscendSharedProfileEnable), // from Ascend with Enum value, tag=128, type=integer
	VsaAscendPrimaryHomeAgent(&'a [u8]), // from Ascend with plain value, tag=129, type=string
	VsaAscendSecondaryHomeAgent(&'a [u8]), // from Ascend with plain value, tag=130, type=string
	VsaAscendDialoutAllowed(ascend::AscendDialoutAllowed), // from Ascend with Enum value, tag=131, type=integer
	VsaAscendClientGateway(Ipv4Addr), // from Ascend with plain value, tag=132, type=ipaddr
	VsaAscendBacpEnable(ascend::AscendBacpEnable), // from Ascend with Enum value, tag=133, type=integer
	VsaAscendDhcpMaximumLeases(u32), // from Ascend with plain value, tag=134, type=integer
	VsaAscendClientPrimaryDns(Ipv4Addr), // from Ascend with plain value, tag=135, type=ipaddr
	VsaAscendClientSecondaryDns(Ipv4Addr), // from Ascend with plain value, tag=136, type=ipaddr
	VsaAscendClientAssignDns(ascend::AscendClientAssignDns), // from Ascend with Enum value, tag=137, type=integer
	VsaAscendUserAcctType(ascend::AscendUserAcctType), // from Ascend with Enum value, tag=138, type=integer
	VsaAscendUserAcctHost(Ipv4Addr), // from Ascend with plain value, tag=139, type=ipaddr
	VsaAscendUserAcctPort(u32), // from Ascend with plain value, tag=140, type=integer
	VsaAscendUserAcctKey(&'a [u8]), // from Ascend with plain value, tag=141, type=string
	VsaAscendUserAcctBase(ascend::AscendUserAcctBase), // from Ascend with Enum value, tag=142, type=integer
	VsaAscendUserAcctTime(u32), // from Ascend with plain value, tag=143, type=integer
	VsaAscendAssignIpClient(Ipv4Addr), // from Ascend with plain value, tag=144, type=ipaddr
	VsaAscendAssignIpServer(Ipv4Addr), // from Ascend with plain value, tag=145, type=ipaddr
	VsaAscendAssignIpGlobalPool(&'a [u8]), // from Ascend with plain value, tag=146, type=string
	VsaAscendDhcpReply(ascend::AscendDhcpReply), // from Ascend with Enum value, tag=147, type=integer
	VsaAscendDhcpPoolNumber(u32), // from Ascend with plain value, tag=148, type=integer
	VsaAscendExpectCallback(ascend::AscendExpectCallback), // from Ascend with Enum value, tag=149, type=integer
	VsaAscendEventType(ascend::AscendEventType), // from Ascend with Enum value, tag=150, type=integer
	VsaAscendSessionSvrKey(&'a [u8]), // from Ascend with plain value, tag=151, type=string
	VsaAscendMulticastRateLimit(u32), // from Ascend with plain value, tag=152, type=integer
	VsaAscendIfNetmask(Ipv4Addr), // from Ascend with plain value, tag=153, type=ipaddr
	VsaAscendRemoteAddr(Ipv4Addr), // from Ascend with plain value, tag=154, type=ipaddr
	VsaAscendMulticastClient(ascend::AscendMulticastClient), // from Ascend with Enum value, tag=155, type=integer
	VsaAscendFrCircuitName(&'a [u8]), // from Ascend with plain value, tag=156, type=string
	VsaAscendFrLinkup(ascend::AscendFrLinkup), // from Ascend with Enum value, tag=157, type=integer
	VsaAscendFrNailedGrp(u32), // from Ascend with plain value, tag=158, type=integer
	VsaAscendFrType(ascend::AscendFrType), // from Ascend with Enum value, tag=159, type=integer
	VsaAscendFrLinkMgt(ascend::AscendFrLinkMgt), // from Ascend with Enum value, tag=160, type=integer
	VsaAscendFrN391(u32), // from Ascend with plain value, tag=161, type=integer
	VsaAscendFrDceN392(u32), // from Ascend with plain value, tag=162, type=integer
	VsaAscendFrDteN392(u32), // from Ascend with plain value, tag=163, type=integer
	VsaAscendFrDceN393(u32), // from Ascend with plain value, tag=164, type=integer
	VsaAscendFrDteN393(u32), // from Ascend with plain value, tag=165, type=integer
	VsaAscendFrT391(u32), // from Ascend with plain value, tag=166, type=integer
	VsaAscendFrT392(u32), // from Ascend with plain value, tag=167, type=integer
	VsaAscendBridgeAddress(&'a [u8]), // from Ascend with plain value, tag=168, type=string
	VsaAscendTsIdleLimit(u32), // from Ascend with plain value, tag=169, type=integer
	VsaAscendTsIdleMode(ascend::AscendTsIdleMode), // from Ascend with Enum value, tag=170, type=integer
	VsaAscendDbaMonitor(ascend::AscendDbaMonitor), // from Ascend with Enum value, tag=171, type=integer
	VsaAscendBaseChannelCount(u32), // from Ascend with plain value, tag=172, type=integer
	VsaAscendMinimumChannels(u32), // from Ascend with plain value, tag=173, type=integer
	VsaAscendIpxRoute(&'a [u8]), // from Ascend with plain value, tag=174, type=string
	VsaAscendFt1Caller(ascend::AscendFt1Caller), // from Ascend with Enum value, tag=175, type=integer
	VsaAscendBackup(&'a [u8]), // from Ascend with plain value, tag=176, type=string
	VsaAscendCallType(ascend::AscendCallType), // from Ascend with Enum value, tag=177, type=integer
	VsaAscendGroup(&'a [u8]), // from Ascend with plain value, tag=178, type=string
	VsaAscendFrDlci(u32), // from Ascend with plain value, tag=179, type=integer
	VsaAscendFrProfileName(&'a [u8]), // from Ascend with plain value, tag=180, type=string
	VsaAscendAraPw(&'a [u8]), // from Ascend with plain value, tag=181, type=string
	VsaAscendIpxNodeAddr(&'a [u8]), // from Ascend with plain value, tag=182, type=string
	VsaAscendHomeAgentIpAddr(Ipv4Addr), // from Ascend with plain value, tag=183, type=ipaddr
	VsaAscendHomeAgentPassword(&'a [u8]), // from Ascend with plain value, tag=184, type=string
	VsaAscendHomeNetworkName(&'a [u8]), // from Ascend with plain value, tag=185, type=string
	VsaAscendHomeAgentUdpPort(u32), // from Ascend with plain value, tag=186, type=integer
	VsaAscendMultilinkId(u32), // from Ascend with plain value, tag=187, type=integer
	VsaAscendNumInMultilink(u32), // from Ascend with plain value, tag=188, type=integer
	VsaAscendFirstDest(Ipv4Addr), // from Ascend with plain value, tag=189, type=ipaddr
	VsaAscendPreInputOctets(u32), // from Ascend with plain value, tag=190, type=integer
	VsaAscendPreOutputOctets(u32), // from Ascend with plain value, tag=191, type=integer
	VsaAscendPreInputPackets(u32), // from Ascend with plain value, tag=192, type=integer
	VsaAscendPreOutputPackets(u32), // from Ascend with plain value, tag=193, type=integer
	VsaAscendMaximumTime(u32), // from Ascend with plain value, tag=194, type=integer
	VsaAscendDisconnectCause(ascend::AscendDisconnectCause), // from Ascend with Enum value, tag=195, type=integer
	VsaAscendConnectProgress(ascend::AscendConnectProgress), // from Ascend with Enum value, tag=196, type=integer
	VsaAscendDataRate(u32), // from Ascend with plain value, tag=197, type=integer
	VsaAscendPresessionTime(u32), // from Ascend with plain value, tag=198, type=integer
	VsaAscendTokenIdle(u32), // from Ascend with plain value, tag=199, type=integer
	VsaAscendTokenImmediate(ascend::AscendTokenImmediate), // from Ascend with Enum value, tag=200, type=integer
	VsaAscendRequireAuth(ascend::AscendRequireAuth), // from Ascend with Enum value, tag=201, type=integer
	VsaAscendNumberSessions(&'a [u8]), // from Ascend with plain value, tag=202, type=string
	VsaAscendAuthenAlias(&'a [u8]), // from Ascend with plain value, tag=203, type=string
	VsaAscendTokenExpiry(u32), // from Ascend with plain value, tag=204, type=integer
	VsaAscendMenuSelector(&'a [u8]), // from Ascend with plain value, tag=205, type=string
	VsaAscendMenuItem(&'a [u8]), // from Ascend with plain value, tag=206, type=string
	VsaAscendPwWarntime(ascend::AscendPwWarntime), // from Ascend with Enum value, tag=207, type=integer
	VsaAscendPwLifetime(ascend::AscendPwLifetime), // from Ascend with Enum value, tag=208, type=integer
	VsaAscendIpDirect(Ipv4Addr), // from Ascend with plain value, tag=209, type=ipaddr
	VsaAscendPppVjSlotComp(ascend::AscendPppVjSlotComp), // from Ascend with Enum value, tag=210, type=integer
	VsaAscendPppVj1172(ascend::AscendPppVj1172), // from Ascend with Enum value, tag=211, type=integer
	VsaAscendPppAsyncMap(u32), // from Ascend with plain value, tag=212, type=integer
	VsaAscendThirdPrompt(&'a [u8]), // from Ascend with plain value, tag=213, type=string
	VsaAscendSendSecret(&'a [u8]), // from Ascend with plain value, tag=214, type=string
	VsaAscendReceiveSecret(&'a [u8]), // from Ascend with plain value, tag=215, type=string
	VsaAscendIpxPeerMode(ascend::AscendIpxPeerMode), // from Ascend with Enum value, tag=216, type=integer
	VsaAscendIpPoolDefinition(&'a [u8]), // from Ascend with plain value, tag=217, type=string
	VsaAscendAssignIpPool(u32), // from Ascend with plain value, tag=218, type=integer
	VsaAscendFrDirect(ascend::AscendFrDirect), // from Ascend with Enum value, tag=219, type=integer
	VsaAscendFrDirectProfile(&'a [u8]), // from Ascend with plain value, tag=220, type=string
	VsaAscendFrDirectDlci(u32), // from Ascend with plain value, tag=221, type=integer
	VsaAscendHandleIpx(ascend::AscendHandleIpx), // from Ascend with Enum value, tag=222, type=integer
	VsaAscendNetwareTimeout(u32), // from Ascend with plain value, tag=223, type=integer
	VsaAscendIpxAlias(u32), // from Ascend with plain value, tag=224, type=integer
	VsaAscendMetric(u32), // from Ascend with plain value, tag=225, type=integer
	VsaAscendPriNumberType(ascend::AscendPriNumberType), // from Ascend with Enum value, tag=226, type=integer
	VsaAscendDialNumber(&'a [u8]), // from Ascend with plain value, tag=227, type=string
	VsaAscendRouteIp(ascend::AscendRouteIp), // from Ascend with Enum value, tag=228, type=integer
	VsaAscendRouteIpx(ascend::AscendRouteIpx), // from Ascend with Enum value, tag=229, type=integer
	VsaAscendBridge(ascend::AscendBridge), // from Ascend with Enum value, tag=230, type=integer
	VsaAscendSendAuth(ascend::AscendSendAuth), // from Ascend with Enum value, tag=231, type=integer
	VsaAscendSendPasswd(&'a [u8]), // from Ascend with plain value, tag=232, type=string
	VsaAscendLinkCompression(ascend::AscendLinkCompression), // from Ascend with Enum value, tag=233, type=integer
	VsaAscendTargetUtil(u32), // from Ascend with plain value, tag=234, type=integer
	VsaAscendMaximumChannels(u32), // from Ascend with plain value, tag=235, type=integer
	VsaAscendIncChannelCount(u32), // from Ascend with plain value, tag=236, type=integer
	VsaAscendDecChannelCount(u32), // from Ascend with plain value, tag=237, type=integer
	VsaAscendSecondsOfHistory(u32), // from Ascend with plain value, tag=238, type=integer
	VsaAscendHistoryWeighType(ascend::AscendHistoryWeighType), // from Ascend with Enum value, tag=239, type=integer
	VsaAscendAddSeconds(u32), // from Ascend with plain value, tag=240, type=integer
	VsaAscendRemoveSeconds(u32), // from Ascend with plain value, tag=241, type=integer
	VsaAscendDataFilter(&'a [u8]), // from Ascend with plain value, tag=242, type=abinary
	VsaAscendCallFilter(&'a [u8]), // from Ascend with plain value, tag=243, type=abinary
	VsaAscendIdleLimit(u32), // from Ascend with plain value, tag=244, type=integer
	VsaAscendPreemptLimit(u32), // from Ascend with plain value, tag=245, type=integer
	VsaAscendCallback(ascend::AscendCallback), // from Ascend with Enum value, tag=246, type=integer
	VsaAscendDataSvc(ascend::AscendDataSvc), // from Ascend with Enum value, tag=247, type=integer
	VsaAscendForce56(ascend::AscendForce56), // from Ascend with Enum value, tag=248, type=integer
	VsaAscendBillingNumber(&'a [u8]), // from Ascend with plain value, tag=249, type=string
	VsaAscendCallByCall(u32), // from Ascend with plain value, tag=250, type=integer
	VsaAscendTransitNumber(&'a [u8]), // from Ascend with plain value, tag=251, type=string
	VsaAscendHostInfo(&'a [u8]), // from Ascend with plain value, tag=252, type=string
	VsaAscendPppAddress(Ipv4Addr), // from Ascend with plain value, tag=253, type=ipaddr
	VsaAscendMppIdlePercent(u32), // from Ascend with plain value, tag=254, type=integer
	VsaAscendXmitRate(u32), // from Ascend with plain value, tag=255, type=integer
	VsaAnnexFilter(&'a [u8]), // from Bay-Networks with plain value, tag=28, type=string
	VsaAnnexCliCommand(&'a [u8]), // from Bay-Networks with plain value, tag=29, type=string
	VsaAnnexCliFilter(&'a [u8]), // from Bay-Networks with plain value, tag=30, type=string
	VsaAnnexHostRestrict(&'a [u8]), // from Bay-Networks with plain value, tag=31, type=string
	VsaAnnexHostAllow(&'a [u8]), // from Bay-Networks with plain value, tag=32, type=string
	VsaAnnexProductName(&'a [u8]), // from Bay-Networks with plain value, tag=33, type=string
	VsaAnnexSwVersion(&'a [u8]), // from Bay-Networks with plain value, tag=34, type=string
	VsaAnnexLocalIpAddress(Ipv4Addr), // from Bay-Networks with plain value, tag=35, type=ipaddr
	VsaAnnexCallbackPortlist(u32), // from Bay-Networks with plain value, tag=36, type=integer
	VsaAnnexSecProfileIndex(u32), // from Bay-Networks with plain value, tag=37, type=integer
	VsaAnnexTunnelAuthenType(bay_networks::AnnexTunnelAuthenType), // from Bay-Networks with Enum value, tag=38, type=integer
	VsaAnnexTunnelAuthenMode(bay_networks::AnnexTunnelAuthenMode), // from Bay-Networks with Enum value, tag=39, type=integer
	VsaAnnexAuthenServers(&'a [u8]), // from Bay-Networks with plain value, tag=40, type=string
	VsaAnnexAcctServers(&'a [u8]), // from Bay-Networks with plain value, tag=41, type=string
	VsaAnnexUserServerLocation(bay_networks::AnnexUserServerLocation), // from Bay-Networks with Enum value, tag=42, type=integer
	VsaAnnexLocalUsername(&'a [u8]), // from Bay-Networks with plain value, tag=43, type=string
	VsaAnnexSystemDiscReason(bay_networks::AnnexSystemDiscReason), // from Bay-Networks with Enum value, tag=44, type=integer
	VsaAnnexModemDiscReason(bay_networks::AnnexModemDiscReason), // from Bay-Networks with Enum value, tag=45, type=integer
	VsaAnnexDisconnectReason(u32), // from Bay-Networks with plain value, tag=46, type=integer
	VsaAnnexAddrResolutionProtocol(bay_networks::AnnexAddrResolutionProtocol), // from Bay-Networks with Enum value, tag=47, type=integer
	VsaAnnexAddrResolutionServers(&'a [u8]), // from Bay-Networks with plain value, tag=48, type=string
	VsaAnnexDomainName(&'a [u8]), // from Bay-Networks with plain value, tag=49, type=string
	VsaAnnexTransmitSpeed(u32), // from Bay-Networks with plain value, tag=50, type=integer
	VsaAnnexReceiveSpeed(u32), // from Bay-Networks with plain value, tag=51, type=integer
	VsaAnnexInputFilter(&'a [u8]), // from Bay-Networks with plain value, tag=52, type=string
	VsaAnnexOutputFilter(&'a [u8]), // from Bay-Networks with plain value, tag=53, type=string
	VsaAnnexPrimaryDnsServer(Ipv4Addr), // from Bay-Networks with plain value, tag=54, type=ipaddr
	VsaAnnexSecondaryDnsServer(Ipv4Addr), // from Bay-Networks with plain value, tag=55, type=ipaddr
	VsaAnnexPrimaryNbnsServer(Ipv4Addr), // from Bay-Networks with plain value, tag=56, type=ipaddr
	VsaAnnexSecondaryNbnsServer(Ipv4Addr), // from Bay-Networks with plain value, tag=57, type=ipaddr
	VsaAnnexSyslogTap(u32), // from Bay-Networks with plain value, tag=58, type=integer
	VsaAnnexKeypressTimeout(u32), // from Bay-Networks with plain value, tag=59, type=integer
	VsaAnnexUnauthenticatedTime(u32), // from Bay-Networks with plain value, tag=60, type=integer
	VsaAnnexReChapTimeout(u32), // from Bay-Networks with plain value, tag=61, type=integer
	VsaAnnexMrru(u32), // from Bay-Networks with plain value, tag=62, type=integer
	VsaAnnexEdo(&'a [u8]), // from Bay-Networks with plain value, tag=63, type=string
	VsaAnnexPppTraceLevel(u32), // from Bay-Networks with plain value, tag=64, type=integer
	VsaAnnexPreInputOctets(u32), // from Bay-Networks with plain value, tag=65, type=integer
	VsaAnnexPreOutputOctets(u32), // from Bay-Networks with plain value, tag=66, type=integer
	VsaAnnexPreInputPackets(u32), // from Bay-Networks with plain value, tag=67, type=integer
	VsaAnnexPreOutputPackets(u32), // from Bay-Networks with plain value, tag=68, type=integer
	VsaAnnexConnectProgress(u32), // from Bay-Networks with plain value, tag=69, type=integer
	VsaAnnexMulticastRateLimit(u32), // from Bay-Networks with plain value, tag=73, type=integer
	VsaAnnexMaximumCallDuration(u32), // from Bay-Networks with plain value, tag=74, type=integer
	VsaAnnexMultilinkId(u32), // from Bay-Networks with plain value, tag=75, type=integer
	VsaAnnexNumInMultilink(u32), // from Bay-Networks with plain value, tag=76, type=integer
	VsaAnnexSecondarySrvEndpoint(&'a [u8]), // from Bay-Networks with plain value, tag=79, type=string
	VsaAnnexGwySelectionMode(u32), // from Bay-Networks with plain value, tag=80, type=integer
	VsaAnnexLogicalChannelNumber(u32), // from Bay-Networks with plain value, tag=81, type=integer
	VsaAnnexWanNumber(u32), // from Bay-Networks with plain value, tag=82, type=integer
	VsaAnnexPort(u32), // from Bay-Networks with plain value, tag=83, type=integer
	VsaAnnexPoolId(u32), // from Bay-Networks with plain value, tag=85, type=integer
	VsaAnnexCompressionProtocol(&'a [u8]), // from Bay-Networks with plain value, tag=86, type=string
	VsaAnnexTransmittedPackets(u32), // from Bay-Networks with plain value, tag=87, type=integer
	VsaAnnexRetransmittedPackets(u32), // from Bay-Networks with plain value, tag=88, type=integer
	VsaAnnexSignalToNoiseRatio(u32), // from Bay-Networks with plain value, tag=89, type=integer
	VsaAnnexRetrainRequestsSent(u32), // from Bay-Networks with plain value, tag=90, type=integer
	VsaAnnexRetrainRequestsRcvd(u32), // from Bay-Networks with plain value, tag=91, type=integer
	VsaAnnexRateRenegReqSent(u32), // from Bay-Networks with plain value, tag=92, type=integer
	VsaAnnexRateRenegReqRcvd(u32), // from Bay-Networks with plain value, tag=93, type=integer
	VsaAnnexBeginReceiveLineLevel(u32), // from Bay-Networks with plain value, tag=94, type=integer
	VsaAnnexEndReceiveLineLevel(u32), // from Bay-Networks with plain value, tag=95, type=integer
	VsaAnnexBeginModulation(&'a [u8]), // from Bay-Networks with plain value, tag=96, type=string
	VsaAnnexErrorCorrectionProt(&'a [u8]), // from Bay-Networks with plain value, tag=97, type=string
	VsaAnnexEndModulation(&'a [u8]), // from Bay-Networks with plain value, tag=98, type=string
	VsaAnnexUserLevel(bay_networks::AnnexUserLevel), // from Bay-Networks with Enum value, tag=100, type=integer
	VsaAnnexAuditLevel(bay_networks::AnnexAuditLevel), // from Bay-Networks with Enum value, tag=101, type=integer
	VsaCesGroup(&'a [u8]), // from Bay-Networks with plain value, tag=102, type=string
	VsaPassportAccessPriority(bay_networks::PassportAccessPriority), // from Bay-Networks with Enum value, tag=192, type=integer
	VsaAnnexCliCommands(&'a [u8]), // from Bay-Networks with plain value, tag=193, type=string
	VsaAnnexCommandAccess(bay_networks::AnnexCommandAccess), // from Bay-Networks with Enum value, tag=194, type=integer
	VsaCommands(&'a [u8]), // from Bay-Networks with plain value, tag=195, type=string
	VsaBintecBiboppptable(&'a [u8]), // from BinTec with plain value, tag=224, type=string
	VsaBintecBibodialtable(&'a [u8]), // from BinTec with plain value, tag=225, type=string
	VsaBintecIpextiftable(&'a [u8]), // from BinTec with plain value, tag=226, type=string
	VsaBintecIproutetable(&'a [u8]), // from BinTec with plain value, tag=227, type=string
	VsaBintecIpextrttable(&'a [u8]), // from BinTec with plain value, tag=228, type=string
	VsaBintecIpnatpresettable(&'a [u8]), // from BinTec with plain value, tag=229, type=string
	VsaBintecIpxcirctable(&'a [u8]), // from BinTec with plain value, tag=230, type=string
	VsaBintecRipcirctable(&'a [u8]), // from BinTec with plain value, tag=231, type=string
	VsaBintecSapcirctable(&'a [u8]), // from BinTec with plain value, tag=232, type=string
	VsaBintecIpxstaticroutetable(&'a [u8]), // from BinTec with plain value, tag=233, type=string
	VsaBintecIpxstaticservtable(&'a [u8]), // from BinTec with plain value, tag=234, type=string
	VsaBintecOspfiftable(&'a [u8]), // from BinTec with plain value, tag=235, type=string
	VsaBintecPppextiftable(&'a [u8]), // from BinTec with plain value, tag=236, type=string
	VsaBintecIpfiltertable(&'a [u8]), // from BinTec with plain value, tag=237, type=string
	VsaBintecIpqostable(&'a [u8]), // from BinTec with plain value, tag=238, type=string
	VsaBintecQosiftable(&'a [u8]), // from BinTec with plain value, tag=239, type=string
	VsaBintecQospolicytable(&'a [u8]), // from BinTec with plain value, tag=240, type=string
	VsaBlueCoatGroup(&'a [u8]), // from BlueCoat with plain value, tag=1, type=string
	VsaBlueCoatAuthorization(bluecoat::BlueCoatAuthorization), // from BlueCoat with Enum value, tag=2, type=integer
	VsaBwasRecordId(&'a [u8]), // from BroadSoft with plain value, tag=1, type=string
	VsaBwasServiceProvider(&'a [u8]), // from BroadSoft with plain value, tag=2, type=string
	VsaBwasType(&'a [u8]), // from BroadSoft with plain value, tag=3, type=string
	VsaBwasUserNumber(&'a [u8]), // from BroadSoft with plain value, tag=4, type=string
	VsaBwasGroupNumber(&'a [u8]), // from BroadSoft with plain value, tag=5, type=string
	VsaBwasDirection(&'a [u8]), // from BroadSoft with plain value, tag=6, type=string
	VsaBwasCallingNumber(&'a [u8]), // from BroadSoft with plain value, tag=7, type=string
	VsaBwasCallingPresentationIndic(&'a [u8]), // from BroadSoft with plain value, tag=8, type=string
	VsaBwasCalledNumber(&'a [u8]), // from BroadSoft with plain value, tag=9, type=string
	VsaBwasStartTime(&'a [u8]), // from BroadSoft with plain value, tag=10, type=string
	VsaBwasUserTimezone(&'a [u8]), // from BroadSoft with plain value, tag=11, type=string
	VsaBwasAnswerIndic(&'a [u8]), // from BroadSoft with plain value, tag=12, type=string
	VsaBwasAnswerTime(&'a [u8]), // from BroadSoft with plain value, tag=13, type=string
	VsaBwasReleaseTime(&'a [u8]), // from BroadSoft with plain value, tag=14, type=string
	VsaBwasTerminationCause(&'a [u8]), // from BroadSoft with plain value, tag=15, type=string
	VsaBwasNetworkType(&'a [u8]), // from BroadSoft with plain value, tag=16, type=string
	VsaBwasCarrierIdentificationCode(&'a [u8]), // from BroadSoft with plain value, tag=17, type=string
	VsaBwasDialedDigits(&'a [u8]), // from BroadSoft with plain value, tag=18, type=string
	VsaBwasCallCategory(&'a [u8]), // from BroadSoft with plain value, tag=19, type=string
	VsaBwasNetworkCallType(&'a [u8]), // from BroadSoft with plain value, tag=20, type=string
	VsaBwasNetworkTranslatedNumber(&'a [u8]), // from BroadSoft with plain value, tag=21, type=string
	VsaBwasNetworkTranslatedGroup(&'a [u8]), // from BroadSoft with plain value, tag=22, type=string
	VsaBwasReleasingParty(&'a [u8]), // from BroadSoft with plain value, tag=23, type=string
	VsaBwasRoute(&'a [u8]), // from BroadSoft with plain value, tag=24, type=string
	VsaBwasNetworkCallid(&'a [u8]), // from BroadSoft with plain value, tag=25, type=string
	VsaBwasCodec(&'a [u8]), // from BroadSoft with plain value, tag=26, type=string
	VsaBwasAccessDeviceAddress(&'a [u8]), // from BroadSoft with plain value, tag=27, type=string
	VsaBwasAccessCallid(&'a [u8]), // from BroadSoft with plain value, tag=28, type=string
	VsaBwasSpare29(&'a [u8]), // from BroadSoft with plain value, tag=29, type=string
	VsaBwasFailoverCorrelationId(&'a [u8]), // from BroadSoft with plain value, tag=30, type=string
	VsaBwasSpare31(&'a [u8]), // from BroadSoft with plain value, tag=31, type=string
	VsaBwasGroup(&'a [u8]), // from BroadSoft with plain value, tag=32, type=string
	VsaBwasDepartment(&'a [u8]), // from BroadSoft with plain value, tag=33, type=string
	VsaBwasAccountCode(&'a [u8]), // from BroadSoft with plain value, tag=34, type=string
	VsaBwasAuthorizationCode(&'a [u8]), // from BroadSoft with plain value, tag=35, type=string
	VsaBwasOriginalCalledNumber(&'a [u8]), // from BroadSoft with plain value, tag=36, type=string
	VsaBwasOriginalCalledPresentationIndic(&'a [u8]), // from BroadSoft with plain value, tag=37, type=string
	VsaBwasOriginalCalledReason(&'a [u8]), // from BroadSoft with plain value, tag=38, type=string
	VsaBwasRedirectingNumber(&'a [u8]), // from BroadSoft with plain value, tag=39, type=string
	VsaBwasRedirectingPresentationIndic(&'a [u8]), // from BroadSoft with plain value, tag=40, type=string
	VsaBwasRedirectingReason(&'a [u8]), // from BroadSoft with plain value, tag=41, type=string
	VsaBwasChargeIndic(&'a [u8]), // from BroadSoft with plain value, tag=42, type=string
	VsaBwasTypeOfNetwork(&'a [u8]), // from BroadSoft with plain value, tag=43, type=string
	VsaBwasVpCallingInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=44, type=string
	VsaBwasLocalCallid(&'a [u8]), // from BroadSoft with plain value, tag=45, type=string
	VsaBwasRemoteCallid(&'a [u8]), // from BroadSoft with plain value, tag=46, type=string
	VsaBwasCallingPartyCategory(&'a [u8]), // from BroadSoft with plain value, tag=47, type=string
	VsaBwasConferenceInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=48, type=string
	VsaBwasConferenceCallid(&'a [u8]), // from BroadSoft with plain value, tag=49, type=string
	VsaBwasConferenceTo(&'a [u8]), // from BroadSoft with plain value, tag=50, type=string
	VsaBwasConferenceFrom(&'a [u8]), // from BroadSoft with plain value, tag=51, type=string
	VsaBwasConferenceId(&'a [u8]), // from BroadSoft with plain value, tag=52, type=string
	VsaBwasConferenceRole(&'a [u8]), // from BroadSoft with plain value, tag=53, type=string
	VsaBwasConferenceBridge(&'a [u8]), // from BroadSoft with plain value, tag=54, type=string
	VsaBwasConferenceOwner(&'a [u8]), // from BroadSoft with plain value, tag=55, type=string
	VsaBwasConferenceOwnerDn(&'a [u8]), // from BroadSoft with plain value, tag=56, type=string
	VsaBwasConferenceTitle(&'a [u8]), // from BroadSoft with plain value, tag=57, type=string
	VsaBwasConferenceProjectCode(&'a [u8]), // from BroadSoft with plain value, tag=58, type=string
	VsaBwasChargingVectorKey(&'a [u8]), // from BroadSoft with plain value, tag=59, type=string
	VsaBwasChargingVectionCreator(&'a [u8]), // from BroadSoft with plain value, tag=60, type=string
	VsaBwasChargingVectionOrig(&'a [u8]), // from BroadSoft with plain value, tag=61, type=string
	VsaBwasChargingVectionTerm(&'a [u8]), // from BroadSoft with plain value, tag=62, type=string
	VsaBwasAccPerCallInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=63, type=string
	VsaBwasAccPerCallFacResult(&'a [u8]), // from BroadSoft with plain value, tag=64, type=string
	VsaBwasAcbActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=65, type=string
	VsaBwasAcbActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=66, type=string
	VsaBwasAcbDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=67, type=string
	VsaBwasAcbDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=68, type=string
	VsaBwasCallParkInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=69, type=string
	VsaBwasCallParkFacResult(&'a [u8]), // from BroadSoft with plain value, tag=70, type=string
	VsaBwasCallParkRetrInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=71, type=string
	VsaBwasCallParkRetrFacResult(&'a [u8]), // from BroadSoft with plain value, tag=72, type=string
	VsaBwasCallPickupInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=73, type=string
	VsaBwasCallPickupFacResult(&'a [u8]), // from BroadSoft with plain value, tag=74, type=string
	VsaBwasDirectedCallPickupInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=75, type=string
	VsaBwasDirectedCallPickupFacResult(&'a [u8]), // from BroadSoft with plain value, tag=76, type=string
	VsaBwasDpubiInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=77, type=string
	VsaBwasDpubiFacResult(&'a [u8]), // from BroadSoft with plain value, tag=78, type=string
	VsaBwasCancelCwtPerCallInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=79, type=string
	VsaBwasCancelCwtPerCallFacResult(&'a [u8]), // from BroadSoft with plain value, tag=80, type=string
	VsaBwasCfaActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=81, type=string
	VsaBwasCfaActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=82, type=string
	VsaBwasCfaDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=83, type=string
	VsaBwasCfaDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=84, type=string
	VsaBwasCfbActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=85, type=string
	VsaBwasCfbActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=86, type=string
	VsaBwasCfbDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=87, type=string
	VsaBwasCfbDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=88, type=string
	VsaBwasCfnaActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=89, type=string
	VsaBwasCfnaActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=90, type=string
	VsaBwasCfnaDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=91, type=string
	VsaBwasCfnaDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=92, type=string
	VsaBwasClidDeliveryPerCallInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=93, type=string
	VsaBwasClidDeliveryPerCallFacResult(&'a [u8]), // from BroadSoft with plain value, tag=94, type=string
	VsaBwasClidBlockingPerCallInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=95, type=string
	VsaBwasClidBlockingPerCallFacResult(&'a [u8]), // from BroadSoft with plain value, tag=96, type=string
	VsaBwasCotInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=97, type=string
	VsaBwasCotFacResult(&'a [u8]), // from BroadSoft with plain value, tag=98, type=string
	VsaBwasDirectVmXferInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=99, type=string
	VsaBwasDirectVmXferFacResult(&'a [u8]), // from BroadSoft with plain value, tag=100, type=string
	VsaBwasDndActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=101, type=string
	VsaBwasDndActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=102, type=string
	VsaBwasDndDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=103, type=string
	VsaBwasDndDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=104, type=string
	VsaBwasSacLockInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=105, type=string
	VsaBwasSacLockFacResult(&'a [u8]), // from BroadSoft with plain value, tag=106, type=string
	VsaBwasSacUnlockInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=107, type=string
	VsaBwasSacUnlockFacResult(&'a [u8]), // from BroadSoft with plain value, tag=108, type=string
	VsaBwasFlashCallHoldInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=109, type=string
	VsaBwasFlashCallHoldFacResult(&'a [u8]), // from BroadSoft with plain value, tag=110, type=string
	VsaBwasLastNumberRedialInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=111, type=string
	VsaBwasLastNumberRedialFacResult(&'a [u8]), // from BroadSoft with plain value, tag=112, type=string
	VsaBwasReturnCallInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=113, type=string
	VsaBwasReturnCallFacResult(&'a [u8]), // from BroadSoft with plain value, tag=114, type=string
	VsaBwasSd100ProgrammingInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=115, type=string
	VsaBwasSd100ProgrammingFacResult(&'a [u8]), // from BroadSoft with plain value, tag=116, type=string
	VsaBwasSd8ProgrammingInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=117, type=string
	VsaBwasSd8ProgrammingFacResult(&'a [u8]), // from BroadSoft with plain value, tag=118, type=string
	VsaBwasClearMwiInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=119, type=string
	VsaBwasClearMwiFacResult(&'a [u8]), // from BroadSoft with plain value, tag=120, type=string
	VsaBwasUserid(&'a [u8]), // from BroadSoft with plain value, tag=121, type=string
	VsaBwasOtherPartyName(&'a [u8]), // from BroadSoft with plain value, tag=122, type=string
	VsaBwasOtherPartyNamePresIndic(&'a [u8]), // from BroadSoft with plain value, tag=123, type=string
	VsaBwasMohDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=125, type=string
	VsaBwasPushToTalkInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=126, type=string
	VsaBwasPushToTalkFacResult(&'a [u8]), // from BroadSoft with plain value, tag=127, type=string
	VsaBwasHotelingInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=128, type=string
	VsaBwasHotelingGroup(&'a [u8]), // from BroadSoft with plain value, tag=129, type=string
	VsaBwasHotelingUserid(&'a [u8]), // from BroadSoft with plain value, tag=130, type=string
	VsaBwasHotelingUserNumber(&'a [u8]), // from BroadSoft with plain value, tag=131, type=string
	VsaBwasHotelingGroupNumber(&'a [u8]), // from BroadSoft with plain value, tag=132, type=string
	VsaBwasDiversionInhibitorInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=133, type=string
	VsaBwasDiversionInhibitorFacResult(&'a [u8]), // from BroadSoft with plain value, tag=134, type=string
	VsaBwasTrunkGroupName(&'a [u8]), // from BroadSoft with plain value, tag=135, type=string
	VsaBwasSpare136(&'a [u8]), // from BroadSoft with plain value, tag=136, type=string
	VsaBwasInstantgroupcallInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=137, type=string
	VsaBwasInstantgroupcallPushtotalk(&'a [u8]), // from BroadSoft with plain value, tag=138, type=string
	VsaBwasInstantgroupcallRelatedCallid(&'a [u8]), // from BroadSoft with plain value, tag=139, type=string
	VsaBwasCustomringbackInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=140, type=string
	VsaBwasClidPermitted(&'a [u8]), // from BroadSoft with plain value, tag=141, type=string
	VsaBwasAhrInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=142, type=string
	VsaBwasAhrAction(&'a [u8]), // from BroadSoft with plain value, tag=143, type=string
	VsaBwasAccessNetworkInfo(&'a [u8]), // from BroadSoft with plain value, tag=144, type=string
	VsaBwasChargingFunctionAddresses(&'a [u8]), // from BroadSoft with plain value, tag=145, type=string
	VsaBwasChargeNumber(&'a [u8]), // from BroadSoft with plain value, tag=146, type=string
	VsaBwasRelatedCallid(&'a [u8]), // from BroadSoft with plain value, tag=147, type=string
	VsaBwasRelatedCallidReason(&'a [u8]), // from BroadSoft with plain value, tag=148, type=string
	VsaBwasTransferInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=149, type=string
	VsaBwasTransferResult(&'a [u8]), // from BroadSoft with plain value, tag=150, type=string
	VsaBwasTransferRelatedCallid(&'a [u8]), // from BroadSoft with plain value, tag=151, type=string
	VsaBwasTransferType(&'a [u8]), // from BroadSoft with plain value, tag=152, type=string
	VsaBwasConfStartTime(&'a [u8]), // from BroadSoft with plain value, tag=153, type=string
	VsaBwasConfStopTime(&'a [u8]), // from BroadSoft with plain value, tag=154, type=string
	VsaBwasConfId(&'a [u8]), // from BroadSoft with plain value, tag=155, type=string
	VsaBwasConfType(&'a [u8]), // from BroadSoft with plain value, tag=156, type=string
	VsaBwasCodecUsage(&'a [u8]), // from BroadSoft with plain value, tag=157, type=string
	VsaBwasVmbActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=158, type=string
	VsaBwasVmbActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=159, type=string
	VsaBwasVmbDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=160, type=string
	VsaBwasVmbDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=161, type=string
	VsaBwasVmnaActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=162, type=string
	VsaBwasVmnaActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=163, type=string
	VsaBwasVmnaDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=164, type=string
	VsaBwasVmnaDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=165, type=string
	VsaBwasVmaActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=166, type=string
	VsaBwasVmaActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=167, type=string
	VsaBwasVmaDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=168, type=string
	VsaBwasVmaDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=169, type=string
	VsaBwasNoAnswerSetInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=170, type=string
	VsaBwasNoAnswerSetFacResult(&'a [u8]), // from BroadSoft with plain value, tag=171, type=string
	VsaBwasClidBlockingActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=172, type=string
	VsaBwasClidBlockingActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=173, type=string
	VsaBwasClidBlockingDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=174, type=string
	VsaBwasClidBlockingDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=175, type=string
	VsaBwasCallWaitingActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=176, type=string
	VsaBwasCallWaitingActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=177, type=string
	VsaBwasCallWaitingDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=178, type=string
	VsaBwasCallWaitingDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=179, type=string
	VsaBwasFaxMessaging(&'a [u8]), // from BroadSoft with plain value, tag=180, type=string
	VsaBwasTsdDigits(&'a [u8]), // from BroadSoft with plain value, tag=181, type=string
	VsaBwasTrunkGroupInfo(&'a [u8]), // from BroadSoft with plain value, tag=182, type=string
	VsaBwasRecallType(&'a [u8]), // from BroadSoft with plain value, tag=183, type=string
	VsaBwasCfnrcActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=184, type=string
	VsaBwasCfnrcActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=185, type=string
	VsaBwasCfnrcDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=186, type=string
	VsaBwasCfnrcDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=187, type=string
	VsaBwasQ850Cause(&'a [u8]), // from BroadSoft with plain value, tag=188, type=string
	VsaBwasDialedDigitsContext(&'a [u8]), // from BroadSoft with plain value, tag=189, type=string
	VsaBwasCalledNumberContext(&'a [u8]), // from BroadSoft with plain value, tag=190, type=string
	VsaBwasNetworkTranslatedNumberContext(&'a [u8]), // from BroadSoft with plain value, tag=191, type=string
	VsaBwasCallingNumberContext(&'a [u8]), // from BroadSoft with plain value, tag=192, type=string
	VsaBwasOriginalCalledNumberContext(&'a [u8]), // from BroadSoft with plain value, tag=193, type=string
	VsaBwasRedirectingNumberContext(&'a [u8]), // from BroadSoft with plain value, tag=194, type=string
	VsaBwasLocationControlActResult(&'a [u8]), // from BroadSoft with plain value, tag=195, type=string
	VsaBwasLocationControlDeactResult(&'a [u8]), // from BroadSoft with plain value, tag=196, type=string
	VsaBwasCallRetrieveResult(&'a [u8]), // from BroadSoft with plain value, tag=197, type=string
	VsaBwasRoutingNumber(&'a [u8]), // from BroadSoft with plain value, tag=198, type=string
	VsaBwasOriginationMethod(&'a [u8]), // from BroadSoft with plain value, tag=199, type=string
	VsaBwasCallParkedInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=200, type=string
	VsaBwasBaRelatedCallId(&'a [u8]), // from BroadSoft with plain value, tag=201, type=string
	VsaBwasAcrActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=202, type=string
	VsaBwasAcrActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=203, type=string
	VsaBwasAcrDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=204, type=string
	VsaBwasAcrDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=205, type=string
	VsaBwasOutsideAccessCode(&'a [u8]), // from BroadSoft with plain value, tag=206, type=string
	VsaBwasPrimaryDeviceLinePort(&'a [u8]), // from BroadSoft with plain value, tag=207, type=string
	VsaBwasCalledAssertedIdentity(&'a [u8]), // from BroadSoft with plain value, tag=208, type=string
	VsaBwasCalledAssertedPresIndicator(&'a [u8]), // from BroadSoft with plain value, tag=209, type=string
	VsaBwasSdp(&'a [u8]), // from BroadSoft with plain value, tag=210, type=string
	VsaBwasMediaInitiatorFlag(&'a [u8]), // from BroadSoft with plain value, tag=211, type=string
	VsaBwasSdpOfferTimestamp(&'a [u8]), // from BroadSoft with plain value, tag=212, type=string
	VsaBwasSdpAnswerTimestamp(&'a [u8]), // from BroadSoft with plain value, tag=213, type=string
	VsaBwasEarlyMediaSdp(&'a [u8]), // from BroadSoft with plain value, tag=214, type=string
	VsaBwasEarlyMediaInitiatorFlag(&'a [u8]), // from BroadSoft with plain value, tag=215, type=string
	VsaBwasBodyContentType(&'a [u8]), // from BroadSoft with plain value, tag=216, type=string
	VsaBwasBodyContentLength(&'a [u8]), // from BroadSoft with plain value, tag=217, type=string
	VsaBwasBodyContentDisposition(&'a [u8]), // from BroadSoft with plain value, tag=218, type=string
	VsaBwasBodyOriginator(&'a [u8]), // from BroadSoft with plain value, tag=219, type=string
	VsaBwasSipErrorCode(&'a [u8]), // from BroadSoft with plain value, tag=220, type=string
	VsaBwasOtherinfoinpcv(&'a [u8]), // from BroadSoft with plain value, tag=221, type=string
	VsaBwasReceivedCallingNumber(&'a [u8]), // from BroadSoft with plain value, tag=222, type=string
	VsaBwasCustomringbackMediaSelection(&'a [u8]), // from BroadSoft with plain value, tag=223, type=string
	VsaBwasAocType(&'a [u8]), // from BroadSoft with plain value, tag=224, type=string
	VsaBwasAocCharge(&'a [u8]), // from BroadSoft with plain value, tag=225, type=string
	VsaBwasAocCurrency(&'a [u8]), // from BroadSoft with plain value, tag=226, type=string
	VsaBwasAocTime(&'a [u8]), // from BroadSoft with plain value, tag=227, type=string
	VsaBwasAocSum(&'a [u8]), // from BroadSoft with plain value, tag=228, type=string
	VsaBwasAocActivationTime(&'a [u8]), // from BroadSoft with plain value, tag=229, type=string
	VsaBwasAocResult(&'a [u8]), // from BroadSoft with plain value, tag=230, type=string
	VsaBwasAsCallType(&'a [u8]), // from BroadSoft with plain value, tag=231, type=string
	VsaBwasScfActInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=232, type=string
	VsaBwasScfActFacResult(&'a [u8]), // from BroadSoft with plain value, tag=233, type=string
	VsaBwasScfDeactInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=234, type=string
	VsaBwasScfDeactFacResult(&'a [u8]), // from BroadSoft with plain value, tag=235, type=string
	VsaBwasCfaInterInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=236, type=string
	VsaBwasCfaInterFacResult(&'a [u8]), // from BroadSoft with plain value, tag=237, type=string
	VsaBwasCfnaInterInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=238, type=string
	VsaBwasCfnaInterFacResult(&'a [u8]), // from BroadSoft with plain value, tag=239, type=string
	VsaBwasCfbInterInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=240, type=string
	VsaBwasCfbInterFacResult(&'a [u8]), // from BroadSoft with plain value, tag=241, type=string
	VsaBwasCbfAuthCode(&'a [u8]), // from BroadSoft with plain value, tag=242, type=string
	VsaBwasCallBridgeResult(&'a [u8]), // from BroadSoft with plain value, tag=243, type=string
	VsaBwasReturnCallNumberDeletionInvokeTime(&'a [u8]), // from BroadSoft with plain value, tag=244, type=string
	VsaBwasReturnCallNumberDeletionFacResult(&'a [u8]), // from BroadSoft with plain value, tag=245, type=string
	VsaBwasPrepaidStatus(&'a [u8]), // from BroadSoft with plain value, tag=246, type=string
	VsaBwasConfigurableClid(&'a [u8]), // from BroadSoft with plain value, tag=247, type=string
	VsaBwasCallCenterNightServiceActResult(&'a [u8]), // from BroadSoft with plain value, tag=248, type=string
	VsaBwasCallCenterNightServiceDeactResult(&'a [u8]), // from BroadSoft with plain value, tag=249, type=string
	VsaBwasCallCenterForcedForwardingActResult(&'a [u8]), // from BroadSoft with plain value, tag=250, type=string
	VsaBwasCallCenterForcedForwardingDeactResult(&'a [u8]), // from BroadSoft with plain value, tag=251, type=string
	VsaBwasCallCenterOutgoingCallFacResult(&'a [u8]), // from BroadSoft with plain value, tag=252, type=string
	VsaBwasCallCenterOutgoingPersonalCallFacResult(&'a [u8]), // from BroadSoft with plain value, tag=253, type=string
	VsaBwasCallCenterOutgoingPhoneNumber(&'a [u8]), // from BroadSoft with plain value, tag=254, type=string
	VsaBroadsoftAttr255(&'a [u8]), // from BroadSoft with plain value, tag=255, type=string
	VsaBrocadeAuthRole(&'a [u8]), // from Brocade with plain value, tag=1, type=string
	VsaBrocadeAvpairs1(&'a [u8]), // from Brocade with plain value, tag=2, type=string
	VsaBrocadeAvpairs2(&'a [u8]), // from Brocade with plain value, tag=3, type=string
	VsaBrocadeAvpairs3(&'a [u8]), // from Brocade with plain value, tag=4, type=string
	VsaBrocadeAvpairs4(&'a [u8]), // from Brocade with plain value, tag=5, type=string
	VsaBrocadePasswdExpirydate(&'a [u8]), // from Brocade with plain value, tag=6, type=string
	VsaBrocadePasswdWarnperiod(&'a [u8]), // from Brocade with plain value, tag=7, type=string
	VsaSkyWifiApId(u32), // from BSkyB with plain value, tag=1, type=integer
	VsaSkyWifiServiceId(u32), // from BSkyB with plain value, tag=2, type=integer
	VsaSkyWifiFilterProfile(&'a [u8]), // from BSkyB with plain value, tag=3, type=string
	VsaSkyWifiBillingClass(&'a [u8]), // from BSkyB with plain value, tag=4, type=octets
	VsaSkyWifiProviderId(u32), // from BSkyB with plain value, tag=5, type=integer
	VsaSkyWifiCredentials(&'a [u8]), // from BSkyB with plain value, tag=6, type=string
	VsaSidAuth(&'a [u8]), // from BT with plain value, tag=1, type=string
	VsaCablelabsReserved(&'a [u8]), // from CableLabs with plain value, tag=0, type=octets
	VsaCablelabsEventMessage(&'a [u8]), // from CableLabs with plain value, tag=1, type=octets
	VsaCablelabsMtaEndpointName(&'a [u8]), // from CableLabs with plain value, tag=3, type=string
	VsaCablelabsCallingPartyNumber(&'a [u8]), // from CableLabs with plain value, tag=4, type=string
	VsaCablelabsCalledPartyNumber(&'a [u8]), // from CableLabs with plain value, tag=5, type=string
	VsaCablelabsDatabaseId(&'a [u8]), // from CableLabs with plain value, tag=6, type=string
	VsaCablelabsQueryType(cablelabs::CablelabsQueryType), // from CableLabs with Enum value, tag=7, type=integer
	VsaCablelabsReturnedNumber(&'a [u8]), // from CableLabs with plain value, tag=9, type=string
	VsaCablelabsCallTerminationCause(&'a [u8]), // from CableLabs with plain value, tag=11, type=octets
	VsaCablelabsRelatedCallBillingCrlId(&'a [u8]), // from CableLabs with plain value, tag=13, type=octets
	VsaCablelabsFirstCallCallingPartyNum(&'a [u8]), // from CableLabs with plain value, tag=14, type=string
	VsaCablelabsSecondCallCallingPartyNum(&'a [u8]), // from CableLabs with plain value, tag=15, type=string
	VsaCablelabsChargeNumber(&'a [u8]), // from CableLabs with plain value, tag=16, type=string
	VsaCablelabsForwardedNumber(&'a [u8]), // from CableLabs with plain value, tag=17, type=string
	VsaCablelabsServiceName(&'a [u8]), // from CableLabs with plain value, tag=18, type=string
	VsaCablelabsIntlCode(&'a [u8]), // from CableLabs with plain value, tag=20, type=string
	VsaCablelabsDialAroundCode(&'a [u8]), // from CableLabs with plain value, tag=21, type=string
	VsaCablelabsLocationRoutingNumber(&'a [u8]), // from CableLabs with plain value, tag=22, type=string
	VsaCablelabsCarrierIdentificationCode(&'a [u8]), // from CableLabs with plain value, tag=23, type=string
	VsaCablelabsTrunkGroupId(&'a [u8]), // from CableLabs with plain value, tag=24, type=octets
	VsaCablelabsRoutingNumber(&'a [u8]), // from CableLabs with plain value, tag=25, type=string
	VsaCablelabsMtaUdpPortnum(u32), // from CableLabs with plain value, tag=26, type=integer
	VsaCablelabsChannelState(cablelabs::CablelabsChannelState), // from CableLabs with Enum value, tag=29, type=integer
	VsaCablelabsSfId(u32), // from CableLabs with plain value, tag=30, type=integer
	VsaCablelabsErrorDescription(&'a [u8]), // from CableLabs with plain value, tag=31, type=string
	VsaCablelabsQosDescriptor(&'a [u8]), // from CableLabs with plain value, tag=32, type=octets
	VsaCablelabsDirectionIndicator(cablelabs::CablelabsDirectionIndicator), // from CableLabs with Enum value, tag=37, type=integer
	VsaCablelabsTimeAdjustment(&'a [u8]), // from CableLabs with plain value, tag=38, type=octets
	VsaCablelabsSdpUpstream(&'a [u8]), // from CableLabs with plain value, tag=39, type=string
	VsaCablelabsSdpDownstream(&'a [u8]), // from CableLabs with plain value, tag=40, type=string
	VsaCablelabsUserInput(&'a [u8]), // from CableLabs with plain value, tag=41, type=string
	VsaCablelabsTranslationInput(&'a [u8]), // from CableLabs with plain value, tag=42, type=string
	VsaCablelabsRedirectedFromInfo(&'a [u8]), // from CableLabs with plain value, tag=43, type=octets
	VsaCablelabsElectronicSurveillanceInd(&'a [u8]), // from CableLabs with plain value, tag=44, type=octets
	VsaCablelabsRedirectedFromPartyNumber(&'a [u8]), // from CableLabs with plain value, tag=45, type=string
	VsaCablelabsRedirectedToPartyNumber(&'a [u8]), // from CableLabs with plain value, tag=46, type=string
	VsaCablelabsElSurveillanceDfSecurity(&'a [u8]), // from CableLabs with plain value, tag=47, type=octets
	VsaCablelabsCccId(&'a [u8]), // from CableLabs with plain value, tag=48, type=octets
	VsaCablelabsFinancialEntityId(&'a [u8]), // from CableLabs with plain value, tag=49, type=string
	VsaCablelabsFlowDirection(cablelabs::CablelabsFlowDirection), // from CableLabs with Enum value, tag=50, type=integer
	VsaCablelabsSignalType(cablelabs::CablelabsSignalType), // from CableLabs with Enum value, tag=51, type=integer
	VsaCablelabsAlertingSignal(cablelabs::CablelabsAlertingSignal), // from CableLabs with Enum value, tag=52, type=integer
	VsaCablelabsSubjectAudibleSignal(u32), // from CableLabs with plain value, tag=53, type=integer
	VsaCablelabsTerminalDisplayInfo(&'a [u8]), // from CableLabs with plain value, tag=54, type=octets
	VsaCablelabsSwitchHookFlash(&'a [u8]), // from CableLabs with plain value, tag=55, type=string
	VsaCablelabsDialedDigits(&'a [u8]), // from CableLabs with plain value, tag=56, type=string
	VsaCablelabsMiscSignalingInformation(&'a [u8]), // from CableLabs with plain value, tag=57, type=string
	VsaCablelabsAmOpaqueData(cablelabs::CablelabsAmOpaqueData), // from CableLabs with Enum value, tag=61, type=integer
	VsaCablelabsSubscriberId(u32), // from CableLabs with plain value, tag=62, type=integer
	VsaCablelabsVolumeUsageLimit(u32), // from CableLabs with plain value, tag=63, type=integer
	VsaCablelabsGateUsageInfo(u32), // from CableLabs with plain value, tag=64, type=integer
	VsaCablelabsElementRequestingQos(cablelabs::CablelabsElementRequestingQos), // from CableLabs with Enum value, tag=65, type=integer
	VsaCablelabsQosReleaseReason(cablelabs::CablelabsQosReleaseReason), // from CableLabs with Enum value, tag=66, type=integer
	VsaCablelabsPolicyDeniedReason(cablelabs::CablelabsPolicyDeniedReason), // from CableLabs with Enum value, tag=67, type=integer
	VsaCablelabsPolicyDeletedReason(cablelabs::CablelabsPolicyDeletedReason), // from CableLabs with Enum value, tag=68, type=integer
	VsaCablelabsPolicyUpdateReason(cablelabs::CablelabsPolicyUpdateReason), // from CableLabs with Enum value, tag=69, type=integer
	VsaCablelabsPolicyDecisionStatus(cablelabs::CablelabsPolicyDecisionStatus), // from CableLabs with Enum value, tag=70, type=integer
	VsaCablelabsApplicationManagerId(u32), // from CableLabs with plain value, tag=71, type=integer
	VsaCablelabsTimeUsageLimit(u32), // from CableLabs with plain value, tag=72, type=integer
	VsaCablelabsGateTimeInfo(u32), // from CableLabs with plain value, tag=73, type=integer
	VsaCablelabsIpv6SubscriberId(Ipv6Addr), // from CableLabs with plain value, tag=74, type=ipv6addr
	VsaCablelabsUserId(&'a [u8]), // from CableLabs with plain value, tag=75, type=string
	VsaCablelabsAccountCode(&'a [u8]), // from CableLabs with plain value, tag=80, type=string
	VsaCablelabsAuthorizationCode(&'a [u8]), // from CableLabs with plain value, tag=81, type=string
	VsaCablelabsJurisdictionInfoParameter(&'a [u8]), // from CableLabs with plain value, tag=82, type=string
	VsaCablelabsCalledPartyNpSource(u32), // from CableLabs with plain value, tag=83, type=integer
	VsaCablelabsCallingPartyNpSource(u32), // from CableLabs with plain value, tag=84, type=integer
	VsaCablelabsPortedInCallingNumber(u32), // from CableLabs with plain value, tag=85, type=integer
	VsaCablelabsPortedInCalledNumber(u32), // from CableLabs with plain value, tag=86, type=integer
	VsaCablelabsBillingType(u32), // from CableLabs with plain value, tag=87, type=integer
	VsaCablelabsSignaledToNumber(&'a [u8]), // from CableLabs with plain value, tag=88, type=string
	VsaCablelabsSignaledFromNumber(&'a [u8]), // from CableLabs with plain value, tag=89, type=string
	VsaCablelabsCommunicatingParty(&'a [u8]), // from CableLabs with plain value, tag=90, type=octets
	VsaCablelabsJoinedParty(&'a [u8]), // from CableLabs with plain value, tag=91, type=octets
	VsaCablelabsRemovedParty(&'a [u8]), // from CableLabs with plain value, tag=92, type=octets
	VsaCablelabsRtcpData(&'a [u8]), // from CableLabs with plain value, tag=93, type=string
	VsaCablelabsLocalXrBlock(&'a [u8]), // from CableLabs with plain value, tag=94, type=string
	VsaCablelabsRemoteXrBlock(&'a [u8]), // from CableLabs with plain value, tag=95, type=string
	VsaSurveillanceStopType(u32), // from CableLabs with plain value, tag=96, type=integer
	VsaSurveillanceStopDestination(u32), // from CableLabs with plain value, tag=97, type=integer
	VsaRelatedIcid(&'a [u8]), // from CableLabs with plain value, tag=98, type=string
	VsaCabletronProtocolEnable(cabletron::CabletronProtocolEnable), // from Cabletron with Enum value, tag=201, type=integer
	VsaCabletronProtocolCallable(cabletron::CabletronProtocolCallable), // from Cabletron with Enum value, tag=202, type=integer
	VsaCamiantMiRole(&'a [u8]), // from Camiant with plain value, tag=1, type=string
	VsaCamiantSuiRole(camiant::CamiantSuiRole), // from Camiant with Enum value, tag=2, type=integer
	VsaCamiantMiScope(&'a [u8]), // from Camiant with plain value, tag=3, type=string
	VsaChillispotMaxInputOctets(u32), // from ChilliSpot with plain value, tag=1, type=integer
	VsaChillispotMaxOutputOctets(u32), // from ChilliSpot with plain value, tag=2, type=integer
	VsaChillispotMaxTotalOctets(u32), // from ChilliSpot with plain value, tag=3, type=integer
	VsaChillispotBandwidthMaxUp(u32), // from ChilliSpot with plain value, tag=4, type=integer
	VsaChillispotBandwidthMaxDown(u32), // from ChilliSpot with plain value, tag=5, type=integer
	VsaChillispotConfig(&'a [u8]), // from ChilliSpot with plain value, tag=6, type=string
	VsaChillispotLang(&'a [u8]), // from ChilliSpot with plain value, tag=7, type=string
	VsaChillispotVersion(&'a [u8]), // from ChilliSpot with plain value, tag=8, type=string
	VsaChillispotOriginalurl(&'a [u8]), // from ChilliSpot with plain value, tag=9, type=string
	VsaChillispotUamAllowed(&'a [u8]), // from ChilliSpot with plain value, tag=100, type=string
	VsaChillispotMacAllowed(&'a [u8]), // from ChilliSpot with plain value, tag=101, type=string
	VsaChillispotInterval(u32), // from ChilliSpot with plain value, tag=102, type=integer
	VsaCiscoAvpair(&'a [u8]), // from Cisco with plain value, tag=1, type=string
	VsaCiscoNasPort(&'a [u8]), // from Cisco with plain value, tag=2, type=string
	VsaCiscoFaxAccountIdOrigin(&'a [u8]), // from Cisco with plain value, tag=3, type=string
	VsaCiscoFaxMsgId(&'a [u8]), // from Cisco with plain value, tag=4, type=string
	VsaCiscoFaxPages(&'a [u8]), // from Cisco with plain value, tag=5, type=string
	VsaCiscoFaxCoverpageFlag(&'a [u8]), // from Cisco with plain value, tag=6, type=string
	VsaCiscoFaxModemTime(&'a [u8]), // from Cisco with plain value, tag=7, type=string
	VsaCiscoFaxConnectSpeed(&'a [u8]), // from Cisco with plain value, tag=8, type=string
	VsaCiscoFaxRecipientCount(&'a [u8]), // from Cisco with plain value, tag=9, type=string
	VsaCiscoFaxProcessAbortFlag(&'a [u8]), // from Cisco with plain value, tag=10, type=string
	VsaCiscoFaxDsnAddress(&'a [u8]), // from Cisco with plain value, tag=11, type=string
	VsaCiscoFaxDsnFlag(&'a [u8]), // from Cisco with plain value, tag=12, type=string
	VsaCiscoFaxMdnAddress(&'a [u8]), // from Cisco with plain value, tag=13, type=string
	VsaCiscoFaxMdnFlag(&'a [u8]), // from Cisco with plain value, tag=14, type=string
	VsaCiscoFaxAuthStatus(&'a [u8]), // from Cisco with plain value, tag=15, type=string
	VsaCiscoEmailServerAddress(&'a [u8]), // from Cisco with plain value, tag=16, type=string
	VsaCiscoEmailServerAckFlag(&'a [u8]), // from Cisco with plain value, tag=17, type=string
	VsaCiscoGatewayId(&'a [u8]), // from Cisco with plain value, tag=18, type=string
	VsaCiscoCallType(&'a [u8]), // from Cisco with plain value, tag=19, type=string
	VsaCiscoPortUsed(&'a [u8]), // from Cisco with plain value, tag=20, type=string
	VsaCiscoAbortCause(&'a [u8]), // from Cisco with plain value, tag=21, type=string
	VsaH323RemoteAddress(&'a [u8]), // from Cisco with plain value, tag=23, type=string
	VsaH323ConfId(&'a [u8]), // from Cisco with plain value, tag=24, type=string
	VsaH323SetupTime(&'a [u8]), // from Cisco with plain value, tag=25, type=string
	VsaH323CallOrigin(&'a [u8]), // from Cisco with plain value, tag=26, type=string
	VsaH323CallType(&'a [u8]), // from Cisco with plain value, tag=27, type=string
	VsaH323ConnectTime(&'a [u8]), // from Cisco with plain value, tag=28, type=string
	VsaH323DisconnectTime(&'a [u8]), // from Cisco with plain value, tag=29, type=string
	VsaH323DisconnectCause(&'a [u8]), // from Cisco with plain value, tag=30, type=string
	VsaH323VoiceQuality(&'a [u8]), // from Cisco with plain value, tag=31, type=string
	VsaH323GwId(&'a [u8]), // from Cisco with plain value, tag=33, type=string
	VsaH323IncomingConfId(&'a [u8]), // from Cisco with plain value, tag=35, type=string
	VsaCiscoPolicyUp(&'a [u8]), // from Cisco with plain value, tag=37, type=string
	VsaCiscoPolicyDown(&'a [u8]), // from Cisco with plain value, tag=38, type=string
	VsaSipConfId(&'a [u8]), // from Cisco with plain value, tag=100, type=string
	VsaH323CreditAmount(&'a [u8]), // from Cisco with plain value, tag=101, type=string
	VsaH323CreditTime(&'a [u8]), // from Cisco with plain value, tag=102, type=string
	VsaH323ReturnCode(&'a [u8]), // from Cisco with plain value, tag=103, type=string
	VsaH323PromptId(&'a [u8]), // from Cisco with plain value, tag=104, type=string
	VsaH323TimeAndDay(&'a [u8]), // from Cisco with plain value, tag=105, type=string
	VsaH323RedirectNumber(&'a [u8]), // from Cisco with plain value, tag=106, type=string
	VsaH323PreferredLang(&'a [u8]), // from Cisco with plain value, tag=107, type=string
	VsaH323RedirectIpAddress(&'a [u8]), // from Cisco with plain value, tag=108, type=string
	VsaH323BillingModel(&'a [u8]), // from Cisco with plain value, tag=109, type=string
	VsaH323Currency(&'a [u8]), // from Cisco with plain value, tag=110, type=string
	VsaSubscriber(&'a [u8]), // from Cisco with plain value, tag=111, type=string
	VsaGwRxdCdn(&'a [u8]), // from Cisco with plain value, tag=112, type=string
	VsaGwFinalXlatedCdn(&'a [u8]), // from Cisco with plain value, tag=113, type=string
	VsaRemoteMediaAddress(&'a [u8]), // from Cisco with plain value, tag=114, type=string
	VsaReleaseSource(&'a [u8]), // from Cisco with plain value, tag=115, type=string
	VsaGwRxdCgn(&'a [u8]), // from Cisco with plain value, tag=116, type=string
	VsaGwFinalXlatedCgn(&'a [u8]), // from Cisco with plain value, tag=117, type=string
	VsaCallId(&'a [u8]), // from Cisco with plain value, tag=141, type=string
	VsaSessionProtocol(&'a [u8]), // from Cisco with plain value, tag=142, type=string
	VsaMethod(&'a [u8]), // from Cisco with plain value, tag=143, type=string
	VsaPrevHopVia(&'a [u8]), // from Cisco with plain value, tag=144, type=string
	VsaPrevHopIp(&'a [u8]), // from Cisco with plain value, tag=145, type=string
	VsaIncomingReqUri(&'a [u8]), // from Cisco with plain value, tag=146, type=string
	VsaOutgoingReqUri(&'a [u8]), // from Cisco with plain value, tag=147, type=string
	VsaNextHopIp(&'a [u8]), // from Cisco with plain value, tag=148, type=string
	VsaNextHopDn(&'a [u8]), // from Cisco with plain value, tag=149, type=string
	VsaSipHdr(&'a [u8]), // from Cisco with plain value, tag=150, type=string
	VsaDspId(&'a [u8]), // from Cisco with plain value, tag=151, type=string
	VsaCiscoMultilinkId(u32), // from Cisco with plain value, tag=187, type=integer
	VsaCiscoNumInMultilink(u32), // from Cisco with plain value, tag=188, type=integer
	VsaCiscoPreInputOctets(u32), // from Cisco with plain value, tag=190, type=integer
	VsaCiscoPreOutputOctets(u32), // from Cisco with plain value, tag=191, type=integer
	VsaCiscoPreInputPackets(u32), // from Cisco with plain value, tag=192, type=integer
	VsaCiscoPreOutputPackets(u32), // from Cisco with plain value, tag=193, type=integer
	VsaCiscoMaximumTime(u32), // from Cisco with plain value, tag=194, type=integer
	VsaCiscoDisconnectCause(cisco::CiscoDisconnectCause), // from Cisco with Enum value, tag=195, type=integer
	VsaCiscoDataRate(u32), // from Cisco with plain value, tag=197, type=integer
	VsaCiscoPresessionTime(u32), // from Cisco with plain value, tag=198, type=integer
	VsaCiscoPwLifetime(u32), // from Cisco with plain value, tag=208, type=integer
	VsaCiscoIpDirect(u32), // from Cisco with plain value, tag=209, type=integer
	VsaCiscoPppVjSlotComp(u32), // from Cisco with plain value, tag=210, type=integer
	VsaCiscoPppAsyncMap(u32), // from Cisco with plain value, tag=212, type=integer
	VsaCiscoIpPoolDefinition(&'a [u8]), // from Cisco with plain value, tag=217, type=string
	VsaCiscoAssignIpPool(u32), // from Cisco with plain value, tag=218, type=integer
	VsaCiscoRouteIp(u32), // from Cisco with plain value, tag=228, type=integer
	VsaCiscoLinkCompression(u32), // from Cisco with plain value, tag=233, type=integer
	VsaCiscoTargetUtil(u32), // from Cisco with plain value, tag=234, type=integer
	VsaCiscoMaximumChannels(u32), // from Cisco with plain value, tag=235, type=integer
	VsaCiscoDataFilter(u32), // from Cisco with plain value, tag=242, type=integer
	VsaCiscoCallFilter(u32), // from Cisco with plain value, tag=243, type=integer
	VsaCiscoIdleLimit(u32), // from Cisco with plain value, tag=244, type=integer
	VsaCiscoSubscriberPassword(&'a [u8]), // from Cisco with plain value, tag=249, type=string
	VsaCiscoAccountInfo(&'a [u8]), // from Cisco with plain value, tag=250, type=string
	VsaCiscoServiceInfo(&'a [u8]), // from Cisco with plain value, tag=251, type=string
	VsaCiscoCommandCode(&'a [u8]), // from Cisco with plain value, tag=252, type=string
	VsaCiscoControlInfo(&'a [u8]), // from Cisco with plain value, tag=253, type=string
	VsaCiscoXmitRate(u32), // from Cisco with plain value, tag=255, type=integer
	VsaAsaSimultaneousLogins(u32), // from Cisco-ASA with plain value, tag=2, type=integer
	VsaAsaPrimaryDns(Ipv4Addr), // from Cisco-ASA with plain value, tag=5, type=ipaddr
	VsaAsaSecondaryDns(Ipv4Addr), // from Cisco-ASA with plain value, tag=6, type=ipaddr
	VsaAsaPrimaryWins(Ipv4Addr), // from Cisco-ASA with plain value, tag=7, type=ipaddr
	VsaAsaSecondaryWins(Ipv4Addr), // from Cisco-ASA with plain value, tag=8, type=ipaddr
	VsaAsaSepCardAssignment(u32), // from Cisco-ASA with plain value, tag=9, type=integer
	VsaAsaTunnelingProtocols(u32), // from Cisco-ASA with plain value, tag=11, type=integer
	VsaAsaIpsecSecAssociation(&'a [u8]), // from Cisco-ASA with plain value, tag=12, type=string
	VsaAsaIpsecAuthentication(cisco_asa::AsaIpsecAuthentication), // from Cisco-ASA with Enum value, tag=13, type=integer
	VsaAsaBanner1(&'a [u8]), // from Cisco-ASA with plain value, tag=15, type=string
	VsaAsaIpsecAllowPasswdStore(cisco_asa::AsaIpsecAllowPasswdStore), // from Cisco-ASA with Enum value, tag=16, type=integer
	VsaAsaUseClientAddress(cisco_asa::AsaUseClientAddress), // from Cisco-ASA with Enum value, tag=17, type=integer
	VsaAsaPptpEncryption(u32), // from Cisco-ASA with plain value, tag=20, type=integer
	VsaAsaL2TpEncryption(u32), // from Cisco-ASA with plain value, tag=21, type=integer
	VsaAsaGroupPolicy(&'a [u8]), // from Cisco-ASA with plain value, tag=25, type=string
	VsaAsaIpsecSplitTunnelList(&'a [u8]), // from Cisco-ASA with plain value, tag=27, type=string
	VsaAsaIpsecDefaultDomain(&'a [u8]), // from Cisco-ASA with plain value, tag=28, type=string
	VsaAsaIpsecSplitDnsNames(&'a [u8]), // from Cisco-ASA with plain value, tag=29, type=string
	VsaAsaIpsecTunnelType(cisco_asa::AsaIpsecTunnelType), // from Cisco-ASA with Enum value, tag=30, type=integer
	VsaAsaIpsecModeConfig(cisco_asa::AsaIpsecModeConfig), // from Cisco-ASA with Enum value, tag=31, type=integer
	VsaAsaIpsecOverUdp(cisco_asa::AsaIpsecOverUdp), // from Cisco-ASA with Enum value, tag=34, type=integer
	VsaAsaIpsecOverUdpPort(u32), // from Cisco-ASA with plain value, tag=35, type=integer
	VsaAsaBanner2(&'a [u8]), // from Cisco-ASA with plain value, tag=36, type=string
	VsaAsaPptpMppcCompression(cisco_asa::AsaPptpMppcCompression), // from Cisco-ASA with Enum value, tag=37, type=integer
	VsaAsaL2TpMppcCompression(cisco_asa::AsaL2TpMppcCompression), // from Cisco-ASA with Enum value, tag=38, type=integer
	VsaAsaIpsecIpCompression(cisco_asa::AsaIpsecIpCompression), // from Cisco-ASA with Enum value, tag=39, type=integer
	VsaAsaIpsecIkePeerIdCheck(cisco_asa::AsaIpsecIkePeerIdCheck), // from Cisco-ASA with Enum value, tag=40, type=integer
	VsaAsaIkeKeepAlives(cisco_asa::AsaIkeKeepAlives), // from Cisco-ASA with Enum value, tag=41, type=integer
	VsaAsaIpsecAuthOnRekey(cisco_asa::AsaIpsecAuthOnRekey), // from Cisco-ASA with Enum value, tag=42, type=integer
	VsaAsaRequiredClientFirewallVendorCode(cisco_asa::AsaRequiredClientFirewallVendorCode), // from Cisco-ASA with Enum value, tag=45, type=integer
	VsaAsaRequiredClientFirewallProductCode(u32), // from Cisco-ASA with plain value, tag=46, type=integer
	VsaAsaRequiredClientFirewallDescription(&'a [u8]), // from Cisco-ASA with plain value, tag=47, type=string
	VsaAsaRequireHwClientAuth(cisco_asa::AsaRequireHwClientAuth), // from Cisco-ASA with Enum value, tag=48, type=integer
	VsaAsaRequiredIndividualUserAuth(cisco_asa::AsaRequiredIndividualUserAuth), // from Cisco-ASA with Enum value, tag=49, type=integer
	VsaAsaAuthenticatedUserIdleTimeout(u32), // from Cisco-ASA with plain value, tag=50, type=integer
	VsaAsaCiscoIpPhoneBypass(cisco_asa::AsaCiscoIpPhoneBypass), // from Cisco-ASA with Enum value, tag=51, type=integer
	VsaAsaIpsecSplitTunnelingPolicy(cisco_asa::AsaIpsecSplitTunnelingPolicy), // from Cisco-ASA with Enum value, tag=55, type=integer
	VsaAsaIpsecRequiredClientFirewallCapability(cisco_asa::AsaIpsecRequiredClientFirewallCapability), // from Cisco-ASA with Enum value, tag=56, type=integer
	VsaAsaIpsecClientFirewallFilterName(&'a [u8]), // from Cisco-ASA with plain value, tag=57, type=string
	VsaAsaIpsecClientFirewallFilterOptional(cisco_asa::AsaIpsecClientFirewallFilterOptional), // from Cisco-ASA with Enum value, tag=58, type=integer
	VsaAsaIpsecBackupServers(cisco_asa::AsaIpsecBackupServers), // from Cisco-ASA with Enum value, tag=59, type=integer
	VsaAsaIpsecBackupServerList(&'a [u8]), // from Cisco-ASA with plain value, tag=60, type=string
	VsaAsaDhcpNetworkScope(Ipv4Addr), // from Cisco-ASA with plain value, tag=61, type=ipaddr
	VsaAsaInterceptDhcpConfigureMsg(cisco_asa::AsaInterceptDhcpConfigureMsg), // from Cisco-ASA with Enum value, tag=62, type=integer
	VsaAsaMsClientSubnetMask(Ipv4Addr), // from Cisco-ASA with plain value, tag=63, type=ipaddr
	VsaAsaAllowNetworkExtensionMode(cisco_asa::AsaAllowNetworkExtensionMode), // from Cisco-ASA with Enum value, tag=64, type=integer
	VsaAsaAuthorizationType(cisco_asa::AsaAuthorizationType), // from Cisco-ASA with Enum value, tag=65, type=integer
	VsaAsaAuthorizationRequired(cisco_asa::AsaAuthorizationRequired), // from Cisco-ASA with Enum value, tag=66, type=integer
	VsaAsaAuthorizationDnField(&'a [u8]), // from Cisco-ASA with plain value, tag=67, type=string
	VsaAsaIkeKeepaliveConfidenceInterval(u32), // from Cisco-ASA with plain value, tag=68, type=integer
	VsaAsaWebvpnContentFilterParameters(u32), // from Cisco-ASA with plain value, tag=69, type=integer
	VsaAsaWebvpnHtmlFilter(u32), // from Cisco-ASA with plain value, tag=70, type=integer
	VsaAsaWebvpnUrlList(&'a [u8]), // from Cisco-ASA with plain value, tag=71, type=string
	VsaAsaWebvpnPortForwardingList(&'a [u8]), // from Cisco-ASA with plain value, tag=72, type=string
	VsaAsaWebvpnAccessList(&'a [u8]), // from Cisco-ASA with plain value, tag=73, type=string
	VsaAsaWebvpnHttpProxyIpAddress(&'a [u8]), // from Cisco-ASA with plain value, tag=74, type=string
	VsaAsaCiscoLeapBypass(cisco_asa::AsaCiscoLeapBypass), // from Cisco-ASA with Enum value, tag=75, type=integer
	VsaAsaWebvpnDefaultHomepage(&'a [u8]), // from Cisco-ASA with plain value, tag=76, type=string
	VsaAsaClientTypeVersionLimiting(&'a [u8]), // from Cisco-ASA with plain value, tag=77, type=string
	VsaAsaWebvpnGroupBasedHttpOrHttpsProxyExceptionList(&'a [u8]), // from Cisco-ASA with plain value, tag=78, type=string
	VsaAsaWebvpnPortForwardingName(&'a [u8]), // from Cisco-ASA with plain value, tag=79, type=string
	VsaAsaIeProxyServer(&'a [u8]), // from Cisco-ASA with plain value, tag=80, type=string
	VsaAsaIeProxyServerPolicy(cisco_asa::AsaIeProxyServerPolicy), // from Cisco-ASA with Enum value, tag=81, type=integer
	VsaAsaIeProxyExceptionList(&'a [u8]), // from Cisco-ASA with plain value, tag=82, type=string
	VsaAsaIeProxyBypassLocal(cisco_asa::AsaIeProxyBypassLocal), // from Cisco-ASA with Enum value, tag=83, type=integer
	VsaAsaIkeKeepaliveRetryInterval(u32), // from Cisco-ASA with plain value, tag=84, type=integer
	VsaAsaTunnelGroupLock(&'a [u8]), // from Cisco-ASA with plain value, tag=85, type=string
	VsaAsaAccessListInbound(&'a [u8]), // from Cisco-ASA with plain value, tag=86, type=string
	VsaAsaAccessListOutbound(&'a [u8]), // from Cisco-ASA with plain value, tag=87, type=string
	VsaAsaPerfectForwardSecrecyEnable(cisco_asa::AsaPerfectForwardSecrecyEnable), // from Cisco-ASA with Enum value, tag=88, type=integer
	VsaAsaNacEnable(cisco_asa::AsaNacEnable), // from Cisco-ASA with Enum value, tag=89, type=integer
	VsaAsaNacStatusQueryTimer(u32), // from Cisco-ASA with plain value, tag=90, type=integer
	VsaAsaNacRevalidationTimer(u32), // from Cisco-ASA with plain value, tag=91, type=integer
	VsaAsaNacDefaultAcl(&'a [u8]), // from Cisco-ASA with plain value, tag=92, type=string
	VsaAsaWebvpnUrlEntryEnable(cisco_asa::AsaWebvpnUrlEntryEnable), // from Cisco-ASA with Enum value, tag=93, type=integer
	VsaAsaWebvpnFileAccessEnable(cisco_asa::AsaWebvpnFileAccessEnable), // from Cisco-ASA with Enum value, tag=94, type=integer
	VsaAsaWebvpnFileServerEntryEnable(cisco_asa::AsaWebvpnFileServerEntryEnable), // from Cisco-ASA with Enum value, tag=95, type=integer
	VsaAsaWebvpnFileServerBrowsingEnable(cisco_asa::AsaWebvpnFileServerBrowsingEnable), // from Cisco-ASA with Enum value, tag=96, type=integer
	VsaAsaWebvpnPortForwardingEnable(cisco_asa::AsaWebvpnPortForwardingEnable), // from Cisco-ASA with Enum value, tag=97, type=integer
	VsaAsaWebvpnPortForwardingExchangeProxyEnable(cisco_asa::AsaWebvpnPortForwardingExchangeProxyEnable), // from Cisco-ASA with Enum value, tag=98, type=integer
	VsaAsaWebvpnPortForwardingHttpProxy(cisco_asa::AsaWebvpnPortForwardingHttpProxy), // from Cisco-ASA with Enum value, tag=99, type=integer
	VsaAsaWebvpnCitrixMetaframeEnable(cisco_asa::AsaWebvpnCitrixMetaframeEnable), // from Cisco-ASA with Enum value, tag=101, type=integer
	VsaAsaWebvpnApplyAcl(cisco_asa::AsaWebvpnApplyAcl), // from Cisco-ASA with Enum value, tag=102, type=integer
	VsaAsaWebvpnSslVpnClientEnable(cisco_asa::AsaWebvpnSslVpnClientEnable), // from Cisco-ASA with Enum value, tag=103, type=integer
	VsaAsaWebvpnSslVpnClientRequired(cisco_asa::AsaWebvpnSslVpnClientRequired), // from Cisco-ASA with Enum value, tag=104, type=integer
	VsaAsaWebvpnSslVpnClientKeepInstallation(cisco_asa::AsaWebvpnSslVpnClientKeepInstallation), // from Cisco-ASA with Enum value, tag=105, type=integer
	VsaAsaSvcKeepalive(u32), // from Cisco-ASA with plain value, tag=107, type=integer
	VsaAsaSvcDpdIntervalClient(u32), // from Cisco-ASA with plain value, tag=108, type=integer
	VsaAsaSvcDpdIntervalGateway(u32), // from Cisco-ASA with plain value, tag=109, type=integer
	VsaAsaSvcRekeyTime(u32), // from Cisco-ASA with plain value, tag=110, type=integer
	VsaAsaWebvpnSvcRekeyMethod(cisco_asa::AsaWebvpnSvcRekeyMethod), // from Cisco-ASA with Enum value, tag=111, type=integer
	VsaAsaWebvpnSvcCompression(cisco_asa::AsaWebvpnSvcCompression), // from Cisco-ASA with Enum value, tag=112, type=integer
	VsaAsaWebvpnCustomization(&'a [u8]), // from Cisco-ASA with plain value, tag=113, type=string
	VsaAsaWebvpnSsoServerName(&'a [u8]), // from Cisco-ASA with plain value, tag=114, type=string
	VsaAsaWebvpnDenyMessage(&'a [u8]), // from Cisco-ASA with plain value, tag=116, type=string
	VsaAsaWebvpnHttpCompression(cisco_asa::AsaWebvpnHttpCompression), // from Cisco-ASA with Enum value, tag=120, type=integer
	VsaAsaWebvpnKeepaliveIgnore(u32), // from Cisco-ASA with plain value, tag=121, type=integer
	VsaAsaExtendedAuthenticationOnRekey(cisco_asa::AsaExtendedAuthenticationOnRekey), // from Cisco-ASA with Enum value, tag=122, type=integer
	VsaAsaSvcDtls(cisco_asa::AsaSvcDtls), // from Cisco-ASA with Enum value, tag=123, type=integer
	VsaAsaWebvpnAutoHttpSignon(&'a [u8]), // from Cisco-ASA with plain value, tag=124, type=string
	VsaAsaSvcMtu(u32), // from Cisco-ASA with plain value, tag=125, type=integer
	VsaAsaWebvpnHiddenShares(cisco_asa::AsaWebvpnHiddenShares), // from Cisco-ASA with Enum value, tag=126, type=integer
	VsaAsaSvcModules(&'a [u8]), // from Cisco-ASA with plain value, tag=127, type=string
	VsaAsaSvcProfiles(&'a [u8]), // from Cisco-ASA with plain value, tag=128, type=string
	VsaAsaSvcAsk(cisco_asa::AsaSvcAsk), // from Cisco-ASA with Enum value, tag=131, type=integer
	VsaAsaSvcAskTimeout(u32), // from Cisco-ASA with plain value, tag=132, type=integer
	VsaAsaIeProxyPacUrl(&'a [u8]), // from Cisco-ASA with plain value, tag=133, type=string
	VsaAsaStripRealm(cisco_asa::AsaStripRealm), // from Cisco-ASA with Enum value, tag=135, type=integer
	VsaAsaSmartTunnel(&'a [u8]), // from Cisco-ASA with plain value, tag=136, type=string
	VsaAsaWebvpnActivexRelay(u32), // from Cisco-ASA with plain value, tag=137, type=integer
	VsaAsaSmartTunnelAuto(cisco_asa::AsaSmartTunnelAuto), // from Cisco-ASA with Enum value, tag=138, type=integer
	VsaAsaSmartTunnelAutoSignonEnable(&'a [u8]), // from Cisco-ASA with plain value, tag=139, type=string
	VsaAsaVlan(u32), // from Cisco-ASA with plain value, tag=140, type=integer
	VsaAsaNacSettings(&'a [u8]), // from Cisco-ASA with plain value, tag=141, type=string
	VsaAsaMemberOf(&'a [u8]), // from Cisco-ASA with plain value, tag=145, type=string
	VsaAsaTunnelgroupname(&'a [u8]), // from Cisco-ASA with plain value, tag=146, type=string
	VsaAsaWebvpnIdleTimeoutAlertInterval(u32), // from Cisco-ASA with plain value, tag=148, type=integer
	VsaAsaWebvpnSessionTimeoutAlertInterval(u32), // from Cisco-ASA with plain value, tag=149, type=integer
	VsaAsaClienttype(cisco_asa::AsaClienttype), // from Cisco-ASA with Enum value, tag=150, type=integer
	VsaAsaSessiontype(cisco_asa::AsaSessiontype), // from Cisco-ASA with Enum value, tag=151, type=integer
	VsaAsaSessionsubtype(cisco_asa::AsaSessionsubtype), // from Cisco-ASA with Enum value, tag=152, type=integer
	VsaAsaWebvpnDownloadMaxSize(u32), // from Cisco-ASA with plain value, tag=157, type=integer
	VsaAsaWebvpnUploadMaxSize(u32), // from Cisco-ASA with plain value, tag=158, type=integer
	VsaAsaWebvpnPostMaxSize(u32), // from Cisco-ASA with plain value, tag=159, type=integer
	VsaAsaWebvpnUserStorage(&'a [u8]), // from Cisco-ASA with plain value, tag=160, type=string
	VsaAsaWebvpnStorageObjects(&'a [u8]), // from Cisco-ASA with plain value, tag=161, type=string
	VsaAsaWebvpnStorageKey(&'a [u8]), // from Cisco-ASA with plain value, tag=162, type=string
	VsaAsaWebvpnVdi(&'a [u8]), // from Cisco-ASA with plain value, tag=163, type=string
	VsaAsaAddressPools(&'a [u8]), // from Cisco-ASA with plain value, tag=217, type=string
	VsaAsaIpv6AddressPools(&'a [u8]), // from Cisco-ASA with plain value, tag=218, type=string
	VsaAsaIpv6VpnFilter(&'a [u8]), // from Cisco-ASA with plain value, tag=219, type=string
	VsaAsaPrivilegeLevel(u32), // from Cisco-ASA with plain value, tag=220, type=integer
	VsaAsaWebvpnUnixUserId(u32), // from Cisco-ASA with plain value, tag=221, type=integer
	VsaAsaWebvpnUnixGroupId(u32), // from Cisco-ASA with plain value, tag=222, type=integer
	VsaAsaWebvpnMacroSubstitutionValue1(&'a [u8]), // from Cisco-ASA with plain value, tag=223, type=string
	VsaAsaWebvpnMacroSubstitutionValue2(&'a [u8]), // from Cisco-ASA with plain value, tag=224, type=string
	VsaAsaWebvpnsmartCardRemovalDisconnect(cisco_asa::AsaWebvpnsmartCardRemovalDisconnect), // from Cisco-ASA with Enum value, tag=225, type=integer
	VsaAsaWebvpnSmartTunnelTunnelPolicy(&'a [u8]), // from Cisco-ASA with plain value, tag=227, type=string
	VsaAsaWebvpnHomePageUseSmartTunnel(u32), // from Cisco-ASA with plain value, tag=228, type=integer
	VsaCbbsmBandwidth(u32), // from Cisco-BBSM with plain value, tag=1, type=integer
	VsaCitrixUid(u32), // from Citrix with plain value, tag=10, type=integer
	VsaCitrixGid(u32), // from Citrix with plain value, tag=11, type=integer
	VsaCitrixHome(&'a [u8]), // from Citrix with plain value, tag=12, type=string
	VsaCitrixShell(&'a [u8]), // from Citrix with plain value, tag=13, type=string
	VsaCitrixGroupNames(&'a [u8]), // from Citrix with plain value, tag=14, type=string
	VsaCitrixGroupIds(&'a [u8]), // from Citrix with plain value, tag=15, type=string
	VsaCitrixUserGroups(&'a [u8]), // from Citrix with plain value, tag=16, type=string
	VsaClavisterUserGroup(&'a [u8]), // from Clavister with plain value, tag=1, type=string
	VsaBelrasUpSpeedLimit(u32), // from Cnergee with plain value, tag=1, type=integer
	VsaBelrasDownSpeedLimit(u32), // from Cnergee with plain value, tag=2, type=integer
	VsaBelrasQosSpeed(u32), // from Cnergee with plain value, tag=3, type=integer
	VsaBelrasUser(&'a [u8]), // from Cnergee with plain value, tag=4, type=string
	VsaBelrasDhcpRouterIpAddress(Ipv4Addr), // from Cnergee with plain value, tag=5, type=ipaddr
	VsaBelrasDhcpMask(u32), // from Cnergee with plain value, tag=6, type=integer
	VsaBelrasRedirect(u32), // from Cnergee with plain value, tag=7, type=integer
	VsaBelrasRedirectPool(cnergee::BelrasRedirectPool), // from Cnergee with Enum value, tag=8, type=integer
	VsaBelrasDhcpOption82(&'a [u8]), // from Cnergee with plain value, tag=9, type=octets
	VsaBelrasSessionOctetsLimit(u32), // from Cnergee with plain value, tag=10, type=integer
	VsaBelrasOctetsDirection(u32), // from Cnergee with plain value, tag=11, type=integer
	VsaBelrasAkamaiSpeed(u32), // from Cnergee with plain value, tag=12, type=integer
	VsaBelrasCacheSpeed(u32), // from Cnergee with plain value, tag=13, type=integer
	VsaBelrasCacheflySpeed(u32), // from Cnergee with plain value, tag=14, type=integer
	VsaBelrasGgcSpeed(u32), // from Cnergee with plain value, tag=15, type=integer
	VsaBelrasGoogleSpeed(u32), // from Cnergee with plain value, tag=16, type=integer
	VsaBelrasIncapsulaSpeed(u32), // from Cnergee with plain value, tag=17, type=integer
	VsaBelrasLimelightSpeed(u32), // from Cnergee with plain value, tag=18, type=integer
	VsaBelrasOthersSpeed(u32), // from Cnergee with plain value, tag=19, type=integer
	VsaBelrasRediffSpeed(u32), // from Cnergee with plain value, tag=20, type=integer
	VsaBelrasTorrentSpeed(u32), // from Cnergee with plain value, tag=21, type=integer
	VsaBelrasBelcacheSpeed(u32), // from Cnergee with plain value, tag=22, type=integer
	VsaBelrasDhcpLeaseTime(u32), // from Cnergee with plain value, tag=23, type=integer
	VsaColubrisAvpair(&'a [u8]), // from Colubris with plain value, tag=0, type=string
	VsaColubrisIntercept(u32), // from Colubris with plain value, tag=1, type=integer
	VsaCompatibleTunnelDelay(u32), // from Compatible with plain value, tag=0, type=integer
	VsaCompatibleTunnelThroughput(u32), // from Compatible with plain value, tag=1, type=integer
	VsaCompatibleTunnelServerEndpoint(Ipv4Addr), // from Compatible with plain value, tag=3, type=ipaddr
	VsaCompatibleTunnelGroupInfo(&'a [u8]), // from Compatible with plain value, tag=4, type=string
	VsaCompatibleTunnelPassword(&'a [u8]), // from Compatible with plain value, tag=5, type=string
	VsaCompatibleEcho(u32), // from Compatible with plain value, tag=6, type=integer
	VsaCompatibleTunnelIpx(u32), // from Compatible with plain value, tag=7, type=integer
	VsaCosineConnectionProfileName(&'a [u8]), // from Cosine with plain value, tag=1, type=string
	VsaCosineEnterpriseId(&'a [u8]), // from Cosine with plain value, tag=2, type=string
	VsaCosineAddressPoolName(&'a [u8]), // from Cosine with plain value, tag=3, type=string
	VsaCosineDsByte(u32), // from Cosine with plain value, tag=4, type=integer
	VsaCosineVpiVci(&'a [u8]), // from Cosine with plain value, tag=5, type=octets
	VsaCosineDlci(u32), // from Cosine with plain value, tag=6, type=integer
	VsaCosineLnsIpAddress(Ipv4Addr), // from Cosine with plain value, tag=7, type=ipaddr
	VsaCosineCliUserPermissionId(&'a [u8]), // from Cosine with plain value, tag=8, type=string
	VsaDefaultTtl(u32), // from DANTE with plain value, tag=1, type=integer
	VsaDellemcAvpair(&'a [u8]), // from DellEMC with plain value, tag=1, type=string
	VsaDellemcGroupName(&'a [u8]), // from DellEMC with plain value, tag=2, type=string
	VsaDlinkUserLevel(dlink::DlinkUserLevel), // from Dlink with Enum value, tag=1, type=integer
	VsaDlinkIngressBandwidthAssignment(u32), // from Dlink with plain value, tag=2, type=integer
	VsaDlinkEgressBandwidthAssignment(u32), // from Dlink with plain value, tag=3, type=integer
	VsaDlink1PPriority(u32), // from Dlink with plain value, tag=4, type=integer
	VsaDlinkVlanName(&'a [u8]), // from Dlink with plain value, tag=10, type=string
	VsaDlinkVlanId(&'a [u8]), // from Dlink with plain value, tag=11, type=string
	VsaDlinkAclProfile(&'a [u8]), // from Dlink with plain value, tag=12, type=string
	VsaDlinkAclRule(&'a [u8]), // from Dlink with plain value, tag=13, type=string
	VsaDlinkAclScript(&'a [u8]), // from Dlink with plain value, tag=14, type=string
	VsaAsteriskAccCode(&'a [u8]), // from Digium with plain value, tag=101, type=string
	VsaAsteriskSrc(&'a [u8]), // from Digium with plain value, tag=102, type=string
	VsaAsteriskDst(&'a [u8]), // from Digium with plain value, tag=103, type=string
	VsaAsteriskDstCtx(&'a [u8]), // from Digium with plain value, tag=104, type=string
	VsaAsteriskClid(&'a [u8]), // from Digium with plain value, tag=105, type=string
	VsaAsteriskChan(&'a [u8]), // from Digium with plain value, tag=106, type=string
	VsaAsteriskDstChan(&'a [u8]), // from Digium with plain value, tag=107, type=string
	VsaAsteriskLastApp(&'a [u8]), // from Digium with plain value, tag=108, type=string
	VsaAsteriskLastData(&'a [u8]), // from Digium with plain value, tag=109, type=string
	VsaAsteriskStartTime(&'a [u8]), // from Digium with plain value, tag=110, type=string
	VsaAsteriskAnswerTime(&'a [u8]), // from Digium with plain value, tag=111, type=string
	VsaAsteriskEndTime(&'a [u8]), // from Digium with plain value, tag=112, type=string
	VsaAsteriskDuration(u32), // from Digium with plain value, tag=113, type=integer
	VsaAsteriskBillSec(u32), // from Digium with plain value, tag=114, type=integer
	VsaAsteriskDisposition(&'a [u8]), // from Digium with plain value, tag=115, type=string
	VsaAsteriskAmaFlags(&'a [u8]), // from Digium with plain value, tag=116, type=string
	VsaAsteriskUniqueId(&'a [u8]), // from Digium with plain value, tag=117, type=string
	VsaAsteriskUserField(&'a [u8]), // from Digium with plain value, tag=118, type=string
	VsaDragonwavePrivilegeLevel(dragonwave::DragonwavePrivilegeLevel), // from DragonWave with Enum value, tag=1, type=integer
	VsaEfficientipVersion(u32), // from EfficientIP with plain value, tag=1, type=integer
	VsaEfficientipServiceClass(u32), // from EfficientIP with plain value, tag=2, type=integer
	VsaEfficientipIdentityType(u32), // from EfficientIP with plain value, tag=3, type=integer
	VsaEfficientipFirstName(&'a [u8]), // from EfficientIP with plain value, tag=16, type=string
	VsaEfficientipLastName(&'a [u8]), // from EfficientIP with plain value, tag=17, type=string
	VsaEfficientipPseudonym(&'a [u8]), // from EfficientIP with plain value, tag=18, type=string
	VsaEfficientipIpHost(&'a [u8]), // from EfficientIP with plain value, tag=19, type=string
	VsaEfficientipEmail(&'a [u8]), // from EfficientIP with plain value, tag=20, type=string
	VsaEfficientipFirstLoginPath(&'a [u8]), // from EfficientIP with plain value, tag=32, type=string
	VsaEfficientipMaintainerGroup(&'a [u8]), // from EfficientIP with plain value, tag=33, type=string
	VsaEfficientipGroups(&'a [u8]), // from EfficientIP with plain value, tag=34, type=string
	VsaEfficientipAdminGroup(&'a [u8]), // from EfficientIP with plain value, tag=35, type=string
	VsaEfficientipExtraBlob(&'a [u8]), // from EfficientIP with plain value, tag=64, type=string
	VsaEltexAvpair(&'a [u8]), // from Eltex with plain value, tag=1, type=string
	VsaEltexDisconnectCodeLocal(eltex::EltexDisconnectCodeLocal), // from Eltex with Enum value, tag=11, type=integer
	VsaEpygiAvpair(&'a [u8]), // from Epygi with plain value, tag=1, type=string
	VsaEpygiNasPort(&'a [u8]), // from Epygi with plain value, tag=2, type=string
	VsaEpygiH323RemoteAddress(&'a [u8]), // from Epygi with plain value, tag=23, type=string
	VsaEpygiH323ConfId(&'a [u8]), // from Epygi with plain value, tag=24, type=string
	VsaEpygiH323SetupTime(&'a [u8]), // from Epygi with plain value, tag=25, type=string
	VsaEpygiH323CallOrigin(&'a [u8]), // from Epygi with plain value, tag=26, type=string
	VsaEpygiH323CallType(&'a [u8]), // from Epygi with plain value, tag=27, type=string
	VsaEpygiH323ConnectTime(&'a [u8]), // from Epygi with plain value, tag=28, type=string
	VsaEpygiH323DisconnectTime(&'a [u8]), // from Epygi with plain value, tag=29, type=string
	VsaEpygiH323DisconnectCause(&'a [u8]), // from Epygi with plain value, tag=30, type=string
	VsaEpygiH323VoiceQuality(&'a [u8]), // from Epygi with plain value, tag=31, type=string
	VsaEpygiH323GwId(&'a [u8]), // from Epygi with plain value, tag=33, type=string
	VsaEpygiH323IncomingConfId(&'a [u8]), // from Epygi with plain value, tag=35, type=string
	VsaEpygiH323CreditAmount(&'a [u8]), // from Epygi with plain value, tag=101, type=string
	VsaEpygiH323CreditTime(&'a [u8]), // from Epygi with plain value, tag=102, type=string
	VsaEpygiH323ReturnCode(&'a [u8]), // from Epygi with plain value, tag=103, type=string
	VsaEpygiH323PromptId(&'a [u8]), // from Epygi with plain value, tag=104, type=string
	VsaEpygiH323TimeAndDay(&'a [u8]), // from Epygi with plain value, tag=105, type=string
	VsaEpygiH323RedirectNumber(&'a [u8]), // from Epygi with plain value, tag=106, type=string
	VsaEpygiH323PreferredLang(&'a [u8]), // from Epygi with plain value, tag=107, type=string
	VsaEpygiH323RedirectIpAddress(&'a [u8]), // from Epygi with plain value, tag=108, type=string
	VsaEpygiH323BillingModel(&'a [u8]), // from Epygi with plain value, tag=109, type=string
	VsaEpygiH323Currency(&'a [u8]), // from Epygi with plain value, tag=110, type=string
	VsaEpygiRegexpdate(&'a [u8]), // from Epygi with plain value, tag=150, type=string
	VsaEpygiFiadid(&'a [u8]), // from Epygi with plain value, tag=151, type=string
	VsaEpygiPortid(&'a [u8]), // from Epygi with plain value, tag=152, type=string
	VsaEpygiAccesstype(&'a [u8]), // from Epygi with plain value, tag=153, type=string
	VsaEpygiCallinfo(&'a [u8]), // from Epygi with plain value, tag=154, type=string
	VsaEpygiOrigcallid(&'a [u8]), // from Epygi with plain value, tag=170, type=string
	VsaEpygiParentcallid(&'a [u8]), // from Epygi with plain value, tag=171, type=string
	VsaEpygiCalltype(epygi::EpygiCalltype), // from Epygi with Enum value, tag=172, type=integer
	VsaEpygiDevicename(&'a [u8]), // from Epygi with plain value, tag=173, type=string
	VsaEpygiInterfacename(epygi::EpygiInterfacename), // from Epygi with Enum value, tag=174, type=integer
	VsaEpygiInterfacenumber(u32), // from Epygi with plain value, tag=175, type=integer
	VsaEpygiTimeslotnumber(u32), // from Epygi with plain value, tag=176, type=integer
	VsaEpygiOrigipaddr(u32), // from Epygi with plain value, tag=177, type=integer
	VsaEpygiDestipaddr(u32), // from Epygi with plain value, tag=178, type=integer
	VsaEpygiOrigipport(u32), // from Epygi with plain value, tag=179, type=integer
	VsaEpygiDestipport(u32), // from Epygi with plain value, tag=180, type=integer
	VsaEpygiCallingpartynumber(&'a [u8]), // from Epygi with plain value, tag=181, type=string
	VsaEpygiCalledpartynumber(&'a [u8]), // from Epygi with plain value, tag=182, type=string
	VsaEpygiDatetimeorigination(u32), // from Epygi with plain value, tag=183, type=integer
	VsaEpygiDatetimeconnect(u32), // from Epygi with plain value, tag=184, type=integer
	VsaEpygiDatetimedisconnect(u32), // from Epygi with plain value, tag=185, type=integer
	VsaEpygiDuration(u32), // from Epygi with plain value, tag=186, type=integer
	VsaEpygiOutsourcertpIp(u32), // from Epygi with plain value, tag=187, type=integer
	VsaEpygiOutdestrtpIp(u32), // from Epygi with plain value, tag=188, type=integer
	VsaEpygiInsourcertpIp(u32), // from Epygi with plain value, tag=189, type=integer
	VsaEpygiIndestrtpIp(u32), // from Epygi with plain value, tag=190, type=integer
	VsaEpygiOutsourcertpPort(u32), // from Epygi with plain value, tag=191, type=integer
	VsaEpygiOutdestrtpPort(u32), // from Epygi with plain value, tag=192, type=integer
	VsaEpygiInsourcertpPort(u32), // from Epygi with plain value, tag=193, type=integer
	VsaEpygiIndestrtpPort(u32), // from Epygi with plain value, tag=194, type=integer
	VsaEpygiCallredirectreason(epygi::EpygiCallredirectreason), // from Epygi with Enum value, tag=195, type=integer
	VsaEpygiCalldisconnectreason(epygi::EpygiCalldisconnectreason), // from Epygi with Enum value, tag=196, type=integer
	VsaEpygiOutrtpPayload(u32), // from Epygi with plain value, tag=197, type=integer
	VsaEpygiOutrtpPacketsize(u32), // from Epygi with plain value, tag=198, type=integer
	VsaEpygiOutrtpPackets(u32), // from Epygi with plain value, tag=199, type=integer
	VsaEpygiOutrtpOctets(u32), // from Epygi with plain value, tag=200, type=integer
	VsaEpygiInrtpPayload(u32), // from Epygi with plain value, tag=201, type=integer
	VsaEpygiInrtpPacketsize(u32), // from Epygi with plain value, tag=202, type=integer
	VsaEpygiInrtpPackets(u32), // from Epygi with plain value, tag=203, type=integer
	VsaEpygiInrtpOctets(u32), // from Epygi with plain value, tag=204, type=integer
	VsaEpygiInrtpPacketslost(u32), // from Epygi with plain value, tag=205, type=integer
	VsaEpygiInrtpPacketsdupl(u32), // from Epygi with plain value, tag=206, type=integer
	VsaEpygiInrtpJitter(u32), // from Epygi with plain value, tag=207, type=integer
	VsaEpygiInrtpLatency(u32), // from Epygi with plain value, tag=208, type=integer
	VsaEquallogicAdminFullName(&'a [u8]), // from Equallogic with plain value, tag=1, type=string
	VsaEquallogicAdminEmail(&'a [u8]), // from Equallogic with plain value, tag=2, type=string
	VsaEquallogicAdminPhone(&'a [u8]), // from Equallogic with plain value, tag=3, type=string
	VsaEquallogicAdminMobile(&'a [u8]), // from Equallogic with plain value, tag=4, type=string
	VsaEquallogicPollInterval(u32), // from Equallogic with plain value, tag=5, type=integer
	VsaEquallogicEqlAdminPrivilege(equallogic::EquallogicEqlAdminPrivilege), // from Equallogic with Enum value, tag=6, type=integer
	VsaEquallogicAdminPoolAccess(&'a [u8]), // from Equallogic with plain value, tag=7, type=string
	VsaEquallogicAdminReplSiteAccess(&'a [u8]), // from Equallogic with plain value, tag=8, type=string
	VsaEquallogicAdminAccountType(&'a [u8]), // from Equallogic with plain value, tag=9, type=string
	VsaEricssonVigBalance(u32), // from Ericsson with plain value, tag=3, type=integer
	VsaEricssonVigCodec(u32), // from Ericsson with plain value, tag=4, type=integer
	VsaEricssonVigCurrency(&'a [u8]), // from Ericsson with plain value, tag=5, type=string
	VsaEricssonVigCurrencyQuote(&'a [u8]), // from Ericsson with plain value, tag=6, type=string
	VsaEricssonVigEndpointType(u32), // from Ericsson with plain value, tag=8, type=integer
	VsaEricssonVigSequenceNumber(u32), // from Ericsson with plain value, tag=9, type=integer
	VsaEricssonVigAccessAgentIpAddress(Ipv4Addr), // from Ericsson with plain value, tag=11, type=ipaddr
	VsaEricssonVigQosClass(u32), // from Ericsson with plain value, tag=12, type=integer
	VsaEricssonVigDigestResponse(&'a [u8]), // from Ericsson with plain value, tag=14, type=string
	VsaEricssonVigDigestAttributes(&'a [u8]), // from Ericsson with plain value, tag=15, type=octets
	VsaEricssonVigBusinessAgreementName(&'a [u8]), // from Ericsson with plain value, tag=16, type=string
	VsaEricssonVigCallRole(u32), // from Ericsson with plain value, tag=17, type=integer
	VsaEricssonVigRemoteSkUaIpAddress(Ipv4Addr), // from Ericsson with plain value, tag=20, type=ipaddr
	VsaEricssonVigSite(&'a [u8]), // from Ericsson with plain value, tag=23, type=string
	VsaEricssonVigTtlRelative(u32), // from Ericsson with plain value, tag=32, type=integer
	VsaEricssonVigAccountErrorReason(u32), // from Ericsson with plain value, tag=33, type=integer
	VsaEricssonVigLayerIdentity(u32), // from Ericsson with plain value, tag=34, type=integer
	VsaEricssonVigMajorProtocolVersion(u32), // from Ericsson with plain value, tag=35, type=integer
	VsaEricssonVigMinorProtocolVersion(u32), // from Ericsson with plain value, tag=36, type=integer
	VsaEricssonVigAuthenticationType(u32), // from Ericsson with plain value, tag=37, type=integer
	VsaEricssonVigTrustedAccess(u32), // from Ericsson with plain value, tag=38, type=integer
	VsaEricssonVigUserName(&'a [u8]), // from Ericsson with plain value, tag=39, type=string
	VsaEricssonVigGlobalUniqueCallId(&'a [u8]), // from Ericsson with plain value, tag=40, type=string
	VsaEricssonVigGlobalUniqueServiceId(&'a [u8]), // from Ericsson with plain value, tag=41, type=string
	VsaEricssonVigInterimInterval(u32), // from Ericsson with plain value, tag=42, type=integer
	VsaEricssonVigAliveIndicator(u32), // from Ericsson with plain value, tag=43, type=integer
	VsaEricssonVigTtlAbsolute(u32), // from Ericsson with plain value, tag=44, type=integer
	VsaEricssonVigTtlStartEvent(u32), // from Ericsson with plain value, tag=45, type=integer
	VsaEricssonVigSkIpAddress(Ipv4Addr), // from Ericsson with plain value, tag=46, type=ipaddr
	VsaEricssonVigUaIpAddress(Ipv4Addr), // from Ericsson with plain value, tag=47, type=ipaddr
	VsaEricssonVigSaIpAddress(Ipv4Addr), // from Ericsson with plain value, tag=48, type=ipaddr
	VsaEricssonVigCallingE164Number(&'a [u8]), // from Ericsson with plain value, tag=49, type=string
	VsaEricssonVigCallingH323Id(&'a [u8]), // from Ericsson with plain value, tag=50, type=string
	VsaEricssonVigCallingEmailAddress(&'a [u8]), // from Ericsson with plain value, tag=51, type=string
	VsaEricssonVigDialledE164Number(&'a [u8]), // from Ericsson with plain value, tag=52, type=string
	VsaEricssonVigDialledH323Id(&'a [u8]), // from Ericsson with plain value, tag=53, type=string
	VsaEricssonVigDialledEmailAddress(&'a [u8]), // from Ericsson with plain value, tag=54, type=string
	VsaEricssonVigRoutedE164Number(&'a [u8]), // from Ericsson with plain value, tag=55, type=string
	VsaEricssonVigRoutedH323Id(&'a [u8]), // from Ericsson with plain value, tag=56, type=string
	VsaEricssonVigRoutedEmailAddress(&'a [u8]), // from Ericsson with plain value, tag=57, type=string
	VsaEricssonVigSitekeeperName(&'a [u8]), // from Ericsson with plain value, tag=58, type=string
	VsaEricssonVigAccessGroupName(&'a [u8]), // from Ericsson with plain value, tag=59, type=string
	VsaEricssonVigAccessAgentName(&'a [u8]), // from Ericsson with plain value, tag=60, type=string
	VsaEricssonVigUserAgentGroupName(&'a [u8]), // from Ericsson with plain value, tag=61, type=string
	VsaEricssonVigUserAgentName(&'a [u8]), // from Ericsson with plain value, tag=62, type=string
	VsaEricssonVigRoutingTariff(u32), // from Ericsson with plain value, tag=63, type=integer
	VsaEricssonVigReSelectionCounter(u32), // from Ericsson with plain value, tag=64, type=integer
	VsaEricssonVigCpnDigits(&'a [u8]), // from Ericsson with plain value, tag=65, type=string
	VsaEricssonVigCpnTon(u32), // from Ericsson with plain value, tag=66, type=integer
	VsaEricssonVigCpnNp(u32), // from Ericsson with plain value, tag=67, type=integer
	VsaEricssonVigCpnPi(u32), // from Ericsson with plain value, tag=68, type=integer
	VsaEricssonVigCpnSi(u32), // from Ericsson with plain value, tag=69, type=integer
	VsaEricssonVigDialledNumDigits(&'a [u8]), // from Ericsson with plain value, tag=70, type=string
	VsaEricssonVigDialledNumTon(u32), // from Ericsson with plain value, tag=71, type=integer
	VsaEricssonVigDialledNumNp(u32), // from Ericsson with plain value, tag=72, type=integer
	VsaEricssonVigRoutingNumDigits(&'a [u8]), // from Ericsson with plain value, tag=73, type=string
	VsaEricssonVigRoutingNumTon(u32), // from Ericsson with plain value, tag=74, type=integer
	VsaEricssonVigRoutingNumNp(u32), // from Ericsson with plain value, tag=75, type=integer
	VsaEricssonVigRedirectingNumDigits(&'a [u8]), // from Ericsson with plain value, tag=76, type=string
	VsaEricssonVigRedirectingNumTon(u32), // from Ericsson with plain value, tag=77, type=integer
	VsaEricssonVigRedirectingNumNp(u32), // from Ericsson with plain value, tag=78, type=integer
	VsaEricssonVigRedirectingNumPi(u32), // from Ericsson with plain value, tag=79, type=integer
	VsaEricssonVigRedirectingNumRfd(u32), // from Ericsson with plain value, tag=80, type=integer
	VsaEricssonVigTimeStampUtc(u32), // from Ericsson with plain value, tag=81, type=integer
	VsaEricssonVigTimeStampTz(u32), // from Ericsson with plain value, tag=82, type=integer
	VsaEricssonVigTimeStampDst(u32), // from Ericsson with plain value, tag=83, type=integer
	VsaEricssonVigSessionRoutingDuration(u32), // from Ericsson with plain value, tag=84, type=integer
	VsaEricssonVigSessionRingingDuration(u32), // from Ericsson with plain value, tag=85, type=integer
	VsaEricssonVigAccessType(u32), // from Ericsson with plain value, tag=86, type=integer
	VsaEricssonVigRequestedBandwidth(u32), // from Ericsson with plain value, tag=87, type=integer
	VsaEricssonVigAllowedBandwidth(u32), // from Ericsson with plain value, tag=88, type=integer
	VsaEricssonVigMediaChannelCount(u32), // from Ericsson with plain value, tag=89, type=integer
	VsaEricssonVigVoiceMediaRecForward(&'a [u8]), // from Ericsson with plain value, tag=90, type=string
	VsaEricssonVigVoiceMediaRecBackward(&'a [u8]), // from Ericsson with plain value, tag=91, type=string
	VsaEricssonVigVideoMediaRecForward(&'a [u8]), // from Ericsson with plain value, tag=92, type=string
	VsaEricssonVigVideoMediaRecBackward(&'a [u8]), // from Ericsson with plain value, tag=93, type=string
	VsaEricssonVigFaxMediaRecForward(&'a [u8]), // from Ericsson with plain value, tag=94, type=string
	VsaEricssonVigFaxMediaRecBackward(&'a [u8]), // from Ericsson with plain value, tag=95, type=string
	VsaEricssonVigDataMediaRecForward(&'a [u8]), // from Ericsson with plain value, tag=96, type=string
	VsaEricssonVigDataMediaRecBackward(&'a [u8]), // from Ericsson with plain value, tag=97, type=string
	VsaEricssonVigChargingCase(u32), // from Ericsson with plain value, tag=98, type=integer
	VsaEricssonVigRelCauseCodingStd(u32), // from Ericsson with plain value, tag=99, type=integer
	VsaEricssonVigRelCauseLocation(u32), // from Ericsson with plain value, tag=100, type=integer
	VsaEricssonVigRelCauseClass(u32), // from Ericsson with plain value, tag=101, type=integer
	VsaEricssonVigRelCauseValue(u32), // from Ericsson with plain value, tag=102, type=integer
	VsaEricssonVigRelReason(u32), // from Ericsson with plain value, tag=103, type=integer
	VsaEricssonVigInternalRelReasonVal(u32), // from Ericsson with plain value, tag=104, type=integer
	VsaEricssonVigInternalRelReasonOrig(u32), // from Ericsson with plain value, tag=105, type=integer
	VsaEricssonVigServiceId(u32), // from Ericsson with plain value, tag=106, type=integer
	VsaEricssonVigUserId(&'a [u8]), // from Ericsson with plain value, tag=107, type=string
	VsaEricssonVigServiceName(&'a [u8]), // from Ericsson with plain value, tag=108, type=string
	VsaEricssonVigTestCallIndicator(u32), // from Ericsson with plain value, tag=109, type=integer
	VsaEricssonVigEmergencyCallIndicator(u32), // from Ericsson with plain value, tag=110, type=integer
	VsaEricssonVigCallingId(&'a [u8]), // from Ericsson with plain value, tag=111, type=string
	VsaEricssonVigCalledId(&'a [u8]), // from Ericsson with plain value, tag=112, type=string
	VsaEricssonVigTranslatedId(&'a [u8]), // from Ericsson with plain value, tag=113, type=string
	VsaEricssonVigCallingUserGroupId(&'a [u8]), // from Ericsson with plain value, tag=114, type=string
	VsaEricssonVigCallingUsrSubGroupId(&'a [u8]), // from Ericsson with plain value, tag=115, type=string
	VsaEricssonVigCalledUsrGroupId(&'a [u8]), // from Ericsson with plain value, tag=116, type=string
	VsaEricssonVigCalledUsrSubGroupId(&'a [u8]), // from Ericsson with plain value, tag=117, type=string
	VsaEricssonVigTerminalType(&'a [u8]), // from Ericsson with plain value, tag=118, type=string
	VsaEricssonVigServiceDuration(u32), // from Ericsson with plain value, tag=119, type=integer
	VsaEricssonVigServiceExecutionResult(u32), // from Ericsson with plain value, tag=120, type=integer
	VsaEricssonVigServiceExeRsltDesc(&'a [u8]), // from Ericsson with plain value, tag=121, type=string
	VsaEricssonVigServiceDescription(&'a [u8]), // from Ericsson with plain value, tag=122, type=string
	VsaEricssonVigServiceSpecificInfo(&'a [u8]), // from Ericsson with plain value, tag=123, type=string
	VsaEricssonVigProxyIpAddress(Ipv4Addr), // from Ericsson with plain value, tag=124, type=ipaddr
	VsaEricssonVigAuthDatarequest(u32), // from Ericsson with plain value, tag=125, type=integer
	VsaEricssonVigIptTimeStamp(u32), // from Ericsson with plain value, tag=126, type=integer
	VsaEricssonVigUserNameInfo(u32), // from Ericsson with plain value, tag=127, type=integer
	VsaClientDnsPri(Ipv4Addr), // from Ericsson-AB with plain value, tag=1, type=ipaddr
	VsaClientDnsSec(Ipv4Addr), // from Ericsson-AB with plain value, tag=2, type=ipaddr
	VsaDhcpMaxLeases(u32), // from Ericsson-AB with plain value, tag=3, type=integer
	VsaContextName(&'a [u8]), // from Ericsson-AB with plain value, tag=4, type=string
	VsaBridgeGroup(&'a [u8]), // from Ericsson-AB with plain value, tag=5, type=string
	VsaBgAgingTime(&'a [u8]), // from Ericsson-AB with plain value, tag=6, type=string
	VsaBgPathCost(&'a [u8]), // from Ericsson-AB with plain value, tag=7, type=string
	VsaBgSpanDis(&'a [u8]), // from Ericsson-AB with plain value, tag=8, type=string
	VsaBgTransBpdu(&'a [u8]), // from Ericsson-AB with plain value, tag=9, type=string
	VsaRateLimitRate(u32), // from Ericsson-AB with plain value, tag=10, type=integer
	VsaRateLimitBurst(u32), // from Ericsson-AB with plain value, tag=11, type=integer
	VsaPoliceRate(u32), // from Ericsson-AB with plain value, tag=12, type=integer
	VsaPoliceBurst(u32), // from Ericsson-AB with plain value, tag=13, type=integer
	VsaSourceValidation(ericsson_ab::SourceValidation), // from Ericsson-AB with Enum value, tag=14, type=integer
	VsaTunnelDomain(ericsson_ab::TunnelDomain), // from Ericsson-AB with Enum value, tag=15, type=integer
	VsaTunnelLocalName(&'a [u8]), // from Ericsson-AB with plain value, tag=16, type=string
	VsaTunnelRemoteName(&'a [u8]), // from Ericsson-AB with plain value, tag=17, type=string
	VsaTunnelFunction(ericsson_ab::TunnelFunction), // from Ericsson-AB with Enum value, tag=18, type=integer
	VsaTunnelFlowControl(u32), // from Ericsson-AB with plain value, tag=19, type=integer
	VsaTunnelStatic(u32), // from Ericsson-AB with plain value, tag=20, type=integer
	VsaTunnelMaxSessions(u32), // from Ericsson-AB with plain value, tag=21, type=integer
	VsaTunnelMaxTunnels(u32), // from Ericsson-AB with plain value, tag=22, type=integer
	VsaTunnelSessionAuth(ericsson_ab::TunnelSessionAuth), // from Ericsson-AB with Enum value, tag=23, type=integer
	VsaTunnelWindow(u32), // from Ericsson-AB with plain value, tag=24, type=integer
	VsaTunnelRetransmit(u32), // from Ericsson-AB with plain value, tag=25, type=integer
	VsaTunnelCmdTimeout(u32), // from Ericsson-AB with plain value, tag=26, type=integer
	VsaPppoeUrl(&'a [u8]), // from Ericsson-AB with plain value, tag=27, type=string
	VsaPppoeMotm(&'a [u8]), // from Ericsson-AB with plain value, tag=28, type=string
	VsaTunnelGroup(ericsson_ab::TunnelGroup), // from Ericsson-AB with Enum value, tag=29, type=integer
	VsaTunnelContext(&'a [u8]), // from Ericsson-AB with plain value, tag=30, type=string
	VsaTunnelAlgorithm(ericsson_ab::TunnelAlgorithm), // from Ericsson-AB with Enum value, tag=31, type=integer
	VsaTunnelDeadtime(u32), // from Ericsson-AB with plain value, tag=32, type=integer
	VsaMcastSend(ericsson_ab::McastSend), // from Ericsson-AB with Enum value, tag=33, type=integer
	VsaMcastReceive(ericsson_ab::McastReceive), // from Ericsson-AB with Enum value, tag=34, type=integer
	VsaMcastMaxgroups(u32), // from Ericsson-AB with plain value, tag=35, type=integer
	VsaIpAddressPoolName(&'a [u8]), // from Ericsson-AB with plain value, tag=36, type=string
	VsaTunnelDnis(ericsson_ab::TunnelDnis), // from Ericsson-AB with Enum value, tag=37, type=integer
	VsaMediumType(ericsson_ab::MediumType), // from Ericsson-AB with Enum value, tag=38, type=integer
	VsaPvcEncapsulationType(ericsson_ab::PvcEncapsulationType), // from Ericsson-AB with Enum value, tag=39, type=integer
	VsaPvcProfileName(&'a [u8]), // from Ericsson-AB with plain value, tag=40, type=string
	VsaPvcCircuitPadding(ericsson_ab::PvcCircuitPadding), // from Ericsson-AB with Enum value, tag=41, type=integer
	VsaBindType(ericsson_ab::BindType), // from Ericsson-AB with Enum value, tag=42, type=integer
	VsaBindAuthProtocol(ericsson_ab::BindAuthProtocol), // from Ericsson-AB with Enum value, tag=43, type=integer
	VsaBindAuthMaxSessions(u32), // from Ericsson-AB with plain value, tag=44, type=integer
	VsaBindBypassBypass(&'a [u8]), // from Ericsson-AB with plain value, tag=45, type=string
	VsaBindAuthContext(&'a [u8]), // from Ericsson-AB with plain value, tag=46, type=string
	VsaBindAuthServiceGrp(&'a [u8]), // from Ericsson-AB with plain value, tag=47, type=string
	VsaBindBypassContext(&'a [u8]), // from Ericsson-AB with plain value, tag=48, type=string
	VsaBindIntContext(&'a [u8]), // from Ericsson-AB with plain value, tag=49, type=string
	VsaBindTunContext(&'a [u8]), // from Ericsson-AB with plain value, tag=50, type=string
	VsaBindSesContext(&'a [u8]), // from Ericsson-AB with plain value, tag=51, type=string
	VsaBindDot1QSlot(u32), // from Ericsson-AB with plain value, tag=52, type=integer
	VsaBindDot1QPort(u32), // from Ericsson-AB with plain value, tag=53, type=integer
	VsaBindDot1QVlanTagId(u32), // from Ericsson-AB with plain value, tag=54, type=integer
	VsaBindIntInterfaceName(&'a [u8]), // from Ericsson-AB with plain value, tag=55, type=string
	VsaBindL2TpTunnelName(&'a [u8]), // from Ericsson-AB with plain value, tag=56, type=string
	VsaBindL2TpFlowControl(u32), // from Ericsson-AB with plain value, tag=57, type=integer
	VsaBindSubUserAtContext(&'a [u8]), // from Ericsson-AB with plain value, tag=58, type=string
	VsaBindSubPassword(&'a [u8]), // from Ericsson-AB with plain value, tag=59, type=string
	VsaIpHostAddr(&'a [u8]), // from Ericsson-AB with plain value, tag=60, type=string
	VsaIpTosField(ericsson_ab::IpTosField), // from Ericsson-AB with Enum value, tag=61, type=integer
	VsaNasRealPort(u32), // from Ericsson-AB with plain value, tag=62, type=integer
	VsaTunnelSessionAuthCtx(&'a [u8]), // from Ericsson-AB with plain value, tag=63, type=string
	VsaTunnelSessionAuthServiceGrp(&'a [u8]), // from Ericsson-AB with plain value, tag=64, type=string
	VsaTunnelRateLimitRate(u32), // from Ericsson-AB with plain value, tag=65, type=integer
	VsaTunnelRateLimitBurst(u32), // from Ericsson-AB with plain value, tag=66, type=integer
	VsaTunnelPoliceRate(u32), // from Ericsson-AB with plain value, tag=67, type=integer
	VsaTunnelPoliceBurst(u32), // from Ericsson-AB with plain value, tag=68, type=integer
	VsaTunnelL2FSecondPassword(&'a [u8]), // from Ericsson-AB with plain value, tag=69, type=string
	VsaAclDefinition(&'a [u8]), // from Ericsson-AB with plain value, tag=70, type=string
	VsaPppoeIpRouteAdd(&'a [u8]), // from Ericsson-AB with plain value, tag=71, type=string
	VsaTtyLevelMax(u32), // from Ericsson-AB with plain value, tag=72, type=integer
	VsaTtyLevelStart(u32), // from Ericsson-AB with plain value, tag=73, type=integer
	VsaTunnelChecksum(u32), // from Ericsson-AB with plain value, tag=74, type=integer
	VsaTunnelProfile(&'a [u8]), // from Ericsson-AB with plain value, tag=75, type=string
	VsaTunnelClientVpn(&'a [u8]), // from Ericsson-AB with plain value, tag=78, type=string
	VsaTunnelServerVpn(&'a [u8]), // from Ericsson-AB with plain value, tag=79, type=string
	VsaTunnelClientRhost(&'a [u8]), // from Ericsson-AB with plain value, tag=80, type=string
	VsaTunnelServerRhost(&'a [u8]), // from Ericsson-AB with plain value, tag=81, type=string
	VsaTunnelClientIntAddr(Ipv4Addr), // from Ericsson-AB with plain value, tag=82, type=ipaddr
	VsaTunnelServerIntAddr(Ipv4Addr), // from Ericsson-AB with plain value, tag=83, type=ipaddr
	VsaPppCompression(u32), // from Ericsson-AB with plain value, tag=84, type=integer
	VsaTunnelHelloTimer(u32), // from Ericsson-AB with plain value, tag=85, type=integer
	VsaRedbackReason(u32), // from Ericsson-AB with plain value, tag=86, type=integer
	VsaQosPolicingProfileName(&'a [u8]), // from Ericsson-AB with plain value, tag=87, type=string
	VsaQosMeteringProfileName(&'a [u8]), // from Ericsson-AB with plain value, tag=88, type=string
	VsaQosPolicyQueuing(&'a [u8]), // from Ericsson-AB with plain value, tag=89, type=string
	VsaIgmpServiceProfileName(&'a [u8]), // from Ericsson-AB with plain value, tag=90, type=string
	VsaSubscriberProfileName(&'a [u8]), // from Ericsson-AB with plain value, tag=91, type=string
	VsaForwardPolicy(&'a [u8]), // from Ericsson-AB with plain value, tag=92, type=string
	VsaRemotePort(&'a [u8]), // from Ericsson-AB with plain value, tag=93, type=string
	VsaReauth(&'a [u8]), // from Ericsson-AB with plain value, tag=94, type=string
	VsaReauthMore(u32), // from Ericsson-AB with plain value, tag=95, type=integer
	VsaAgentRemoteId(&'a [u8]), // from Ericsson-AB with plain value, tag=96, type=string
	VsaAgentCircuitId(&'a [u8]), // from Ericsson-AB with plain value, tag=97, type=string
	VsaPlatformType(ericsson_ab::PlatformType), // from Ericsson-AB with Enum value, tag=98, type=integer
	VsaClientNbnsPri(Ipv4Addr), // from Ericsson-AB with plain value, tag=99, type=ipaddr
	VsaClientNbnsSec(Ipv4Addr), // from Ericsson-AB with plain value, tag=100, type=ipaddr
	VsaShapingProfileName(&'a [u8]), // from Ericsson-AB with plain value, tag=101, type=string
	VsaBgCctAddrMax(u32), // from Ericsson-AB with plain value, tag=103, type=integer
	VsaIpInterfaceName(&'a [u8]), // from Ericsson-AB with plain value, tag=104, type=string
	VsaNatPolicyName(&'a [u8]), // from Ericsson-AB with plain value, tag=105, type=string
	VsaRbNpmServiceId(&'a [u8]), // from Ericsson-AB with plain value, tag=106, type=string
	VsaHttpRedirectProfileName(&'a [u8]), // from Ericsson-AB with plain value, tag=107, type=string
	VsaBindAutoSubUser(&'a [u8]), // from Ericsson-AB with plain value, tag=108, type=string
	VsaBindAutoSubContext(&'a [u8]), // from Ericsson-AB with plain value, tag=109, type=string
	VsaBindAutoSubPassword(&'a [u8]), // from Ericsson-AB with plain value, tag=110, type=string
	VsaCircuitProtocolEncap(ericsson_ab::CircuitProtocolEncap), // from Ericsson-AB with Enum value, tag=111, type=integer
	VsaOsVersion(&'a [u8]), // from Ericsson-AB with plain value, tag=112, type=string
	VsaSessionTrafficLimit(&'a [u8]), // from Ericsson-AB with plain value, tag=113, type=string
	VsaQosReference(&'a [u8]), // from Ericsson-AB with plain value, tag=114, type=string
	VsaRateLimitExcessBurst(&'a [u8]), // from Ericsson-AB with plain value, tag=121, type=octets
	VsaPoliceExcessBurst(&'a [u8]), // from Ericsson-AB with plain value, tag=122, type=octets
	VsaTunnelRateLimitExcessBurst(&'a [u8]), // from Ericsson-AB with plain value, tag=123, type=octets
	VsaTunnelPoliceExcessBurst(&'a [u8]), // from Ericsson-AB with plain value, tag=124, type=octets
	VsaDhcpVendorClassId(&'a [u8]), // from Ericsson-AB with plain value, tag=125, type=string
	VsaQosRate(&'a [u8]), // from Ericsson-AB with plain value, tag=126, type=string
	VsaDhcpVendorEncapOption(&'a [u8]), // from Ericsson-AB with plain value, tag=127, type=string
	VsaAcctInputOctets64(&'a [u8]), // from Ericsson-AB with plain value, tag=128, type=octets
	VsaAcctOutputOctets64(&'a [u8]), // from Ericsson-AB with plain value, tag=129, type=octets
	VsaAcctInputPackets64(&'a [u8]), // from Ericsson-AB with plain value, tag=130, type=octets
	VsaAcctOutputPackets64(&'a [u8]), // from Ericsson-AB with plain value, tag=131, type=octets
	VsaAssignedIpAddress(Ipv4Addr), // from Ericsson-AB with plain value, tag=132, type=ipaddr
	VsaAcctMcastInOctets64(&'a [u8]), // from Ericsson-AB with plain value, tag=133, type=octets
	VsaAcctMcastOutOctets64(&'a [u8]), // from Ericsson-AB with plain value, tag=134, type=octets
	VsaAcctMcastInPackets64(&'a [u8]), // from Ericsson-AB with plain value, tag=135, type=octets
	VsaAcctMcastOutPackets64(&'a [u8]), // from Ericsson-AB with plain value, tag=136, type=octets
	VsaLacPort(u32), // from Ericsson-AB with plain value, tag=137, type=integer
	VsaLacRealPort(u32), // from Ericsson-AB with plain value, tag=138, type=integer
	VsaLacPortType(ericsson_ab::LacPortType), // from Ericsson-AB with Enum value, tag=139, type=integer
	VsaLacRealPortType(ericsson_ab::LacRealPortType), // from Ericsson-AB with Enum value, tag=140, type=integer
	VsaAcctDynAcEnt(&'a [u8]), // from Ericsson-AB with plain value, tag=141, type=string
	VsaSessionErrorCode(u32), // from Ericsson-AB with plain value, tag=142, type=integer
	VsaSessionErrorMsg(&'a [u8]), // from Ericsson-AB with plain value, tag=143, type=string
	VsaAcctUpdateReason(ericsson_ab::AcctUpdateReason), // from Ericsson-AB with Enum value, tag=144, type=integer
	VsaMacAddr(&'a [u8]), // from Ericsson-AB with plain value, tag=145, type=string
	VsaVlanSourceInfo(&'a [u8]), // from Ericsson-AB with plain value, tag=146, type=string
	VsaAcctMcastInOctets(u32), // from Ericsson-AB with plain value, tag=147, type=integer
	VsaAcctMcastOutOctets(u32), // from Ericsson-AB with plain value, tag=148, type=integer
	VsaAcctMcastInPackets(u32), // from Ericsson-AB with plain value, tag=149, type=integer
	VsaAcctMcastOutPackets(u32), // from Ericsson-AB with plain value, tag=150, type=integer
	VsaReauthSessionId(&'a [u8]), // from Ericsson-AB with plain value, tag=151, type=string
	VsaQosRateInbound(&'a [u8]), // from Ericsson-AB with plain value, tag=156, type=string
	VsaQosRateOutbound(&'a [u8]), // from Ericsson-AB with plain value, tag=157, type=string
	VsaRouteTag(u32), // from Ericsson-AB with plain value, tag=158, type=integer
	VsaLiId(u32), // from Ericsson-AB with plain value, tag=159, type=integer
	VsaLiMdAddress(Ipv4Addr), // from Ericsson-AB with plain value, tag=160, type=ipaddr
	VsaLiMdPort(u32), // from Ericsson-AB with plain value, tag=161, type=integer
	VsaLiAction(u32), // from Ericsson-AB with plain value, tag=162, type=integer
	VsaLiProfile(&'a [u8]), // from Ericsson-AB with plain value, tag=163, type=string
	VsaDynamicPolicyFilter(&'a [u8]), // from Ericsson-AB with plain value, tag=164, type=string
	VsaHttpRedirectUrl(&'a [u8]), // from Ericsson-AB with plain value, tag=165, type=string
	VsaDslActualRateUp(u32), // from Ericsson-AB with plain value, tag=166, type=integer
	VsaDslActualRateDown(u32), // from Ericsson-AB with plain value, tag=167, type=integer
	VsaDslMinRateUp(u32), // from Ericsson-AB with plain value, tag=168, type=integer
	VsaDslMinRateDown(u32), // from Ericsson-AB with plain value, tag=169, type=integer
	VsaDslAttainableRateUp(u32), // from Ericsson-AB with plain value, tag=170, type=integer
	VsaDslAttainableRateDown(u32), // from Ericsson-AB with plain value, tag=171, type=integer
	VsaDslMaxRateUp(u32), // from Ericsson-AB with plain value, tag=172, type=integer
	VsaDslMaxRateDown(u32), // from Ericsson-AB with plain value, tag=173, type=integer
	VsaDslMinLowPowerRateUp(u32), // from Ericsson-AB with plain value, tag=174, type=integer
	VsaDslMinLowPowerRateDown(u32), // from Ericsson-AB with plain value, tag=175, type=integer
	VsaDslMaxInterDelayUp(u32), // from Ericsson-AB with plain value, tag=176, type=integer
	VsaDslActualInterDelayUp(u32), // from Ericsson-AB with plain value, tag=177, type=integer
	VsaDslMaxInterDelayDown(u32), // from Ericsson-AB with plain value, tag=178, type=integer
	VsaDslActualInterDelayDown(u32), // from Ericsson-AB with plain value, tag=179, type=integer
	VsaDslLineState(ericsson_ab::DslLineState), // from Ericsson-AB with Enum value, tag=180, type=integer
	VsaDslL2Encapsulation(u32), // from Ericsson-AB with plain value, tag=181, type=integer
	VsaDslTransmissionSystem(ericsson_ab::DslTransmissionSystem), // from Ericsson-AB with Enum value, tag=182, type=integer
	VsaDslPppoaPppoeInterWorkFlag(u32), // from Ericsson-AB with plain value, tag=183, type=integer
	VsaDslActualRateDownFactor(u32), // from Ericsson-AB with plain value, tag=185, type=integer
	VsaDslCombinedLineInfo(&'a [u8]), // from Ericsson-AB with plain value, tag=184, type=string
	VsaClassVolumeLimit(&'a [u8]), // from Ericsson-AB with plain value, tag=186, type=string
	VsaClassVolumeInCounter(&'a [u8]), // from Ericsson-AB with plain value, tag=187, type=string
	VsaClassVolumeOutCounter(&'a [u8]), // from Ericsson-AB with plain value, tag=188, type=string
	VsaFlowFacProfile(&'a [u8]), // from Ericsson-AB with plain value, tag=189, type=string
	VsaServiceName(&'a [u8]), // from Ericsson-AB with plain value, tag=190, type=string
	VsaServiceAction(ericsson_ab::ServiceAction), // from Ericsson-AB with Enum value, tag=191, type=integer
	VsaServiceParameter(&'a [u8]), // from Ericsson-AB with plain value, tag=192, type=string
	VsaServiceErrorCause(u32), // from Ericsson-AB with plain value, tag=193, type=integer
	VsaDeactivateServiceName(&'a [u8]), // from Ericsson-AB with plain value, tag=194, type=string
	VsaQosProfileOverhead(&'a [u8]), // from Ericsson-AB with plain value, tag=195, type=string
	VsaDynamicQosParam(&'a [u8]), // from Ericsson-AB with plain value, tag=196, type=string
	VsaAcctAltSessionId(&'a [u8]), // from Ericsson-AB with plain value, tag=197, type=string
	VsaIdleTimeoutThreshold(u32), // from Ericsson-AB with plain value, tag=198, type=integer
	VsaDoubleAuthentication(u32), // from Ericsson-AB with plain value, tag=199, type=integer
	VsaSbcAdjacency(&'a [u8]), // from Ericsson-AB with plain value, tag=200, type=string
	VsaDhcpField(&'a [u8]), // from Ericsson-AB with plain value, tag=201, type=octets
	VsaDhcpOption(&'a [u8]), // from Ericsson-AB with plain value, tag=202, type=octets
	VsaSecurityService(&'a [u8]), // from Ericsson-AB with plain value, tag=203, type=string
	VsaReauthServiceName(&'a [u8]), // from Ericsson-AB with plain value, tag=204, type=string
	VsaFlowIpProfile(&'a [u8]), // from Ericsson-AB with plain value, tag=205, type=string
	VsaRadiusThrottleWatermark(u32), // from Ericsson-AB with plain value, tag=206, type=integer
	VsaRbIpv6Dns(&'a [u8]), // from Ericsson-AB with plain value, tag=207, type=string
	VsaRbIpv6Option(&'a [u8]), // from Ericsson-AB with plain value, tag=208, type=string
	VsaClusterPartitionId(&'a [u8]), // from Ericsson-AB with plain value, tag=209, type=string
	VsaCircuitGroupMember(&'a [u8]), // from Ericsson-AB with plain value, tag=210, type=string
	VsaDelegatedMaxPrefix(u32), // from Ericsson-AB with plain value, tag=212, type=integer
	VsaIpv4AddressReleaseControl(&'a [u8]), // from Ericsson-AB with plain value, tag=213, type=string
	VsaAcctInputIpv4Octet(u32), // from Ericsson-AB with plain value, tag=214, type=integer
	VsaAcctOutputIpv4Octets(u32), // from Ericsson-AB with plain value, tag=215, type=integer
	VsaAcctInputIpv4Packets(u32), // from Ericsson-AB with plain value, tag=216, type=integer
	VsaAcctOutputIpv4Packets(u32), // from Ericsson-AB with plain value, tag=217, type=integer
	VsaAcctInputIpv4Gigawords(u32), // from Ericsson-AB with plain value, tag=218, type=integer
	VsaAcctOutputIpv4Gigawords(u32), // from Ericsson-AB with plain value, tag=219, type=integer
	VsaAcctInputIpv6Octets(u32), // from Ericsson-AB with plain value, tag=220, type=integer
	VsaAcctOutputIpv6Octets(u32), // from Ericsson-AB with plain value, tag=221, type=integer
	VsaAcctInputIpv6Packets(u32), // from Ericsson-AB with plain value, tag=222, type=integer
	VsaAcctOutputIpv6Packets(u32), // from Ericsson-AB with plain value, tag=223, type=integer
	VsaAcctInputIpv6Gigawords(u32), // from Ericsson-AB with plain value, tag=224, type=integer
	VsaAcctOutputIpv6Gigawords(u32), // from Ericsson-AB with plain value, tag=225, type=integer
	VsaSuggestedRuleSpace(&'a [u8]), // from Ericsson-Packet-Core-Networks with plain value, tag=30, type=string
	VsaSuggestedSecondaryRuleSpace(&'a [u8]), // from Ericsson-Packet-Core-Networks with plain value, tag=31, type=string
	VsaExtremeCliAuthorization(extreme::ExtremeCliAuthorization), // from Extreme with Enum value, tag=201, type=integer
	VsaExtremeShellCommand(&'a [u8]), // from Extreme with plain value, tag=202, type=string
	VsaExtremeNetloginVlan(&'a [u8]), // from Extreme with plain value, tag=203, type=string
	VsaExtremeNetloginUrl(&'a [u8]), // from Extreme with plain value, tag=204, type=string
	VsaExtremeNetloginUrlDesc(&'a [u8]), // from Extreme with plain value, tag=205, type=string
	VsaExtremeNetloginOnly(extreme::ExtremeNetloginOnly), // from Extreme with Enum value, tag=206, type=integer
	VsaExtremeUserLocation(&'a [u8]), // from Extreme with plain value, tag=208, type=string
	VsaExtremeNetloginVlanTag(u32), // from Extreme with plain value, tag=209, type=integer
	VsaExtremeNetloginExtendedVlan(&'a [u8]), // from Extreme with plain value, tag=211, type=string
	VsaExtremeSecurityProfile(&'a [u8]), // from Extreme with plain value, tag=212, type=string
	VsaExtremeVmName(&'a [u8]), // from Extreme with plain value, tag=213, type=string
	VsaExtremeVmVppName(&'a [u8]), // from Extreme with plain value, tag=214, type=string
	VsaExtremeVmIpAddr(Ipv4Addr), // from Extreme with plain value, tag=215, type=ipaddr
	VsaExtremeVmVlanId(u32), // from Extreme with plain value, tag=216, type=integer
	VsaExtremeVmVrName(&'a [u8]), // from Extreme with plain value, tag=217, type=string
	VsaF5LtmUserRole(f5::F5LtmUserRole), // from F5 with Enum value, tag=1, type=integer
	VsaF5LtmUserRoleUniversal(f5::F5LtmUserRoleUniversal), // from F5 with Enum value, tag=2, type=integer
	VsaF5LtmUserPartition(&'a [u8]), // from F5 with plain value, tag=3, type=string
	VsaF5LtmUserConsole(f5::F5LtmUserConsole), // from F5 with Enum value, tag=4, type=integer
	VsaF5LtmUserShell(&'a [u8]), // from F5 with plain value, tag=5, type=string
	VsaF5LtmUserContext1(u32), // from F5 with plain value, tag=10, type=integer
	VsaF5LtmUserContext2(u32), // from F5 with plain value, tag=11, type=integer
	VsaF5LtmUserInfo1(&'a [u8]), // from F5 with plain value, tag=12, type=string
	VsaF5LtmUserInfo2(&'a [u8]), // from F5 with plain value, tag=13, type=string
	VsaF5LtmAuditMsg(&'a [u8]), // from F5 with plain value, tag=14, type=string
	VsaFdxtendedBandwidthUp(u32), // from fdXtended with plain value, tag=1, type=integer
	VsaFdxtendedBandwidthDown(u32), // from fdXtended with plain value, tag=2, type=integer
	VsaFdxtendedPostauthurl(&'a [u8]), // from fdXtended with plain value, tag=3, type=string
	VsaFdxtendedOne2OnenatIp(&'a [u8]), // from fdXtended with plain value, tag=4, type=string
	VsaFdxtendedContentfilter(u32), // from fdXtended with plain value, tag=5, type=integer
	VsaFdxtendedNetworkpolicy(u32), // from fdXtended with plain value, tag=6, type=integer
	VsaFdxtendedBytesdown(u32), // from fdXtended with plain value, tag=7, type=integer
	VsaFdxtendedBytesup(u32), // from fdXtended with plain value, tag=8, type=integer
	VsaFdxtendedExpiration(&'a [u8]), // from fdXtended with plain value, tag=9, type=string
	VsaFdxtendedSessiontimeout(u32), // from fdXtended with plain value, tag=10, type=integer
	VsaFdxtendedWanInterface(&'a [u8]), // from fdXtended with plain value, tag=11, type=string
	VsaFreeradiusProxiedTo(Ipv4Addr), // from FreeRADIUS with plain value, tag=1, type=ipaddr
	VsaFreeradiusAcctSessionStartTime(u32), // from FreeRADIUS with plain value, tag=2, type=date
	VsaFreeradiusStatisticsType(freeradius::FreeradiusStatisticsType), // from FreeRADIUS with Enum value, tag=127, type=integer
	VsaFreeradiusTotalAccessRequests(u32), // from FreeRADIUS with plain value, tag=128, type=integer
	VsaFreeradiusTotalAccessAccepts(u32), // from FreeRADIUS with plain value, tag=129, type=integer
	VsaFreeradiusTotalAccessRejects(u32), // from FreeRADIUS with plain value, tag=130, type=integer
	VsaFreeradiusTotalAccessChallenges(u32), // from FreeRADIUS with plain value, tag=131, type=integer
	VsaFreeradiusTotalAuthResponses(u32), // from FreeRADIUS with plain value, tag=132, type=integer
	VsaFreeradiusTotalAuthDuplicateRequests(u32), // from FreeRADIUS with plain value, tag=133, type=integer
	VsaFreeradiusTotalAuthMalformedRequests(u32), // from FreeRADIUS with plain value, tag=134, type=integer
	VsaFreeradiusTotalAuthInvalidRequests(u32), // from FreeRADIUS with plain value, tag=135, type=integer
	VsaFreeradiusTotalAuthDroppedRequests(u32), // from FreeRADIUS with plain value, tag=136, type=integer
	VsaFreeradiusTotalAuthUnknownTypes(u32), // from FreeRADIUS with plain value, tag=137, type=integer
	VsaFreeradiusTotalProxyAccessRequests(u32), // from FreeRADIUS with plain value, tag=138, type=integer
	VsaFreeradiusTotalProxyAccessAccepts(u32), // from FreeRADIUS with plain value, tag=139, type=integer
	VsaFreeradiusTotalProxyAccessRejects(u32), // from FreeRADIUS with plain value, tag=140, type=integer
	VsaFreeradiusTotalProxyAccessChallenges(u32), // from FreeRADIUS with plain value, tag=141, type=integer
	VsaFreeradiusTotalProxyAuthResponses(u32), // from FreeRADIUS with plain value, tag=142, type=integer
	VsaFreeradiusTotalProxyAuthDuplicateRequests(u32), // from FreeRADIUS with plain value, tag=143, type=integer
	VsaFreeradiusTotalProxyAuthMalformedRequests(u32), // from FreeRADIUS with plain value, tag=144, type=integer
	VsaFreeradiusTotalProxyAuthInvalidRequests(u32), // from FreeRADIUS with plain value, tag=145, type=integer
	VsaFreeradiusTotalProxyAuthDroppedRequests(u32), // from FreeRADIUS with plain value, tag=146, type=integer
	VsaFreeradiusTotalProxyAuthUnknownTypes(u32), // from FreeRADIUS with plain value, tag=147, type=integer
	VsaFreeradiusTotalAccountingRequests(u32), // from FreeRADIUS with plain value, tag=148, type=integer
	VsaFreeradiusTotalAccountingResponses(u32), // from FreeRADIUS with plain value, tag=149, type=integer
	VsaFreeradiusTotalAcctDuplicateRequests(u32), // from FreeRADIUS with plain value, tag=150, type=integer
	VsaFreeradiusTotalAcctMalformedRequests(u32), // from FreeRADIUS with plain value, tag=151, type=integer
	VsaFreeradiusTotalAcctInvalidRequests(u32), // from FreeRADIUS with plain value, tag=152, type=integer
	VsaFreeradiusTotalAcctDroppedRequests(u32), // from FreeRADIUS with plain value, tag=153, type=integer
	VsaFreeradiusTotalAcctUnknownTypes(u32), // from FreeRADIUS with plain value, tag=154, type=integer
	VsaFreeradiusTotalProxyAccountingRequests(u32), // from FreeRADIUS with plain value, tag=155, type=integer
	VsaFreeradiusTotalProxyAccountingResponses(u32), // from FreeRADIUS with plain value, tag=156, type=integer
	VsaFreeradiusTotalProxyAcctDuplicateRequests(u32), // from FreeRADIUS with plain value, tag=157, type=integer
	VsaFreeradiusTotalProxyAcctMalformedRequests(u32), // from FreeRADIUS with plain value, tag=158, type=integer
	VsaFreeradiusTotalProxyAcctInvalidRequests(u32), // from FreeRADIUS with plain value, tag=159, type=integer
	VsaFreeradiusTotalProxyAcctDroppedRequests(u32), // from FreeRADIUS with plain value, tag=160, type=integer
	VsaFreeradiusTotalProxyAcctUnknownTypes(u32), // from FreeRADIUS with plain value, tag=161, type=integer
	VsaFreeradiusQueueLenInternal(u32), // from FreeRADIUS with plain value, tag=162, type=integer
	VsaFreeradiusQueueLenProxy(u32), // from FreeRADIUS with plain value, tag=163, type=integer
	VsaFreeradiusQueueLenAuth(u32), // from FreeRADIUS with plain value, tag=164, type=integer
	VsaFreeradiusQueueLenAcct(u32), // from FreeRADIUS with plain value, tag=165, type=integer
	VsaFreeradiusQueueLenDetail(u32), // from FreeRADIUS with plain value, tag=166, type=integer
	VsaFreeradiusStatsClientIpAddress(Ipv4Addr), // from FreeRADIUS with plain value, tag=167, type=ipaddr
	VsaFreeradiusStatsClientNumber(u32), // from FreeRADIUS with plain value, tag=168, type=integer
	VsaFreeradiusStatsClientNetmask(u32), // from FreeRADIUS with plain value, tag=169, type=integer
	VsaFreeradiusStatsServerIpAddress(Ipv4Addr), // from FreeRADIUS with plain value, tag=170, type=ipaddr
	VsaFreeradiusStatsServerPort(u32), // from FreeRADIUS with plain value, tag=171, type=integer
	VsaFreeradiusStatsServerOutstandingRequests(u32), // from FreeRADIUS with plain value, tag=172, type=integer
	VsaFreeradiusStatsServerState(freeradius::FreeradiusStatsServerState), // from FreeRADIUS with Enum value, tag=173, type=integer
	VsaFreeradiusStatsServerTimeOfDeath(u32), // from FreeRADIUS with plain value, tag=174, type=date
	VsaFreeradiusStatsServerTimeOfLife(u32), // from FreeRADIUS with plain value, tag=175, type=date
	VsaFreeradiusStatsStartTime(u32), // from FreeRADIUS with plain value, tag=176, type=date
	VsaFreeradiusStatsHupTime(u32), // from FreeRADIUS with plain value, tag=177, type=date
	VsaFreeradiusServerEmaWindow(u32), // from FreeRADIUS with plain value, tag=178, type=integer
	VsaFreeradiusServerEmaUsecWindow1(u32), // from FreeRADIUS with plain value, tag=179, type=integer
	VsaFreeradiusServerEmaUsecWindow10(u32), // from FreeRADIUS with plain value, tag=180, type=integer
	VsaFreeradiusQueuePpsIn(u32), // from FreeRADIUS with plain value, tag=181, type=integer
	VsaFreeradiusQueuePpsOut(u32), // from FreeRADIUS with plain value, tag=182, type=integer
	VsaFreeradiusQueueUsePercentage(u32), // from FreeRADIUS with plain value, tag=183, type=integer
	VsaFreeradiusStatsLastPacketRecv(u32), // from FreeRADIUS with plain value, tag=184, type=date
	VsaFreeradiusStatsLastPacketSent(u32), // from FreeRADIUS with plain value, tag=185, type=date
	VsaFreeswitchAvpair(&'a [u8]), // from Freeswitch with plain value, tag=1, type=string
	VsaFreeswitchClid(&'a [u8]), // from Freeswitch with plain value, tag=2, type=string
	VsaFreeswitchDialplan(&'a [u8]), // from Freeswitch with plain value, tag=3, type=string
	VsaFreeswitchSrc(&'a [u8]), // from Freeswitch with plain value, tag=4, type=string
	VsaFreeswitchDst(&'a [u8]), // from Freeswitch with plain value, tag=5, type=string
	VsaFreeswitchSrcChannel(&'a [u8]), // from Freeswitch with plain value, tag=6, type=string
	VsaFreeswitchDstChannel(&'a [u8]), // from Freeswitch with plain value, tag=7, type=string
	VsaFreeswitchAni(&'a [u8]), // from Freeswitch with plain value, tag=8, type=string
	VsaFreeswitchAniii(&'a [u8]), // from Freeswitch with plain value, tag=9, type=string
	VsaFreeswitchLastapp(&'a [u8]), // from Freeswitch with plain value, tag=10, type=string
	VsaFreeswitchLastdata(&'a [u8]), // from Freeswitch with plain value, tag=11, type=string
	VsaFreeswitchDisposition(&'a [u8]), // from Freeswitch with plain value, tag=12, type=string
	VsaFreeswitchHangupcause(freeswitch::FreeswitchHangupcause), // from Freeswitch with Enum value, tag=13, type=integer
	VsaFreeswitchBillusec(u32), // from Freeswitch with plain value, tag=15, type=integer
	VsaFreeswitchAmaflags(u32), // from Freeswitch with plain value, tag=16, type=integer
	VsaFreeswitchRdnis(&'a [u8]), // from Freeswitch with plain value, tag=17, type=string
	VsaFreeswitchContext(&'a [u8]), // from Freeswitch with plain value, tag=18, type=string
	VsaFreeswitchSource(&'a [u8]), // from Freeswitch with plain value, tag=19, type=string
	VsaFreeswitchCallstartdate(&'a [u8]), // from Freeswitch with plain value, tag=20, type=string
	VsaFreeswitchCallanswerdate(&'a [u8]), // from Freeswitch with plain value, tag=21, type=string
	VsaFreeswitchCalltransferdate(&'a [u8]), // from Freeswitch with plain value, tag=22, type=string
	VsaFreeswitchCallenddate(&'a [u8]), // from Freeswitch with plain value, tag=23, type=string
	VsaFreeswitchSignalbond(&'a [u8]), // from Freeswitch with plain value, tag=24, type=string
	VsaFortinetGroupName(&'a [u8]), // from Fortinet with plain value, tag=1, type=string
	VsaFortinetClientIpAddress(Ipv4Addr), // from Fortinet with plain value, tag=2, type=ipaddr
	VsaFortinetVdomName(&'a [u8]), // from Fortinet with plain value, tag=3, type=string
	VsaFortinetClientIpv6Address(&'a [u8]), // from Fortinet with plain value, tag=4, type=octets
	VsaFortinetInterfaceName(&'a [u8]), // from Fortinet with plain value, tag=5, type=string
	VsaFortinetAccessProfile(&'a [u8]), // from Fortinet with plain value, tag=6, type=string
	VsaFoundryPrivilegeLevel(u32), // from Foundry with plain value, tag=1, type=integer
	VsaFoundryCommandString(&'a [u8]), // from Foundry with plain value, tag=2, type=string
	VsaFoundryCommandExceptionFlag(u32), // from Foundry with plain value, tag=3, type=integer
	VsaFoundryInmPrivilege(foundry::FoundryInmPrivilege), // from Foundry with Enum value, tag=4, type=integer
	VsaFoundryAccessList(&'a [u8]), // from Foundry with plain value, tag=5, type=string
	VsaFoundryMacAuthentNeeds802dot1X(u32), // from Foundry with plain value, tag=6, type=integer
	VsaFoundry802dot1XValidLookup(u32), // from Foundry with plain value, tag=7, type=integer
	VsaFoundryMacBasedVlanQos(foundry::FoundryMacBasedVlanQos), // from Foundry with Enum value, tag=8, type=integer
	VsaFoundryInmRoleAorList(&'a [u8]), // from Foundry with plain value, tag=9, type=string
	VsaFoundrySiContextRole(&'a [u8]), // from Foundry with plain value, tag=10, type=string
	VsaFoundrySiRoleTemplate(&'a [u8]), // from Foundry with plain value, tag=11, type=string
	VsaGandalfRemoteLanName(&'a [u8]), // from Gandalf with plain value, tag=0, type=string
	VsaGandalfOperationalModes(gandalf::GandalfOperationalModes), // from Gandalf with Enum value, tag=1, type=integer
	VsaGandalfCompressionStatus(gandalf::GandalfCompressionStatus), // from Gandalf with Enum value, tag=2, type=integer
	VsaGandalfMinOutgoingBearer(gandalf::GandalfMinOutgoingBearer), // from Gandalf with Enum value, tag=3, type=integer
	VsaGandalfAuthenticationString(&'a [u8]), // from Gandalf with plain value, tag=5, type=string
	VsaGandalfPppAuthentication(gandalf::GandalfPppAuthentication), // from Gandalf with Enum value, tag=6, type=integer
	VsaGandalfPppNcpType(gandalf::GandalfPppNcpType), // from Gandalf with Enum value, tag=7, type=integer
	VsaGandalfFwdMulticastIn(gandalf::GandalfFwdMulticastIn), // from Gandalf with Enum value, tag=8, type=integer
	VsaGandalfFwdBroadcastIn(gandalf::GandalfFwdBroadcastIn), // from Gandalf with Enum value, tag=9, type=integer
	VsaGandalfFwdUnicastIn(gandalf::GandalfFwdUnicastIn), // from Gandalf with Enum value, tag=10, type=integer
	VsaGandalfFwdMulticastOut(gandalf::GandalfFwdMulticastOut), // from Gandalf with Enum value, tag=11, type=integer
	VsaGandalfFwdBroadcastOut(gandalf::GandalfFwdBroadcastOut), // from Gandalf with Enum value, tag=12, type=integer
	VsaGandalfFwdUnicastOut(gandalf::GandalfFwdUnicastOut), // from Gandalf with Enum value, tag=13, type=integer
	VsaGandalfAroundTheCorner(u32), // from Gandalf with plain value, tag=14, type=integer
	VsaGandalfChannelGroupName1(&'a [u8]), // from Gandalf with plain value, tag=15, type=string
	VsaGandalfDialPrefixName1(&'a [u8]), // from Gandalf with plain value, tag=16, type=string
	VsaGandalfPhoneNumber1(&'a [u8]), // from Gandalf with plain value, tag=17, type=string
	VsaGandalfCallingLineId1(&'a [u8]), // from Gandalf with plain value, tag=18, type=string
	VsaGandalfChannelGroupName2(&'a [u8]), // from Gandalf with plain value, tag=19, type=string
	VsaGandalfDialPrefixName2(&'a [u8]), // from Gandalf with plain value, tag=20, type=string
	VsaGandalfPhoneNumber2(&'a [u8]), // from Gandalf with plain value, tag=21, type=string
	VsaGandalfCallingLineId2(&'a [u8]), // from Gandalf with plain value, tag=22, type=string
	VsaGandalfIpxSpoofingState(gandalf::GandalfIpxSpoofingState), // from Gandalf with Enum value, tag=23, type=integer
	VsaGandalfIpxWatchdogSpoof(gandalf::GandalfIpxWatchdogSpoof), // from Gandalf with Enum value, tag=24, type=integer
	VsaGandalfSapGroupName1(&'a [u8]), // from Gandalf with plain value, tag=25, type=string
	VsaGandalfSapGroupName2(&'a [u8]), // from Gandalf with plain value, tag=26, type=string
	VsaGandalfSapGroupName3(&'a [u8]), // from Gandalf with plain value, tag=27, type=string
	VsaGandalfSapGroupName4(&'a [u8]), // from Gandalf with plain value, tag=28, type=string
	VsaGandalfSapGroupName5(&'a [u8]), // from Gandalf with plain value, tag=29, type=string
	VsaGandalfHuntGroup(&'a [u8]), // from Gandalf with plain value, tag=30, type=string
	VsaGandalfModemMode(gandalf::GandalfModemMode), // from Gandalf with Enum value, tag=31, type=integer
	VsaGandalfModemRequired1(gandalf::GandalfModemRequired1), // from Gandalf with Enum value, tag=32, type=integer
	VsaGandalfModemRequired2(gandalf::GandalfModemRequired2), // from Gandalf with Enum value, tag=33, type=integer
	VsaAcctSessionInputOctets(u32), // from Gemtek with plain value, tag=21, type=integer
	VsaAcctSessionInputGigawords(u32), // from Gemtek with plain value, tag=22, type=integer
	VsaAcctSessionOutputOctets(u32), // from Gemtek with plain value, tag=23, type=integer
	VsaAcctSessionOutputGigawords(u32), // from Gemtek with plain value, tag=24, type=integer
	VsaAcctSessionOctets(u32), // from Gemtek with plain value, tag=25, type=integer
	VsaAcctSessionGigawords(u32), // from Gemtek with plain value, tag=26, type=integer
	VsaH3CInputPeakRate(u32), // from H3C with plain value, tag=1, type=integer
	VsaH3CInputAverageRate(u32), // from H3C with plain value, tag=2, type=integer
	VsaH3CInputBasicRate(u32), // from H3C with plain value, tag=3, type=integer
	VsaH3CRemanentVolume(u32), // from H3C with plain value, tag=15, type=integer
	VsaH3CCommand(h3c::H3CCommand), // from H3C with Enum value, tag=20, type=integer
	VsaH3CControlIdentifier(u32), // from H3C with plain value, tag=24, type=integer
	VsaH3CResultCode(u32), // from H3C with plain value, tag=25, type=integer
	VsaH3CConnectId(u32), // from H3C with plain value, tag=26, type=integer
	VsaH3CFtpDirectory(&'a [u8]), // from H3C with plain value, tag=28, type=string
	VsaH3CExecPrivilege(h3c::H3CExecPrivilege), // from H3C with Enum value, tag=29, type=integer
	VsaH3CNasStartupTimestamp(u32), // from H3C with plain value, tag=59, type=integer
	VsaH3CIpHostAddr(&'a [u8]), // from H3C with plain value, tag=60, type=string
	VsaH3CUserNotify(&'a [u8]), // from H3C with plain value, tag=61, type=string
	VsaH3CUserHeartbeat(&'a [u8]), // from H3C with plain value, tag=62, type=string
	VsaH3CUserGroup(&'a [u8]), // from H3C with plain value, tag=140, type=string
	VsaH3CSecurityLevel(u32), // from H3C with plain value, tag=141, type=integer
	VsaH3CInputIntervalOctets(u32), // from H3C with plain value, tag=201, type=integer
	VsaH3COutputIntervalOctets(u32), // from H3C with plain value, tag=202, type=integer
	VsaH3CInputIntervalPackets(u32), // from H3C with plain value, tag=203, type=integer
	VsaH3COutputIntervalPackets(u32), // from H3C with plain value, tag=204, type=integer
	VsaH3CInputIntervalGigawords(u32), // from H3C with plain value, tag=205, type=integer
	VsaH3COutputIntervalGigawords(u32), // from H3C with plain value, tag=206, type=integer
	VsaH3CBackupNasIp(Ipv4Addr), // from H3C with plain value, tag=207, type=ipaddr
	VsaH3CProductId(&'a [u8]), // from H3C with plain value, tag=255, type=string
	VsaHpPrivilegeLevel(u32), // from HP with plain value, tag=1, type=integer
	VsaHpCommandString(&'a [u8]), // from HP with plain value, tag=2, type=string
	VsaHpCommandException(hp::HpCommandException), // from HP with Enum value, tag=3, type=integer
	VsaHpManagementProtocol(hp::HpManagementProtocol), // from HP with Enum value, tag=26, type=integer
	VsaHpPortClientLimitDot1X(u32), // from HP with plain value, tag=10, type=integer
	VsaHpPortClientLimitMa(u32), // from HP with plain value, tag=11, type=integer
	VsaHpPortClientLimitWa(u32), // from HP with plain value, tag=12, type=integer
	VsaHpPortAuthModeDot1X(hp::HpPortAuthModeDot1X), // from HP with Enum value, tag=13, type=integer
	VsaHpPortBounceHost(u32), // from HP with plain value, tag=23, type=integer
	VsaHpCaptivePortalUrl(&'a [u8]), // from HP with plain value, tag=24, type=string
	VsaHpPortPriorityRegenerationTable(&'a [u8]), // from HP with plain value, tag=40, type=string
	VsaHpBandwidthMaxIngress(u32), // from HP with plain value, tag=46, type=integer
	VsaHpBandwidthMaxEgress(u32), // from HP with plain value, tag=48, type=integer
	VsaHpIpFilterRaw(&'a [u8]), // from HP with plain value, tag=61, type=string
	VsaHpNasRulesIpv6(u32), // from HP with plain value, tag=63, type=integer
	VsaHpEgressVlanid(u32), // from HP with plain value, tag=64, type=integer
	VsaHpEgressVlanName(&'a [u8]), // from HP with plain value, tag=65, type=string
	VsaHpCapabilityAdvert(&'a [u8]), // from HP with plain value, tag=255, type=octets
	VsaHuaweiInputBurstSize(u32), // from Huawei with plain value, tag=1, type=integer
	VsaHuaweiInputAverageRate(u32), // from Huawei with plain value, tag=2, type=integer
	VsaHuaweiInputPeakRate(u32), // from Huawei with plain value, tag=3, type=integer
	VsaHuaweiOutputBurstSize(u32), // from Huawei with plain value, tag=4, type=integer
	VsaHuaweiOutputAverageRate(u32), // from Huawei with plain value, tag=5, type=integer
	VsaHuaweiOutputPeakRate(u32), // from Huawei with plain value, tag=6, type=integer
	VsaHuaweiInKbBeforeTSwitch(u32), // from Huawei with plain value, tag=7, type=integer
	VsaHuaweiOutKbBeforeTSwitch(u32), // from Huawei with plain value, tag=8, type=integer
	VsaHuaweiInPktBeforeTSwitch(u32), // from Huawei with plain value, tag=9, type=integer
	VsaHuaweiOutPktBeforeTSwitch(u32), // from Huawei with plain value, tag=10, type=integer
	VsaHuaweiInKbAfterTSwitch(u32), // from Huawei with plain value, tag=11, type=integer
	VsaHuaweiOutKbAfterTSwitch(u32), // from Huawei with plain value, tag=12, type=integer
	VsaHuaweiInPktAfterTSwitch(u32), // from Huawei with plain value, tag=13, type=integer
	VsaHuaweiOutPktAfterTSwitch(u32), // from Huawei with plain value, tag=14, type=integer
	VsaHuaweiRemanentVolume(u32), // from Huawei with plain value, tag=15, type=integer
	VsaHuaweiTariffSwitchInterval(u32), // from Huawei with plain value, tag=16, type=integer
	VsaHuaweiIspId(&'a [u8]), // from Huawei with plain value, tag=17, type=string
	VsaHuaweiMaxUsersPerLogicPort(u32), // from Huawei with plain value, tag=18, type=integer
	VsaHuaweiCommand(u32), // from Huawei with plain value, tag=20, type=integer
	VsaHuaweiPriority(u32), // from Huawei with plain value, tag=22, type=integer
	VsaHuaweiControlIdentifier(u32), // from Huawei with plain value, tag=24, type=integer
	VsaHuaweiResultCode(huawei::HuaweiResultCode), // from Huawei with Enum value, tag=25, type=integer
	VsaHuaweiConnectId(u32), // from Huawei with plain value, tag=26, type=integer
	VsaHuaweiPortalurl(&'a [u8]), // from Huawei with plain value, tag=27, type=string
	VsaHuaweiFtpDirectory(&'a [u8]), // from Huawei with plain value, tag=28, type=string
	VsaHuaweiExecPrivilege(u32), // from Huawei with plain value, tag=29, type=integer
	VsaHuaweiIpAddress(u32), // from Huawei with plain value, tag=30, type=integer
	VsaHuaweiQosProfileName(&'a [u8]), // from Huawei with plain value, tag=31, type=string
	VsaHuaweiSipServer(&'a [u8]), // from Huawei with plain value, tag=32, type=string
	VsaHuaweiUserPassword(&'a [u8]), // from Huawei with plain value, tag=33, type=string
	VsaHuaweiCommandMode(&'a [u8]), // from Huawei with plain value, tag=34, type=string
	VsaHuaweiRenewalTime(u32), // from Huawei with plain value, tag=35, type=integer
	VsaHuaweiRebindingTime(u32), // from Huawei with plain value, tag=36, type=integer
	VsaHuaweiIgmpEnable(u32), // from Huawei with plain value, tag=37, type=integer
	VsaHuaweiDestnationIpAddr(&'a [u8]), // from Huawei with plain value, tag=39, type=string
	VsaHuaweiDestnationVolume(&'a [u8]), // from Huawei with plain value, tag=40, type=string
	VsaHuaweiStartupStamp(u32), // from Huawei with plain value, tag=59, type=integer
	VsaHuaweiIphostAddr(&'a [u8]), // from Huawei with plain value, tag=60, type=string
	VsaHuaweiUpPriority(u32), // from Huawei with plain value, tag=61, type=integer
	VsaHuaweiDownPriority(u32), // from Huawei with plain value, tag=62, type=integer
	VsaHuaweiTunnelVpnInstance(&'a [u8]), // from Huawei with plain value, tag=63, type=string
	VsaHuaweiVtName(u32), // from Huawei with plain value, tag=64, type=integer
	VsaHuaweiUserDate(&'a [u8]), // from Huawei with plain value, tag=65, type=string
	VsaHuaweiUserClass(&'a [u8]), // from Huawei with plain value, tag=66, type=string
	VsaHuaweiPppNcpType(u32), // from Huawei with plain value, tag=70, type=integer
	VsaHuaweiVsiName(&'a [u8]), // from Huawei with plain value, tag=71, type=string
	VsaHuaweiSubnetMask(Ipv4Addr), // from Huawei with plain value, tag=72, type=ipaddr
	VsaHuaweiGatewayAddress(Ipv4Addr), // from Huawei with plain value, tag=73, type=ipaddr
	VsaHuaweiLeaseTime(u32), // from Huawei with plain value, tag=74, type=integer
	VsaHuaweiPrimaryWins(Ipv4Addr), // from Huawei with plain value, tag=75, type=ipaddr
	VsaHuaweiSecondaryWins(Ipv4Addr), // from Huawei with plain value, tag=76, type=ipaddr
	VsaHuaweiInputPeakBurstSize(u32), // from Huawei with plain value, tag=77, type=integer
	VsaHuaweiOutputPeakBurstSize(u32), // from Huawei with plain value, tag=78, type=integer
	VsaHuaweiReducedCir(u32), // from Huawei with plain value, tag=79, type=integer
	VsaHuaweiTunnelSessionLimit(u32), // from Huawei with plain value, tag=80, type=integer
	VsaHuaweiZoneName(&'a [u8]), // from Huawei with plain value, tag=81, type=string
	VsaHuaweiDataFilter(&'a [u8]), // from Huawei with plain value, tag=82, type=string
	VsaHuaweiAccessService(&'a [u8]), // from Huawei with plain value, tag=83, type=string
	VsaHuaweiAccountingLevel(u32), // from Huawei with plain value, tag=84, type=integer
	VsaHuaweiPortalMode(huawei::HuaweiPortalMode), // from Huawei with Enum value, tag=85, type=integer
	VsaHuaweiDpiPolicyName(&'a [u8]), // from Huawei with plain value, tag=86, type=string
	VsaHuaweiPolicyRoute(Ipv4Addr), // from Huawei with plain value, tag=87, type=ipaddr
	VsaHuaweiFramedPool(&'a [u8]), // from Huawei with plain value, tag=88, type=string
	VsaHuaweiL2TpTerminateCause(&'a [u8]), // from Huawei with plain value, tag=89, type=string
	VsaHuaweiMultiAccountMode(u32), // from Huawei with plain value, tag=90, type=integer
	VsaHuaweiQueueProfile(&'a [u8]), // from Huawei with plain value, tag=91, type=string
	VsaHuaweiLayer4SessionLimit(u32), // from Huawei with plain value, tag=92, type=integer
	VsaHuaweiMulticastProfile(&'a [u8]), // from Huawei with plain value, tag=93, type=string
	VsaHuaweiVpnInstance(&'a [u8]), // from Huawei with plain value, tag=94, type=string
	VsaHuaweiPolicyName(&'a [u8]), // from Huawei with plain value, tag=95, type=string
	VsaHuaweiTunnelGroupName(&'a [u8]), // from Huawei with plain value, tag=96, type=string
	VsaHuaweiMulticastSourceGroup(&'a [u8]), // from Huawei with plain value, tag=97, type=string
	VsaHuaweiMulticastReceiveGroup(Ipv4Addr), // from Huawei with plain value, tag=98, type=ipaddr
	VsaHuaweiUserMulticastType(u32), // from Huawei with plain value, tag=99, type=integer
	VsaHuaweiReducedPir(u32), // from Huawei with plain value, tag=100, type=integer
	VsaHuaweiLiId(&'a [u8]), // from Huawei with plain value, tag=101, type=string
	VsaHuaweiLiMdAddress(Ipv4Addr), // from Huawei with plain value, tag=102, type=ipaddr
	VsaHuaweiLiMdPort(u32), // from Huawei with plain value, tag=103, type=integer
	VsaHuaweiLiMdVpninstance(&'a [u8]), // from Huawei with plain value, tag=104, type=string
	VsaHuaweiServiceChgCmd(u32), // from Huawei with plain value, tag=105, type=integer
	VsaHuaweiAcctPacketType(u32), // from Huawei with plain value, tag=106, type=integer
	VsaHuaweiCallReference(u32), // from Huawei with plain value, tag=107, type=integer
	VsaHuaweiPstnPort(u32), // from Huawei with plain value, tag=108, type=integer
	VsaHuaweiVoipServiceType(u32), // from Huawei with plain value, tag=109, type=integer
	VsaHuaweiAcctConnectionTime(u32), // from Huawei with plain value, tag=110, type=integer
	VsaHuaweiErrorReason(u32), // from Huawei with plain value, tag=112, type=integer
	VsaHuaweiRemainMonney(u32), // from Huawei with plain value, tag=113, type=integer
	VsaHuaweiOrgGkIpaddr(Ipv4Addr), // from Huawei with plain value, tag=123, type=ipaddr
	VsaHuaweiOrgGwIpaddr(Ipv4Addr), // from Huawei with plain value, tag=124, type=ipaddr
	VsaHuaweiDstGkIpaddr(Ipv4Addr), // from Huawei with plain value, tag=125, type=ipaddr
	VsaHuaweiDstGwIpaddr(Ipv4Addr), // from Huawei with plain value, tag=126, type=ipaddr
	VsaHuaweiAccessNum(&'a [u8]), // from Huawei with plain value, tag=127, type=string
	VsaHuaweiRemainTime(u32), // from Huawei with plain value, tag=128, type=integer
	VsaHuaweiCodecType(u32), // from Huawei with plain value, tag=131, type=integer
	VsaHuaweiTransferNum(&'a [u8]), // from Huawei with plain value, tag=132, type=string
	VsaHuaweiNewUserName(&'a [u8]), // from Huawei with plain value, tag=133, type=string
	VsaHuaweiTransferStationId(&'a [u8]), // from Huawei with plain value, tag=134, type=string
	VsaHuaweiPrimaryDns(Ipv4Addr), // from Huawei with plain value, tag=135, type=ipaddr
	VsaHuaweiSecondaryDns(Ipv4Addr), // from Huawei with plain value, tag=136, type=ipaddr
	VsaHuaweiOnlyAccountType(u32), // from Huawei with plain value, tag=137, type=integer
	VsaHuaweiDomainName(&'a [u8]), // from Huawei with plain value, tag=138, type=string
	VsaHuaweiAncpProfile(&'a [u8]), // from Huawei with plain value, tag=139, type=string
	VsaHuaweiHttpRedirectUrl(&'a [u8]), // from Huawei with plain value, tag=140, type=string
	VsaHuaweiLoopbackAddress(&'a [u8]), // from Huawei with plain value, tag=141, type=string
	VsaHuaweiQosProfileType(huawei::HuaweiQosProfileType), // from Huawei with Enum value, tag=142, type=integer
	VsaHuaweiMaxListNum(u32), // from Huawei with plain value, tag=143, type=integer
	VsaHuaweiAcctIpv6InputOctets(u32), // from Huawei with plain value, tag=144, type=integer
	VsaHuaweiAcctIpv6OutputOctets(u32), // from Huawei with plain value, tag=145, type=integer
	VsaHuaweiAcctIpv6InputPackets(u32), // from Huawei with plain value, tag=146, type=integer
	VsaHuaweiAcctIpv6OutputPackets(u32), // from Huawei with plain value, tag=147, type=integer
	VsaHuaweiAcctIpv6InputGigawords(u32), // from Huawei with plain value, tag=148, type=integer
	VsaHuaweiAcctIpv6OutputGigawords(u32), // from Huawei with plain value, tag=149, type=integer
	VsaHuaweiDhcpv6Option37(&'a [u8]), // from Huawei with plain value, tag=150, type=string
	VsaHuaweiDhcpv6Option38(&'a [u8]), // from Huawei with plain value, tag=151, type=string
	VsaHuaweiUserMac(&'a [u8]), // from Huawei with plain value, tag=153, type=string
	VsaHuaweiDnsServerIpv6Address(&'a [u8]), // from Huawei with plain value, tag=154, type=ipv6prefix
	VsaHuaweiDhcpv4Option121(&'a [u8]), // from Huawei with plain value, tag=155, type=string
	VsaHuaweiDhcpv4Option43(&'a [u8]), // from Huawei with plain value, tag=156, type=string
	VsaHuaweiFramedPoolGroup(&'a [u8]), // from Huawei with plain value, tag=157, type=string
	VsaHuaweiFramedIpv6Address(&'a [u8]), // from Huawei with plain value, tag=158, type=ipv6prefix
	VsaHuaweiAcctUpdateAddress(u32), // from Huawei with plain value, tag=159, type=integer
	VsaHuaweiNatPolicyName(&'a [u8]), // from Huawei with plain value, tag=160, type=string
	VsaHuaweiNatPublicAddress(&'a [u8]), // from Huawei with plain value, tag=161, type=string
	VsaHuaweiNatStartPort(&'a [u8]), // from Huawei with plain value, tag=162, type=string
	VsaHuaweiNatEndPort(&'a [u8]), // from Huawei with plain value, tag=163, type=string
	VsaHuaweiNatPortForwarding(&'a [u8]), // from Huawei with plain value, tag=164, type=string
	VsaHuaweiNatPortRangeUpdate(u32), // from Huawei with plain value, tag=165, type=integer
	VsaHuaweiDsLiteTunnelName(&'a [u8]), // from Huawei with plain value, tag=166, type=string
	VsaHuaweiPcpServerName(&'a [u8]), // from Huawei with plain value, tag=167, type=string
	VsaHuaweiPublicIpAddrState(huawei::HuaweiPublicIpAddrState), // from Huawei with Enum value, tag=168, type=integer
	VsaHuaweiAuthType(huawei::HuaweiAuthType), // from Huawei with Enum value, tag=180, type=integer
	VsaHuaweiAcctTerminateSubcause(&'a [u8]), // from Huawei with plain value, tag=181, type=string
	VsaHuaweiDownQosProfileName(&'a [u8]), // from Huawei with plain value, tag=182, type=string
	VsaHuaweiPortMirror(huawei::HuaweiPortMirror), // from Huawei with Enum value, tag=183, type=integer
	VsaHuaweiAccountInfo(&'a [u8]), // from Huawei with plain value, tag=184, type=string
	VsaHuaweiServiceInfo(&'a [u8]), // from Huawei with plain value, tag=185, type=string
	VsaHuaweiDhcpOption(&'a [u8]), // from Huawei with plain value, tag=187, type=octets
	VsaHuaweiAvpair(&'a [u8]), // from Huawei with plain value, tag=188, type=string
	VsaHuaweiDelegatedIpv6PrefixPool(&'a [u8]), // from Huawei with plain value, tag=191, type=string
	VsaHuaweiIpv6PrefixLease(&'a [u8]), // from Huawei with plain value, tag=192, type=octets
	VsaHuaweiIpv6AddressLease(&'a [u8]), // from Huawei with plain value, tag=193, type=octets
	VsaHuaweiIpv6PolicyRoute(&'a [u8]), // from Huawei with plain value, tag=194, type=ipv6prefix
	VsaHuaweiMngIpv6(huawei::HuaweiMngIpv6), // from Huawei with Enum value, tag=196, type=integer
	VsaHuaweiFlowInfo(&'a [u8]), // from Huawei with plain value, tag=211, type=string
	VsaHuaweiFlowId(u32), // from Huawei with plain value, tag=212, type=integer
	VsaHuaweiDhcpServerIp(Ipv4Addr), // from Huawei with plain value, tag=214, type=ipaddr
	VsaHuaweiApplicationType(huawei::HuaweiApplicationType), // from Huawei with Enum value, tag=215, type=integer
	VsaHuaweiIndicationFlag(&'a [u8]), // from Huawei with plain value, tag=216, type=octets
	VsaHuaweiOriginalNasIpAddress(Ipv4Addr), // from Huawei with plain value, tag=217, type=ipaddr
	VsaHuaweiUserPriority(huawei::HuaweiUserPriority), // from Huawei with Enum value, tag=218, type=integer
	VsaHuaweiAcsUrl(&'a [u8]), // from Huawei with plain value, tag=219, type=string
	VsaHuaweiProvisionCode(&'a [u8]), // from Huawei with plain value, tag=220, type=string
	VsaHuaweiApplicationScene(&'a [u8]), // from Huawei with plain value, tag=221, type=octets
	VsaHuaweiMsMaximumMacStudyNumber(&'a [u8]), // from Huawei with plain value, tag=222, type=octets
	VsaHuaweiGgsnVendor(&'a [u8]), // from Huawei with plain value, tag=232, type=string
	VsaHuaweiGgsnVersion(&'a [u8]), // from Huawei with plain value, tag=233, type=string
	VsaHuaweiWebUrl(&'a [u8]), // from Huawei with plain value, tag=253, type=string
	VsaHuaweiVersion(&'a [u8]), // from Huawei with plain value, tag=254, type=string
	VsaHuaweiProductId(&'a [u8]), // from Huawei with plain value, tag=255, type=string
	VsaAmInterruptHtmlfile(&'a [u8]), // from IEA-Software with plain value, tag=1, type=string
	VsaAmInterruptInterval(u32), // from IEA-Software with plain value, tag=2, type=integer
	VsaAmInterruptTimeout(u32), // from IEA-Software with plain value, tag=3, type=integer
	VsaAmStatusHtmlfile(&'a [u8]), // from IEA-Software with plain value, tag=4, type=string
	VsaAmHttpProxyPort(u32), // from IEA-Software with plain value, tag=5, type=integer
	VsaAmAckHtmlfile(&'a [u8]), // from IEA-Software with plain value, tag=6, type=string
	VsaAmNakHtmlfile(&'a [u8]), // from IEA-Software with plain value, tag=7, type=string
	VsaAmBandwidthPool(&'a [u8]), // from IEA-Software with plain value, tag=8, type=string
	VsaAmBandwidthPoolMaxUp(u32), // from IEA-Software with plain value, tag=9, type=integer
	VsaAmBandwidthPoolMaxDown(u32), // from IEA-Software with plain value, tag=10, type=integer
	VsaAmMirroring(iea_software::AmMirroring), // from IEA-Software with Enum value, tag=11, type=integer
	VsaAmDisconnectAccess(iea_software::AmDisconnectAccess), // from IEA-Software with Enum value, tag=12, type=integer
	VsaInfonetProxy(&'a [u8]), // from infonet with plain value, tag=238, type=string
	VsaInfonetConfig(&'a [u8]), // from infonet with plain value, tag=239, type=string
	VsaInfonetMcsCountry(&'a [u8]), // from infonet with plain value, tag=240, type=string
	VsaInfonetMcsRegion(&'a [u8]), // from infonet with plain value, tag=241, type=string
	VsaInfonetMcsOffPeak(&'a [u8]), // from infonet with plain value, tag=242, type=string
	VsaInfonetMcsOverflow(&'a [u8]), // from infonet with plain value, tag=243, type=string
	VsaInfonetMcsPort(&'a [u8]), // from infonet with plain value, tag=244, type=string
	VsaInfonetMcsPortCount(&'a [u8]), // from infonet with plain value, tag=245, type=string
	VsaInfonetAccountNumber(&'a [u8]), // from infonet with plain value, tag=247, type=string
	VsaInfonetType(&'a [u8]), // from infonet with plain value, tag=248, type=string
	VsaInfonetPoolRequest(&'a [u8]), // from infonet with plain value, tag=252, type=string
	VsaInfonetSurchargeType(u32), // from infonet with plain value, tag=254, type=integer
	VsaInfonetNasLocation(&'a [u8]), // from infonet with plain value, tag=255, type=string
	VsaInfonetRandomIpPool(&'a [u8]), // from infonet with plain value, tag=246, type=string
	VsaInfonetRealmType(&'a [u8]), // from infonet with plain value, tag=249, type=string
	VsaInfonetLoginhostDest(&'a [u8]), // from infonet with plain value, tag=250, type=string
	VsaInfonetTunnelDecisionIp(&'a [u8]), // from infonet with plain value, tag=251, type=string
	VsaIssanniSoftflowTemplate(&'a [u8]), // from Issanni with plain value, tag=1, type=string
	VsaIssanniNatSupport(&'a [u8]), // from Issanni with plain value, tag=2, type=string
	VsaIssanniRoutingContext(&'a [u8]), // from Issanni with plain value, tag=3, type=string
	VsaIssanniTunnelName(&'a [u8]), // from Issanni with plain value, tag=4, type=string
	VsaIssanniIpPoolName(&'a [u8]), // from Issanni with plain value, tag=5, type=string
	VsaIssanniPppoeUrl(&'a [u8]), // from Issanni with plain value, tag=6, type=string
	VsaIssanniPppoeMotm(&'a [u8]), // from Issanni with plain value, tag=7, type=string
	VsaIssanniService(&'a [u8]), // from Issanni with plain value, tag=8, type=string
	VsaIssanniPriDns(Ipv4Addr), // from Issanni with plain value, tag=9, type=ipaddr
	VsaIssanniSecDns(Ipv4Addr), // from Issanni with plain value, tag=10, type=ipaddr
	VsaIssanniPriNbns(Ipv4Addr), // from Issanni with plain value, tag=11, type=ipaddr
	VsaIssanniSecNbns(Ipv4Addr), // from Issanni with plain value, tag=12, type=ipaddr
	VsaIssanniTrafficClass(&'a [u8]), // from Issanni with plain value, tag=13, type=string
	VsaIssanniTunnelType(issanni::IssanniTunnelType), // from Issanni with Enum value, tag=14, type=integer
	VsaIssanniNatType(issanni::IssanniNatType), // from Issanni with Enum value, tag=15, type=integer
	VsaIssanniQosClass(&'a [u8]), // from Issanni with plain value, tag=16, type=string
	VsaIssanniInterfaceName(&'a [u8]), // from Issanni with plain value, tag=17, type=string
	VsaItkAuthServIp(Ipv4Addr), // from ITK with plain value, tag=100, type=ipaddr
	VsaItkAuthServProt(u32), // from ITK with plain value, tag=101, type=integer
	VsaItkProviderId(u32), // from ITK with plain value, tag=102, type=integer
	VsaItkUsergroup(u32), // from ITK with plain value, tag=103, type=integer
	VsaItkBanner(&'a [u8]), // from ITK with plain value, tag=104, type=string
	VsaItkUsernamePrompt(&'a [u8]), // from ITK with plain value, tag=105, type=string
	VsaItkPasswordPrompt(&'a [u8]), // from ITK with plain value, tag=106, type=string
	VsaItkWelcomeMessage(&'a [u8]), // from ITK with plain value, tag=107, type=string
	VsaItkPrompt(&'a [u8]), // from ITK with plain value, tag=108, type=string
	VsaItkIpPool(u32), // from ITK with plain value, tag=109, type=integer
	VsaItkTunnelIp(Ipv4Addr), // from ITK with plain value, tag=110, type=ipaddr
	VsaItkTunnelProt(u32), // from ITK with plain value, tag=111, type=integer
	VsaItkAcctServIp(Ipv4Addr), // from ITK with plain value, tag=112, type=ipaddr
	VsaItkAcctServProt(u32), // from ITK with plain value, tag=113, type=integer
	VsaItkFilterRule(&'a [u8]), // from ITK with plain value, tag=114, type=string
	VsaItkChannelBinding(u32), // from ITK with plain value, tag=115, type=integer
	VsaItkStartDelay(u32), // from ITK with plain value, tag=116, type=integer
	VsaItkNasName(&'a [u8]), // from ITK with plain value, tag=117, type=string
	VsaItkIsdnProt(u32), // from ITK with plain value, tag=118, type=integer
	VsaItkPppAuthType(u32), // from ITK with plain value, tag=119, type=integer
	VsaItkDialoutType(u32), // from ITK with plain value, tag=120, type=integer
	VsaItkFtpAuthIp(Ipv4Addr), // from ITK with plain value, tag=121, type=ipaddr
	VsaItkUsersDefaultEntry(&'a [u8]), // from ITK with plain value, tag=122, type=string
	VsaItkUsersDefaultPw(&'a [u8]), // from ITK with plain value, tag=123, type=string
	VsaItkAuthReqType(&'a [u8]), // from ITK with plain value, tag=124, type=string
	VsaItkModemPoolId(u32), // from ITK with plain value, tag=125, type=integer
	VsaItkModemInitString(&'a [u8]), // from ITK with plain value, tag=126, type=string
	VsaItkPppClientServerMode(u32), // from ITK with plain value, tag=127, type=integer
	VsaItkPppCompressionProt(&'a [u8]), // from ITK with plain value, tag=128, type=string
	VsaItkUsername(&'a [u8]), // from ITK with plain value, tag=129, type=string
	VsaItkDestNo(&'a [u8]), // from ITK with plain value, tag=130, type=string
	VsaItkDdi(&'a [u8]), // from ITK with plain value, tag=131, type=string
	VsaIpuMipSpi(u32), // from ipUnplugged with plain value, tag=51, type=integer
	VsaIpuMipKey(&'a [u8]), // from ipUnplugged with plain value, tag=52, type=string
	VsaIpuMipAlgType(u32), // from ipUnplugged with plain value, tag=53, type=integer
	VsaIpuMipAlgMode(u32), // from ipUnplugged with plain value, tag=54, type=integer
	VsaIpuMipReplayProt(u32), // from ipUnplugged with plain value, tag=55, type=integer
	VsaIpuIkeRemoteAddr(Ipv4Addr), // from ipUnplugged with plain value, tag=61, type=ipaddr
	VsaIpuIkeLocalAddr(Ipv4Addr), // from ipUnplugged with plain value, tag=62, type=ipaddr
	VsaIpuIkeAuth(&'a [u8]), // from ipUnplugged with plain value, tag=63, type=string
	VsaIpuIkeConfName(&'a [u8]), // from ipUnplugged with plain value, tag=64, type=string
	VsaIpuIkeCmd(&'a [u8]), // from ipUnplugged with plain value, tag=65, type=string
	VsaJuniperLocalUserName(&'a [u8]), // from Juniper with plain value, tag=1, type=string
	VsaJuniperAllowCommands(&'a [u8]), // from Juniper with plain value, tag=2, type=string
	VsaJuniperDenyCommands(&'a [u8]), // from Juniper with plain value, tag=3, type=string
	VsaJuniperAllowConfiguration(&'a [u8]), // from Juniper with plain value, tag=4, type=string
	VsaJuniperDenyConfiguration(&'a [u8]), // from Juniper with plain value, tag=5, type=string
	VsaJuniperInteractiveCommand(&'a [u8]), // from Juniper with plain value, tag=8, type=string
	VsaJuniperConfigurationChange(&'a [u8]), // from Juniper with plain value, tag=9, type=string
	VsaJuniperUserPermissions(&'a [u8]), // from Juniper with plain value, tag=10, type=string
	VsaJuniperJunosspaceProfile(&'a [u8]), // from Juniper with plain value, tag=11, type=String
	VsaJuniperCtpGroup(juniper::JuniperCtpGroup), // from Juniper with Enum value, tag=21, type=integer
	VsaJuniperCtpviewAppGroup(juniper::JuniperCtpviewAppGroup), // from Juniper with Enum value, tag=22, type=integer
	VsaJuniperCtpviewOsGroup(juniper::JuniperCtpviewOsGroup), // from Juniper with Enum value, tag=23, type=integer
	VsaJuniperPrimaryDns(Ipv4Addr), // from Juniper with plain value, tag=31, type=ipaddr
	VsaJuniperPrimaryWins(Ipv4Addr), // from Juniper with plain value, tag=32, type=ipaddr
	VsaJuniperSecondaryDns(Ipv4Addr), // from Juniper with plain value, tag=33, type=ipaddr
	VsaJuniperSecondaryWins(Ipv4Addr), // from Juniper with plain value, tag=34, type=ipaddr
	VsaJuniperInterfaceId(&'a [u8]), // from Juniper with plain value, tag=35, type=string
	VsaJuniperIpPoolName(&'a [u8]), // from Juniper with plain value, tag=36, type=string
	VsaJuniperKeepAlive(u32), // from Juniper with plain value, tag=37, type=integer
	VsaJuniperCosTrafficControlProfile(&'a [u8]), // from Juniper with plain value, tag=38, type=string
	VsaJuniperCosParameter(&'a [u8]), // from Juniper with plain value, tag=39, type=string
	VsaJuniperEncapsulationOverhead(u32), // from Juniper with plain value, tag=40, type=integer
	VsaJuniperCellOverhead(u32), // from Juniper with plain value, tag=41, type=integer
	VsaJuniperTxConnectSpeed(u32), // from Juniper with plain value, tag=42, type=integer
	VsaJuniperRxConnectSpeed(u32), // from Juniper with plain value, tag=43, type=integer
	VsaJuniperFirewallFilterName(&'a [u8]), // from Juniper with plain value, tag=44, type=string
	VsaJuniperPolicerParameter(&'a [u8]), // from Juniper with plain value, tag=45, type=string
	VsaJuniperLocalGroupName(&'a [u8]), // from Juniper with plain value, tag=46, type=string
	VsaJuniperLocalInterface(&'a [u8]), // from Juniper with plain value, tag=47, type=string
	VsaJuniperSwitchingFilter(&'a [u8]), // from Juniper with plain value, tag=48, type=string
	VsaJuniperVoipVlan(&'a [u8]), // from Juniper with plain value, tag=49, type=string
	VsaJradiusRequestId(&'a [u8]), // from JRadius with plain value, tag=1, type=string
	VsaJradiusSessionId(&'a [u8]), // from JRadius with plain value, tag=2, type=string
	VsaJradiusProxyClient(&'a [u8]), // from JRadius with plain value, tag=3, type=octets
	VsaKarlnetTurbocellName(&'a [u8]), // from KarlNet with plain value, tag=151, type=string
	VsaKarlnetTurbocellTxrate(karlnet::KarlnetTurbocellTxrate), // from KarlNet with Enum value, tag=152, type=integer
	VsaKarlnetTurbocellOpstate(karlnet::KarlnetTurbocellOpstate), // from KarlNet with Enum value, tag=153, type=integer
	VsaKarlnetTurbocellOpmode(karlnet::KarlnetTurbocellOpmode), // from KarlNet with Enum value, tag=154, type=integer
	VsaKinetoUmaReleaseIndicator(&'a [u8]), // from Kineto with plain value, tag=2, type=octets
	VsaKinetoUmaApRadioIdentity(&'a [u8]), // from Kineto with plain value, tag=3, type=octets
	VsaKinetoUmaCellIdentity(&'a [u8]), // from Kineto with plain value, tag=4, type=octets
	VsaKinetoUmaLocationAreaIdentification(&'a [u8]), // from Kineto with plain value, tag=5, type=octets
	VsaKinetoUmaCoverageIndicator(&'a [u8]), // from Kineto with plain value, tag=6, type=octets
	VsaKinetoUmaClassmark(&'a [u8]), // from Kineto with plain value, tag=7, type=octets
	VsaKinetoUmaGeographicalLocation(&'a [u8]), // from Kineto with plain value, tag=8, type=octets
	VsaKinetoUmaSgwIpAddress(&'a [u8]), // from Kineto with plain value, tag=9, type=octets
	VsaKinetoUmaSgwFqdn(&'a [u8]), // from Kineto with plain value, tag=10, type=octets
	VsaKinetoUmaRedirectionCounter(&'a [u8]), // from Kineto with plain value, tag=11, type=octets
	VsaKinetoUmaDiscoveryRejectCause(&'a [u8]), // from Kineto with plain value, tag=12, type=octets
	VsaKinetoUmaRrcState(&'a [u8]), // from Kineto with plain value, tag=17, type=octets
	VsaKinetoUmaRegisterRejectCause(&'a [u8]), // from Kineto with plain value, tag=21, type=octets
	VsaKinetoUmaRoutingAreaCode(&'a [u8]), // from Kineto with plain value, tag=41, type=octets
	VsaKinetoUmaApLocation(&'a [u8]), // from Kineto with plain value, tag=42, type=octets
	VsaKinetoUmaLocationStatus(&'a [u8]), // from Kineto with plain value, tag=44, type=octets
	VsaKinetoUmaUtranCellIdentity(&'a [u8]), // from Kineto with plain value, tag=49, type=octets
	VsaKinetoUmaLocationBlacklistIndicator(&'a [u8]), // from Kineto with plain value, tag=58, type=octets
	VsaKinetoUmaApServiceName(&'a [u8]), // from Kineto with plain value, tag=61, type=octets
	VsaKinetoUmaServiceZoneInformation(&'a [u8]), // from Kineto with plain value, tag=62, type=octets
	VsaKinetoUmaServingUncTableIndicator(&'a [u8]), // from Kineto with plain value, tag=67, type=octets
	VsaKinetoUmaRegistrationIndicators(&'a [u8]), // from Kineto with plain value, tag=68, type=octets
	VsaKinetoUmaUmaPlmnList(&'a [u8]), // from Kineto with plain value, tag=69, type=octets
	VsaKinetoUmaRequiredUmaServices(&'a [u8]), // from Kineto with plain value, tag=71, type=octets
	VsaKinetoUma3GCellIdentity(&'a [u8]), // from Kineto with plain value, tag=73, type=octets
	VsaKinetoUmaMsRadioIdentity(&'a [u8]), // from Kineto with plain value, tag=96, type=octets
	VsaKinetoUmaUncIpAddress(&'a [u8]), // from Kineto with plain value, tag=97, type=octets
	VsaKinetoUmaUncFqdn(&'a [u8]), // from Kineto with plain value, tag=98, type=octets
	VsaKinetoUrrTransactionType(&'a [u8]), // from Kineto with plain value, tag=65281, type=octets
	VsaKinetoLocationKey(&'a [u8]), // from Kineto with plain value, tag=65282, type=octets
	VsaKinetoUpClientRemoteAddress(&'a [u8]), // from Kineto with plain value, tag=65283, type=octets
	VsaKinetoHandInControlFlag(&'a [u8]), // from Kineto with plain value, tag=65284, type=octets
	VsaKinetoHandOutControlFlag(&'a [u8]), // from Kineto with plain value, tag=65285, type=octets
	VsaKinetoBillingRateIndicator(&'a [u8]), // from Kineto with plain value, tag=65286, type=octets
	VsaKinetoServiceAreaCode(&'a [u8]), // from Kineto with plain value, tag=65289, type=octets
	VsaKwIuhMessageType(&'a [u8]), // from Kineto with plain value, tag=65408, type=string
	VsaKwHnbRemoteAddress(Ipv4Addr), // from Kineto with plain value, tag=65409, type=ipaddr
	VsaKwHnbIdentity(&'a [u8]), // from Kineto with plain value, tag=65410, type=string
	VsaKwHnbLocInfoMacroCoverageInd(kineto::KwHnbLocInfoMacroCoverageInd), // from Kineto with Enum value, tag=65411, type=integer
	VsaKwHnbLocInfoGeranCellId(&'a [u8]), // from Kineto with plain value, tag=65412, type=string
	VsaKwHnbLocInfoUtranCellId(&'a [u8]), // from Kineto with plain value, tag=65413, type=string
	VsaKwHnbLocInfoGeoCoordinates(kineto::KwHnbLocInfoGeoCoordinates), // from Kineto with Enum value, tag=65414, type=integer
	VsaKwHnbLocInfoAltitudeDirection(kineto::KwHnbLocInfoAltitudeDirection), // from Kineto with Enum value, tag=65415, type=integer
	VsaKwHnbLocInfoIpAddress(&'a [u8]), // from Kineto with plain value, tag=65416, type=string
	VsaKwHnbPlmnId(&'a [u8]), // from Kineto with plain value, tag=65417, type=string
	VsaKwHnbCellId(&'a [u8]), // from Kineto with plain value, tag=65418, type=string
	VsaKwHnbLac(&'a [u8]), // from Kineto with plain value, tag=65419, type=string
	VsaKwHnbRac(&'a [u8]), // from Kineto with plain value, tag=65420, type=string
	VsaKwHnbSac(&'a [u8]), // from Kineto with plain value, tag=65421, type=string
	VsaKwHnbCsgId(&'a [u8]), // from Kineto with plain value, tag=65422, type=string
	VsaKwUeCapabilities(kineto::KwUeCapabilities), // from Kineto with Enum value, tag=65423, type=integer
	VsaKwHnbLocationAreaInd(&'a [u8]), // from Kineto with plain value, tag=65424, type=octets
	VsaKwIuhBillingRateIndicator(&'a [u8]), // from Kineto with plain value, tag=65425, type=octets
	VsaKwRegistrationRejectCause(&'a [u8]), // from Kineto with plain value, tag=65426, type=octets
	VsaKwHnbLocationBlacklistInd(&'a [u8]), // from Kineto with plain value, tag=65427, type=octets
	VsaKwHnbCellAccessMode(&'a [u8]), // from Kineto with plain value, tag=65428, type=octets
	VsaKwUeMembershipStatus(&'a [u8]), // from Kineto with plain value, tag=65429, type=octets
	VsaLcsTrafficLimit(u32), // from Lancom with plain value, tag=1, type=integer
	VsaLcsMacAddress(&'a [u8]), // from Lancom with plain value, tag=2, type=string
	VsaLcsRedirectionUrl(&'a [u8]), // from Lancom with plain value, tag=3, type=string
	VsaLcsComment(&'a [u8]), // from Lancom with plain value, tag=4, type=string
	VsaLcsAccountEnd(u32), // from Lancom with plain value, tag=5, type=integer
	VsaLcsWpaPassphrase(&'a [u8]), // from Lancom with plain value, tag=6, type=string
	VsaLcsPbspotusername(&'a [u8]), // from Lancom with plain value, tag=7, type=string
	VsaLcsTxratelimit(u32), // from Lancom with plain value, tag=8, type=integer
	VsaLcsRxratelimit(u32), // from Lancom with plain value, tag=9, type=integer
	VsaLcsAccessRights(u32), // from Lancom with plain value, tag=11, type=integer
	VsaLcsFunctionRights(u32), // from Lancom with plain value, tag=12, type=integer
	VsaLcsAdvertisementUrl(&'a [u8]), // from Lancom with plain value, tag=13, type=string
	VsaLcsAdvertisementInterval(u32), // from Lancom with plain value, tag=14, type=integer
	VsaLcsTrafficLimitGigawords(u32), // from Lancom with plain value, tag=15, type=integer
	VsaLcsOrigNasIdentifier(&'a [u8]), // from Lancom with plain value, tag=16, type=string
	VsaLcsOrigNasIpAddress(Ipv4Addr), // from Lancom with plain value, tag=17, type=ipaddr
	VsaLcsOrigNasIpv6Address(Ipv6Addr), // from Lancom with plain value, tag=18, type=ipv6addr
	VsaLcsIkev2LocalPassword(&'a [u8]), // from Lancom with plain value, tag=19, type=string
	VsaLcsIkev2RemotePassword(&'a [u8]), // from Lancom with plain value, tag=20, type=string
	VsaLcsDnsServerIpv4Address(Ipv4Addr), // from Lancom with plain value, tag=21, type=ipaddr
	VsaLcsVpnIpv4Rule(&'a [u8]), // from Lancom with plain value, tag=22, type=string
	VsaLcsVpnIpv6Rule(&'a [u8]), // from Lancom with plain value, tag=23, type=string
	VsaLcsRoutingTag(u32), // from Lancom with plain value, tag=24, type=integer
	VsaLcsIkev2Ipv4Route(&'a [u8]), // from Lancom with plain value, tag=25, type=string
	VsaLcsIkev2Ipv6Route(&'a [u8]), // from Lancom with plain value, tag=26, type=string
	VsaLeTerminateDetail(&'a [u8]), // from Livingston with plain value, tag=2, type=string
	VsaLeAdviceOfCharge(&'a [u8]), // from Livingston with plain value, tag=3, type=string
	VsaLeConnectDetail(&'a [u8]), // from Livingston with plain value, tag=4, type=string
	VsaLeIpPool(&'a [u8]), // from Livingston with plain value, tag=6, type=string
	VsaLeIpGateway(Ipv4Addr), // from Livingston with plain value, tag=7, type=ipaddr
	VsaLeModemInfo(&'a [u8]), // from Livingston with plain value, tag=8, type=string
	VsaLeIpsecLogOptions(livingston::LeIpsecLogOptions), // from Livingston with Enum value, tag=9, type=integer
	VsaLeIpsecDenyAction(livingston::LeIpsecDenyAction), // from Livingston with Enum value, tag=10, type=integer
	VsaLeIpsecActiveProfile(&'a [u8]), // from Livingston with plain value, tag=11, type=string
	VsaLeIpsecOutsourceProfile(&'a [u8]), // from Livingston with plain value, tag=12, type=string
	VsaLeIpsecPassiveProfile(&'a [u8]), // from Livingston with plain value, tag=13, type=string
	VsaLeNatTcpSessionTimeout(u32), // from Livingston with plain value, tag=14, type=integer
	VsaLeNatOtherSessionTimeout(u32), // from Livingston with plain value, tag=15, type=integer
	VsaLeNatLogOptions(livingston::LeNatLogOptions), // from Livingston with Enum value, tag=16, type=integer
	VsaLeNatSessDirFailAction(livingston::LeNatSessDirFailAction), // from Livingston with Enum value, tag=17, type=integer
	VsaLeNatInmap(&'a [u8]), // from Livingston with plain value, tag=18, type=string
	VsaLeNatOutmap(&'a [u8]), // from Livingston with plain value, tag=19, type=string
	VsaLeNatOutsourceInmap(&'a [u8]), // from Livingston with plain value, tag=20, type=string
	VsaLeNatOutsourceOutmap(&'a [u8]), // from Livingston with plain value, tag=21, type=string
	VsaLeAdminGroup(&'a [u8]), // from Livingston with plain value, tag=22, type=string
	VsaLeMulticastClient(livingston::LeMulticastClient), // from Livingston with Enum value, tag=23, type=integer
	VsaLocalWebClientIp(&'a [u8]), // from Local-Web with plain value, tag=192, type=string
	VsaLocalWebBorderRouter(&'a [u8]), // from Local-Web with plain value, tag=193, type=string
	VsaLocalWebTxLimit(u32), // from Local-Web with plain value, tag=200, type=integer
	VsaLocalWebRxLimit(u32), // from Local-Web with plain value, tag=201, type=integer
	VsaLocalWebAcctTime(u32), // from Local-Web with plain value, tag=210, type=integer
	VsaLocalWebAcctDuration(u32), // from Local-Web with plain value, tag=211, type=integer
	VsaLocalWebAcctInterimTxBytes(u32), // from Local-Web with plain value, tag=212, type=integer
	VsaLocalWebAcctInterimRxBytes(u32), // from Local-Web with plain value, tag=213, type=integer
	VsaLocalWebAcctInterimTxGigawords(u32), // from Local-Web with plain value, tag=214, type=integer
	VsaLocalWebAcctInterimRxGigawords(u32), // from Local-Web with plain value, tag=215, type=integer
	VsaLocalWebAcctInterimTxMgmt(u32), // from Local-Web with plain value, tag=216, type=integer
	VsaLocalWebAcctInterimRxMgmt(u32), // from Local-Web with plain value, tag=217, type=integer
	VsaLocalWebAcctTxMgmt(u32), // from Local-Web with plain value, tag=230, type=integer
	VsaLocalWebAcctRxMgmt(u32), // from Local-Web with plain value, tag=231, type=integer
	VsaLocalWebReauthCounter(u32), // from Local-Web with plain value, tag=240, type=integer
	VsaLucentMaxSharedUsers(u32), // from Lucent with plain value, tag=2, type=integer
	VsaLucentIpDscp(u32), // from Lucent with plain value, tag=3, type=integer
	VsaLucentX25X121SourceAddress(&'a [u8]), // from Lucent with plain value, tag=4, type=string
	VsaLucentPppCircuit(u32), // from Lucent with plain value, tag=5, type=integer
	VsaLucentPppCircuitName(&'a [u8]), // from Lucent with plain value, tag=6, type=string
	VsaLucentUuInfo(&'a [u8]), // from Lucent with plain value, tag=7, type=string
	VsaLucentUserPriority(u32), // from Lucent with plain value, tag=8, type=integer
	VsaLucentCirTimer(u32), // from Lucent with plain value, tag=9, type=integer
	VsaLucentFr08Mode(u32), // from Lucent with plain value, tag=10, type=integer
	VsaLucentDestinationNasPort(u32), // from Lucent with plain value, tag=11, type=integer
	VsaLucentFrSvcAddr(&'a [u8]), // from Lucent with plain value, tag=12, type=string
	VsaLucentNasPortFormat(u32), // from Lucent with plain value, tag=13, type=integer
	VsaLucentAtmFaultManagement(u32), // from Lucent with plain value, tag=14, type=integer
	VsaLucentAtmLoopbackCellLoss(u32), // from Lucent with plain value, tag=15, type=integer
	VsaLucentCktType(u32), // from Lucent with plain value, tag=16, type=integer
	VsaLucentSvcEnabled(u32), // from Lucent with plain value, tag=17, type=integer
	VsaLucentSessionType(u32), // from Lucent with plain value, tag=18, type=integer
	VsaLucentH323Gatekeeper(Ipv4Addr), // from Lucent with plain value, tag=19, type=ipaddr
	VsaLucentGlobalCallId(&'a [u8]), // from Lucent with plain value, tag=20, type=string
	VsaLucentH323ConferenceId(u32), // from Lucent with plain value, tag=21, type=integer
	VsaLucentH323DestinationNasId(Ipv4Addr), // from Lucent with plain value, tag=22, type=ipaddr
	VsaLucentH323DialedTime(u32), // from Lucent with plain value, tag=23, type=integer
	VsaLucentDialedNumber(&'a [u8]), // from Lucent with plain value, tag=24, type=string
	VsaLucentInterArrivalJitter(u32), // from Lucent with plain value, tag=25, type=integer
	VsaLucentDroppedOctets(u32), // from Lucent with plain value, tag=26, type=integer
	VsaLucentDroppedPackets(u32), // from Lucent with plain value, tag=27, type=integer
	VsaLucentAuthDelay(u32), // from Lucent with plain value, tag=28, type=integer
	VsaLucentX25PadX3Profile(u32), // from Lucent with plain value, tag=29, type=integer
	VsaLucentX25PadX3Parameters(&'a [u8]), // from Lucent with plain value, tag=30, type=string
	VsaLucentTunnelVrouterName(&'a [u8]), // from Lucent with plain value, tag=31, type=string
	VsaLucentX25ReverseCharging(u32), // from Lucent with plain value, tag=32, type=integer
	VsaLucentX25NuiPrompt(&'a [u8]), // from Lucent with plain value, tag=33, type=string
	VsaLucentX25NuiPasswordPrompt(&'a [u8]), // from Lucent with plain value, tag=34, type=string
	VsaLucentX25Cug(&'a [u8]), // from Lucent with plain value, tag=35, type=string
	VsaLucentX25PadAlias1(&'a [u8]), // from Lucent with plain value, tag=36, type=string
	VsaLucentX25PadAlias2(&'a [u8]), // from Lucent with plain value, tag=37, type=string
	VsaLucentX25PadAlias3(&'a [u8]), // from Lucent with plain value, tag=38, type=string
	VsaLucentX25X121Address(&'a [u8]), // from Lucent with plain value, tag=39, type=string
	VsaLucentX25Nui(&'a [u8]), // from Lucent with plain value, tag=40, type=string
	VsaLucentX25Rpoa(&'a [u8]), // from Lucent with plain value, tag=41, type=string
	VsaLucentX25PadPrompt(&'a [u8]), // from Lucent with plain value, tag=42, type=string
	VsaLucentX25PadBanner(&'a [u8]), // from Lucent with plain value, tag=43, type=string
	VsaLucentX25ProfileName(&'a [u8]), // from Lucent with plain value, tag=44, type=string
	VsaLucentRecvName(&'a [u8]), // from Lucent with plain value, tag=45, type=string
	VsaLucentBiDirectionalAuth(u32), // from Lucent with plain value, tag=46, type=integer
	VsaLucentMtu(u32), // from Lucent with plain value, tag=47, type=integer
	VsaLucentCallDirection(u32), // from Lucent with plain value, tag=48, type=integer
	VsaLucentServiceType(u32), // from Lucent with plain value, tag=49, type=integer
	VsaLucentFilterRequired(u32), // from Lucent with plain value, tag=50, type=integer
	VsaLucentTrafficShaper(u32), // from Lucent with plain value, tag=51, type=integer
	VsaLucentAccessInterceptLea(&'a [u8]), // from Lucent with plain value, tag=52, type=string
	VsaLucentAccessInterceptLog(&'a [u8]), // from Lucent with plain value, tag=53, type=string
	VsaLucentPrivateRouteTableId(&'a [u8]), // from Lucent with plain value, tag=54, type=string
	VsaLucentPrivateRouteRequired(u32), // from Lucent with plain value, tag=55, type=integer
	VsaLucentCacheRefresh(u32), // from Lucent with plain value, tag=56, type=integer
	VsaLucentCacheTime(u32), // from Lucent with plain value, tag=57, type=integer
	VsaLucentEgressEnabled(u32), // from Lucent with plain value, tag=58, type=integer
	VsaLucentQosUpstream(&'a [u8]), // from Lucent with plain value, tag=59, type=string
	VsaLucentQosDownstream(&'a [u8]), // from Lucent with plain value, tag=60, type=string
	VsaLucentAtmConnectVpi(u32), // from Lucent with plain value, tag=61, type=integer
	VsaLucentAtmConnectVci(u32), // from Lucent with plain value, tag=62, type=integer
	VsaLucentAtmConnectGroup(u32), // from Lucent with plain value, tag=63, type=integer
	VsaLucentAtmGroup(u32), // from Lucent with plain value, tag=64, type=integer
	VsaLucentIpxHeaderCompression(u32), // from Lucent with plain value, tag=65, type=integer
	VsaLucentCallingIdTypeOfNumber(u32), // from Lucent with plain value, tag=66, type=integer
	VsaLucentCallingIdNumberingPlan(u32), // from Lucent with plain value, tag=67, type=integer
	VsaLucentCallingIdPresentation(u32), // from Lucent with plain value, tag=68, type=integer
	VsaLucentCallingIdScreening(u32), // from Lucent with plain value, tag=69, type=integer
	VsaLucentBirEnable(u32), // from Lucent with plain value, tag=70, type=integer
	VsaLucentBirProxy(u32), // from Lucent with plain value, tag=71, type=integer
	VsaLucentBirBridgeGroup(u32), // from Lucent with plain value, tag=72, type=integer
	VsaLucentIpsecProfile(&'a [u8]), // from Lucent with plain value, tag=73, type=string
	VsaLucentPppoeEnable(u32), // from Lucent with plain value, tag=74, type=integer
	VsaLucentBridgeNonPppoe(u32), // from Lucent with plain value, tag=75, type=integer
	VsaLucentAtmDirect(u32), // from Lucent with plain value, tag=76, type=integer
	VsaLucentAtmDirectProfile(&'a [u8]), // from Lucent with plain value, tag=77, type=string
	VsaLucentClientPrimaryWins(Ipv4Addr), // from Lucent with plain value, tag=78, type=ipaddr
	VsaLucentClientSecondaryWins(Ipv4Addr), // from Lucent with plain value, tag=79, type=ipaddr
	VsaLucentClientAssignWins(u32), // from Lucent with plain value, tag=80, type=integer
	VsaLucentAuthType(u32), // from Lucent with plain value, tag=81, type=integer
	VsaLucentPortRedirProtocol(u32), // from Lucent with plain value, tag=82, type=integer
	VsaLucentPortRedirPortnum(u32), // from Lucent with plain value, tag=83, type=integer
	VsaLucentPortRedirServer(Ipv4Addr), // from Lucent with plain value, tag=84, type=ipaddr
	VsaLucentIpPoolChaining(u32), // from Lucent with plain value, tag=85, type=integer
	VsaLucentOwnerIpAddr(Ipv4Addr), // from Lucent with plain value, tag=86, type=ipaddr
	VsaLucentIpTos(u32), // from Lucent with plain value, tag=87, type=integer
	VsaLucentIpTosPrecedence(u32), // from Lucent with plain value, tag=88, type=integer
	VsaLucentIpTosApplyTo(u32), // from Lucent with plain value, tag=89, type=integer
	VsaLucentFilter(&'a [u8]), // from Lucent with plain value, tag=90, type=string
	VsaLucentTelnetProfile(&'a [u8]), // from Lucent with plain value, tag=91, type=string
	VsaLucentDslRateType(u32), // from Lucent with plain value, tag=92, type=integer
	VsaLucentRedirectNumber(&'a [u8]), // from Lucent with plain value, tag=93, type=string
	VsaLucentAtmVpi(u32), // from Lucent with plain value, tag=94, type=integer
	VsaLucentAtmVci(u32), // from Lucent with plain value, tag=95, type=integer
	VsaLucentSourceIpCheck(u32), // from Lucent with plain value, tag=96, type=integer
	VsaLucentDslRateMode(u32), // from Lucent with plain value, tag=97, type=integer
	VsaLucentDslUpstreamLimit(u32), // from Lucent with plain value, tag=98, type=integer
	VsaLucentDslDownstreamLimit(u32), // from Lucent with plain value, tag=99, type=integer
	VsaLucentDslCirRecvLimit(u32), // from Lucent with plain value, tag=100, type=integer
	VsaLucentDslCirXmitLimit(u32), // from Lucent with plain value, tag=101, type=integer
	VsaLucentVrouterName(&'a [u8]), // from Lucent with plain value, tag=102, type=string
	VsaLucentSourceAuth(&'a [u8]), // from Lucent with plain value, tag=103, type=string
	VsaLucentPrivateRoute(&'a [u8]), // from Lucent with plain value, tag=104, type=string
	VsaLucentNumberingPlanId(u32), // from Lucent with plain value, tag=105, type=integer
	VsaLucentFrLinkStatusDlci(u32), // from Lucent with plain value, tag=106, type=integer
	VsaLucentCallingSubaddress(&'a [u8]), // from Lucent with plain value, tag=107, type=string
	VsaLucentCallbackDelay(u32), // from Lucent with plain value, tag=108, type=integer
	VsaLucentEndpointDisc(&'a [u8]), // from Lucent with plain value, tag=109, type=octets
	VsaLucentRemoteFw(&'a [u8]), // from Lucent with plain value, tag=110, type=string
	VsaLucentMulticastGleaveDelay(u32), // from Lucent with plain value, tag=111, type=integer
	VsaLucentCbcpEnable(u32), // from Lucent with plain value, tag=112, type=integer
	VsaLucentCbcpMode(u32), // from Lucent with plain value, tag=113, type=integer
	VsaLucentCbcpDelay(u32), // from Lucent with plain value, tag=114, type=integer
	VsaLucentCbcpTrunkGroup(u32), // from Lucent with plain value, tag=115, type=integer
	VsaLucentAppletalkRoute(&'a [u8]), // from Lucent with plain value, tag=116, type=string
	VsaLucentAppletalkPeerMode(u32), // from Lucent with plain value, tag=117, type=integer
	VsaLucentRouteAppletalk(u32), // from Lucent with plain value, tag=118, type=integer
	VsaLucentFcpParameter(&'a [u8]), // from Lucent with plain value, tag=119, type=string
	VsaLucentModemPortno(u32), // from Lucent with plain value, tag=120, type=integer
	VsaLucentModemSlotno(u32), // from Lucent with plain value, tag=121, type=integer
	VsaLucentModemShelfno(u32), // from Lucent with plain value, tag=122, type=integer
	VsaLucentCallAttemptLimit(u32), // from Lucent with plain value, tag=123, type=integer
	VsaLucentCallBlockDuration(u32), // from Lucent with plain value, tag=124, type=integer
	VsaLucentMaximumCallDuration(u32), // from Lucent with plain value, tag=125, type=integer
	VsaLucentRoutePreference(u32), // from Lucent with plain value, tag=126, type=integer
	VsaLucentTunnelingProtocol(u32), // from Lucent with plain value, tag=127, type=integer
	VsaLucentSharedProfileEnable(u32), // from Lucent with plain value, tag=128, type=integer
	VsaLucentPrimaryHomeAgent(&'a [u8]), // from Lucent with plain value, tag=129, type=string
	VsaLucentSecondaryHomeAgent(&'a [u8]), // from Lucent with plain value, tag=130, type=string
	VsaLucentDialoutAllowed(u32), // from Lucent with plain value, tag=131, type=integer
	VsaLucentClientGateway(Ipv4Addr), // from Lucent with plain value, tag=132, type=ipaddr
	VsaLucentBacpEnable(u32), // from Lucent with plain value, tag=133, type=integer
	VsaLucentDhcpMaximumLeases(u32), // from Lucent with plain value, tag=134, type=integer
	VsaLucentClientPrimaryDns(Ipv4Addr), // from Lucent with plain value, tag=135, type=ipaddr
	VsaLucentClientSecondaryDns(Ipv4Addr), // from Lucent with plain value, tag=136, type=ipaddr
	VsaLucentClientAssignDns(u32), // from Lucent with plain value, tag=137, type=integer
	VsaLucentUserAcctType(u32), // from Lucent with plain value, tag=138, type=integer
	VsaLucentUserAcctHost(Ipv4Addr), // from Lucent with plain value, tag=139, type=ipaddr
	VsaLucentUserAcctPort(u32), // from Lucent with plain value, tag=140, type=integer
	VsaLucentUserAcctKey(&'a [u8]), // from Lucent with plain value, tag=141, type=string
	VsaLucentUserAcctBase(u32), // from Lucent with plain value, tag=142, type=integer
	VsaLucentUserAcctTime(u32), // from Lucent with plain value, tag=143, type=integer
	VsaLucentAssignIpClient(Ipv4Addr), // from Lucent with plain value, tag=144, type=ipaddr
	VsaLucentAssignIpServer(Ipv4Addr), // from Lucent with plain value, tag=145, type=ipaddr
	VsaLucentAssignIpGlobalPool(&'a [u8]), // from Lucent with plain value, tag=146, type=string
	VsaLucentDhcpReply(u32), // from Lucent with plain value, tag=147, type=integer
	VsaLucentDhcpPoolNumber(u32), // from Lucent with plain value, tag=148, type=integer
	VsaLucentExpectCallback(u32), // from Lucent with plain value, tag=149, type=integer
	VsaLucentEventType(u32), // from Lucent with plain value, tag=150, type=integer
	VsaLucentSessionSvrKey(&'a [u8]), // from Lucent with plain value, tag=151, type=string
	VsaLucentMulticastRateLimit(u32), // from Lucent with plain value, tag=152, type=integer
	VsaLucentIfNetmask(Ipv4Addr), // from Lucent with plain value, tag=153, type=ipaddr
	VsaLucentRemoteAddr(Ipv4Addr), // from Lucent with plain value, tag=154, type=ipaddr
	VsaLucentMulticastClient(u32), // from Lucent with plain value, tag=155, type=integer
	VsaLucentFrCircuitName(&'a [u8]), // from Lucent with plain value, tag=156, type=string
	VsaLucentFrLinkup(u32), // from Lucent with plain value, tag=157, type=integer
	VsaLucentFrNailedGrp(u32), // from Lucent with plain value, tag=158, type=integer
	VsaLucentFrType(u32), // from Lucent with plain value, tag=159, type=integer
	VsaLucentFrLinkMgt(u32), // from Lucent with plain value, tag=160, type=integer
	VsaLucentFrN391(u32), // from Lucent with plain value, tag=161, type=integer
	VsaLucentFrDceN392(u32), // from Lucent with plain value, tag=162, type=integer
	VsaLucentFrDteN392(u32), // from Lucent with plain value, tag=163, type=integer
	VsaLucentFrDceN393(u32), // from Lucent with plain value, tag=164, type=integer
	VsaLucentFrDteN393(u32), // from Lucent with plain value, tag=165, type=integer
	VsaLucentFrT391(u32), // from Lucent with plain value, tag=166, type=integer
	VsaLucentFrT392(u32), // from Lucent with plain value, tag=167, type=integer
	VsaLucentBridgeAddress(&'a [u8]), // from Lucent with plain value, tag=168, type=string
	VsaLucentTsIdleLimit(u32), // from Lucent with plain value, tag=169, type=integer
	VsaLucentTsIdleMode(u32), // from Lucent with plain value, tag=170, type=integer
	VsaLucentDbaMonitor(u32), // from Lucent with plain value, tag=171, type=integer
	VsaLucentBaseChannelCount(u32), // from Lucent with plain value, tag=172, type=integer
	VsaLucentMinimumChannels(u32), // from Lucent with plain value, tag=173, type=integer
	VsaLucentIpxRoute(&'a [u8]), // from Lucent with plain value, tag=174, type=string
	VsaLucentFt1Caller(u32), // from Lucent with plain value, tag=175, type=integer
	VsaLucentBackup(&'a [u8]), // from Lucent with plain value, tag=176, type=string
	VsaLucentCallType(u32), // from Lucent with plain value, tag=177, type=integer
	VsaLucentGroup(&'a [u8]), // from Lucent with plain value, tag=178, type=string
	VsaLucentFrDlci(u32), // from Lucent with plain value, tag=179, type=integer
	VsaLucentFrProfileName(&'a [u8]), // from Lucent with plain value, tag=180, type=string
	VsaLucentAraPw(&'a [u8]), // from Lucent with plain value, tag=181, type=string
	VsaLucentIpxNodeAddr(&'a [u8]), // from Lucent with plain value, tag=182, type=string
	VsaLucentHomeAgentIpAddr(Ipv4Addr), // from Lucent with plain value, tag=183, type=ipaddr
	VsaLucentHomeAgentPassword(&'a [u8]), // from Lucent with plain value, tag=184, type=string
	VsaLucentHomeNetworkName(&'a [u8]), // from Lucent with plain value, tag=185, type=string
	VsaLucentHomeAgentUdpPort(u32), // from Lucent with plain value, tag=186, type=integer
	VsaLucentMultilinkId(u32), // from Lucent with plain value, tag=187, type=integer
	VsaLucentNumInMultilink(u32), // from Lucent with plain value, tag=188, type=integer
	VsaLucentFirstDest(Ipv4Addr), // from Lucent with plain value, tag=189, type=ipaddr
	VsaLucentPreInputOctets(u32), // from Lucent with plain value, tag=190, type=integer
	VsaLucentPreOutputOctets(u32), // from Lucent with plain value, tag=191, type=integer
	VsaLucentPreInputPackets(u32), // from Lucent with plain value, tag=192, type=integer
	VsaLucentPreOutputPackets(u32), // from Lucent with plain value, tag=193, type=integer
	VsaLucentMaximumTime(u32), // from Lucent with plain value, tag=194, type=integer
	VsaLucentDisconnectCause(u32), // from Lucent with plain value, tag=195, type=integer
	VsaLucentConnectProgress(u32), // from Lucent with plain value, tag=196, type=integer
	VsaLucentDataRate(u32), // from Lucent with plain value, tag=197, type=integer
	VsaLucentPresessionTime(u32), // from Lucent with plain value, tag=198, type=integer
	VsaLucentTokenIdle(u32), // from Lucent with plain value, tag=199, type=integer
	VsaLucentTokenImmediate(u32), // from Lucent with plain value, tag=200, type=integer
	VsaLucentRequireAuth(u32), // from Lucent with plain value, tag=201, type=integer
	VsaLucentNumberSessions(&'a [u8]), // from Lucent with plain value, tag=202, type=string
	VsaLucentAuthenAlias(&'a [u8]), // from Lucent with plain value, tag=203, type=string
	VsaLucentTokenExpiry(u32), // from Lucent with plain value, tag=204, type=integer
	VsaLucentMenuSelector(&'a [u8]), // from Lucent with plain value, tag=205, type=string
	VsaLucentMenuItem(&'a [u8]), // from Lucent with plain value, tag=206, type=string
	VsaLucentPwWarntime(u32), // from Lucent with plain value, tag=207, type=integer
	VsaLucentPwLifetime(u32), // from Lucent with plain value, tag=208, type=integer
	VsaLucentIpDirect(Ipv4Addr), // from Lucent with plain value, tag=209, type=ipaddr
	VsaLucentPppVjSlotComp(u32), // from Lucent with plain value, tag=210, type=integer
	VsaLucentPppVj1172(u32), // from Lucent with plain value, tag=211, type=integer
	VsaLucentPppAsyncMap(u32), // from Lucent with plain value, tag=212, type=integer
	VsaLucentThirdPrompt(&'a [u8]), // from Lucent with plain value, tag=213, type=string
	VsaLucentSendSecret(&'a [u8]), // from Lucent with plain value, tag=214, type=string
	VsaLucentReceiveSecret(&'a [u8]), // from Lucent with plain value, tag=215, type=string
	VsaLucentIpxPeerMode(u32), // from Lucent with plain value, tag=216, type=integer
	VsaLucentIpPoolDefinition(&'a [u8]), // from Lucent with plain value, tag=217, type=string
	VsaLucentAssignIpPool(u32), // from Lucent with plain value, tag=218, type=integer
	VsaLucentFrDirect(u32), // from Lucent with plain value, tag=219, type=integer
	VsaLucentFrDirectProfile(&'a [u8]), // from Lucent with plain value, tag=220, type=string
	VsaLucentFrDirectDlci(u32), // from Lucent with plain value, tag=221, type=integer
	VsaLucentHandleIpx(u32), // from Lucent with plain value, tag=222, type=integer
	VsaLucentNetwareTimeout(u32), // from Lucent with plain value, tag=223, type=integer
	VsaLucentIpxAlias(u32), // from Lucent with plain value, tag=224, type=integer
	VsaLucentMetric(u32), // from Lucent with plain value, tag=225, type=integer
	VsaLucentPriNumberType(u32), // from Lucent with plain value, tag=226, type=integer
	VsaLucentDialNumber(&'a [u8]), // from Lucent with plain value, tag=227, type=string
	VsaLucentRouteIp(u32), // from Lucent with plain value, tag=228, type=integer
	VsaLucentRouteIpx(u32), // from Lucent with plain value, tag=229, type=integer
	VsaLucentBridge(u32), // from Lucent with plain value, tag=230, type=integer
	VsaLucentSendAuth(u32), // from Lucent with plain value, tag=231, type=integer
	VsaLucentSendPasswd(&'a [u8]), // from Lucent with plain value, tag=232, type=string
	VsaLucentLinkCompression(u32), // from Lucent with plain value, tag=233, type=integer
	VsaLucentTargetUtil(u32), // from Lucent with plain value, tag=234, type=integer
	VsaLucentMaximumChannels(u32), // from Lucent with plain value, tag=235, type=integer
	VsaLucentIncChannelCount(u32), // from Lucent with plain value, tag=236, type=integer
	VsaLucentDecChannelCount(u32), // from Lucent with plain value, tag=237, type=integer
	VsaLucentSecondsOfHistory(u32), // from Lucent with plain value, tag=238, type=integer
	VsaLucentHistoryWeighType(u32), // from Lucent with plain value, tag=239, type=integer
	VsaLucentAddSeconds(u32), // from Lucent with plain value, tag=240, type=integer
	VsaLucentRemoveSeconds(u32), // from Lucent with plain value, tag=241, type=integer
	VsaLucentDataFilter(&'a [u8]), // from Lucent with plain value, tag=242, type=abinary
	VsaLucentCallFilter(&'a [u8]), // from Lucent with plain value, tag=243, type=abinary
	VsaLucentIdleLimit(u32), // from Lucent with plain value, tag=244, type=integer
	VsaLucentPreemptLimit(u32), // from Lucent with plain value, tag=245, type=integer
	VsaLucentCallback(u32), // from Lucent with plain value, tag=246, type=integer
	VsaLucentDataSvc(u32), // from Lucent with plain value, tag=247, type=integer
	VsaLucentForce56(u32), // from Lucent with plain value, tag=248, type=integer
	VsaLucentBillingNumber(&'a [u8]), // from Lucent with plain value, tag=249, type=string
	VsaLucentCallByCall(u32), // from Lucent with plain value, tag=250, type=integer
	VsaLucentTransitNumber(&'a [u8]), // from Lucent with plain value, tag=251, type=string
	VsaLucentHostInfo(&'a [u8]), // from Lucent with plain value, tag=252, type=string
	VsaLucentPppAddress(Ipv4Addr), // from Lucent with plain value, tag=253, type=ipaddr
	VsaLucentMppIdlePercent(u32), // from Lucent with plain value, tag=254, type=integer
	VsaLucentXmitRate(u32), // from Lucent with plain value, tag=255, type=integer
	VsaLucentFr05TrafficShaper(u32), // from Lucent with plain value, tag=256, type=integer
	VsaLucentFr05Vpi(u32), // from Lucent with plain value, tag=257, type=integer
	VsaLucentFr05Vci(u32), // from Lucent with plain value, tag=258, type=integer
	VsaLucentFr05Enabled(u32), // from Lucent with plain value, tag=259, type=integer
	VsaLucentTunnelAuthType(&'a [u8]), // from Lucent with plain value, tag=260, type=octets
	VsaLucentMohTimeout(u32), // from Lucent with plain value, tag=261, type=integer
	VsaLucentAtmCircuitName(&'a [u8]), // from Lucent with plain value, tag=262, type=string
	VsaLucentPriorityForPpp(u32), // from Lucent with plain value, tag=263, type=integer
	VsaLucentMaxRtpDelay(u32), // from Lucent with plain value, tag=264, type=integer
	VsaLucentRtpPortRange(&'a [u8]), // from Lucent with plain value, tag=265, type=string
	VsaLucentTosCopying(u32), // from Lucent with plain value, tag=266, type=integer
	VsaLucentPacketClassification(u32), // from Lucent with plain value, tag=267, type=integer
	VsaLucentNoHighPrioPktDuratio(u32), // from Lucent with plain value, tag=268, type=integer
	VsaLucentAtAnswerString(&'a [u8]), // from Lucent with plain value, tag=269, type=string
	VsaLucentIpOutgoingTos(u32), // from Lucent with plain value, tag=270, type=integer
	VsaLucentIpOutgoingTosPrecedence(u32), // from Lucent with plain value, tag=271, type=integer
	VsaLucentIpOutgoingDscp(u32), // from Lucent with plain value, tag=272, type=integer
	VsaLucentTermsrvLoginPrompt(&'a [u8]), // from Lucent with plain value, tag=273, type=string
	VsaLucentMulticastServiceProfileName(&'a [u8]), // from Lucent with plain value, tag=274, type=string
	VsaLucentMulticastMaxGroups(u32), // from Lucent with plain value, tag=275, type=integer
	VsaLucentMulticastServiceName(&'a [u8]), // from Lucent with plain value, tag=276, type=string
	VsaLucentMulticastServiceActive(u32), // from Lucent with plain value, tag=277, type=integer
	VsaLucentMulticastServiceSnmpTrap(u32), // from Lucent with plain value, tag=278, type=integer
	VsaLucentMulticastServiceFilterType(u32), // from Lucent with plain value, tag=279, type=integer
	VsaLucentMulticastFilterActive(u32), // from Lucent with plain value, tag=280, type=integer
	VsaLucentMulticastFilterAddress(Ipv4Addr), // from Lucent with plain value, tag=281, type=ipaddr
	VsaLucentTunnelTos(u32), // from Lucent with plain value, tag=282, type=integer
	VsaLucentTunnelTosPrecedence(u32), // from Lucent with plain value, tag=283, type=integer
	VsaLucentTunnelDscp(u32), // from Lucent with plain value, tag=284, type=integer
	VsaLucentTunnelTosFilter(&'a [u8]), // from Lucent with plain value, tag=285, type=string
	VsaLucentTunnelTosCopy(u32), // from Lucent with plain value, tag=286, type=integer
	VsaLucentHttpRedirectUrl(&'a [u8]), // from Lucent with plain value, tag=287, type=string
	VsaLucentHttpRedirectPort(u32), // from Lucent with plain value, tag=288, type=integer
	VsaLucentL2TpDciDisconnectCode(u32), // from Lucent with plain value, tag=289, type=integer
	VsaLucentL2TpDciProtocolNumber(u32), // from Lucent with plain value, tag=290, type=integer
	VsaLucentL2TpDciDirection(u32), // from Lucent with plain value, tag=291, type=integer
	VsaLucentL2TpDciMessage(&'a [u8]), // from Lucent with plain value, tag=292, type=string
	VsaLucentL2TpQ931CauseCode(u32), // from Lucent with plain value, tag=293, type=integer
	VsaLucentL2TpQ931CauseMessage(u32), // from Lucent with plain value, tag=294, type=integer
	VsaLucentL2TpQ931AdvisoryMessage(&'a [u8]), // from Lucent with plain value, tag=295, type=string
	VsaLucentL2TpRcResultCode(u32), // from Lucent with plain value, tag=296, type=integer
	VsaLucentL2TpRcErrorCode(u32), // from Lucent with plain value, tag=297, type=integer
	VsaLucentL2TpRcErrorMessage(&'a [u8]), // from Lucent with plain value, tag=298, type=string
	VsaLucentL2TpDisconnectScenario(u32), // from Lucent with plain value, tag=299, type=integer
	VsaLucentL2TpPeerDisconnectCause(u32), // from Lucent with plain value, tag=300, type=integer
	VsaLucentL2TpPeerConnectProgress(u32), // from Lucent with plain value, tag=301, type=integer
	VsaLucentQuickconnectAttempted(u32), // from Lucent with plain value, tag=302, type=integer
	VsaLucentNumMohSessions(u32), // from Lucent with plain value, tag=303, type=integer
	VsaLucentCumulativeHoldTime(u32), // from Lucent with plain value, tag=304, type=integer
	VsaLucentModemModulation(u32), // from Lucent with plain value, tag=305, type=integer
	VsaLucentUserAcctExpiration(u32), // from Lucent with plain value, tag=306, type=date
	VsaLucentUserLoginLevel(u32), // from Lucent with plain value, tag=307, type=integer
	VsaLucentFirstLevelUser(&'a [u8]), // from Lucent with plain value, tag=308, type=string
	VsaLucentIpSourceIf(&'a [u8]), // from Lucent with plain value, tag=309, type=string
	VsaLucentReversePathCheck(u32), // from Lucent with plain value, tag=310, type=integer
	VsaLucentLcpKeepalivePeriod(u32), // from Lucent with plain value, tag=321, type=integer
	VsaLucentLcpKeepaliveMissedLimit(u32), // from Lucent with plain value, tag=322, type=integer
	VsaLucentDslAtucChanUncorrectBlks(u32), // from Lucent with plain value, tag=10000, type=integer
	VsaLucentDslAtucChanCorrectedBlks(u32), // from Lucent with plain value, tag=10001, type=integer
	VsaLucentDslAtucChanXmitBlks(u32), // from Lucent with plain value, tag=10002, type=integer
	VsaLucentDslAtucChanRecdBlks(u32), // from Lucent with plain value, tag=10003, type=integer
	VsaLucentDslAtucPerfInits(u32), // from Lucent with plain value, tag=10004, type=integer
	VsaLucentDslAtucPerfEss(u32), // from Lucent with plain value, tag=10005, type=integer
	VsaLucentDslAtucPerfLprs(u32), // from Lucent with plain value, tag=10006, type=integer
	VsaLucentDslAtucPerfLols(u32), // from Lucent with plain value, tag=10007, type=integer
	VsaLucentDslAtucPerfLoss(u32), // from Lucent with plain value, tag=10008, type=integer
	VsaLucentDslAtucPerfLofs(u32), // from Lucent with plain value, tag=10009, type=integer
	VsaLucentDslAtucCurrAttainableRateDn(u32), // from Lucent with plain value, tag=10010, type=integer
	VsaLucentDslAtucCurrOutputPwrDn(u32), // from Lucent with plain value, tag=10011, type=integer
	VsaLucentDslAtucCurrAtnUp(u32), // from Lucent with plain value, tag=10012, type=integer
	VsaLucentDslAtucCurrSnrMgnUp(u32), // from Lucent with plain value, tag=10013, type=integer
	VsaLucentDslAtucPsFastRetrains(u32), // from Lucent with plain value, tag=10014, type=integer
	VsaLucentDslAtucPsFailedFastRetrains(u32), // from Lucent with plain value, tag=10015, type=integer
	VsaLucentDslCodeViolations(u32), // from Lucent with plain value, tag=10016, type=integer
	VsaLucentLineType(u32), // from Lucent with plain value, tag=10017, type=integer
	VsaLucentDslCurrUpRate(u32), // from Lucent with plain value, tag=10018, type=integer
	VsaLucentDslCurrDnRate(u32), // from Lucent with plain value, tag=10019, type=integer
	VsaLucentDslPhysicalSlot(u32), // from Lucent with plain value, tag=10020, type=integer
	VsaLucentDslPhysicalLine(u32), // from Lucent with plain value, tag=10021, type=integer
	VsaLucentDslIfIndex(u32), // from Lucent with plain value, tag=10022, type=integer
	VsaLucentDslOperStatus(u32), // from Lucent with plain value, tag=10023, type=integer
	VsaLucentDslRelatedIfIndex(u32), // from Lucent with plain value, tag=10024, type=integer
	VsaLucentDslAtucCurrAttainableRateUp(u32), // from Lucent with plain value, tag=10025, type=integer
	VsaLucentDslAtucCurrOutputPwrUp(u32), // from Lucent with plain value, tag=10026, type=integer
	VsaLucentDslAtucCurrAtnDn(u32), // from Lucent with plain value, tag=10027, type=integer
	VsaLucentDslAtucCurrSnrMgnD(u32), // from Lucent with plain value, tag=10028, type=integer
	VsaLucentDslRelatedSlot(u32), // from Lucent with plain value, tag=10029, type=integer
	VsaLucentDslRelatedPort(u32), // from Lucent with plain value, tag=10030, type=integer
	VsaLucentDslSparingRole(u32), // from Lucent with plain value, tag=10031, type=integer
	VsaLucentAbsoluteTime(u32), // from Lucent with plain value, tag=10032, type=integer
	VsaLucentConfiguredRateUpMin(u32), // from Lucent with plain value, tag=10033, type=integer
	VsaLucentConfiguredRateUpMax(u32), // from Lucent with plain value, tag=10034, type=integer
	VsaLucentConfiguredRateDnMin(u32), // from Lucent with plain value, tag=10035, type=integer
	VsaLucentConfiguredRateDnMax(u32), // from Lucent with plain value, tag=10036, type=integer
	VsaLucentDslPhysicalChannel(u32), // from Lucent with plain value, tag=10037, type=integer
	VsaLucentSonetSectionEss(u32), // from Lucent with plain value, tag=10100, type=integer
	VsaLucentSonetSectionSess(u32), // from Lucent with plain value, tag=10101, type=integer
	VsaLucentSonetSectionSefss(u32), // from Lucent with plain value, tag=10102, type=integer
	VsaLucentSonetSectionCvs(u32), // from Lucent with plain value, tag=10103, type=integer
	VsaLucentSonetLineEssNear(u32), // from Lucent with plain value, tag=10104, type=integer
	VsaLucentSonetLineSessNear(u32), // from Lucent with plain value, tag=10105, type=integer
	VsaLucentSonetLineCvsNear(u32), // from Lucent with plain value, tag=10106, type=integer
	VsaLucentSonetLineUssNear(u32), // from Lucent with plain value, tag=10107, type=integer
	VsaLucentSonetLineEssFar(u32), // from Lucent with plain value, tag=10108, type=integer
	VsaLucentSonetLineSessFar(u32), // from Lucent with plain value, tag=10109, type=integer
	VsaLucentSonetLineCvsFar(u32), // from Lucent with plain value, tag=10110, type=integer
	VsaLucentSonetLineUssFar(u32), // from Lucent with plain value, tag=10111, type=integer
	VsaLucentSonetPathEssNear(u32), // from Lucent with plain value, tag=10112, type=integer
	VsaLucentSonetPathSessNear(u32), // from Lucent with plain value, tag=10113, type=integer
	VsaLucentSonetPathCvsNear(u32), // from Lucent with plain value, tag=10114, type=integer
	VsaLucentSonetPathUssNear(u32), // from Lucent with plain value, tag=10115, type=integer
	VsaLucentSonetPathEssFar(u32), // from Lucent with plain value, tag=10116, type=integer
	VsaLucentSonetPathSessFar(u32), // from Lucent with plain value, tag=10117, type=integer
	VsaLucentSonetPathCvsFar(u32), // from Lucent with plain value, tag=10118, type=integer
	VsaLucentSonetPathUssFar(u32), // from Lucent with plain value, tag=10119, type=integer
	VsaLucentDs3FBitErr(u32), // from Lucent with plain value, tag=10200, type=integer
	VsaLucentDs3PBitErr(u32), // from Lucent with plain value, tag=10201, type=integer
	VsaLucentDs3Ccvs(u32), // from Lucent with plain value, tag=10202, type=integer
	VsaLucentDs3Pess(u32), // from Lucent with plain value, tag=10203, type=integer
	VsaLucentDs3Psess(u32), // from Lucent with plain value, tag=10204, type=integer
	VsaLucentDs3Sefs(u32), // from Lucent with plain value, tag=10205, type=integer
	VsaLucentDs3Uass(u32), // from Lucent with plain value, tag=10206, type=integer
	VsaLucentDs3Lcvs(u32), // from Lucent with plain value, tag=10207, type=integer
	VsaLucentDs3Pcvs(u32), // from Lucent with plain value, tag=10208, type=integer
	VsaLucentDs3Less(u32), // from Lucent with plain value, tag=10209, type=integer
	VsaLucentDs3Cess(u32), // from Lucent with plain value, tag=10210, type=integer
	VsaLucentDs3Csess(u32), // from Lucent with plain value, tag=10211, type=integer
	VsaLucentRtpLocalNumberOfSamples(u32), // from Lucent with plain value, tag=10300, type=integer
	VsaLucentRtpRemoteNumberOfSamples(u32), // from Lucent with plain value, tag=10301, type=integer
	VsaLucentRtpLocalJitterMinimum(u32), // from Lucent with plain value, tag=10302, type=integer
	VsaLucentRtpLocalJitterMaximum(u32), // from Lucent with plain value, tag=10303, type=integer
	VsaLucentRtpLocalJitterMean(u32), // from Lucent with plain value, tag=10304, type=integer
	VsaLucentRtpLocalJitterVariance(u32), // from Lucent with plain value, tag=10305, type=integer
	VsaLucentRtpLocalDelayMinimum(u32), // from Lucent with plain value, tag=10306, type=integer
	VsaLucentRtpLocalDelayMaximum(u32), // from Lucent with plain value, tag=10307, type=integer
	VsaLucentRtpLocalDelayMean(u32), // from Lucent with plain value, tag=10308, type=integer
	VsaLucentRtpLocalDelayVariance(u32), // from Lucent with plain value, tag=10309, type=integer
	VsaLucentRtpLocalPacketsSent(u32), // from Lucent with plain value, tag=10310, type=integer
	VsaLucentRtpLocalPacketsLost(u32), // from Lucent with plain value, tag=10311, type=integer
	VsaLucentRtpLocalPacketsLate(u32), // from Lucent with plain value, tag=10312, type=integer
	VsaLucentRtpLocalBytesSent(u32), // from Lucent with plain value, tag=10313, type=integer
	VsaLucentRtpLocalSilencePercent(u32), // from Lucent with plain value, tag=10314, type=integer
	VsaLucentRtpRemoteJitterMinimum(u32), // from Lucent with plain value, tag=10315, type=integer
	VsaLucentRtpRemoteJitterMaximum(u32), // from Lucent with plain value, tag=10316, type=integer
	VsaLucentRtpRemoteJitterMean(u32), // from Lucent with plain value, tag=10317, type=integer
	VsaLucentRtpRemoteJitterVariance(u32), // from Lucent with plain value, tag=10318, type=integer
	VsaLucentRtpRemoteDelayMinimum(u32), // from Lucent with plain value, tag=10319, type=integer
	VsaLucentRtpRemoteDelayMaximum(u32), // from Lucent with plain value, tag=10320, type=integer
	VsaLucentRtpRemoteDelayMean(u32), // from Lucent with plain value, tag=10321, type=integer
	VsaLucentRtpRemoteDelayVariance(u32), // from Lucent with plain value, tag=10322, type=integer
	VsaLucentRtpRemotePacketsSent(u32), // from Lucent with plain value, tag=10323, type=integer
	VsaLucentRtpRemotePacketsLost(u32), // from Lucent with plain value, tag=10324, type=integer
	VsaLucentRtpRemotePacketsLate(u32), // from Lucent with plain value, tag=10325, type=integer
	VsaLucentRtpRemoteBytesSent(u32), // from Lucent with plain value, tag=10326, type=integer
	VsaLucentRtpRemoteSilencePercent(u32), // from Lucent with plain value, tag=10327, type=integer
	VsaLucentTunnelAuthType2(u32), // from Lucent with plain value, tag=19999, type=integer
	VsaLucentMultiPacketSeparator(u32), // from Lucent with plain value, tag=20000, type=integer
	VsaLucentMinXmitRate(u32), // from Lucent with plain value, tag=20100, type=integer
	VsaLucentMaxXmitRate(u32), // from Lucent with plain value, tag=20101, type=integer
	VsaLucentMinRecvRate(u32), // from Lucent with plain value, tag=20102, type=integer
	VsaLucentMaxRecvRate(u32), // from Lucent with plain value, tag=20103, type=integer
	VsaLucentErrorCorrectionProtocol(u32), // from Lucent with plain value, tag=20104, type=integer
	VsaLucentCompressionProtocol(u32), // from Lucent with plain value, tag=20105, type=integer
	VsaLucentModulation(u32), // from Lucent with plain value, tag=20106, type=integer
	VsaLucentXmitSymbolRate(u32), // from Lucent with plain value, tag=20107, type=integer
	VsaLucentRecvSymbolRate(u32), // from Lucent with plain value, tag=20108, type=integer
	VsaLucentCurrentXmitLevel(u32), // from Lucent with plain value, tag=20109, type=integer
	VsaLucentCurrentRecvLevel(u32), // from Lucent with plain value, tag=20110, type=integer
	VsaLucentCurrentLineQuality(u32), // from Lucent with plain value, tag=20111, type=integer
	VsaLucentCurrentSnr(u32), // from Lucent with plain value, tag=20112, type=integer
	VsaLucentMinSnr(u32), // from Lucent with plain value, tag=20113, type=integer
	VsaLucentMaxSnr(u32), // from Lucent with plain value, tag=20114, type=integer
	VsaLucentLocalRetrainRequested(u32), // from Lucent with plain value, tag=20115, type=integer
	VsaLucentRemoteRetrainRequested(u32), // from Lucent with plain value, tag=20116, type=integer
	VsaLucentConnectionTime(u32), // from Lucent with plain value, tag=20117, type=integer
	VsaLucentModemDisconnectReason(u32), // from Lucent with plain value, tag=20118, type=integer
	VsaLucentRetrainReason(u32), // from Lucent with plain value, tag=20119, type=integer
	VsaManzaraUserUid(u32), // from Manzara with plain value, tag=1, type=integer
	VsaManzaraUserGid(u32), // from Manzara with plain value, tag=2, type=integer
	VsaManzaraUserHome(&'a [u8]), // from Manzara with plain value, tag=3, type=string
	VsaManzaraUserShell(&'a [u8]), // from Manzara with plain value, tag=4, type=string
	VsaManzaraPppAddrString(&'a [u8]), // from Manzara with plain value, tag=5, type=string
	VsaManzaraFullLoginString(&'a [u8]), // from Manzara with plain value, tag=6, type=string
	VsaManzaraTariffUnits(u32), // from Manzara with plain value, tag=7, type=integer
	VsaManzaraTariffType(manzara::ManzaraTariffType), // from Manzara with Enum value, tag=8, type=integer
	VsaManzaraEcpSessionKey(&'a [u8]), // from Manzara with plain value, tag=9, type=octets
	VsaManzaraMapName(&'a [u8]), // from Manzara with plain value, tag=10, type=string
	VsaManzaraMapKey(&'a [u8]), // from Manzara with plain value, tag=11, type=string
	VsaManzaraMapValue(&'a [u8]), // from Manzara with plain value, tag=12, type=string
	VsaManzaraMapError(&'a [u8]), // from Manzara with plain value, tag=13, type=string
	VsaManzaraServiceType(&'a [u8]), // from Manzara with plain value, tag=14, type=string
	VsaMbgManagementPrivilegeLevel(meinberg::MbgManagementPrivilegeLevel), // from Meinberg with Enum value, tag=1, type=integer
	VsaMerakiDeviceName(&'a [u8]), // from Meraki with plain value, tag=1, type=string
	VsaMerakiNetworkName(&'a [u8]), // from Meraki with plain value, tag=2, type=string
	VsaMerakiApName(&'a [u8]), // from Meraki with plain value, tag=3, type=string
	VsaMerakiApTags(&'a [u8]), // from Meraki with plain value, tag=4, type=string
	VsaMeritProxyAction(&'a [u8]), // from Merit with plain value, tag=211, type=string
	VsaMeritUserId(&'a [u8]), // from Merit with plain value, tag=222, type=string
	VsaMeritUserRealm(&'a [u8]), // from Merit with plain value, tag=223, type=string
	VsaMeruAccessPointId(u32), // from Meru with plain value, tag=1, type=integer
	VsaMeruAccessPointName(&'a [u8]), // from Meru with plain value, tag=2, type=string
	VsaMicrosemiUserFullName(&'a [u8]), // from Microsemi with plain value, tag=1, type=string
	VsaMicrosemiUserName(&'a [u8]), // from Microsemi with plain value, tag=2, type=string
	VsaMicrosemiUserInitials(&'a [u8]), // from Microsemi with plain value, tag=3, type=string
	VsaMicrosemiUserEmail(&'a [u8]), // from Microsemi with plain value, tag=4, type=string
	VsaMicrosemiUserGroup(&'a [u8]), // from Microsemi with plain value, tag=5, type=string
	VsaMicrosemiFallbackUserGroup(&'a [u8]), // from Microsemi with plain value, tag=6, type=string
	VsaMicrosemiNetworkElementGroup(&'a [u8]), // from Microsemi with plain value, tag=7, type=string
	VsaMsChapResponse(&'a [u8]), // from Microsoft with plain value, tag=1, type=octets
	VsaMsChapError(&'a [u8]), // from Microsoft with plain value, tag=2, type=string
	VsaMsChapCpw1(&'a [u8]), // from Microsoft with plain value, tag=3, type=octets
	VsaMsChapCpw2(&'a [u8]), // from Microsoft with plain value, tag=4, type=octets
	VsaMsChapLmEncPw(&'a [u8]), // from Microsoft with plain value, tag=5, type=octets
	VsaMsChapNtEncPw(&'a [u8]), // from Microsoft with plain value, tag=6, type=octets
	VsaMsMppeEncryptionPolicy(microsoft::MsMppeEncryptionPolicy), // from Microsoft with Enum value, tag=7, type=integer
	VsaMsMppeEncryptionType(u32), // from Microsoft with plain value, tag=8, type=integer
	VsaMsRasVendor(u32), // from Microsoft with plain value, tag=9, type=integer
	VsaMsChapDomain(&'a [u8]), // from Microsoft with plain value, tag=10, type=string
	VsaMsChapChallenge(&'a [u8]), // from Microsoft with plain value, tag=11, type=octets
	VsaMsChapMppeKeys(&'a [u8]), // from Microsoft with plain value, tag=12, type=octets
	VsaMsBapUsage(microsoft::MsBapUsage), // from Microsoft with Enum value, tag=13, type=integer
	VsaMsLinkUtilizationThreshold(u32), // from Microsoft with plain value, tag=14, type=integer
	VsaMsLinkDropTimeLimit(u32), // from Microsoft with plain value, tag=15, type=integer
	VsaMsMppeSendKey(&'a [u8]), // from Microsoft with plain value, tag=16, type=octets
	VsaMsMppeRecvKey(&'a [u8]), // from Microsoft with plain value, tag=17, type=octets
	VsaMsRasVersion(&'a [u8]), // from Microsoft with plain value, tag=18, type=string
	VsaMsOldArapPassword(&'a [u8]), // from Microsoft with plain value, tag=19, type=octets
	VsaMsNewArapPassword(&'a [u8]), // from Microsoft with plain value, tag=20, type=octets
	VsaMsArapPwChangeReason(microsoft::MsArapPwChangeReason), // from Microsoft with Enum value, tag=21, type=integer
	VsaMsFilter(&'a [u8]), // from Microsoft with plain value, tag=22, type=octets
	VsaMsAcctAuthType(microsoft::MsAcctAuthType), // from Microsoft with Enum value, tag=23, type=integer
	VsaMsAcctEapType(microsoft::MsAcctEapType), // from Microsoft with Enum value, tag=24, type=integer
	VsaMsChap2Response(&'a [u8]), // from Microsoft with plain value, tag=25, type=octets
	VsaMsChap2Success(&'a [u8]), // from Microsoft with plain value, tag=26, type=octets
	VsaMsChap2Cpw(&'a [u8]), // from Microsoft with plain value, tag=27, type=octets
	VsaMsPrimaryDnsServer(Ipv4Addr), // from Microsoft with plain value, tag=28, type=ipaddr
	VsaMsSecondaryDnsServer(Ipv4Addr), // from Microsoft with plain value, tag=29, type=ipaddr
	VsaMsPrimaryNbnsServer(Ipv4Addr), // from Microsoft with plain value, tag=30, type=ipaddr
	VsaMsSecondaryNbnsServer(Ipv4Addr), // from Microsoft with plain value, tag=31, type=ipaddr
	VsaMsRasClientName(&'a [u8]), // from Microsoft with plain value, tag=34, type=string
	VsaMsRasClientVersion(&'a [u8]), // from Microsoft with plain value, tag=35, type=string
	VsaMsQuarantineIpfilter(&'a [u8]), // from Microsoft with plain value, tag=36, type=octets
	VsaMsQuarantineSessionTimeout(u32), // from Microsoft with plain value, tag=37, type=integer
	VsaMsUserSecurityIdentity(&'a [u8]), // from Microsoft with plain value, tag=40, type=string
	VsaMsIdentityType(microsoft::MsIdentityType), // from Microsoft with Enum value, tag=41, type=integer
	VsaMsServiceClass(&'a [u8]), // from Microsoft with plain value, tag=42, type=string
	VsaMsQuarantineUserClass(&'a [u8]), // from Microsoft with plain value, tag=44, type=string
	VsaMsQuarantineState(microsoft::MsQuarantineState), // from Microsoft with Enum value, tag=45, type=integer
	VsaMsQuarantineGraceTime(u32), // from Microsoft with plain value, tag=46, type=integer
	VsaMsNetworkAccessServerType(microsoft::MsNetworkAccessServerType), // from Microsoft with Enum value, tag=47, type=integer
	VsaMsAfwZone(microsoft::MsAfwZone), // from Microsoft with Enum value, tag=48, type=integer
	VsaMsAfwProtectionLevel(microsoft::MsAfwProtectionLevel), // from Microsoft with Enum value, tag=49, type=integer
	VsaMsMachineName(&'a [u8]), // from Microsoft with plain value, tag=50, type=string
	VsaMsIpv6Filter(&'a [u8]), // from Microsoft with plain value, tag=51, type=octets
	VsaMsIpv4RemediationServers(&'a [u8]), // from Microsoft with plain value, tag=52, type=octets
	VsaMsIpv6RemediationServers(&'a [u8]), // from Microsoft with plain value, tag=53, type=octets
	VsaMsRnapNotQuarantineCapable(microsoft::MsRnapNotQuarantineCapable), // from Microsoft with Enum value, tag=54, type=integer
	VsaMsQuarantineSoh(&'a [u8]), // from Microsoft with plain value, tag=55, type=octets
	VsaMsRasCorrelation(&'a [u8]), // from Microsoft with plain value, tag=56, type=octets
	VsaMsExtendedQuarantineState(microsoft::MsExtendedQuarantineState), // from Microsoft with Enum value, tag=57, type=integer
	VsaMsHcapUserGroups(&'a [u8]), // from Microsoft with plain value, tag=58, type=string
	VsaMsHcapLocationGroupName(&'a [u8]), // from Microsoft with plain value, tag=59, type=string
	VsaMsHcapUserName(&'a [u8]), // from Microsoft with plain value, tag=60, type=string
	VsaMsUserIpv4Address(Ipv4Addr), // from Microsoft with plain value, tag=61, type=ipaddr
	VsaMsUserIpv6Address(Ipv6Addr), // from Microsoft with plain value, tag=62, type=ipv6addr
	VsaMsTsgDeviceRedirection(u32), // from Microsoft with plain value, tag=63, type=integer
	VsaMikrotikRecvLimit(u32), // from Mikrotik with plain value, tag=1, type=integer
	VsaMikrotikXmitLimit(u32), // from Mikrotik with plain value, tag=2, type=integer
	VsaMikrotikGroup(&'a [u8]), // from Mikrotik with plain value, tag=3, type=string
	VsaMikrotikWirelessForward(u32), // from Mikrotik with plain value, tag=4, type=integer
	VsaMikrotikWirelessSkipDot1X(u32), // from Mikrotik with plain value, tag=5, type=integer
	VsaMikrotikWirelessEncAlgo(mikrotik::MikrotikWirelessEncAlgo), // from Mikrotik with Enum value, tag=6, type=integer
	VsaMikrotikWirelessEncKey(&'a [u8]), // from Mikrotik with plain value, tag=7, type=string
	VsaMikrotikRateLimit(&'a [u8]), // from Mikrotik with plain value, tag=8, type=string
	VsaMikrotikRealm(&'a [u8]), // from Mikrotik with plain value, tag=9, type=string
	VsaMikrotikHostIp(Ipv4Addr), // from Mikrotik with plain value, tag=10, type=ipaddr
	VsaMikrotikMarkId(&'a [u8]), // from Mikrotik with plain value, tag=11, type=string
	VsaMikrotikAdvertiseUrl(&'a [u8]), // from Mikrotik with plain value, tag=12, type=string
	VsaMikrotikAdvertiseInterval(u32), // from Mikrotik with plain value, tag=13, type=integer
	VsaMikrotikRecvLimitGigawords(u32), // from Mikrotik with plain value, tag=14, type=integer
	VsaMikrotikXmitLimitGigawords(u32), // from Mikrotik with plain value, tag=15, type=integer
	VsaMikrotikWirelessPsk(&'a [u8]), // from Mikrotik with plain value, tag=16, type=string
	VsaMikrotikTotalLimit(u32), // from Mikrotik with plain value, tag=17, type=integer
	VsaMikrotikTotalLimitGigawords(u32), // from Mikrotik with plain value, tag=18, type=integer
	VsaMikrotikAddressList(&'a [u8]), // from Mikrotik with plain value, tag=19, type=string
	VsaMikrotikWirelessMpkey(&'a [u8]), // from Mikrotik with plain value, tag=20, type=string
	VsaMikrotikWirelessComment(&'a [u8]), // from Mikrotik with plain value, tag=21, type=string
	VsaMikrotikDelegatedIpv6Pool(&'a [u8]), // from Mikrotik with plain value, tag=22, type=string
	VsaMikrotikDhcpOptionSet(&'a [u8]), // from Mikrotik with plain value, tag=23, type=string
	VsaMikrotikDhcpOptionParamStr1(&'a [u8]), // from Mikrotik with plain value, tag=24, type=string
	VsaMikortikDhcpOptionParamstr2(&'a [u8]), // from Mikrotik with plain value, tag=25, type=string
	VsaMikrotikWirelessVlanid(u32), // from Mikrotik with plain value, tag=26, type=integer
	VsaMikrotikWirelessVlanidType(u32), // from Mikrotik with plain value, tag=27, type=integer
	VsaMikrotikWirelessMinsignal(&'a [u8]), // from Mikrotik with plain value, tag=28, type=string
	VsaMikrotikWirelessMaxsignal(&'a [u8]), // from Mikrotik with plain value, tag=29, type=string
	VsaMotorolaCanopyLpulcir(u32), // from Motorola with plain value, tag=1, type=integer
	VsaMotorolaCanopyLpdlcir(u32), // from Motorola with plain value, tag=2, type=integer
	VsaMotorolaCanopyHpulcir(u32), // from Motorola with plain value, tag=3, type=integer
	VsaMotorolaCanopyHpdlcir(u32), // from Motorola with plain value, tag=4, type=integer
	VsaMotorolaCanopyHpenable(motorola::MotorolaCanopyHpenable), // from Motorola with Enum value, tag=5, type=integer
	VsaMotorolaCanopyUlbr(u32), // from Motorola with plain value, tag=6, type=integer
	VsaMotorolaCanopyUlbl(u32), // from Motorola with plain value, tag=7, type=integer
	VsaMotorolaCanopyDlbr(u32), // from Motorola with plain value, tag=8, type=integer
	VsaMotorolaCanopyDlbl(u32), // from Motorola with plain value, tag=9, type=integer
	VsaMotorolaCanopyVllearnen(motorola::MotorolaCanopyVllearnen), // from Motorola with Enum value, tag=14, type=integer
	VsaMotorolaCanopyVlframes(motorola::MotorolaCanopyVlframes), // from Motorola with Enum value, tag=15, type=integer
	VsaMotorolaCanopyVlidset(u32), // from Motorola with plain value, tag=16, type=integer
	VsaMotorolaCanopyVlageto(u32), // from Motorola with plain value, tag=20, type=integer
	VsaMotorolaCanopyVligvid(u32), // from Motorola with plain value, tag=21, type=integer
	VsaMotorolaCanopyVlmgvid(u32), // from Motorola with plain value, tag=22, type=integer
	VsaMotorolaCanopyVlsmmgpass(motorola::MotorolaCanopyVlsmmgpass), // from Motorola with Enum value, tag=23, type=integer
	VsaMotorolaCanopyBcastmir(u32), // from Motorola with plain value, tag=24, type=integer
	VsaMotorolaCanopyUserlevel(motorola::MotorolaCanopyUserlevel), // from Motorola with Enum value, tag=50, type=integer
	VsaMotorolaCanopySharedSecret(&'a [u8]), // from Motorola with plain value, tag=224, type=string
	VsaMotorolaCanopySuldr(&'a [u8]), // from Motorola with plain value, tag=225, type=string
	VsaMotorolaCanopySdldr(&'a [u8]), // from Motorola with plain value, tag=226, type=string
	VsaMotorolaCanopyUlba(&'a [u8]), // from Motorola with plain value, tag=227, type=string
	VsaMotorolaCanopyDlba(&'a [u8]), // from Motorola with plain value, tag=228, type=string
	VsaMotorolaCanopyEnable(&'a [u8]), // from Motorola with plain value, tag=229, type=string
	VsaMotorolaCanopyLpsuldr(&'a [u8]), // from Motorola with plain value, tag=230, type=string
	VsaMotorolaCanopyLpsdldr(&'a [u8]), // from Motorola with plain value, tag=231, type=string
	VsaMotorolaCanopyHpcenable(&'a [u8]), // from Motorola with plain value, tag=232, type=string
	VsaMotorolaCanopyHpsuldr(&'a [u8]), // from Motorola with plain value, tag=233, type=string
	VsaMotorolaCanopyHpsdldr(&'a [u8]), // from Motorola with plain value, tag=234, type=string
	VsaMotorolaCanopyHigherbw(&'a [u8]), // from Motorola with plain value, tag=235, type=string
	VsaMotorolaCanopyCirenable(&'a [u8]), // from Motorola with plain value, tag=236, type=string
	VsaMotorolaWimaxMipMnHomeAddress(Ipv4Addr), // from Motorola with plain value, tag=10, type=ipaddr
	VsaMotorolaWimaxMipKey(&'a [u8]), // from Motorola with plain value, tag=11, type=string
	VsaMotorolaWimaxMipSpi(u32), // from Motorola with plain value, tag=12, type=integer
	VsaMotorolaWimaxMnHa(Ipv4Addr), // from Motorola with plain value, tag=13, type=ipaddr
	VsaMotorolaWimaxNetworkDomainName(&'a [u8]), // from Motorola with plain value, tag=30, type=string
	VsaMotorolaWimaxEmsAddress(Ipv4Addr), // from Motorola with plain value, tag=31, type=ipaddr
	VsaMotorolaWimaxProvisioningServer(&'a [u8]), // from Motorola with plain value, tag=32, type=string
	VsaMotorolaWimaxNtpServer(&'a [u8]), // from Motorola with plain value, tag=34, type=octets
	VsaMotorolaWimaxHoSvcClass(&'a [u8]), // from Motorola with plain value, tag=35, type=octets
	VsaMotorolaWimaxMaximumTotalBandwidth(&'a [u8]), // from Motorola with plain value, tag=60, type=octets
	VsaMotorolaWimaxMaximumCommitBandwidth(&'a [u8]), // from Motorola with plain value, tag=61, type=octets
	VsaMotorolaWimaxConvergenceSublayer(&'a [u8]), // from Motorola with plain value, tag=63, type=octets
	VsaMotorolaWimaxServiceFlows(&'a [u8]), // from Motorola with plain value, tag=64, type=string
	VsaMotorolaWimaxVlanId(&'a [u8]), // from Motorola with plain value, tag=65, type=octets
	VsaMotorolaAccountingMessage(&'a [u8]), // from Motorola with plain value, tag=80, type=string
	VsaNaviniAvpair(&'a [u8]), // from Navini with plain value, tag=1, type=string
	VsaNsAdminPrivilege(netscreen::NsAdminPrivilege), // from Netscreen with Enum value, tag=1, type=integer
	VsaNsVsysName(&'a [u8]), // from Netscreen with plain value, tag=2, type=string
	VsaNsUserGroup(&'a [u8]), // from Netscreen with plain value, tag=3, type=string
	VsaNsPrimaryDns(Ipv4Addr), // from Netscreen with plain value, tag=4, type=ipaddr
	VsaNsSecondaryDns(Ipv4Addr), // from Netscreen with plain value, tag=5, type=ipaddr
	VsaNsPrimaryWins(Ipv4Addr), // from Netscreen with plain value, tag=6, type=ipaddr
	VsaNsSecondaryWins(Ipv4Addr), // from Netscreen with plain value, tag=7, type=ipaddr
	VsaNsNsmUserDomainName(&'a [u8]), // from Netscreen with plain value, tag=220, type=string
	VsaNsNsmUserRoleMapping(&'a [u8]), // from Netscreen with plain value, tag=221, type=string
	VsaNetsensoryPrivilege(&'a [u8]), // from NetworkPhysics with plain value, tag=33, type=string
	VsaNexansPortDefaultVlanId(u32), // from Nexans with plain value, tag=1, type=integer
	VsaNexansPortVoiceVlanId(u32), // from Nexans with plain value, tag=2, type=integer
	VsaUserlogonUid(u32), // from NTUA with plain value, tag=10, type=integer
	VsaUserlogonGid(u32), // from NTUA with plain value, tag=11, type=integer
	VsaUserlogonHomedir(&'a [u8]), // from NTUA with plain value, tag=12, type=string
	VsaUserlogonType(ntua::UserlogonType), // from NTUA with Enum value, tag=13, type=integer
	VsaUserlogonQuotabytes(u32), // from NTUA with plain value, tag=14, type=integer
	VsaUserlogonQuotafiles(u32), // from NTUA with plain value, tag=15, type=integer
	VsaUserlogonShell(&'a [u8]), // from NTUA with plain value, tag=16, type=string
	VsaUserlogonRestriction(ntua::UserlogonRestriction), // from NTUA with Enum value, tag=17, type=integer
	VsaUserlogonGroupnames(&'a [u8]), // from NTUA with plain value, tag=18, type=string
	VsaUserlogonDrivenames(&'a [u8]), // from NTUA with plain value, tag=19, type=string
	VsaUserlogonUserdescription(&'a [u8]), // from NTUA with plain value, tag=20, type=string
	VsaUserlogonUserfullname(&'a [u8]), // from NTUA with plain value, tag=21, type=string
	VsaUserlogonUserdomain(&'a [u8]), // from NTUA with plain value, tag=22, type=string
	VsaUserlogonLogontask(&'a [u8]), // from NTUA with plain value, tag=23, type=string
	VsaUserlogonLogofftask(&'a [u8]), // from NTUA with plain value, tag=24, type=string
	VsaUserlogonExpiration(&'a [u8]), // from NTUA with plain value, tag=25, type=string
	VsaUserlogonUserprofile(&'a [u8]), // from NTUA with plain value, tag=26, type=string
	VsaUserlogonAcctTerminatecause(&'a [u8]), // from NTUA with plain value, tag=50, type=string
	VsaNokiaAvpair(&'a [u8]), // from Nokia with plain value, tag=1, type=string
	VsaNokiaUserProfile(&'a [u8]), // from Nokia with plain value, tag=2, type=string
	VsaNokiaServiceName(&'a [u8]), // from Nokia with plain value, tag=3, type=octets
	VsaNokiaServiceId(&'a [u8]), // from Nokia with plain value, tag=4, type=octets
	VsaNokiaServiceUsername(&'a [u8]), // from Nokia with plain value, tag=5, type=octets
	VsaNokiaServicePassword(&'a [u8]), // from Nokia with plain value, tag=6, type=octets
	VsaNokiaServicePrimaryIndicator(&'a [u8]), // from Nokia with plain value, tag=7, type=octets
	VsaNokiaServiceChargingType(&'a [u8]), // from Nokia with plain value, tag=8, type=octets
	VsaNokiaServiceEncryptedPassword(&'a [u8]), // from Nokia with plain value, tag=9, type=octets
	VsaNokiaSessionAccessMethod(&'a [u8]), // from Nokia with plain value, tag=10, type=octets
	VsaNokiaSessionChargingType(&'a [u8]), // from Nokia with plain value, tag=11, type=octets
	VsaNokiaOcsId1(u32), // from Nokia with plain value, tag=12, type=integer
	VsaNokiaOcsId2(u32), // from Nokia with plain value, tag=13, type=integer
	VsaNokiaTrecIndex(u32), // from Nokia with plain value, tag=14, type=integer
	VsaNokiaRequestedApn(&'a [u8]), // from Nokia with plain value, tag=15, type=string
	VsaNomadixBwUp(u32), // from Nomadix with plain value, tag=1, type=integer
	VsaNomadixBwDown(u32), // from Nomadix with plain value, tag=2, type=integer
	VsaNomadixUrlRedirection(&'a [u8]), // from Nomadix with plain value, tag=3, type=string
	VsaNomadixIpUpsell(nomadix::NomadixIpUpsell), // from Nomadix with Enum value, tag=4, type=integer
	VsaNomadixExpiration(&'a [u8]), // from Nomadix with plain value, tag=5, type=string
	VsaNomadixSubnet(&'a [u8]), // from Nomadix with plain value, tag=6, type=string
	VsaNomadixMaxbytesup(u32), // from Nomadix with plain value, tag=7, type=integer
	VsaNomadixMaxbytesdown(u32), // from Nomadix with plain value, tag=8, type=integer
	VsaNomadixEndofsession(u32), // from Nomadix with plain value, tag=9, type=integer
	VsaNomadixLogoffUrl(&'a [u8]), // from Nomadix with plain value, tag=10, type=string
	VsaNomadixNetVlan(u32), // from Nomadix with plain value, tag=11, type=integer
	VsaNomadixConfigUrl(&'a [u8]), // from Nomadix with plain value, tag=12, type=string
	VsaNomadixGoodbyeUrl(&'a [u8]), // from Nomadix with plain value, tag=13, type=string
	VsaNomadixQosPolicy(&'a [u8]), // from Nomadix with plain value, tag=14, type=string
	VsaNomadixSmtpRedirect(u32), // from Nomadix with plain value, tag=17, type=integer
	VsaNomadixCentralizedMgmt(&'a [u8]), // from Nomadix with plain value, tag=18, type=string
	VsaNomadixGroupPolicyId(u32), // from Nomadix with plain value, tag=19, type=integer
	VsaNomadixGroupBwMaxUp(u32), // from Nomadix with plain value, tag=20, type=integer
	VsaNomadixGroupBwMaxDown(u32), // from Nomadix with plain value, tag=21, type=integer
	VsaNortelUserRole(&'a [u8]), // from Nortel with plain value, tag=110, type=string
	VsaNortelPrivilegeLevel(nortel::NortelPrivilegeLevel), // from Nortel with Enum value, tag=166, type=integer
	VsaPassportCommandScope(u32), // from Nortel with plain value, tag=200, type=integer
	VsaPassportCommandImpact(u32), // from Nortel with plain value, tag=201, type=integer
	VsaPassportCustomerIdentifier(u32), // from Nortel with plain value, tag=202, type=integer
	VsaPassportAllowedAccess(u32), // from Nortel with plain value, tag=203, type=integer
	VsaPassportAllowedoutAccess(u32), // from Nortel with plain value, tag=204, type=integer
	VsaPassportLoginDirectory(&'a [u8]), // from Nortel with plain value, tag=205, type=string
	VsaPassportTimeoutProtocol(u32), // from Nortel with plain value, tag=206, type=integer
	VsaPassportRole(&'a [u8]), // from Nortel with plain value, tag=207, type=string
	VsaPacketeerAvpair(&'a [u8]), // from Packeteer with plain value, tag=1, type=string
	VsaPacketeerPcAvpair(&'a [u8]), // from Packeteer with plain value, tag=2, type=string
	VsaPaloaltoAdminRole(&'a [u8]), // from PaloAlto with plain value, tag=1, type=string
	VsaPaloaltoAdminAccessDomain(&'a [u8]), // from PaloAlto with plain value, tag=2, type=string
	VsaPaloaltoPanoramaAdminRole(&'a [u8]), // from PaloAlto with plain value, tag=3, type=string
	VsaPaloaltoPanoramaAdminAccessDomain(&'a [u8]), // from PaloAlto with plain value, tag=4, type=string
	VsaPaloaltoUserGroup(&'a [u8]), // from PaloAlto with plain value, tag=5, type=string
	VsaPattonProtocol(&'a [u8]), // from Patton with plain value, tag=16, type=string
	VsaPattonSetupTime(&'a [u8]), // from Patton with plain value, tag=32, type=string
	VsaPattonConnectTime(&'a [u8]), // from Patton with plain value, tag=33, type=string
	VsaPattonDisconnectTime(&'a [u8]), // from Patton with plain value, tag=34, type=string
	VsaPattonDisconnectCause(patton::PattonDisconnectCause), // from Patton with Enum value, tag=35, type=integer
	VsaPattonDisconnectSource(&'a [u8]), // from Patton with plain value, tag=36, type=string
	VsaPattonCalledUniqueId(&'a [u8]), // from Patton with plain value, tag=48, type=string
	VsaPattonCalledIpAddress(Ipv4Addr), // from Patton with plain value, tag=49, type=ipaddr
	VsaPattonCalledNumberingPlan(&'a [u8]), // from Patton with plain value, tag=50, type=string
	VsaPattonCalledTypeOfNumber(&'a [u8]), // from Patton with plain value, tag=51, type=string
	VsaPattonCalledName(&'a [u8]), // from Patton with plain value, tag=52, type=string
	VsaPattonCalledStationId(&'a [u8]), // from Patton with plain value, tag=53, type=string
	VsaPattonCalledRxOctets(u32), // from Patton with plain value, tag=64, type=integer
	VsaPattonCalledTxOctets(u32), // from Patton with plain value, tag=65, type=integer
	VsaPattonCalledRxPackets(u32), // from Patton with plain value, tag=66, type=integer
	VsaPattonCalledTxPackets(u32), // from Patton with plain value, tag=67, type=integer
	VsaPattonCalledRxLostPackets(u32), // from Patton with plain value, tag=68, type=integer
	VsaPattonCalledTxLostPackets(u32), // from Patton with plain value, tag=69, type=integer
	VsaPattonCalledRxJitter(u32), // from Patton with plain value, tag=70, type=integer
	VsaPattonCalledTxJitter(u32), // from Patton with plain value, tag=71, type=integer
	VsaPattonCalledCodec(&'a [u8]), // from Patton with plain value, tag=72, type=string
	VsaPattonCalledRemoteIp(u32), // from Patton with plain value, tag=73, type=integer
	VsaPattonCalledRemoteUdpPort(u32), // from Patton with plain value, tag=74, type=integer
	VsaPattonCalledLocalUdpPort(u32), // from Patton with plain value, tag=75, type=integer
	VsaPattonCalledQos(u32), // from Patton with plain value, tag=76, type=integer
	VsaPattonCalledMos(u32), // from Patton with plain value, tag=77, type=integer
	VsaPattonCalledRoundTripTime(u32), // from Patton with plain value, tag=78, type=integer
	VsaPattonCallingUniqueId(&'a [u8]), // from Patton with plain value, tag=80, type=string
	VsaPattonCallingIpAddress(Ipv4Addr), // from Patton with plain value, tag=81, type=ipaddr
	VsaPattonCallingNumberingPlan(&'a [u8]), // from Patton with plain value, tag=82, type=string
	VsaPattonCallingTypeOfNumber(&'a [u8]), // from Patton with plain value, tag=83, type=string
	VsaPattonCallingPresentationIndicator(&'a [u8]), // from Patton with plain value, tag=88, type=string
	VsaPattonCallingScreeningIndicator(&'a [u8]), // from Patton with plain value, tag=89, type=string
	VsaPattonCallingName(&'a [u8]), // from Patton with plain value, tag=84, type=string
	VsaPattonCallingStationId(&'a [u8]), // from Patton with plain value, tag=85, type=string
	VsaPattonCallingRxOctets(u32), // from Patton with plain value, tag=96, type=integer
	VsaPattonCallingTxOctets(u32), // from Patton with plain value, tag=97, type=integer
	VsaPattonCallingRxPackets(u32), // from Patton with plain value, tag=98, type=integer
	VsaPattonCallingTxPackets(u32), // from Patton with plain value, tag=99, type=integer
	VsaPattonCallingLostTxPackets(u32), // from Patton with plain value, tag=100, type=integer
	VsaPattonCallingLostRxPackets(u32), // from Patton with plain value, tag=101, type=integer
	VsaPattonCallingRxJitter(u32), // from Patton with plain value, tag=102, type=integer
	VsaPattonCallingTxJitter(u32), // from Patton with plain value, tag=103, type=integer
	VsaPattonCallingCodec(&'a [u8]), // from Patton with plain value, tag=104, type=string
	VsaPattonCallingRemoteIp(u32), // from Patton with plain value, tag=105, type=integer
	VsaPattonCallingRemoteUdpPort(u32), // from Patton with plain value, tag=106, type=integer
	VsaPattonCallingLocalUdpPort(u32), // from Patton with plain value, tag=107, type=integer
	VsaPattonCallingQos(u32), // from Patton with plain value, tag=108, type=integer
	VsaPattonCallingMos(u32), // from Patton with plain value, tag=109, type=integer
	VsaPattonCallingRoundTripTime(u32), // from Patton with plain value, tag=110, type=integer
	VsaPerleClusteredPortAccess(perle::PerleClusteredPortAccess), // from Perle with Enum value, tag=99, type=integer
	VsaPerleUserLevel(perle::PerleUserLevel), // from Perle with Enum value, tag=100, type=integer
	VsaPerleLineAccessPort1(perle::PerleLineAccessPort1), // from Perle with Enum value, tag=101, type=integer
	VsaPerleLineAccessPort2(perle::PerleLineAccessPort2), // from Perle with Enum value, tag=102, type=integer
	VsaPerleLineAccessPort3(perle::PerleLineAccessPort3), // from Perle with Enum value, tag=103, type=integer
	VsaPerleLineAccessPort4(perle::PerleLineAccessPort4), // from Perle with Enum value, tag=104, type=integer
	VsaPerleLineAccessPort5(perle::PerleLineAccessPort5), // from Perle with Enum value, tag=105, type=integer
	VsaPerleLineAccessPort6(perle::PerleLineAccessPort6), // from Perle with Enum value, tag=106, type=integer
	VsaPerleLineAccessPort7(perle::PerleLineAccessPort7), // from Perle with Enum value, tag=107, type=integer
	VsaPerleLineAccessPort8(perle::PerleLineAccessPort8), // from Perle with Enum value, tag=108, type=integer
	VsaPerleLineAccessPort9(perle::PerleLineAccessPort9), // from Perle with Enum value, tag=109, type=integer
	VsaPerleLineAccessPort10(perle::PerleLineAccessPort10), // from Perle with Enum value, tag=110, type=integer
	VsaPerleLineAccessPort11(perle::PerleLineAccessPort11), // from Perle with Enum value, tag=111, type=integer
	VsaPerleLineAccessPort12(perle::PerleLineAccessPort12), // from Perle with Enum value, tag=112, type=integer
	VsaPerleLineAccessPort13(perle::PerleLineAccessPort13), // from Perle with Enum value, tag=113, type=integer
	VsaPerleLineAccessPort14(perle::PerleLineAccessPort14), // from Perle with Enum value, tag=114, type=integer
	VsaPerleLineAccessPort15(perle::PerleLineAccessPort15), // from Perle with Enum value, tag=115, type=integer
	VsaPerleLineAccessPort16(perle::PerleLineAccessPort16), // from Perle with Enum value, tag=116, type=integer
	VsaPerleLineAccessPort17(perle::PerleLineAccessPort17), // from Perle with Enum value, tag=117, type=integer
	VsaPerleLineAccessPort18(perle::PerleLineAccessPort18), // from Perle with Enum value, tag=118, type=integer
	VsaPerleLineAccessPort19(perle::PerleLineAccessPort19), // from Perle with Enum value, tag=119, type=integer
	VsaPerleLineAccessPort20(perle::PerleLineAccessPort20), // from Perle with Enum value, tag=120, type=integer
	VsaPerleLineAccessPort21(perle::PerleLineAccessPort21), // from Perle with Enum value, tag=121, type=integer
	VsaPerleLineAccessPort22(perle::PerleLineAccessPort22), // from Perle with Enum value, tag=122, type=integer
	VsaPerleLineAccessPort23(perle::PerleLineAccessPort23), // from Perle with Enum value, tag=123, type=integer
	VsaPerleLineAccessPort24(perle::PerleLineAccessPort24), // from Perle with Enum value, tag=124, type=integer
	VsaPerleLineAccessPort25(perle::PerleLineAccessPort25), // from Perle with Enum value, tag=125, type=integer
	VsaPerleLineAccessPort26(perle::PerleLineAccessPort26), // from Perle with Enum value, tag=126, type=integer
	VsaPerleLineAccessPort27(perle::PerleLineAccessPort27), // from Perle with Enum value, tag=127, type=integer
	VsaPerleLineAccessPort28(perle::PerleLineAccessPort28), // from Perle with Enum value, tag=128, type=integer
	VsaPerleLineAccessPort29(perle::PerleLineAccessPort29), // from Perle with Enum value, tag=129, type=integer
	VsaPerleLineAccessPort30(perle::PerleLineAccessPort30), // from Perle with Enum value, tag=130, type=integer
	VsaPerleLineAccessPort31(perle::PerleLineAccessPort31), // from Perle with Enum value, tag=131, type=integer
	VsaPerleLineAccessPort32(perle::PerleLineAccessPort32), // from Perle with Enum value, tag=132, type=integer
	VsaPerleLineAccessPort33(perle::PerleLineAccessPort33), // from Perle with Enum value, tag=133, type=integer
	VsaPerleLineAccessPort34(perle::PerleLineAccessPort34), // from Perle with Enum value, tag=134, type=integer
	VsaPerleLineAccessPort35(perle::PerleLineAccessPort35), // from Perle with Enum value, tag=135, type=integer
	VsaPerleLineAccessPort36(perle::PerleLineAccessPort36), // from Perle with Enum value, tag=136, type=integer
	VsaPerleLineAccessPort37(perle::PerleLineAccessPort37), // from Perle with Enum value, tag=137, type=integer
	VsaPerleLineAccessPort38(perle::PerleLineAccessPort38), // from Perle with Enum value, tag=138, type=integer
	VsaPerleLineAccessPort39(perle::PerleLineAccessPort39), // from Perle with Enum value, tag=139, type=integer
	VsaPerleLineAccessPort40(perle::PerleLineAccessPort40), // from Perle with Enum value, tag=140, type=integer
	VsaPerleLineAccessPort41(perle::PerleLineAccessPort41), // from Perle with Enum value, tag=141, type=integer
	VsaPerleLineAccessPort42(perle::PerleLineAccessPort42), // from Perle with Enum value, tag=142, type=integer
	VsaPerleLineAccessPort43(perle::PerleLineAccessPort43), // from Perle with Enum value, tag=143, type=integer
	VsaPerleLineAccessPort44(perle::PerleLineAccessPort44), // from Perle with Enum value, tag=144, type=integer
	VsaPerleLineAccessPort45(perle::PerleLineAccessPort45), // from Perle with Enum value, tag=145, type=integer
	VsaPerleLineAccessPort46(perle::PerleLineAccessPort46), // from Perle with Enum value, tag=146, type=integer
	VsaPerleLineAccessPort47(perle::PerleLineAccessPort47), // from Perle with Enum value, tag=147, type=integer
	VsaPerleLineAccessPort48(perle::PerleLineAccessPort48), // from Perle with Enum value, tag=148, type=integer
	VsaPerleLineAccessPort49(perle::PerleLineAccessPort49), // from Perle with Enum value, tag=149, type=integer
	VsaPropelAccelerate(u32), // from Propel with plain value, tag=1, type=integer
	VsaPropelDialedDigits(&'a [u8]), // from Propel with plain value, tag=2, type=string
	VsaPropelClientIpAddress(Ipv4Addr), // from Propel with plain value, tag=3, type=ipaddr
	VsaPropelClientNasIpAddress(Ipv4Addr), // from Propel with plain value, tag=4, type=ipaddr
	VsaPropelClientSourceId(u32), // from Propel with plain value, tag=5, type=integer
	VsaPropelContentFilterId(u32), // from Propel with plain value, tag=6, type=integer
	VsaProsoftHomeAgentAddress(Ipv4Addr), // from Prosoft with plain value, tag=0, type=ipaddr
	VsaProsoftDefaultGateway(Ipv4Addr), // from Prosoft with plain value, tag=1, type=ipaddr
	VsaProsoftPrimaryDns(Ipv4Addr), // from Prosoft with plain value, tag=2, type=ipaddr
	VsaProsoftSecondaryDns(Ipv4Addr), // from Prosoft with plain value, tag=3, type=ipaddr
	VsaProsoftSecurityParameterIndex(u32), // from Prosoft with plain value, tag=4, type=integer
	VsaProsoftSecurityKey(&'a [u8]), // from Prosoft with plain value, tag=5, type=string
	VsaProsoftMacAddress(&'a [u8]), // from Prosoft with plain value, tag=7, type=string
	VsaProsoftAuthenticationReason(u32), // from Prosoft with plain value, tag=8, type=integer
	VsaProsoftAtmInterface(u32), // from Prosoft with plain value, tag=9, type=integer
	VsaProsoftAtmVpi(u32), // from Prosoft with plain value, tag=10, type=integer
	VsaProsoftAtmVci(u32), // from Prosoft with plain value, tag=11, type=integer
	VsaProsoftRscIdentifier(&'a [u8]), // from Prosoft with plain value, tag=12, type=string
	VsaProsoftNpmIdentifier(&'a [u8]), // from Prosoft with plain value, tag=13, type=string
	VsaProsoftNpmIp(&'a [u8]), // from Prosoft with plain value, tag=14, type=string
	VsaProsoftSectorId(&'a [u8]), // from Prosoft with plain value, tag=15, type=string
	VsaProsoftAuthRole(prosoft::ProsoftAuthRole), // from Prosoft with Enum value, tag=16, type=integer
	VsaProximE1VlanMode(u32), // from Proxim with plain value, tag=4, type=integer
	VsaProximSuVlanName(&'a [u8]), // from Proxim with plain value, tag=5, type=string
	VsaProximE1AccessVlanId(u32), // from Proxim with plain value, tag=6, type=integer
	VsaProximE1AccessVlanPri(u32), // from Proxim with plain value, tag=7, type=integer
	VsaProximMgmtVlanId(u32), // from Proxim with plain value, tag=8, type=integer
	VsaProximMgmtVlanPri(u32), // from Proxim with plain value, tag=9, type=integer
	VsaProximE1Trunkid01(u32), // from Proxim with plain value, tag=10, type=integer
	VsaProximE1Trunkid02(u32), // from Proxim with plain value, tag=11, type=integer
	VsaProximE1Trunkid03(u32), // from Proxim with plain value, tag=12, type=integer
	VsaProximE1Trunkid04(u32), // from Proxim with plain value, tag=13, type=integer
	VsaProximE1Trunkid05(u32), // from Proxim with plain value, tag=14, type=integer
	VsaProximE1Trunkid06(u32), // from Proxim with plain value, tag=15, type=integer
	VsaProximE1Trunkid07(u32), // from Proxim with plain value, tag=16, type=integer
	VsaProximE1Trunkid08(u32), // from Proxim with plain value, tag=17, type=integer
	VsaProximE1Trunkid09(u32), // from Proxim with plain value, tag=18, type=integer
	VsaProximE1Trunkid10(u32), // from Proxim with plain value, tag=19, type=integer
	VsaProximE1Trunkid11(u32), // from Proxim with plain value, tag=20, type=integer
	VsaProximE1Trunkid12(u32), // from Proxim with plain value, tag=21, type=integer
	VsaProximE1Trunkid13(u32), // from Proxim with plain value, tag=22, type=integer
	VsaProximE1Trunkid14(u32), // from Proxim with plain value, tag=23, type=integer
	VsaProximE1Trunkid15(u32), // from Proxim with plain value, tag=24, type=integer
	VsaProximE1Trunkid16(u32), // from Proxim with plain value, tag=25, type=integer
	VsaProximSuVlanTableStatus(u32), // from Proxim with plain value, tag=26, type=integer
	VsaProximServiceVlanId(u32), // from Proxim with plain value, tag=32, type=integer
	VsaProximServiceVlanPri(u32), // from Proxim with plain value, tag=33, type=integer
	VsaProximQosClassIndex(u32), // from Proxim with plain value, tag=34, type=integer
	VsaProximQosClassSuStatus(u32), // from Proxim with plain value, tag=35, type=integer
	VsaProximE2VlanMode(u32), // from Proxim with plain value, tag=40, type=integer
	VsaProximE2AccessVlanId(u32), // from Proxim with plain value, tag=41, type=integer
	VsaProximE2AccessVlanPri(u32), // from Proxim with plain value, tag=42, type=integer
	VsaProximE2Trunkid01(u32), // from Proxim with plain value, tag=43, type=integer
	VsaProximE2Trunkid02(u32), // from Proxim with plain value, tag=44, type=integer
	VsaProximE2Trunkid03(u32), // from Proxim with plain value, tag=45, type=integer
	VsaProximE2Trunkid04(u32), // from Proxim with plain value, tag=46, type=integer
	VsaProximE2Trunkid05(u32), // from Proxim with plain value, tag=47, type=integer
	VsaProximE2Trunkid06(u32), // from Proxim with plain value, tag=48, type=integer
	VsaProximE2Trunkid07(u32), // from Proxim with plain value, tag=49, type=integer
	VsaProximE2Trunkid08(u32), // from Proxim with plain value, tag=50, type=integer
	VsaProximE2Trunkid09(u32), // from Proxim with plain value, tag=51, type=integer
	VsaProximE2Trunkid10(u32), // from Proxim with plain value, tag=52, type=integer
	VsaProximE2Trunkid11(u32), // from Proxim with plain value, tag=53, type=integer
	VsaProximE2Trunkid12(u32), // from Proxim with plain value, tag=54, type=integer
	VsaProximE2Trunkid13(u32), // from Proxim with plain value, tag=55, type=integer
	VsaProximE2Trunkid14(u32), // from Proxim with plain value, tag=56, type=integer
	VsaProximE2Trunkid15(u32), // from Proxim with plain value, tag=57, type=integer
	VsaProximE2Trunkid16(u32), // from Proxim with plain value, tag=58, type=integer
	VsaProximQinqStatus(u32), // from Proxim with plain value, tag=59, type=integer
	VsaProximServiceVlanTpid(u32), // from Proxim with plain value, tag=60, type=integer
	VsaProximE1PortVlanId(u32), // from Proxim with plain value, tag=61, type=integer
	VsaProximE1PortVlanPri(u32), // from Proxim with plain value, tag=62, type=integer
	VsaProximE1AllowUntag(u32), // from Proxim with plain value, tag=63, type=integer
	VsaProximE2PortVlanId(u32), // from Proxim with plain value, tag=64, type=integer
	VsaProximE2PortVlanPri(u32), // from Proxim with plain value, tag=65, type=integer
	VsaProximE2AllowUntag(u32), // from Proxim with plain value, tag=66, type=integer
	VsaProximE1SuAllowUntagMgmt(u32), // from Proxim with plain value, tag=68, type=integer
	VsaProximE2SuAllowUntagMgmt(u32), // from Proxim with plain value, tag=69, type=integer
	VsaPurewaveClientProfile(u32), // from Purewave with plain value, tag=1, type=integer
	VsaPurewaveCsType(purewave::PurewaveCsType), // from Purewave with Enum value, tag=2, type=integer
	VsaPurewaveMaxDownlinkRate(purewave::PurewaveMaxDownlinkRate), // from Purewave with Enum value, tag=3, type=integer
	VsaPurewaveMaxUplinkRate(purewave::PurewaveMaxUplinkRate), // from Purewave with Enum value, tag=4, type=integer
	VsaPurewaveIpAddress(Ipv4Addr), // from Purewave with plain value, tag=5, type=ipaddr
	VsaPurewaveIpNetmask(Ipv4Addr), // from Purewave with plain value, tag=6, type=ipaddr
	VsaPurewaveServiceEnable(u32), // from Purewave with plain value, tag=7, type=integer
	VsaQuiconnectAvpair(&'a [u8]), // from Quiconnect with plain value, tag=1, type=string
	VsaQuiconnectVnpInformation(&'a [u8]), // from Quiconnect with plain value, tag=2, type=string
	VsaQuiconnectHspInformation(&'a [u8]), // from Quiconnect with plain value, tag=3, type=string
	VsaQuintumAvpair(&'a [u8]), // from Quintum with plain value, tag=1, type=string
	VsaQuintumNasPort(&'a [u8]), // from Quintum with plain value, tag=2, type=string
	VsaQuintumH323RemoteAddress(&'a [u8]), // from Quintum with plain value, tag=23, type=string
	VsaQuintumH323ConfId(&'a [u8]), // from Quintum with plain value, tag=24, type=string
	VsaQuintumH323SetupTime(&'a [u8]), // from Quintum with plain value, tag=25, type=string
	VsaQuintumH323CallOrigin(&'a [u8]), // from Quintum with plain value, tag=26, type=string
	VsaQuintumH323CallType(&'a [u8]), // from Quintum with plain value, tag=27, type=string
	VsaQuintumH323ConnectTime(&'a [u8]), // from Quintum with plain value, tag=28, type=string
	VsaQuintumH323DisconnectTime(&'a [u8]), // from Quintum with plain value, tag=29, type=string
	VsaQuintumH323DisconnectCause(&'a [u8]), // from Quintum with plain value, tag=30, type=string
	VsaQuintumH323VoiceQuality(&'a [u8]), // from Quintum with plain value, tag=31, type=string
	VsaQuintumH323GwId(&'a [u8]), // from Quintum with plain value, tag=33, type=string
	VsaQuintumH323IncomingConfId(&'a [u8]), // from Quintum with plain value, tag=35, type=string
	VsaQuintumH323CreditAmount(&'a [u8]), // from Quintum with plain value, tag=101, type=string
	VsaQuintumH323CreditTime(&'a [u8]), // from Quintum with plain value, tag=102, type=string
	VsaQuintumH323ReturnCode(&'a [u8]), // from Quintum with plain value, tag=103, type=string
	VsaQuintumH323PromptId(&'a [u8]), // from Quintum with plain value, tag=104, type=string
	VsaQuintumH323TimeAndDay(&'a [u8]), // from Quintum with plain value, tag=105, type=string
	VsaQuintumH323RedirectNumber(&'a [u8]), // from Quintum with plain value, tag=106, type=string
	VsaQuintumH323PreferredLang(&'a [u8]), // from Quintum with plain value, tag=107, type=string
	VsaQuintumH323RedirectIpAddress(&'a [u8]), // from Quintum with plain value, tag=108, type=string
	VsaQuintumH323BillingModel(&'a [u8]), // from Quintum with plain value, tag=109, type=string
	VsaQuintumH323CurrencyType(&'a [u8]), // from Quintum with plain value, tag=110, type=string
	VsaQuintumTrunkidIn(&'a [u8]), // from Quintum with plain value, tag=230, type=string
	VsaQuintumTrunkidOut(&'a [u8]), // from Quintum with plain value, tag=231, type=string
	VsaRedcreekTunneledIpAddr(Ipv4Addr), // from RedCreek with plain value, tag=5, type=ipaddr
	VsaRedcreekTunneledIpNetmask(Ipv4Addr), // from RedCreek with plain value, tag=6, type=ipaddr
	VsaRedcreekTunneledGateway(Ipv4Addr), // from RedCreek with plain value, tag=7, type=ipaddr
	VsaRedcreekTunneledDnsServer(&'a [u8]), // from RedCreek with plain value, tag=8, type=string
	VsaRedcreekTunneledWinsServer1(&'a [u8]), // from RedCreek with plain value, tag=9, type=string
	VsaRedcreekTunneledWinsServer2(&'a [u8]), // from RedCreek with plain value, tag=10, type=string
	VsaRedcreekTunneledHostname(&'a [u8]), // from RedCreek with plain value, tag=11, type=string
	VsaRedcreekTunneledDomainname(&'a [u8]), // from RedCreek with plain value, tag=12, type=string
	VsaRedcreekTunneledSearchList(&'a [u8]), // from RedCreek with plain value, tag=13, type=string
	VsaRiverbedLocalUser(&'a [u8]), // from Riverbed with plain value, tag=1, type=string
	VsaRiverstoneCommand(&'a [u8]), // from Riverstone with plain value, tag=1, type=string
	VsaRiverstoneSystemEvent(&'a [u8]), // from Riverstone with plain value, tag=2, type=string
	VsaRiverstoneSnmpConfigChange(&'a [u8]), // from Riverstone with plain value, tag=3, type=string
	VsaRiverstoneUserLevel(u32), // from Riverstone with plain value, tag=4, type=integer
	VsaRpUpstreamSpeedLimit(u32), // from Roaring-Penguin with plain value, tag=1, type=integer
	VsaRpDownstreamSpeedLimit(u32), // from Roaring-Penguin with plain value, tag=2, type=integer
	VsaRpHurl(&'a [u8]), // from Roaring-Penguin with plain value, tag=3, type=string
	VsaRpMotm(&'a [u8]), // from Roaring-Penguin with plain value, tag=4, type=string
	VsaRpMaxSessionsPerUser(u32), // from Roaring-Penguin with plain value, tag=5, type=integer
	VsaRuckusUserGroups(&'a [u8]), // from Ruckus with plain value, tag=1, type=string
	VsaRuckusStaRssi(u32), // from Ruckus with plain value, tag=2, type=integer
	VsaRuckusSsid(&'a [u8]), // from Ruckus with plain value, tag=3, type=string
	VsaRuckusWlanId(u32), // from Ruckus with plain value, tag=4, type=integer
	VsaRuckusLocation(&'a [u8]), // from Ruckus with plain value, tag=5, type=string
	VsaRuckusGracePeriod(u32), // from Ruckus with plain value, tag=6, type=integer
	VsaRuckusScgCbladeIp(u32), // from Ruckus with plain value, tag=7, type=integer
	VsaRuckusScgDbladeIp(u32), // from Ruckus with plain value, tag=8, type=integer
	VsaRuckusVlanId(u32), // from Ruckus with plain value, tag=9, type=integer
	VsaRuckusStaExpiration(u32), // from Ruckus with plain value, tag=10, type=integer
	VsaRuckusStaUuid(&'a [u8]), // from Ruckus with plain value, tag=11, type=string
	VsaRuckusAcceptEnhancementReason(u32), // from Ruckus with plain value, tag=12, type=integer
	VsaRuckusStaInnerId(&'a [u8]), // from Ruckus with plain value, tag=13, type=string
	VsaRuckusBssid(&'a [u8]), // from Ruckus with plain value, tag=14, type=octets
	VsaRuckusTriplets(&'a [u8]), // from Ruckus with plain value, tag=101, type=octets
	VsaRuckusImsi(&'a [u8]), // from Ruckus with plain value, tag=102, type=octets
	VsaRuckusMsisdn(&'a [u8]), // from Ruckus with plain value, tag=103, type=octets
	VsaRuckusApnNi(&'a [u8]), // from Ruckus with plain value, tag=104, type=string
	VsaRuckusQos(&'a [u8]), // from Ruckus with plain value, tag=105, type=octets
	VsaRuckusSelectionMode(ruckus::RuckusSelectionMode), // from Ruckus with Enum value, tag=106, type=integer
	VsaRuckusApnResolutionReq(ruckus::RuckusApnResolutionReq), // from Ruckus with Enum value, tag=107, type=integer
	VsaRuckusStartTime(&'a [u8]), // from Ruckus with plain value, tag=108, type=octets
	VsaRuckusNasType(ruckus::RuckusNasType), // from Ruckus with Enum value, tag=109, type=integer
	VsaRuckusStatus(ruckus::RuckusStatus), // from Ruckus with Enum value, tag=110, type=integer
	VsaRuckusApnOi(&'a [u8]), // from Ruckus with plain value, tag=111, type=string
	VsaRuckusAuthType(ruckus::RuckusAuthType), // from Ruckus with Enum value, tag=112, type=integer
	VsaRuckusGnUserName(&'a [u8]), // from Ruckus with plain value, tag=113, type=string
	VsaRuckusBrandCode(&'a [u8]), // from Ruckus with plain value, tag=114, type=string
	VsaRuckusPolicyName(&'a [u8]), // from Ruckus with plain value, tag=115, type=string
	VsaRuckusClientLocalIp(Ipv4Addr), // from Ruckus with plain value, tag=116, type=ipaddr
	VsaRuckusSgsnIp(Ipv4Addr), // from Ruckus with plain value, tag=117, type=ipaddr
	VsaRuckusChargingCharac(&'a [u8]), // from Ruckus with plain value, tag=118, type=octets
	VsaRuckusPdpType(&'a [u8]), // from Ruckus with plain value, tag=119, type=octets
	VsaRuckusDynamicAddressFlag(&'a [u8]), // from Ruckus with plain value, tag=120, type=octets
	VsaRuckusChchSelectionMode(&'a [u8]), // from Ruckus with plain value, tag=121, type=octets
	VsaRuckusAaaIp(Ipv4Addr), // from Ruckus with plain value, tag=122, type=ipaddr
	VsaRuckusCdrType(u32), // from Ruckus with plain value, tag=123, type=integer
	VsaRuckusSgsnNumber(&'a [u8]), // from Ruckus with plain value, tag=124, type=octets
	VsaRuckusSessionType(ruckus::RuckusSessionType), // from Ruckus with Enum value, tag=125, type=integer
	VsaRuckusAccountingStatus(ruckus::RuckusAccountingStatus), // from Ruckus with Enum value, tag=126, type=integer
	VsaRuckusZoneId(&'a [u8]), // from Ruckus with plain value, tag=127, type=string
	VsaRuckusAuthServerId(&'a [u8]), // from Ruckus with plain value, tag=128, type=string
	VsaRuckusUtpId(&'a [u8]), // from Ruckus with plain value, tag=129, type=string
	VsaRuckusAreaCode(&'a [u8]), // from Ruckus with plain value, tag=130, type=octets
	VsaRuckusCellIdentifier(&'a [u8]), // from Ruckus with plain value, tag=131, type=octets
	VsaRuckusWisprRedirectPolicy(&'a [u8]), // from Ruckus with plain value, tag=132, type=string
	VsaRuckusEthProfileId(u32), // from Ruckus with plain value, tag=133, type=integer
	VsaRuckusZoneName(&'a [u8]), // from Ruckus with plain value, tag=134, type=string
	VsaRuckusWlanName(&'a [u8]), // from Ruckus with plain value, tag=135, type=string
	VsaRuggedcomPrivilegeLevel(&'a [u8]), // from RuggedCom with plain value, tag=2, type=string
	VsaNetborderAvpair(&'a [u8]), // from NetBorder with plain value, tag=1, type=string
	VsaNetborderClid(&'a [u8]), // from NetBorder with plain value, tag=2, type=string
	VsaNetborderDialplan(&'a [u8]), // from NetBorder with plain value, tag=3, type=string
	VsaNetborderSrc(&'a [u8]), // from NetBorder with plain value, tag=4, type=string
	VsaNetborderDst(&'a [u8]), // from NetBorder with plain value, tag=5, type=string
	VsaNetborderSrcChannel(&'a [u8]), // from NetBorder with plain value, tag=6, type=string
	VsaNetborderDstChannel(&'a [u8]), // from NetBorder with plain value, tag=7, type=string
	VsaNetborderAni(&'a [u8]), // from NetBorder with plain value, tag=8, type=string
	VsaNetborderAniii(&'a [u8]), // from NetBorder with plain value, tag=9, type=string
	VsaNetborderLastapp(&'a [u8]), // from NetBorder with plain value, tag=10, type=string
	VsaNetborderLastdata(&'a [u8]), // from NetBorder with plain value, tag=11, type=string
	VsaNetborderDisposition(&'a [u8]), // from NetBorder with plain value, tag=12, type=string
	VsaNetborderHangupcause(netborder::NetborderHangupcause), // from NetBorder with Enum value, tag=13, type=integer
	VsaNetborderBillusec(u32), // from NetBorder with plain value, tag=15, type=integer
	VsaNetborderAmaflags(u32), // from NetBorder with plain value, tag=16, type=integer
	VsaNetborderRdnis(&'a [u8]), // from NetBorder with plain value, tag=17, type=string
	VsaNetborderContext(&'a [u8]), // from NetBorder with plain value, tag=18, type=string
	VsaNetborderSource(&'a [u8]), // from NetBorder with plain value, tag=19, type=string
	VsaNetborderCallstartdate(&'a [u8]), // from NetBorder with plain value, tag=20, type=string
	VsaNetborderCallanswerdate(&'a [u8]), // from NetBorder with plain value, tag=21, type=string
	VsaNetborderCalltransferdate(&'a [u8]), // from NetBorder with plain value, tag=22, type=string
	VsaNetborderCallenddate(&'a [u8]), // from NetBorder with plain value, tag=23, type=string
	VsaNetborderSignalbond(&'a [u8]), // from NetBorder with plain value, tag=24, type=string
	VsaShastaUserPrivilege(shasta::ShastaUserPrivilege), // from Shasta with Enum value, tag=1, type=integer
	VsaShastaServiceProfile(&'a [u8]), // from Shasta with plain value, tag=2, type=string
	VsaShastaVpnName(&'a [u8]), // from Shasta with plain value, tag=3, type=string
	VsaSgFilterRedirectGw(Ipv4Addr), // from SG with plain value, tag=1, type=ipaddr
	VsaSgAccounting(sg::SgAccounting), // from SG with Enum value, tag=10, type=integer
	VsaSgOrigName(&'a [u8]), // from SG with plain value, tag=12, type=string
	VsaSgAuthType(sg::SgAuthType), // from SG with Enum value, tag=13, type=integer
	VsaSgAction(sg::SgAction), // from SG with Enum value, tag=14, type=integer
	VsaSgSscHost(Ipv4Addr), // from SG with plain value, tag=15, type=ipaddr
	VsaSgServiceName(&'a [u8]), // from SG with plain value, tag=16, type=string
	VsaSgPersonalSite(&'a [u8]), // from SG with plain value, tag=17, type=string
	VsaSgMacAddress(&'a [u8]), // from SG with plain value, tag=18, type=string
	VsaSgUserGroup(u32), // from SG with plain value, tag=19, type=integer
	VsaSgMaxAllowedSessions(u32), // from SG with plain value, tag=20, type=integer
	VsaSgClass(&'a [u8]), // from SG with plain value, tag=21, type=string
	VsaSgEdsEncKey(&'a [u8]), // from SG with plain value, tag=22, type=string
	VsaSgEdsCookie(&'a [u8]), // from SG with plain value, tag=23, type=string
	VsaSgOriginalUrlPrefix(&'a [u8]), // from SG with plain value, tag=24, type=string
	VsaSgMaxAllowedNodes(u32), // from SG with plain value, tag=25, type=integer
	VsaSgParentUserName(&'a [u8]), // from SG with plain value, tag=26, type=string
	VsaSgNodeGroup(u32), // from SG with plain value, tag=27, type=integer
	VsaSgNodeDefaultService(&'a [u8]), // from SG with plain value, tag=28, type=string
	VsaSgNodeDynamicService(&'a [u8]), // from SG with plain value, tag=29, type=string
	VsaSgDhcpServer(Ipv4Addr), // from SG with plain value, tag=30, type=ipaddr
	VsaSgOpt82RelayRemoteId(&'a [u8]), // from SG with plain value, tag=31, type=string
	VsaSgDiscoverAction(sg::SgDiscoverAction), // from SG with Enum value, tag=32, type=integer
	VsaSgReleaseAction(sg::SgReleaseAction), // from SG with Enum value, tag=33, type=integer
	VsaSgFixedIpAddress(&'a [u8]), // from SG with plain value, tag=34, type=string
	VsaSgNodeFixedIpAddress(&'a [u8]), // from SG with plain value, tag=35, type=string
	VsaSgLeaseTime(u32), // from SG with plain value, tag=36, type=integer
	VsaSgProtocolType(sg::SgProtocolType), // from SG with Enum value, tag=40, type=integer
	VsaSgServiceTimeout(u32), // from SG with plain value, tag=50, type=integer
	VsaSgNextServiceName(&'a [u8]), // from SG with plain value, tag=51, type=string
	VsaSgAutoServiceName(&'a [u8]), // from SG with plain value, tag=52, type=string
	VsaSgAuthSource(sg::SgAuthSource), // from SG with Enum value, tag=53, type=integer
	VsaSgDataQuota(&'a [u8]), // from SG with plain value, tag=54, type=string
	VsaSgAclDataQuota(&'a [u8]), // from SG with plain value, tag=55, type=string
	VsaSgServiceCache(sg::SgServiceCache), // from SG with Enum value, tag=56, type=integer
	VsaSgDataQuotaUsed(&'a [u8]), // from SG with plain value, tag=57, type=string
	VsaSgAclDataQuotaUsed(&'a [u8]), // from SG with plain value, tag=58, type=string
	VsaSgAclPacketQuota(&'a [u8]), // from SG with plain value, tag=59, type=string
	VsaSgAclPacketQuotaUsed(&'a [u8]), // from SG with plain value, tag=60, type=string
	VsaSgRoaming(sg::SgRoaming), // from SG with Enum value, tag=61, type=integer
	VsaSgAclEdsAction(&'a [u8]), // from SG with plain value, tag=62, type=string
	VsaSgAclIdleIgnore(&'a [u8]), // from SG with plain value, tag=63, type=string
	VsaSgServiceQuotaIgnore(&'a [u8]), // from SG with plain value, tag=65, type=string
	VsaSgServiceAclQuotaIgnore(&'a [u8]), // from SG with plain value, tag=66, type=string
	VsaSgServiceAclQuotaIndication(&'a [u8]), // from SG with plain value, tag=67, type=string
	VsaSgRemoteFilterRedirectGw(&'a [u8]), // from SG with plain value, tag=70, type=string
	VsaSgNextHop(Ipv4Addr), // from SG with plain value, tag=71, type=ipaddr
	VsaSgNipPipeNextHop(Ipv4Addr), // from SG with plain value, tag=72, type=ipaddr
	VsaSgAdvertiseProtocol(sg::SgAdvertiseProtocol), // from SG with Enum value, tag=73, type=integer
	VsaSgForwardAddr(Ipv4Addr), // from SG with plain value, tag=74, type=ipaddr
	VsaSgAclTcpNatRedirect(&'a [u8]), // from SG with plain value, tag=75, type=string
	VsaSgAclNextHop(&'a [u8]), // from SG with plain value, tag=76, type=string
	VsaSgTunnelId(&'a [u8]), // from SG with plain value, tag=80, type=string
	VsaSgL2TpTunnelPassword(&'a [u8]), // from SG with plain value, tag=81, type=string
	VsaSgIpAddress(&'a [u8]), // from SG with plain value, tag=82, type=string
	VsaSgTunnelAssignmentId(u32), // from SG with plain value, tag=83, type=integer
	VsaSgTunnelClientIpAddress(Ipv4Addr), // from SG with plain value, tag=84, type=ipaddr
	VsaSgNativeip(sg::SgNativeip), // from SG with Enum value, tag=85, type=integer
	VsaSgIpTunnel(&'a [u8]), // from SG with plain value, tag=86, type=string
	VsaSgUpMeanRate(&'a [u8]), // from SG with plain value, tag=90, type=string
	VsaSgDownMeanRate(&'a [u8]), // from SG with plain value, tag=91, type=string
	VsaSgAclUpMeanRate(&'a [u8]), // from SG with plain value, tag=92, type=string
	VsaSgAclDownMeanRate(&'a [u8]), // from SG with plain value, tag=93, type=string
	VsaSgCos(&'a [u8]), // from SG with plain value, tag=94, type=string
	VsaSgAclPriority(&'a [u8]), // from SG with plain value, tag=95, type=string
	VsaSgBurstSize(u32), // from SG with plain value, tag=96, type=integer
	VsaSgIpPrimary(Ipv4Addr), // from SG with plain value, tag=100, type=ipaddr
	VsaSgIpSecondary(Ipv4Addr), // from SG with plain value, tag=101, type=ipaddr
	VsaSgWimaxReducedResources(sg::SgWimaxReducedResources), // from SG with Enum value, tag=110, type=integer
	VsaSgWimaxAclScheduleType(&'a [u8]), // from SG with plain value, tag=111, type=string
	VsaSgWimaxAclMinReservedTrafficRate(&'a [u8]), // from SG with plain value, tag=112, type=string
	VsaSgWimaxAclMaximumTrafficBurst(&'a [u8]), // from SG with plain value, tag=113, type=string
	VsaSgWimaxAclToleratedJitter(&'a [u8]), // from SG with plain value, tag=114, type=string
	VsaSgWimaxAclMaximumLatency(&'a [u8]), // from SG with plain value, tag=115, type=string
	VsaSgWimaxAclUnsolicitedGrantInt(&'a [u8]), // from SG with plain value, tag=116, type=string
	VsaSgWimaxAclSduSize(&'a [u8]), // from SG with plain value, tag=117, type=string
	VsaSgWimaxAclUnsolicitedPollingInt(&'a [u8]), // from SG with plain value, tag=118, type=string
	VsaSgWimaxMskLifetime(u32), // from SG with plain value, tag=119, type=integer
	VsaSgWimaxDmActionCode(sg::SgWimaxDmActionCode), // from SG with Enum value, tag=120, type=integer
	VsaSgWimaxAclArqEnable(&'a [u8]), // from SG with plain value, tag=121, type=string
	VsaSgWimaxBsidNextHop(Ipv4Addr), // from SG with plain value, tag=122, type=ipaddr
	VsaSgWimaxMobilityFeaturesSupported(sg::SgWimaxMobilityFeaturesSupported), // from SG with Enum value, tag=123, type=integer
	VsaSgWimaxNodeDisconnect(sg::SgWimaxNodeDisconnect), // from SG with Enum value, tag=124, type=integer
	VsaSgWimaxServiceFlowModification(sg::SgWimaxServiceFlowModification), // from SG with Enum value, tag=125, type=integer
	VsaSgWimaxServiceFlowDown(&'a [u8]), // from SG with plain value, tag=126, type=string
	VsaSgNodeAcctUsername(&'a [u8]), // from SG with plain value, tag=130, type=string
	VsaShivaUserAttributes(&'a [u8]), // from Shiva with plain value, tag=1, type=string
	VsaShivaCompression(shiva::ShivaCompression), // from Shiva with Enum value, tag=30, type=integer
	VsaShivaDialbackDelay(u32), // from Shiva with plain value, tag=31, type=integer
	VsaShivaCallDurnTrap(u32), // from Shiva with plain value, tag=32, type=integer
	VsaShivaBandwidthTrap(u32), // from Shiva with plain value, tag=33, type=integer
	VsaShivaMinimumCall(u32), // from Shiva with plain value, tag=34, type=integer
	VsaShivaDefaultHost(&'a [u8]), // from Shiva with plain value, tag=35, type=string
	VsaShivaMenuName(&'a [u8]), // from Shiva with plain value, tag=36, type=string
	VsaShivaUserFlags(&'a [u8]), // from Shiva with plain value, tag=37, type=string
	VsaShivaTermtype(&'a [u8]), // from Shiva with plain value, tag=38, type=string
	VsaShivaBreakKey(&'a [u8]), // from Shiva with plain value, tag=39, type=string
	VsaShivaFwdKey(&'a [u8]), // from Shiva with plain value, tag=40, type=string
	VsaShivaBakKey(&'a [u8]), // from Shiva with plain value, tag=41, type=string
	VsaShivaDialTimeout(u32), // from Shiva with plain value, tag=42, type=integer
	VsaShivaLatPort(&'a [u8]), // from Shiva with plain value, tag=43, type=string
	VsaShivaMaxVcs(u32), // from Shiva with plain value, tag=44, type=integer
	VsaShivaDhcpLeasetime(u32), // from Shiva with plain value, tag=45, type=integer
	VsaShivaLatGroups(&'a [u8]), // from Shiva with plain value, tag=46, type=string
	VsaShivaRtcTimestamp(u32), // from Shiva with plain value, tag=60, type=integer
	VsaShivaCircuitType(shiva::ShivaCircuitType), // from Shiva with Enum value, tag=61, type=integer
	VsaShivaCalledNumber(&'a [u8]), // from Shiva with plain value, tag=90, type=string
	VsaShivaCallingNumber(&'a [u8]), // from Shiva with plain value, tag=91, type=string
	VsaShivaCustomerId(&'a [u8]), // from Shiva with plain value, tag=92, type=string
	VsaShivaTypeOfService(shiva::ShivaTypeOfService), // from Shiva with Enum value, tag=93, type=integer
	VsaShivaLinkSpeed(u32), // from Shiva with plain value, tag=94, type=integer
	VsaShivaLinksInBundle(u32), // from Shiva with plain value, tag=95, type=integer
	VsaShivaCompressionType(u32), // from Shiva with plain value, tag=96, type=integer
	VsaShivaLinkProtocol(shiva::ShivaLinkProtocol), // from Shiva with Enum value, tag=97, type=integer
	VsaShivaNetworkProtocols(u32), // from Shiva with plain value, tag=98, type=integer
	VsaShivaSessionId(u32), // from Shiva with plain value, tag=99, type=integer
	VsaShivaDisconnectReason(shiva::ShivaDisconnectReason), // from Shiva with Enum value, tag=100, type=integer
	VsaShivaAcctServSwitch(Ipv4Addr), // from Shiva with plain value, tag=101, type=ipaddr
	VsaShivaEventFlags(u32), // from Shiva with plain value, tag=102, type=integer
	VsaShivaFunction(shiva::ShivaFunction), // from Shiva with Enum value, tag=103, type=integer
	VsaShivaConnectReason(shiva::ShivaConnectReason), // from Shiva with Enum value, tag=104, type=integer
	VsaSiemensUrlRedirection(&'a [u8]), // from Siemens with plain value, tag=1, type=string
	VsaSiemensApName(&'a [u8]), // from Siemens with plain value, tag=2, type=string
	VsaSiemensApSerial(&'a [u8]), // from Siemens with plain value, tag=3, type=string
	VsaSiemensVnsName(&'a [u8]), // from Siemens with plain value, tag=4, type=string
	VsaSiemensSsid(&'a [u8]), // from Siemens with plain value, tag=5, type=string
	VsaSiemensBssMac(&'a [u8]), // from Siemens with plain value, tag=6, type=string
	VsaSiemensPolicyName(&'a [u8]), // from Siemens with plain value, tag=7, type=string
	VsaSiemensTopologyName(&'a [u8]), // from Siemens with plain value, tag=8, type=string
	VsaSiemensIngressRcName(&'a [u8]), // from Siemens with plain value, tag=9, type=string
	VsaSiemensEgressRcName(&'a [u8]), // from Siemens with plain value, tag=10, type=string
	VsaSlipstreamAuth(&'a [u8]), // from Slipstream with plain value, tag=1, type=string
	VsaSs3FirewallUserPrivilege(u32), // from SonicWall with plain value, tag=1, type=integer
	VsaSonicwallUserGroup(&'a [u8]), // from SonicWall with plain value, tag=3, type=string
	VsaStAcctVcConnectionId(&'a [u8]), // from SpringTide with plain value, tag=1, type=string
	VsaStServiceName(&'a [u8]), // from SpringTide with plain value, tag=2, type=string
	VsaStServiceDomain(u32), // from SpringTide with plain value, tag=3, type=integer
	VsaStPolicyName(&'a [u8]), // from SpringTide with plain value, tag=4, type=string
	VsaStPrimaryDnsServer(Ipv4Addr), // from SpringTide with plain value, tag=5, type=ipaddr
	VsaStSecondaryDnsServer(Ipv4Addr), // from SpringTide with plain value, tag=6, type=ipaddr
	VsaStPrimaryNbnsServer(Ipv4Addr), // from SpringTide with plain value, tag=7, type=ipaddr
	VsaStSecondaryNbnsServer(Ipv4Addr), // from SpringTide with plain value, tag=8, type=ipaddr
	VsaStPhysicalPort(u32), // from SpringTide with plain value, tag=9, type=integer
	VsaStPhysicalSlot(u32), // from SpringTide with plain value, tag=10, type=integer
	VsaStVirtualPathId(u32), // from SpringTide with plain value, tag=11, type=integer
	VsaStVirtualCircuitId(u32), // from SpringTide with plain value, tag=12, type=integer
	VsaStRealmName(&'a [u8]), // from SpringTide with plain value, tag=13, type=string
	VsaStIpsecPfsGroup(u32), // from SpringTide with plain value, tag=14, type=integer
	VsaStIpsecClientFirewall(u32), // from SpringTide with plain value, tag=15, type=integer
	VsaStIpsecClientSubnet(&'a [u8]), // from SpringTide with plain value, tag=16, type=string
	VsaSn1VpnId(u32), // from Starent with plain value, tag=1, type=integer
	VsaSn1VpnName(&'a [u8]), // from Starent with plain value, tag=2, type=string
	VsaSn1DisconnectReason(starent::Sn1DisconnectReason), // from Starent with Enum value, tag=3, type=integer
	VsaSn1PppProgressCode(starent::Sn1PppProgressCode), // from Starent with Enum value, tag=4, type=integer
	VsaSn1PrimaryDnsServer(Ipv4Addr), // from Starent with plain value, tag=5, type=ipaddr
	VsaSn1SecondaryDnsServer(Ipv4Addr), // from Starent with plain value, tag=6, type=ipaddr
	VsaSn1ReChapInterval(u32), // from Starent with plain value, tag=7, type=integer
	VsaSn1IpPoolName(&'a [u8]), // from Starent with plain value, tag=8, type=string
	VsaSn1PppDataCompression(starent::Sn1PppDataCompression), // from Starent with Enum value, tag=9, type=integer
	VsaSn1IpFilterIn(&'a [u8]), // from Starent with plain value, tag=10, type=string
	VsaSn1IpFilterOut(&'a [u8]), // from Starent with plain value, tag=11, type=string
	VsaSn1LocalIpAddress(Ipv4Addr), // from Starent with plain value, tag=13, type=ipaddr
	VsaSn1IpSourceValidation(starent::Sn1IpSourceValidation), // from Starent with Enum value, tag=14, type=integer
	VsaSn1PppOutboundPassword(&'a [u8]), // from Starent with plain value, tag=15, type=string
	VsaSn1PppKeepalive(u32), // from Starent with plain value, tag=16, type=integer
	VsaSn1IpInAcl(&'a [u8]), // from Starent with plain value, tag=17, type=string
	VsaSn1IpOutAcl(&'a [u8]), // from Starent with plain value, tag=18, type=string
	VsaSn1PppDataCompressionMode(starent::Sn1PppDataCompressionMode), // from Starent with Enum value, tag=19, type=integer
	VsaSn1SubscriberPermission(starent::Sn1SubscriberPermission), // from Starent with Enum value, tag=20, type=integer
	VsaSn1AdminPermission(starent::Sn1AdminPermission), // from Starent with Enum value, tag=21, type=integer
	VsaSn1SimultaneousSipMip(starent::Sn1SimultaneousSipMip), // from Starent with Enum value, tag=22, type=integer
	VsaSn1MinCompressSize(u32), // from Starent with plain value, tag=23, type=integer
	VsaSn1ServiceType(starent::Sn1ServiceType), // from Starent with Enum value, tag=24, type=integer
	VsaSn1DnsProxyUseSubscrAddr(starent::Sn1DnsProxyUseSubscrAddr), // from Starent with Enum value, tag=25, type=integer
	VsaSn1TunnelPassword(&'a [u8]), // from Starent with plain value, tag=26, type=octets
	VsaSn1TunnelLoadBalancing(starent::Sn1TunnelLoadBalancing), // from Starent with Enum value, tag=27, type=integer
	VsaSn1MnHaTimestampTolerance(u32), // from Starent with plain value, tag=30, type=integer
	VsaSn1PrepaidCompressedCount(starent::Sn1PrepaidCompressedCount), // from Starent with Enum value, tag=31, type=integer
	VsaSn1PrepaidInboundOctets(u32), // from Starent with plain value, tag=32, type=integer
	VsaSn1PrepaidOutboundOctets(u32), // from Starent with plain value, tag=33, type=integer
	VsaSn1PrepaidTotalOctets(u32), // from Starent with plain value, tag=34, type=integer
	VsaSn1PrepaidTimeout(u32), // from Starent with plain value, tag=35, type=integer
	VsaSn1PrepaidWatermark(u32), // from Starent with plain value, tag=36, type=integer
	VsaSn1NaiConstructionDomain(&'a [u8]), // from Starent with plain value, tag=37, type=string
	VsaSn1TunnelIsakmpCryptoMap(&'a [u8]), // from Starent with plain value, tag=38, type=string
	VsaSn1TunnelIsakmpSecret(&'a [u8]), // from Starent with plain value, tag=39, type=string
	VsaSn1ExtInlineSrvrContext(&'a [u8]), // from Starent with plain value, tag=41, type=string
	VsaSn1L3ToL2TunAddrPolicy(starent::Sn1L3ToL2TunAddrPolicy), // from Starent with Enum value, tag=43, type=integer
	VsaSn1LongDurationTimeout(u32), // from Starent with plain value, tag=44, type=integer
	VsaSn1LongDurationAction(starent::Sn1LongDurationAction), // from Starent with Enum value, tag=45, type=integer
	VsaSn1Pdsn1HandoffReqIpAddr(starent::Sn1Pdsn1HandoffReqIpAddr), // from Starent with Enum value, tag=46, type=integer
	VsaSn1HaSendDnsAddress(starent::Sn1HaSendDnsAddress), // from Starent with Enum value, tag=47, type=integer
	VsaSn1MipSendTermVerification(starent::Sn1MipSendTermVerification), // from Starent with Enum value, tag=48, type=integer
	VsaSn1DataTunnelIgnoreDfBit(starent::Sn1DataTunnelIgnoreDfBit), // from Starent with Enum value, tag=49, type=integer
	VsaSn1MipAaaAssignAddr(starent::Sn1MipAaaAssignAddr), // from Starent with Enum value, tag=50, type=integer
	VsaSn1ProxyMip(starent::Sn1ProxyMip), // from Starent with Enum value, tag=52, type=integer
	VsaSn1MipMatchAaaAssignAddr(starent::Sn1MipMatchAaaAssignAddr), // from Starent with Enum value, tag=51, type=integer
	VsaSn1IpAllocMethod(starent::Sn1IpAllocMethod), // from Starent with Enum value, tag=53, type=integer
	VsaSn1GratuitousArpAggressive(starent::Sn1GratuitousArpAggressive), // from Starent with Enum value, tag=54, type=integer
	VsaSn1ExtInlineSrvrUpAddr(Ipv4Addr), // from Starent with plain value, tag=55, type=ipaddr
	VsaSn1ExtInlineSrvrDownAddr(Ipv4Addr), // from Starent with plain value, tag=56, type=ipaddr
	VsaSn1ExtInlineSrvrPreference(u32), // from Starent with plain value, tag=57, type=integer
	VsaSn1ExtInlineSrvrUpVlan(&'a [u8]), // from Starent with plain value, tag=58, type=octets
	VsaSn1ExtInlineSrvrDownVlan(&'a [u8]), // from Starent with plain value, tag=59, type=octets
	VsaSn1IpHideServiceAddress(starent::Sn1IpHideServiceAddress), // from Starent with Enum value, tag=60, type=integer
	VsaSn1PppOutboundUsername(&'a [u8]), // from Starent with plain value, tag=61, type=string
	VsaSn1GtpVersion(starent::Sn1GtpVersion), // from Starent with Enum value, tag=62, type=integer
	VsaSn1AccessLinkIpFrag(starent::Sn1AccessLinkIpFrag), // from Starent with Enum value, tag=63, type=integer
	VsaSn1SubscriberAccounting(starent::Sn1SubscriberAccounting), // from Starent with Enum value, tag=64, type=integer
	VsaSn1NwReachabilityServerName(&'a [u8]), // from Starent with plain value, tag=65, type=string
	VsaSn1SubscriberIpHdrNegMode(starent::Sn1SubscriberIpHdrNegMode), // from Starent with Enum value, tag=67, type=integer
	VsaSn1Ggsn1MipRequired(starent::Sn1Ggsn1MipRequired), // from Starent with Enum value, tag=68, type=integer
	VsaSn1SubscriberAcctStart(starent::Sn1SubscriberAcctStart), // from Starent with Enum value, tag=69, type=integer
	VsaSn1SubscriberAcctInterim(starent::Sn1SubscriberAcctInterim), // from Starent with Enum value, tag=70, type=integer
	VsaSn1SubscriberAcctStop(starent::Sn1SubscriberAcctStop), // from Starent with Enum value, tag=71, type=integer
	VsaSn1QosTpDnlk(starent::Sn1QosTpDnlk), // from Starent with Enum value, tag=73, type=integer
	VsaSn1TpDnlkCommittedDataRate(u32), // from Starent with plain value, tag=74, type=integer
	VsaSn1TpDnlkPeakDataRate(u32), // from Starent with plain value, tag=75, type=integer
	VsaSn1TpDnlkBurstSize(u32), // from Starent with plain value, tag=76, type=integer
	VsaSn1TpDnlkExceedAction(starent::Sn1TpDnlkExceedAction), // from Starent with Enum value, tag=77, type=integer
	VsaSn1TpDnlkViolateAction(starent::Sn1TpDnlkViolateAction), // from Starent with Enum value, tag=78, type=integer
	VsaSn1QosTpUplk(starent::Sn1QosTpUplk), // from Starent with Enum value, tag=79, type=integer
	VsaSn1TpUplkCommittedDataRate(u32), // from Starent with plain value, tag=80, type=integer
	VsaSn1TpUplkPeakDataRate(u32), // from Starent with plain value, tag=81, type=integer
	VsaSn1TpUplkBurstSize(u32), // from Starent with plain value, tag=82, type=integer
	VsaSn1TpUplkExceedAction(starent::Sn1TpUplkExceedAction), // from Starent with Enum value, tag=83, type=integer
	VsaSn1TpUplkViolateAction(starent::Sn1TpUplkViolateAction), // from Starent with Enum value, tag=84, type=integer
	VsaSn1SubscriberIpTosCopy(starent::Sn1SubscriberIpTosCopy), // from Starent with Enum value, tag=85, type=integer
	VsaSn1QosConversationClass(&'a [u8]), // from Starent with plain value, tag=86, type=octets
	VsaSn1QosStreamingClass(&'a [u8]), // from Starent with plain value, tag=87, type=octets
	VsaSn1QosInteractive1Class(&'a [u8]), // from Starent with plain value, tag=88, type=octets
	VsaSn1QosInteractive2Class(&'a [u8]), // from Starent with plain value, tag=89, type=octets
	VsaSn1QosInteractive3Class(&'a [u8]), // from Starent with plain value, tag=90, type=octets
	VsaSn1QosBackgroundClass(&'a [u8]), // from Starent with plain value, tag=91, type=octets
	VsaSn1PppNwLayerIpv4(starent::Sn1PppNwLayerIpv4), // from Starent with Enum value, tag=92, type=integer
	VsaSn1PppNwLayerIpv6(starent::Sn1PppNwLayerIpv6), // from Starent with Enum value, tag=93, type=integer
	VsaSn1VirtualApnName(&'a [u8]), // from Starent with plain value, tag=94, type=string
	VsaSn1PppAcceptPeerV6Ifid(starent::Sn1PppAcceptPeerV6Ifid), // from Starent with Enum value, tag=95, type=integer
	VsaSn1Ipv6RtrAdvtInterval(u32), // from Starent with plain value, tag=96, type=integer
	VsaSn1Ipv6NumRtrAdvt(u32), // from Starent with plain value, tag=97, type=integer
	VsaSn1NpuQosPriority(starent::Sn1NpuQosPriority), // from Starent with Enum value, tag=98, type=integer
	VsaSn1MnHaHashAlgorithm(starent::Sn1MnHaHashAlgorithm), // from Starent with Enum value, tag=99, type=integer
	VsaSn1SubscriberAcctRspAction(starent::Sn1SubscriberAcctRspAction), // from Starent with Enum value, tag=100, type=integer
	VsaSn1Ipv6PrimaryDns(Ipv6Addr), // from Starent with plain value, tag=101, type=ipv6addr
	VsaSn1Ipv6SecondaryDns(&'a [u8]), // from Starent with plain value, tag=102, type=octets
	VsaSn1Ipv6EgressFiltering(starent::Sn1Ipv6EgressFiltering), // from Starent with Enum value, tag=103, type=integer
	VsaSn1MediationVpnName(&'a [u8]), // from Starent with plain value, tag=104, type=string
	VsaSn1MediationAcctRspAction(starent::Sn1MediationAcctRspAction), // from Starent with Enum value, tag=105, type=integer
	VsaSn1HomeSubUseGgsn(starent::Sn1HomeSubUseGgsn), // from Starent with Enum value, tag=106, type=integer
	VsaSn1VisitingSubUseGgsn(starent::Sn1VisitingSubUseGgsn), // from Starent with Enum value, tag=107, type=integer
	VsaSn1RoamingSubUseGgsn(starent::Sn1RoamingSubUseGgsn), // from Starent with Enum value, tag=108, type=integer
	VsaSn1HomeProfile(u32), // from Starent with plain value, tag=109, type=integer
	VsaSn1IpSrcValidationDropLimit(u32), // from Starent with plain value, tag=110, type=integer
	VsaSn1QosClassConversationalPhb(starent::Sn1QosClassConversationalPhb), // from Starent with Enum value, tag=111, type=integer
	VsaSn1QosClassStreamingPhb(starent::Sn1QosClassStreamingPhb), // from Starent with Enum value, tag=112, type=integer
	VsaSn1QosClassBackgroundPhb(starent::Sn1QosClassBackgroundPhb), // from Starent with Enum value, tag=113, type=integer
	VsaSn1QosClassInteractive1Phb(starent::Sn1QosClassInteractive1Phb), // from Starent with Enum value, tag=114, type=integer
	VsaSn1QosClassInteractive2Phb(starent::Sn1QosClassInteractive2Phb), // from Starent with Enum value, tag=115, type=integer
	VsaSn1QosClassInteractive3Phb(starent::Sn1QosClassInteractive3Phb), // from Starent with Enum value, tag=116, type=integer
	VsaSn1VisitingProfile(u32), // from Starent with plain value, tag=117, type=integer
	VsaSn1RoamingProfile(u32), // from Starent with plain value, tag=118, type=integer
	VsaSn1HomeBehavior(u32), // from Starent with plain value, tag=119, type=integer
	VsaSn1VisitingBehavior(u32), // from Starent with plain value, tag=120, type=integer
	VsaSn1RoamingBehavior(u32), // from Starent with plain value, tag=121, type=integer
	VsaSn1InternalSmIndex(u32), // from Starent with plain value, tag=122, type=integer
	VsaSn1MediationEnabled(starent::Sn1MediationEnabled), // from Starent with Enum value, tag=123, type=integer
	VsaSn1Ipv6SecPool(&'a [u8]), // from Starent with plain value, tag=124, type=string
	VsaSn1Ipv6SecPrefix(&'a [u8]), // from Starent with plain value, tag=125, type=octets
	VsaSn1Ipv6DnsProxy(starent::Sn1Ipv6DnsProxy), // from Starent with Enum value, tag=126, type=integer
	VsaSn1SubscriberNexthopAddress(u32), // from Starent with plain value, tag=127, type=integer
	VsaSn1Prepaid(starent::Sn1Prepaid), // from Starent with Enum value, tag=128, type=integer
	VsaSn1PrepaidPreference(starent::Sn1PrepaidPreference), // from Starent with Enum value, tag=129, type=integer
	VsaSn1PppAlwaysOnVse(starent::Sn1PppAlwaysOnVse), // from Starent with Enum value, tag=130, type=integer
	VsaSn1VoicePushListName(&'a [u8]), // from Starent with plain value, tag=131, type=string
	VsaSn1UnclassifyListName(&'a [u8]), // from Starent with plain value, tag=132, type=string
	VsaSn1SubscriberNoInterims(starent::Sn1SubscriberNoInterims), // from Starent with Enum value, tag=133, type=integer
	VsaSn1PermitUserMcastPdus(starent::Sn1PermitUserMcastPdus), // from Starent with Enum value, tag=134, type=integer
	VsaSn1PrepaidFinalDurationAlg(starent::Sn1PrepaidFinalDurationAlg), // from Starent with Enum value, tag=135, type=integer
	VsaSn1Ipv6MinLinkMtu(u32), // from Starent with plain value, tag=136, type=integer
	VsaSn1ChargingVpnName(&'a [u8]), // from Starent with plain value, tag=137, type=string
	VsaSn1ChrgCharSelectionMode(u32), // from Starent with plain value, tag=138, type=integer
	VsaSn1CauseForRecClosing(u32), // from Starent with plain value, tag=139, type=integer
	VsaSn1ChangeCondition(starent::Sn1ChangeCondition), // from Starent with Enum value, tag=140, type=integer
	VsaSn1DynamicAddrAllocIndFlag(&'a [u8]), // from Starent with plain value, tag=141, type=octets
	VsaSn1NtkInitiatedCtxIndFlag(&'a [u8]), // from Starent with plain value, tag=142, type=octets
	VsaSn1NtkSessionDisconnectFlag(starent::Sn1NtkSessionDisconnectFlag), // from Starent with Enum value, tag=143, type=integer
	VsaSn1EnableQosRenegotiation(starent::Sn1EnableQosRenegotiation), // from Starent with Enum value, tag=144, type=integer
	VsaSn1QosRenegotiationTimeout(u32), // from Starent with plain value, tag=145, type=integer
	VsaSn1QosNegotiated(&'a [u8]), // from Starent with plain value, tag=147, type=string
	VsaSn1MediationNoInterims(starent::Sn1MediationNoInterims), // from Starent with Enum value, tag=146, type=integer
	VsaSn1PrimaryNbnsServer(Ipv4Addr), // from Starent with plain value, tag=148, type=ipaddr
	VsaSn1SecondaryNbnsServer(Ipv4Addr), // from Starent with plain value, tag=149, type=ipaddr
	VsaSn1IpHeaderCompression(starent::Sn1IpHeaderCompression), // from Starent with Enum value, tag=150, type=integer
	VsaSn1Mode(starent::Sn1Mode), // from Starent with Enum value, tag=151, type=integer
	VsaSn1AssignedVlanId(u16), // from Starent with plain value, tag=152, type=short
	VsaSn1Direction(starent::Sn1Direction), // from Starent with Enum value, tag=153, type=integer
	VsaSn1MipHaAssignmentTable(&'a [u8]), // from Starent with plain value, tag=154, type=string
	VsaSn1TunAddrPolicy(starent::Sn1TunAddrPolicy), // from Starent with Enum value, tag=156, type=integer
	VsaSn1DhcpLeaseExpiryPolicy(starent::Sn1DhcpLeaseExpiryPolicy), // from Starent with Enum value, tag=157, type=integer
	VsaSn1SubscriberTemplateName(&'a [u8]), // from Starent with plain value, tag=158, type=string
	VsaSn1SubsImsaServiceName(&'a [u8]), // from Starent with plain value, tag=159, type=string
	VsaSn1TrafficGroup(u32), // from Starent with plain value, tag=161, type=integer
	VsaSn1RadApnName(&'a [u8]), // from Starent with plain value, tag=162, type=octets
	VsaSn1MipSendAncid(starent::Sn1MipSendAncid), // from Starent with Enum value, tag=163, type=integer
	VsaSn1MipSendImsi(starent::Sn1MipSendImsi), // from Starent with Enum value, tag=164, type=integer
	VsaSn1MipDualAnchor(starent::Sn1MipDualAnchor), // from Starent with Enum value, tag=165, type=integer
	VsaSn1MipAncid(&'a [u8]), // from Starent with plain value, tag=166, type=octets
	VsaSn1ImsAmAddress(Ipv4Addr), // from Starent with plain value, tag=167, type=ipaddr
	VsaSn1ImsAmDomainName(&'a [u8]), // from Starent with plain value, tag=168, type=octets
	VsaSn1ServiceAddress(Ipv4Addr), // from Starent with plain value, tag=169, type=ipaddr
	VsaSn1PdifMipRequired(starent::Sn1PdifMipRequired), // from Starent with Enum value, tag=170, type=integer
	VsaSn1FmcLocation(&'a [u8]), // from Starent with plain value, tag=171, type=octets
	VsaSn1PdifMipReleaseTia(starent::Sn1PdifMipReleaseTia), // from Starent with Enum value, tag=172, type=integer
	VsaSn1PdifMipSimpleIpFallback(starent::Sn1PdifMipSimpleIpFallback), // from Starent with Enum value, tag=173, type=integer
	VsaSn1TunnelGn(starent::Sn1TunnelGn), // from Starent with Enum value, tag=174, type=integer
	VsaSn1MipRegLifetimeRealm(u32), // from Starent with plain value, tag=175, type=integer
	VsaSn1EcsDataVolume(&'a [u8]), // from Starent with plain value, tag=176, type=octets
	VsaSn1QosTrafficPolicy(&'a [u8]), // from Starent with plain value, tag=177, type=octets
	VsaSn1Anid(&'a [u8]), // from Starent with plain value, tag=178, type=octets
	VsaSn1PppRenegDisc(starent::Sn1PppRenegDisc), // from Starent with Enum value, tag=187, type=integer
	VsaSn1MipSendCorrelationInfo(starent::Sn1MipSendCorrelationInfo), // from Starent with Enum value, tag=188, type=integer
	VsaSn1Pdsn1CorrelationId(&'a [u8]), // from Starent with plain value, tag=189, type=octets
	VsaSn1Pdsn1NasId(&'a [u8]), // from Starent with plain value, tag=190, type=string
	VsaSn1Pdsn1NasIpAddress(Ipv4Addr), // from Starent with plain value, tag=191, type=ipaddr
	VsaSn1SubscriberAcctMode(starent::Sn1SubscriberAcctMode), // from Starent with Enum value, tag=192, type=integer
	VsaSn1IpInPlcyGrp(&'a [u8]), // from Starent with plain value, tag=193, type=string
	VsaSn1IpOutPlcyGrp(&'a [u8]), // from Starent with plain value, tag=194, type=string
	VsaSn1IpSourceViolateNoAcct(starent::Sn1IpSourceViolateNoAcct), // from Starent with Enum value, tag=196, type=integer
	VsaSn1FirewallEnabled(starent::Sn1FirewallEnabled), // from Starent with Enum value, tag=198, type=integer
	VsaSnaPppUnfrDataInOct(u32), // from Starent with plain value, tag=200, type=integer
	VsaSnaPppUnfrDataOutOct(u32), // from Starent with plain value, tag=201, type=integer
	VsaSnaPppUnfrDataInGig(u32), // from Starent with plain value, tag=202, type=integer
	VsaSnaPppUnfrDataOutGig(u32), // from Starent with plain value, tag=203, type=integer
	VsaSn1AdminExpiry(u32), // from Starent with plain value, tag=204, type=integer
	VsaSnaInputGigawords(u32), // from Starent with plain value, tag=206, type=integer
	VsaSnaOutputGigawords(u32), // from Starent with plain value, tag=207, type=integer
	VsaSn1DnsProxyInterceptList(&'a [u8]), // from Starent with plain value, tag=214, type=string
	VsaSn1SubscriberClass(starent::Sn1SubscriberClass), // from Starent with Enum value, tag=219, type=integer
	VsaSn1CfpolicyId(u32), // from Starent with plain value, tag=220, type=integer
	VsaSn1SubsVjSlotidCmpNegMode(starent::Sn1SubsVjSlotidCmpNegMode), // from Starent with Enum value, tag=221, type=integer
	VsaSn1PrimaryDccaPeer(&'a [u8]), // from Starent with plain value, tag=223, type=string
	VsaSn1SecondaryDccaPeer(&'a [u8]), // from Starent with plain value, tag=224, type=string
	VsaSn1SubsAccFlowTrafficValid(starent::Sn1SubsAccFlowTrafficValid), // from Starent with Enum value, tag=225, type=integer
	VsaSn1AcctInputPacketsDropped(u32), // from Starent with plain value, tag=226, type=integer
	VsaSn1AcctOutputPacketsDropped(u32), // from Starent with plain value, tag=227, type=integer
	VsaSn1AcctInputOctetsDropped(u64), // from Starent with plain value, tag=228, type=integer64
	VsaSn1AcctOutputOctetsDropped(u64), // from Starent with plain value, tag=229, type=integer64
	VsaSn1AcctInputGigaDropped(u32), // from Starent with plain value, tag=230, type=integer
	VsaSn1AcctOutputGigaDropped(u32), // from Starent with plain value, tag=231, type=integer
	VsaSn1OverloadDiscConnectTime(u32), // from Starent with plain value, tag=233, type=integer
	VsaSn1OverloadDisconnect(u32), // from Starent with plain value, tag=235, type=integer
	VsaSn1RadiusReturnedUsername(starent::Sn1RadiusReturnedUsername), // from Starent with Enum value, tag=236, type=integer
	VsaSn1RohcProfileName(&'a [u8]), // from Starent with plain value, tag=238, type=string
	VsaSn1FirewallPolicy(&'a [u8]), // from Starent with plain value, tag=239, type=octets
	VsaSn1TransparentData(&'a [u8]), // from Starent with plain value, tag=247, type=octets
	VsaSn1MsIsdn(&'a [u8]), // from Starent with plain value, tag=248, type=octets
	VsaSn1RoutingAreaId(&'a [u8]), // from Starent with plain value, tag=249, type=string
	VsaSn1CallId(u32), // from Starent with plain value, tag=251, type=integer
	VsaSn1Imsi(&'a [u8]), // from Starent with plain value, tag=252, type=octets
	VsaSn1LongDurationNotification(starent::Sn1LongDurationNotification), // from Starent with Enum value, tag=253, type=integer
	VsaSn1SipMethod(u32), // from Starent with plain value, tag=254, type=integer
	VsaSn1Event(&'a [u8]), // from Starent with plain value, tag=255, type=string
	VsaSurfnetAvpair(&'a [u8]), // from Surfnet with plain value, tag=1, type=string
	VsaSurfnetServiceIdentifier(&'a [u8]), // from Surfnet with plain value, tag=2, type=string
	VsaSurfnetServiceProvider(&'a [u8]), // from Surfnet with plain value, tag=3, type=string
	VsaSymbolAdminRole(symbol::SymbolAdminRole), // from Symbol with Enum value, tag=1, type=integer
	VsaSymbolCurrentEssid(&'a [u8]), // from Symbol with plain value, tag=2, type=string
	VsaSymbolAllowedEssid(&'a [u8]), // from Symbol with plain value, tag=3, type=string
	VsaSymbolWlanIndex(u32), // from Symbol with plain value, tag=4, type=integer
	VsaSymbolQosProfile(symbol::SymbolQosProfile), // from Symbol with Enum value, tag=5, type=integer
	VsaSymbolAllowedRadio(&'a [u8]), // from Symbol with plain value, tag=6, type=string
	VsaSymbolExpiryDateTime(&'a [u8]), // from Symbol with plain value, tag=7, type=string
	VsaSymbolStartDateTime(&'a [u8]), // from Symbol with plain value, tag=8, type=string
	VsaSymbolPostureStatus(&'a [u8]), // from Symbol with plain value, tag=9, type=string
	VsaSymbolDownlinkLimit(u32), // from Symbol with plain value, tag=10, type=integer
	VsaSymbolUplinkLimit(u32), // from Symbol with plain value, tag=11, type=integer
	VsaSymbolUserGroup(&'a [u8]), // from Symbol with plain value, tag=12, type=string
	VsaSymbolApName(&'a [u8]), // from Symbol with plain value, tag=17, type=string
	VsaSymbolApIpAddress(&'a [u8]), // from Symbol with plain value, tag=18, type=string
	VsaSymbolApMacAddress(&'a [u8]), // from Symbol with plain value, tag=19, type=string
	VsaSymbolVlanName(&'a [u8]), // from Symbol with plain value, tag=22, type=string
	VsaSymbolAppPolicy(&'a [u8]), // from Symbol with plain value, tag=31, type=string
	VsaSymbolApRfDomain(&'a [u8]), // from Symbol with plain value, tag=32, type=string
	VsaSymbolNsightAllowedLocation(&'a [u8]), // from Symbol with plain value, tag=33, type=string
	VsaSymbolLoginSource(symbol::SymbolLoginSource), // from Symbol with Enum value, tag=100, type=integer
	VsaTelebitLoginCommand(&'a [u8]), // from Telebit with plain value, tag=1, type=string
	VsaTelebitPortName(&'a [u8]), // from Telebit with plain value, tag=2, type=string
	VsaTelebitActivateCommand(&'a [u8]), // from Telebit with plain value, tag=3, type=string
	VsaTelebitAccountingInfo(&'a [u8]), // from Telebit with plain value, tag=4, type=string
	VsaTelebitLoginOption(&'a [u8]), // from Telebit with plain value, tag=5, type=string
	VsaEduroamSpCountry(&'a [u8]), // from TERENA with plain value, tag=10, type=string
	VsaEduroamMonitoringInflate(&'a [u8]), // from TERENA with plain value, tag=11, type=string
	VsaTrapezeVlanName(&'a [u8]), // from Trapeze with plain value, tag=1, type=string
	VsaTrapezeMobilityProfile(&'a [u8]), // from Trapeze with plain value, tag=2, type=string
	VsaTrapezeEncryptionType(&'a [u8]), // from Trapeze with plain value, tag=3, type=string
	VsaTrapezeTimeOfDay(&'a [u8]), // from Trapeze with plain value, tag=4, type=string
	VsaTrapezeSsid(&'a [u8]), // from Trapeze with plain value, tag=5, type=string
	VsaTrapezeEndDate(&'a [u8]), // from Trapeze with plain value, tag=6, type=string
	VsaTrapezeStartDate(&'a [u8]), // from Trapeze with plain value, tag=7, type=string
	VsaTrapezeUrl(&'a [u8]), // from Trapeze with plain value, tag=8, type=string
	VsaTrapezeUserGroupName(&'a [u8]), // from Trapeze with plain value, tag=9, type=string
	VsaTrapezeQosProfile(&'a [u8]), // from Trapeze with plain value, tag=10, type=string
	VsaTrapezeSimultaneousLogins(&'a [u8]), // from Trapeze with plain value, tag=11, type=string
	VsaTrapezeCoaUsername(&'a [u8]), // from Trapeze with plain value, tag=12, type=string
	VsaTrapezeAudit(&'a [u8]), // from Trapeze with plain value, tag=13, type=string
	VsaTrapezeNmsUserGroup(&'a [u8]), // from Trapeze with plain value, tag=14, type=string
	VsaTrapezeNmsPlatformLocalUser(&'a [u8]), // from Trapeze with plain value, tag=15, type=string
	VsaTrapezeSipCallDetailRecord(&'a [u8]), // from Trapeze with plain value, tag=16, type=string
	VsaTrapezeSmartpassAccessControl(&'a [u8]), // from Trapeze with plain value, tag=17, type=string
	VsaTrapezeDeviceProfile(&'a [u8]), // from Trapeze with plain value, tag=18, type=string
	VsaTrapezeDeviceType(&'a [u8]), // from Trapeze with plain value, tag=19, type=string
	VsaTrapezeAllowedDevices(&'a [u8]), // from Trapeze with plain value, tag=20, type=string
	VsaTrapezeDeviceGroup(&'a [u8]), // from Trapeze with plain value, tag=21, type=string
	VsaTpGatewayVersion(&'a [u8]), // from Travelping with plain value, tag=1, type=string
	VsaTpFirmwareVariant(&'a [u8]), // from Travelping with plain value, tag=2, type=string
	VsaTpFirmwareVersion(&'a [u8]), // from Travelping with plain value, tag=3, type=string
	VsaTpGatewayConfig(&'a [u8]), // from Travelping with plain value, tag=4, type=string
	VsaTpEncIv(&'a [u8]), // from Travelping with plain value, tag=5, type=string
	VsaTpPassword(&'a [u8]), // from Travelping with plain value, tag=6, type=string
	VsaTpUserAgent(&'a [u8]), // from Travelping with plain value, tag=7, type=string
	VsaTpAuthReply(u32), // from Travelping with plain value, tag=8, type=integer
	VsaTpAccessClassId(&'a [u8]), // from Travelping with plain value, tag=9, type=string
	VsaTpHostName(&'a [u8]), // from Travelping with plain value, tag=10, type=string
	VsaTpDhcpRequestOptionList(&'a [u8]), // from Travelping with plain value, tag=11, type=string
	VsaTpDhcpParameterRequestList(&'a [u8]), // from Travelping with plain value, tag=12, type=string
	VsaTpDhcpVendorClassId(&'a [u8]), // from Travelping with plain value, tag=13, type=string
	VsaTpDhcpClientId(&'a [u8]), // from Travelping with plain value, tag=14, type=string
	VsaTpLocationId(&'a [u8]), // from Travelping with plain value, tag=15, type=string
	VsaTpNatIpAddress(Ipv4Addr), // from Travelping with plain value, tag=16, type=ipaddr
	VsaTpZoneId(&'a [u8]), // from Travelping with plain value, tag=17, type=string
	VsaTpMonitorId(&'a [u8]), // from Travelping with plain value, tag=18, type=string
	VsaTpRelatedSessionId(&'a [u8]), // from Travelping with plain value, tag=19, type=string
	VsaTpMonitorSessionId(u32), // from Travelping with plain value, tag=20, type=integer
	VsaTpMaxInputOctets(u64), // from Travelping with plain value, tag=21, type=integer64
	VsaTpMaxOutputOctets(u64), // from Travelping with plain value, tag=22, type=integer64
	VsaTpMaxTotalOctets(u64), // from Travelping with plain value, tag=23, type=integer64
	VsaTpExitAccessClassId(&'a [u8]), // from Travelping with plain value, tag=24, type=string
	VsaTpAccessRule(&'a [u8]), // from Travelping with plain value, tag=25, type=string
	VsaTpAccessGroup(&'a [u8]), // from Travelping with plain value, tag=26, type=string
	VsaTpNatPoolId(&'a [u8]), // from Travelping with plain value, tag=27, type=string
	VsaTpNatPortStart(u32), // from Travelping with plain value, tag=28, type=integer
	VsaTpNatPortEnd(u32), // from Travelping with plain value, tag=29, type=integer
	VsaTpKeepAliveTimeout(u32), // from Travelping with plain value, tag=30, type=integer
	VsaTpTlsAuthType(travelping::TpTlsAuthType), // from Travelping with Enum value, tag=31, type=integer
	VsaTpTlsPreSharedKey(&'a [u8]), // from Travelping with plain value, tag=32, type=string
	VsaTpCapwapTimestamp(u32), // from Travelping with plain value, tag=33, type=integer
	VsaTpCapwapWtpVersion(&'a [u8]), // from Travelping with plain value, tag=34, type=string
	VsaTpCapwapSessionId(&'a [u8]), // from Travelping with plain value, tag=35, type=octets
	VsaTpCapwapRadioId(u32), // from Travelping with plain value, tag=36, type=integer
	VsaTpCapwapWwanId(u32), // from Travelping with plain value, tag=37, type=integer
	VsaTpCapwapWwanRat(u32), // from Travelping with plain value, tag=38, type=integer
	VsaTpCapwapWwanRssi(u32), // from Travelping with plain value, tag=39, type=integer
	VsaTpCapwapWwanCreg(u32), // from Travelping with plain value, tag=40, type=integer
	VsaTpCapwapWwanLac(u32), // from Travelping with plain value, tag=41, type=integer
	VsaTpCapwapWwanLatency(u32), // from Travelping with plain value, tag=42, type=integer
	VsaTpCapwapWwanMcc(u32), // from Travelping with plain value, tag=43, type=integer
	VsaTpCapwapWwanMnc(u32), // from Travelping with plain value, tag=44, type=integer
	VsaTpCapwapWwanCellId(u32), // from Travelping with plain value, tag=45, type=integer
	VsaTpCapwapPowerSaveIdleTimeout(u32), // from Travelping with plain value, tag=46, type=integer
	VsaTpCapwapPowerSaveBusyTimeout(u32), // from Travelping with plain value, tag=47, type=integer
	VsaTpCapwapSsid(&'a [u8]), // from Travelping with plain value, tag=48, type=string
	VsaTpCapwapMaxWifiClients(u32), // from Travelping with plain value, tag=49, type=integer
	VsaTpCapwapWalledGarden(&'a [u8]), // from Travelping with plain value, tag=50, type=string
	VsaTpCapwapGpsLatitude(&'a [u8]), // from Travelping with plain value, tag=51, type=string
	VsaTpCapwapGpsLongitude(&'a [u8]), // from Travelping with plain value, tag=52, type=string
	VsaTpCapwapGpsAltitude(&'a [u8]), // from Travelping with plain value, tag=53, type=string
	VsaTpCapwapGpsHdop(&'a [u8]), // from Travelping with plain value, tag=54, type=string
	VsaTpCapwapGpsTimestamp(&'a [u8]), // from Travelping with plain value, tag=55, type=string
	VsaTpCapwapHardwareVersion(&'a [u8]), // from Travelping with plain value, tag=56, type=string
	VsaTpCapwapSoftwareVersion(&'a [u8]), // from Travelping with plain value, tag=57, type=string
	VsaTpCapwapBootVersion(&'a [u8]), // from Travelping with plain value, tag=58, type=string
	VsaTpCapwapOtherSoftwareVersion(&'a [u8]), // from Travelping with plain value, tag=59, type=string
	VsaTpExcessInputOctets(u64), // from Travelping with plain value, tag=60, type=integer64
	VsaTpExcessOutputOctets(u64), // from Travelping with plain value, tag=61, type=integer64
	VsaTpExcessTotalOctets(u64), // from Travelping with plain value, tag=62, type=integer64
	VsaTpTraceId(&'a [u8]), // from Travelping with plain value, tag=63, type=string
	VsaTroposUnicastCipher(u32), // from Tropos with plain value, tag=1, type=integer
	VsaTroposLayer2InputOctets(u32), // from Tropos with plain value, tag=2, type=integer
	VsaTroposLayer2OutputOctets(u32), // from Tropos with plain value, tag=3, type=integer
	VsaTroposLayer2InputFrames(u32), // from Tropos with plain value, tag=4, type=integer
	VsaTroposLayer2OutputFrames(u32), // from Tropos with plain value, tag=5, type=integer
	VsaTroposLayer2InputDrops(u32), // from Tropos with plain value, tag=6, type=integer
	VsaTroposNoiseFloor(&'a [u8]), // from Tropos with plain value, tag=7, type=ifid
	VsaTroposNoiseUpperBound(&'a [u8]), // from Tropos with plain value, tag=8, type=ifid
	VsaTroposRelease(&'a [u8]), // from Tropos with plain value, tag=9, type=string
	VsaTroposSecondaryIp(&'a [u8]), // from Tropos with plain value, tag=11, type=octets
	VsaTroposTerminateCause(u32), // from Tropos with plain value, tag=12, type=integer
	VsaTroposAverageRssi(u32), // from Tropos with plain value, tag=13, type=integer
	VsaTroposChannel(&'a [u8]), // from Tropos with plain value, tag=15, type=ifid
	VsaTroposRetriesSent(u32), // from Tropos with plain value, tag=16, type=integer
	VsaTroposRetryBits(u32), // from Tropos with plain value, tag=17, type=integer
	VsaTroposRatesSent(&'a [u8]), // from Tropos with plain value, tag=18, type=octets
	VsaTroposRatesReceived(&'a [u8]), // from Tropos with plain value, tag=19, type=octets
	VsaTroposRoutedTime(u32), // from Tropos with plain value, tag=21, type=integer
	VsaTroposRoutlessSince(u32), // from Tropos with plain value, tag=22, type=integer
	VsaTroposCapabilityInfo(&'a [u8]), // from Tropos with plain value, tag=23, type=octets
	VsaTroposInputCap(u32), // from Tropos with plain value, tag=24, type=integer
	VsaTroposOutputCap(u32), // from Tropos with plain value, tag=25, type=integer
	VsaTroposClassMult(u32), // from Tropos with plain value, tag=26, type=integer
	VsaTroposCellName(&'a [u8]), // from Tropos with plain value, tag=27, type=string
	VsaTroposCellLocation(&'a [u8]), // from Tropos with plain value, tag=28, type=string
	VsaTroposSerialNumber(&'a [u8]), // from Tropos with plain value, tag=29, type=string
	VsaTroposLatitude(&'a [u8]), // from Tropos with plain value, tag=30, type=string
	VsaTroposLongitude(&'a [u8]), // from Tropos with plain value, tag=31, type=string
	VsaTSystemsNovaLocationId(&'a [u8]), // from T-Systems-Nova with plain value, tag=1, type=string
	VsaTSystemsNovaLocationName(&'a [u8]), // from T-Systems-Nova with plain value, tag=2, type=string
	VsaTSystemsNovaLogoffUrl(&'a [u8]), // from T-Systems-Nova with plain value, tag=3, type=string
	VsaTSystemsNovaRedirectionUrl(&'a [u8]), // from T-Systems-Nova with plain value, tag=4, type=string
	VsaTSystemsNovaBandwidthMinUp(u32), // from T-Systems-Nova with plain value, tag=5, type=integer
	VsaTSystemsNovaBandwidthMinDown(u32), // from T-Systems-Nova with plain value, tag=6, type=integer
	VsaTSystemsNovaBandwidthMaxUp(u32), // from T-Systems-Nova with plain value, tag=7, type=integer
	VsaTSystemsNovaBandwidthMaxDown(u32), // from T-Systems-Nova with plain value, tag=8, type=integer
	VsaTSystemsNovaSessionTerminateTime(u32), // from T-Systems-Nova with plain value, tag=9, type=integer
	VsaTSystemsNovaSessionTerminateEod(u32), // from T-Systems-Nova with plain value, tag=10, type=integer
	VsaTSystemsNovaBillingClassOfService(&'a [u8]), // from T-Systems-Nova with plain value, tag=11, type=string
	VsaTSystemsNovaServiceName(&'a [u8]), // from T-Systems-Nova with plain value, tag=12, type=string
	VsaTSystemsNovaPriceOfService(u32), // from T-Systems-Nova with plain value, tag=13, type=integer
	VsaTSystemsNovaVisitingProviderCode(&'a [u8]), // from T-Systems-Nova with plain value, tag=14, type=string
	VsaTSystemsNovaUnknownavp(&'a [u8]), // from T-Systems-Nova with plain value, tag=15, type=string
	VsaUkernaGssAcceptorServiceName(&'a [u8]), // from UKERNA with plain value, tag=128, type=string
	VsaUkernaGssAcceptorHostName(&'a [u8]), // from UKERNA with plain value, tag=129, type=string
	VsaUkernaGssAcceptorServiceSpecific(&'a [u8]), // from UKERNA with plain value, tag=130, type=string
	VsaUkernaGssAcceptorRealmName(&'a [u8]), // from UKERNA with plain value, tag=131, type=string
	VsaSamlAaaAssertion(&'a [u8]), // from UKERNA with plain value, tag=132, type=string
	VsaEapChannelBindingMessage(&'a [u8]), // from UKERNA with plain value, tag=135, type=octets
	VsaTrustRouterCoi(&'a [u8]), // from UKERNA with plain value, tag=136, type=string
	VsaTrustRouterApc(&'a [u8]), // from UKERNA with plain value, tag=137, type=string
	VsaMoonshotHostTargetedid(&'a [u8]), // from UKERNA with plain value, tag=138, type=string
	VsaMoonshotRealmTargetedid(&'a [u8]), // from UKERNA with plain value, tag=139, type=string
	VsaMoonshotTrCoiTargetedid(&'a [u8]), // from UKERNA with plain value, tag=140, type=string
	VsaMoonshotMstidGssAcceptor(&'a [u8]), // from UKERNA with plain value, tag=141, type=string
	VsaMoonshotMstidNamespace(&'a [u8]), // from UKERNA with plain value, tag=142, type=string
	VsaMoonshotMstidTargetedid(&'a [u8]), // from UKERNA with plain value, tag=143, type=string
	VsaMoonshotOtpSecret(&'a [u8]), // from UKERNA with plain value, tag=144, type=string
	VsaUnisphereVirtualRouter(&'a [u8]), // from Unisphere with plain value, tag=1, type=string
	VsaUnisphereLocalAddressPool(&'a [u8]), // from Unisphere with plain value, tag=2, type=string
	VsaUnisphereLocalInterface(&'a [u8]), // from Unisphere with plain value, tag=3, type=string
	VsaUnispherePrimaryDns(Ipv4Addr), // from Unisphere with plain value, tag=4, type=ipaddr
	VsaUnisphereSecondaryDns(Ipv4Addr), // from Unisphere with plain value, tag=5, type=ipaddr
	VsaUnispherePrimaryWins(Ipv4Addr), // from Unisphere with plain value, tag=6, type=ipaddr
	VsaUnisphereSecondaryWins(Ipv4Addr), // from Unisphere with plain value, tag=7, type=ipaddr
	VsaUnisphereTunnelVirtualRouter(&'a [u8]), // from Unisphere with plain value, tag=8, type=string
	VsaUnisphereTunnelPassword(&'a [u8]), // from Unisphere with plain value, tag=9, type=string
	VsaUnisphereIngressPolicyName(&'a [u8]), // from Unisphere with plain value, tag=10, type=string
	VsaUnisphereEgressPolicyName(&'a [u8]), // from Unisphere with plain value, tag=11, type=string
	VsaUnisphereIngressStatistics(unisphere::UnisphereIngressStatistics), // from Unisphere with Enum value, tag=12, type=integer
	VsaUnisphereEgressStatistics(unisphere::UnisphereEgressStatistics), // from Unisphere with Enum value, tag=13, type=integer
	VsaUnisphereServiceCategory(unisphere::UnisphereServiceCategory), // from Unisphere with Enum value, tag=14, type=integer
	VsaUnispherePcr(u32), // from Unisphere with plain value, tag=15, type=integer
	VsaUnisphereScr(u32), // from Unisphere with plain value, tag=16, type=integer
	VsaUnisphereMbs(u32), // from Unisphere with plain value, tag=17, type=integer
	VsaUnisphereInitCliAccessLevel(&'a [u8]), // from Unisphere with plain value, tag=18, type=string
	VsaUnisphereAllowAllVrAccess(unisphere::UnisphereAllowAllVrAccess), // from Unisphere with Enum value, tag=19, type=integer
	VsaUnisphereAltCliAccessLevel(&'a [u8]), // from Unisphere with plain value, tag=20, type=string
	VsaUnisphereAltCliVrouterName(&'a [u8]), // from Unisphere with plain value, tag=21, type=string
	VsaUnisphereSaValidate(unisphere::UnisphereSaValidate), // from Unisphere with Enum value, tag=22, type=integer
	VsaUnisphereIgmpEnable(unisphere::UnisphereIgmpEnable), // from Unisphere with Enum value, tag=23, type=integer
	VsaUnispherePppoeDescription(&'a [u8]), // from Unisphere with plain value, tag=24, type=string
	VsaUnisphereRedirectVrouterName(&'a [u8]), // from Unisphere with plain value, tag=25, type=string
	VsaUnisphereQosProfileName(&'a [u8]), // from Unisphere with plain value, tag=26, type=string
	VsaUnispherePppoeMaxSessions(u32), // from Unisphere with plain value, tag=27, type=integer
	VsaUnispherePppoeUrl(&'a [u8]), // from Unisphere with plain value, tag=28, type=string
	VsaUnisphereQosProfileInterfaceType(unisphere::UnisphereQosProfileInterfaceType), // from Unisphere with Enum value, tag=29, type=integer
	VsaUnisphereNasPortMethod(unisphere::UnisphereNasPortMethod), // from Unisphere with Enum value, tag=30, type=integer
	VsaUnisphereServiceBundle(&'a [u8]), // from Unisphere with plain value, tag=31, type=string
	VsaUnisphereTunnelTos(u32), // from Unisphere with plain value, tag=32, type=integer
	VsaUnisphereTunnelMaxSessions(u32), // from Unisphere with plain value, tag=33, type=integer
	VsaUnisphereFramedIpRouteTag(&'a [u8]), // from Unisphere with plain value, tag=34, type=string
	VsaUnisphereTunnelDialoutNumber(&'a [u8]), // from Unisphere with plain value, tag=35, type=string
	VsaUnispherePppUsername(&'a [u8]), // from Unisphere with plain value, tag=36, type=string
	VsaUnispherePppPassword(&'a [u8]), // from Unisphere with plain value, tag=37, type=string
	VsaUnispherePppProtocol(unisphere::UnispherePppProtocol), // from Unisphere with Enum value, tag=38, type=integer
	VsaUnisphereTunnelMinBps(u32), // from Unisphere with plain value, tag=39, type=integer
	VsaUnisphereTunnelMaxBps(u32), // from Unisphere with plain value, tag=40, type=integer
	VsaUnisphereTunnelBearerType(unisphere::UnisphereTunnelBearerType), // from Unisphere with Enum value, tag=41, type=integer
	VsaUnisphereInputGigapackets(u32), // from Unisphere with plain value, tag=42, type=integer
	VsaUnisphereOutputGigapackets(u32), // from Unisphere with plain value, tag=43, type=integer
	VsaUnisphereTunnelInterfaceId(&'a [u8]), // from Unisphere with plain value, tag=44, type=string
	VsaUnisphereIpv6VirtualRouter(&'a [u8]), // from Unisphere with plain value, tag=45, type=string
	VsaUnisphereIpv6LocalInterface(&'a [u8]), // from Unisphere with plain value, tag=46, type=string
	VsaUnisphereIpv6PrimaryDns(Ipv6Addr), // from Unisphere with plain value, tag=47, type=ipv6addr
	VsaUnisphereIpv6SecondaryDns(Ipv6Addr), // from Unisphere with plain value, tag=48, type=ipv6addr
	VsaUnisphereServiceName(&'a [u8]), // from Unisphere with plain value, tag=49, type=string
	VsaUnisphereSessionVolumeQuota(&'a [u8]), // from Unisphere with plain value, tag=50, type=string
	VsaUnisphereDisconnectCause(&'a [u8]), // from Unisphere with plain value, tag=51, type=string
	VsaUnisphereRadiusClientAddress(Ipv4Addr), // from Unisphere with plain value, tag=52, type=ipaddr
	VsaUnisphereServiceDescription(&'a [u8]), // from Unisphere with plain value, tag=53, type=string
	VsaUnisphereL2TpRecvWindowSize(u32), // from Unisphere with plain value, tag=54, type=integer
	VsaUnisphereDhcpOptions(&'a [u8]), // from Unisphere with plain value, tag=55, type=octets
	VsaUnisphereDhcpMacAddr(&'a [u8]), // from Unisphere with plain value, tag=56, type=string
	VsaUnisphereDhcpGiAddress(Ipv4Addr), // from Unisphere with plain value, tag=57, type=ipaddr
	VsaUnisphereLiAction(unisphere::UnisphereLiAction), // from Unisphere with Enum value, tag=58, type=integer
	VsaUnisphereMedDevHandle(&'a [u8]), // from Unisphere with plain value, tag=59, type=octets
	VsaUnisphereMedIpAddress(Ipv4Addr), // from Unisphere with plain value, tag=60, type=ipaddr
	VsaUnisphereMedPortNumber(u32), // from Unisphere with plain value, tag=61, type=integer
	VsaUnisphereMlpppBundleName(&'a [u8]), // from Unisphere with plain value, tag=62, type=string
	VsaUnisphereInterfaceDesc(&'a [u8]), // from Unisphere with plain value, tag=63, type=string
	VsaUnisphereTunnelGroup(&'a [u8]), // from Unisphere with plain value, tag=64, type=string
	VsaUnisphereServiceActivate(&'a [u8]), // from Unisphere with plain value, tag=65, type=string
	VsaUnisphereServiceDeactivate(&'a [u8]), // from Unisphere with plain value, tag=66, type=string
	VsaUnisphereServiceVolume(u32), // from Unisphere with plain value, tag=67, type=integer
	VsaUnisphereServiceTimeout(u32), // from Unisphere with plain value, tag=68, type=integer
	VsaUnisphereServiceStats(unisphere::UnisphereServiceStats), // from Unisphere with Enum value, tag=69, type=integer
	VsaUnisphereDfBit(unisphere::UnisphereDfBit), // from Unisphere with Enum value, tag=70, type=integer
	VsaUnisphereIgmpAccessName(&'a [u8]), // from Unisphere with plain value, tag=71, type=string
	VsaUnisphereIgmpAccessSrcName(&'a [u8]), // from Unisphere with plain value, tag=72, type=string
	VsaUnisphereIgmpOifMapName(&'a [u8]), // from Unisphere with plain value, tag=73, type=string
	VsaUnisphereMldAccessName(&'a [u8]), // from Unisphere with plain value, tag=74, type=string
	VsaUnisphereMldAccessSrcName(&'a [u8]), // from Unisphere with plain value, tag=75, type=string
	VsaUnisphereMldOifMapName(&'a [u8]), // from Unisphere with plain value, tag=76, type=string
	VsaUnisphereMldVersion(unisphere::UnisphereMldVersion), // from Unisphere with Enum value, tag=77, type=integer
	VsaUnisphereIgmpVersion(unisphere::UnisphereIgmpVersion), // from Unisphere with Enum value, tag=78, type=integer
	VsaUnisphereIpMcastAdmBwLimit(u32), // from Unisphere with plain value, tag=79, type=integer
	VsaUnisphereIpv6McastAdmBwLimit(u32), // from Unisphere with plain value, tag=80, type=integer
	VsaUnisphereL2CAccessLoopParameters(&'a [u8]), // from Unisphere with plain value, tag=81, type=string
	VsaUnisphereQosParameters(&'a [u8]), // from Unisphere with plain value, tag=82, type=string
	VsaUnisphereServiceSession(&'a [u8]), // from Unisphere with plain value, tag=83, type=string
	VsaUnisphereMobileIpAlgorithm(u32), // from Unisphere with plain value, tag=84, type=integer
	VsaUnisphereMobileIpSpi(u32), // from Unisphere with plain value, tag=85, type=integer
	VsaUnisphereMobileIpKey(&'a [u8]), // from Unisphere with plain value, tag=86, type=string
	VsaUnisphereMobileIpReplay(u32), // from Unisphere with plain value, tag=87, type=integer
	VsaUnisphereMobileIpAccessControl(&'a [u8]), // from Unisphere with plain value, tag=88, type=string
	VsaUnisphereMobileIpLifetime(u32), // from Unisphere with plain value, tag=89, type=integer
	VsaUnisphereL2TpResynchMethod(unisphere::UnisphereL2TpResynchMethod), // from Unisphere with Enum value, tag=90, type=integer
	VsaUnisphereTunnelSwitchProfile(&'a [u8]), // from Unisphere with plain value, tag=91, type=string
	VsaUnisphereL2CUpStreamData(&'a [u8]), // from Unisphere with plain value, tag=92, type=string
	VsaUnisphereL2CDownStreamData(&'a [u8]), // from Unisphere with plain value, tag=93, type=string
	VsaUnisphereTunnelTxSpeedMethod(unisphere::UnisphereTunnelTxSpeedMethod), // from Unisphere with Enum value, tag=94, type=integer
	VsaUnisphereIgmpQueryInterval(u32), // from Unisphere with plain value, tag=95, type=integer
	VsaUnisphereIgmpMaxRespTime(u32), // from Unisphere with plain value, tag=96, type=integer
	VsaUnisphereIgmpImmediateLeave(unisphere::UnisphereIgmpImmediateLeave), // from Unisphere with Enum value, tag=97, type=integer
	VsaUnisphereMldQueryInterval(u32), // from Unisphere with plain value, tag=98, type=integer
	VsaUnisphereMldMaxRespTime(u32), // from Unisphere with plain value, tag=99, type=integer
	VsaUnisphereMldImmediateLeave(unisphere::UnisphereMldImmediateLeave), // from Unisphere with Enum value, tag=100, type=integer
	VsaUnisphereIpBlockMulticast(unisphere::UnisphereIpBlockMulticast), // from Unisphere with Enum value, tag=101, type=integer
	VsaUnisphereIgmpExplicitTracking(unisphere::UnisphereIgmpExplicitTracking), // from Unisphere with Enum value, tag=102, type=integer
	VsaUnisphereIgmpNoTrackingV2Grps(unisphere::UnisphereIgmpNoTrackingV2Grps), // from Unisphere with Enum value, tag=103, type=integer
	VsaUnisphereMldExplicitTracking(unisphere::UnisphereMldExplicitTracking), // from Unisphere with Enum value, tag=104, type=integer
	VsaUnisphereMldNoTrackingV1Grps(unisphere::UnisphereMldNoTrackingV1Grps), // from Unisphere with Enum value, tag=105, type=integer
	VsaJnprIpv6IngressPolicyName(&'a [u8]), // from Unisphere with plain value, tag=106, type=string
	VsaJnprIpv6EgressPolicyName(&'a [u8]), // from Unisphere with plain value, tag=107, type=string
	VsaJnprCosParameterType(&'a [u8]), // from Unisphere with plain value, tag=108, type=string
	VsaJnprDhcpGuidedRelayServer(Ipv4Addr), // from Unisphere with plain value, tag=109, type=ipaddr
	VsaUnisphereAccLoopCirId(&'a [u8]), // from Unisphere with plain value, tag=110, type=string
	VsaUnisphereAccAggrCirIdBin(&'a [u8]), // from Unisphere with plain value, tag=111, type=octets
	VsaUnisphereAccAggrCirIdAsc(&'a [u8]), // from Unisphere with plain value, tag=112, type=string
	VsaUnisphereActDataRateUp(u32), // from Unisphere with plain value, tag=113, type=integer
	VsaUnisphereActDataRateDn(u32), // from Unisphere with plain value, tag=114, type=integer
	VsaUnisphereMinDataRateUp(u32), // from Unisphere with plain value, tag=115, type=integer
	VsaUnisphereMinDataRateDn(u32), // from Unisphere with plain value, tag=116, type=integer
	VsaUnisphereAttDataRateUp(u32), // from Unisphere with plain value, tag=117, type=integer
	VsaUnisphereAttDataRateDn(u32), // from Unisphere with plain value, tag=118, type=integer
	VsaUnisphereMaxDataRateUp(u32), // from Unisphere with plain value, tag=119, type=integer
	VsaUnisphereMaxDataRateDn(u32), // from Unisphere with plain value, tag=120, type=integer
	VsaUnisphereMinLpDataRateUp(u32), // from Unisphere with plain value, tag=121, type=integer
	VsaUnisphereMinLpDataRateDn(u32), // from Unisphere with plain value, tag=122, type=integer
	VsaUnisphereMaxInterlvDelayUp(u32), // from Unisphere with plain value, tag=123, type=integer
	VsaUnisphereActInterlvDelayUp(u32), // from Unisphere with plain value, tag=124, type=integer
	VsaUnisphereMaxInterlvDelayDn(u32), // from Unisphere with plain value, tag=125, type=integer
	VsaUnisphereActInterlvDelayDn(u32), // from Unisphere with plain value, tag=126, type=integer
	VsaUnisphereDslLineState(unisphere::UnisphereDslLineState), // from Unisphere with Enum value, tag=127, type=integer
	VsaUnisphereDslType(unisphere::UnisphereDslType), // from Unisphere with Enum value, tag=128, type=integer
	VsaUnisphereIpv6NdraPrefix(&'a [u8]), // from Unisphere with plain value, tag=129, type=ipv6prefix
	VsaUnisphereQosSetName(&'a [u8]), // from Unisphere with plain value, tag=130, type=string
	VsaUnisphereServiceAcctint(u32), // from Unisphere with plain value, tag=140, type=integer
	VsaUnisphereDownstreamCalcRate(u32), // from Unisphere with plain value, tag=141, type=integer
	VsaUnisphereUpstreamCalcRate(u32), // from Unisphere with plain value, tag=142, type=integer
	VsaJnprMaxClientsPerInterface(u32), // from Unisphere with plain value, tag=143, type=integer
	VsaUnispherePppIngressOnly(unisphere::UnispherePppIngressOnly), // from Unisphere with Enum value, tag=144, type=integer
	VsaJnprCosSchedulerPmtType(&'a [u8]), // from Unisphere with plain value, tag=146, type=string
	VsaUnisphereBackupAddressPool(&'a [u8]), // from Unisphere with plain value, tag=147, type=string
	VsaUnisphereIcrPartitionId(&'a [u8]), // from Unisphere with plain value, tag=150, type=string
	VsaUnisphereIpv6AcctInputOctets(u32), // from Unisphere with plain value, tag=151, type=integer
	VsaUnisphereIpv6AcctOutputOctets(u32), // from Unisphere with plain value, tag=152, type=integer
	VsaUnisphereIpv6AcctInputPackets(u32), // from Unisphere with plain value, tag=153, type=integer
	VsaUnisphereIpv6AcctOutputPackets(u32), // from Unisphere with plain value, tag=154, type=integer
	VsaUnisphereIpv6AcctInputGigawords(u32), // from Unisphere with plain value, tag=155, type=integer
	VsaUnisphereIpv6AcctOutputGigawords(u32), // from Unisphere with plain value, tag=156, type=integer
	VsaJnprIpv6NdraPoolName(&'a [u8]), // from Unisphere with plain value, tag=157, type=string
	VsaJnprPppoePadn(&'a [u8]), // from Unisphere with plain value, tag=158, type=string
	VsaUnisphereDhcpOption82(&'a [u8]), // from Unisphere with plain value, tag=159, type=octets
	VsaJnprVlanMapId(u32), // from Unisphere with plain value, tag=160, type=integer
	VsaJnprIpv6DelegatedPoolName(&'a [u8]), // from Unisphere with plain value, tag=161, type=string
	VsaJnprTxConnectSpeed(u32), // from Unisphere with plain value, tag=162, type=integer
	VsaJnprRxConnectSpeed(u32), // from Unisphere with plain value, tag=163, type=integer
	VsaUnisphereIpv4ReleaseControl(&'a [u8]), // from Unisphere with plain value, tag=164, type=string
	VsaPcpServerName(&'a [u8]), // from Unisphere with plain value, tag=165, type=string
	VsaUnisphereClientProfileName(&'a [u8]), // from Unisphere with plain value, tag=174, type=string
	VsaJnprRedirectGwAddress(Ipv4Addr), // from Unisphere with plain value, tag=175, type=ipaddr
	VsaJnprApnName(&'a [u8]), // from Unisphere with plain value, tag=176, type=string
	VsaUnisphereCosShapingRate(&'a [u8]), // from Unisphere with plain value, tag=177, type=string
	VsaUnisphereActionReason(&'a [u8]), // from Unisphere with plain value, tag=178, type=string
	VsaUnisphereServiceVolumeGigawords(u32), // from Unisphere with plain value, tag=179, type=integer
	VsaUnisphereUpdateService(&'a [u8]), // from Unisphere with plain value, tag=180, type=string
	VsaUnisphereAccLoopRemoteId(&'a [u8]), // from Unisphere with plain value, tag=182, type=string
	VsaUnisphereAccLoopEncap(&'a [u8]), // from Unisphere with plain value, tag=183, type=octets
	VsaUnisphereInnerVlanMapId(u32), // from Unisphere with plain value, tag=184, type=integer
	VsaUnisphereCoreFacingInterface(&'a [u8]), // from Unisphere with plain value, tag=185, type=string
	VsaUnispherePcpPortMap(&'a [u8]), // from Unisphere with plain value, tag=186, type=string
	VsaUnisphereVcpeLanExtension(&'a [u8]), // from Unisphere with plain value, tag=187, type=string
	VsaUnisphereVcpeIpv4Offload(&'a [u8]), // from Unisphere with plain value, tag=188, type=string
	VsaJnprInputInterfaceFilter(&'a [u8]), // from Unisphere with plain value, tag=191, type=string
	VsaJnprOutputInterfaceFilter(&'a [u8]), // from Unisphere with plain value, tag=192, type=string
	VsaErxBulkCoaTransactionId(u32), // from Unisphere with plain value, tag=194, type=integer
	VsaErxBulkCoaIdentifier(u32), // from Unisphere with plain value, tag=195, type=integer
	VsaUnisphereIpv4InputServiceSet(&'a [u8]), // from Unisphere with plain value, tag=196, type=string
	VsaUnisphereIpv4OutputServiceSet(&'a [u8]), // from Unisphere with plain value, tag=197, type=string
	VsaUnisphereIpv4InputServiceFilter(&'a [u8]), // from Unisphere with plain value, tag=198, type=string
	VsaUnisphereIpv4OutputServiceFilter(&'a [u8]), // from Unisphere with plain value, tag=199, type=string
	VsaUnisphereIpv6InputServiceSet(&'a [u8]), // from Unisphere with plain value, tag=200, type=string
	VsaUnisphereIpv6OutputServiceSet(&'a [u8]), // from Unisphere with plain value, tag=201, type=string
	VsaUnisphereIpv6InputServiceFilter(&'a [u8]), // from Unisphere with plain value, tag=202, type=string
	VsaUnisphereIpv6OutputServiceFilter(&'a [u8]), // from Unisphere with plain value, tag=203, type=string
	VsaUnisphereAdvPcefProfileName(&'a [u8]), // from Unisphere with plain value, tag=204, type=string
	VsaUnisphereAdvPcefRuleName(&'a [u8]), // from Unisphere with plain value, tag=205, type=string
	VsaUnixFtpUid(u32), // from Unix with plain value, tag=10, type=integer
	VsaUnixFtpGid(u32), // from Unix with plain value, tag=11, type=integer
	VsaUnixFtpHome(&'a [u8]), // from Unix with plain value, tag=12, type=string
	VsaUnixFtpShell(&'a [u8]), // from Unix with plain value, tag=13, type=string
	VsaUnixFtpGroupNames(&'a [u8]), // from Unix with plain value, tag=14, type=string
	VsaUnixFtpGroupIds(&'a [u8]), // from Unix with plain value, tag=15, type=string
	VsaUsrLastNumberDialedOut(&'a [u8]), // from USR with plain value, tag=102, type=string
	VsaUsrLastNumberDialedInDnis(&'a [u8]), // from USR with plain value, tag=232, type=string
	VsaUsrLastCallersNumberAni(&'a [u8]), // from USR with plain value, tag=233, type=string
	VsaUsrChannel(u32), // from USR with plain value, tag=48952, type=integer
	VsaUsrEventId(usr::UsrEventId), // from USR with Enum value, tag=49086, type=integer
	VsaUsrEventDateTime(u32), // from USR with plain value, tag=48943, type=date
	VsaUsrCallStartDateTime(u32), // from USR with plain value, tag=49143, type=date
	VsaUsrCallEndDateTime(u32), // from USR with plain value, tag=49142, type=date
	VsaUsrDefaultDteDataRate(usr::UsrDefaultDteDataRate), // from USR with Enum value, tag=94, type=integer
	VsaUsrInitialRxLinkDataRate(usr::UsrInitialRxLinkDataRate), // from USR with Enum value, tag=48941, type=integer
	VsaUsrFinalRxLinkDataRate(usr::UsrFinalRxLinkDataRate), // from USR with Enum value, tag=48940, type=integer
	VsaUsrInitialTxLinkDataRate(usr::UsrInitialTxLinkDataRate), // from USR with Enum value, tag=106, type=integer
	VsaUsrFinalTxLinkDataRate(usr::UsrFinalTxLinkDataRate), // from USR with Enum value, tag=107, type=integer
	VsaUsrChassisTemperature(u32), // from USR with plain value, tag=48945, type=integer
	VsaUsrChassisTempThreshold(u32), // from USR with plain value, tag=48772, type=integer
	VsaUsrActualVoltage(u32), // from USR with plain value, tag=48946, type=integer
	VsaUsrExpectedVoltage(u32), // from USR with plain value, tag=48947, type=integer
	VsaUsrPowerSupplyNumber(u32), // from USR with plain value, tag=48948, type=integer
	VsaUsrCardType(usr::UsrCardType), // from USR with Enum value, tag=48773, type=integer
	VsaUsrChassisSlot(u32), // from USR with plain value, tag=48953, type=integer
	VsaUsrSyncAsyncMode(usr::UsrSyncAsyncMode), // from USR with Enum value, tag=103, type=integer
	VsaUsrOriginateAnswerMode(usr::UsrOriginateAnswerMode), // from USR with Enum value, tag=104, type=integer
	VsaUsrModulationType(usr::UsrModulationType), // from USR with Enum value, tag=108, type=integer
	VsaUsrConnectTermReason(usr::UsrConnectTermReason), // from USR with Enum value, tag=155, type=integer
	VsaUsrFailureToConnectReason(usr::UsrFailureToConnectReason), // from USR with Enum value, tag=105, type=integer
	VsaUsrEqualizationType(usr::UsrEqualizationType), // from USR with Enum value, tag=111, type=integer
	VsaUsrFallbackEnabled(usr::UsrFallbackEnabled), // from USR with Enum value, tag=112, type=integer
	VsaUsrConnectTimeLimit(u32), // from USR with plain value, tag=49127, type=integer
	VsaUsrNumberOfRingsLimit(u32), // from USR with plain value, tag=49126, type=integer
	VsaUsrDteDataIdleTimout(u32), // from USR with plain value, tag=72, type=integer
	VsaUsrCharactersSent(u32), // from USR with plain value, tag=113, type=integer
	VsaUsrCharactersReceived(u32), // from USR with plain value, tag=114, type=integer
	VsaUsrBlocksSent(u32), // from USR with plain value, tag=117, type=integer
	VsaUsrBlocksReceived(u32), // from USR with plain value, tag=118, type=integer
	VsaUsrBlocksResent(u32), // from USR with plain value, tag=119, type=integer
	VsaUsrRetrainsRequested(u32), // from USR with plain value, tag=120, type=integer
	VsaUsrRetrainsGranted(u32), // from USR with plain value, tag=121, type=integer
	VsaUsrLineReversals(u32), // from USR with plain value, tag=122, type=integer
	VsaUsrNumberOfCharactersLost(u32), // from USR with plain value, tag=123, type=integer
	VsaUsrNumberOfBlers(u32), // from USR with plain value, tag=125, type=integer
	VsaUsrNumberOfLinkTimeouts(u32), // from USR with plain value, tag=126, type=integer
	VsaUsrNumberOfFallbacks(u32), // from USR with plain value, tag=127, type=integer
	VsaUsrNumberOfUpshifts(u32), // from USR with plain value, tag=128, type=integer
	VsaUsrNumberOfLinkNaks(u32), // from USR with plain value, tag=129, type=integer
	VsaUsrDtrFalseTimeout(u32), // from USR with plain value, tag=190, type=integer
	VsaUsrFallbackLimit(u32), // from USR with plain value, tag=191, type=integer
	VsaUsrBlockErrorCountLimit(u32), // from USR with plain value, tag=192, type=integer
	VsaUsrDtrTrueTimeout(u32), // from USR with plain value, tag=218, type=integer
	VsaUsrSecurityLoginLimit(u32), // from USR with plain value, tag=48862, type=integer
	VsaUsrSecurityRespLimit(u32), // from USR with plain value, tag=48890, type=integer
	VsaUsrDteRingNoAnswerLimit(u32), // from USR with plain value, tag=48919, type=integer
	VsaUsrBackChannelDataRate(usr::UsrBackChannelDataRate), // from USR with Enum value, tag=124, type=integer
	VsaUsrSimplifiedMnpLevels(usr::UsrSimplifiedMnpLevels), // from USR with Enum value, tag=153, type=integer
	VsaUsrSimplifiedV42BisUsage(usr::UsrSimplifiedV42BisUsage), // from USR with Enum value, tag=199, type=integer
	VsaUsrMbiCtPriCardSlot(u32), // from USR with plain value, tag=388, type=integer
	VsaUsrMbiCtTdmTimeSlot(u32), // from USR with plain value, tag=389, type=integer
	VsaUsrMbiCtPriCardSpanLine(u32), // from USR with plain value, tag=390, type=integer
	VsaUsrMbiCtBchannelUsed(u32), // from USR with plain value, tag=391, type=integer
	VsaUsrPhysicalState(u32), // from USR with plain value, tag=48759, type=integer
	VsaUsrPacketBusSession(u32), // from USR with plain value, tag=48916, type=integer
	VsaUsrServerTime(u32), // from USR with plain value, tag=61440, type=date
	VsaUsrChannelConnectedTo(u32), // from USR with plain value, tag=48733, type=integer
	VsaUsrSlotConnectedTo(u32), // from USR with plain value, tag=48734, type=integer
	VsaUsrDeviceConnectedTo(usr::UsrDeviceConnectedTo), // from USR with Enum value, tag=48735, type=integer
	VsaUsrNfasId(u32), // from USR with plain value, tag=48736, type=integer
	VsaUsrQ931CallReferenceValue(u32), // from USR with plain value, tag=48737, type=integer
	VsaUsrCallEventCode(usr::UsrCallEventCode), // from USR with Enum value, tag=48738, type=integer
	VsaUsrDs0(u32), // from USR with plain value, tag=48739, type=integer
	VsaUsrDs0S(&'a [u8]), // from USR with plain value, tag=48740, type=string
	VsaUsrGatewayIpAddress(Ipv4Addr), // from USR with plain value, tag=48742, type=ipaddr
	VsaCwVersionId(u32), // from USR with plain value, tag=32768, type=integer
	VsaCwAccountId(&'a [u8]), // from USR with plain value, tag=32769, type=string
	VsaCwAcctType(usr::CwAcctType), // from USR with Enum value, tag=32770, type=integer
	VsaCwAcctIdentificationCode(u32), // from USR with plain value, tag=32771, type=integer
	VsaCwServiceType(u32), // from USR with plain value, tag=32772, type=integer
	VsaCwRatePlanId(u32), // from USR with plain value, tag=32773, type=integer
	VsaCwSourceIdentifier(usr::CwSourceIdentifier), // from USR with Enum value, tag=32774, type=integer
	VsaCwSessionId(&'a [u8]), // from USR with plain value, tag=32775, type=string
	VsaCwNumCallAttemptSession(u32), // from USR with plain value, tag=32776, type=integer
	VsaCwSessionSequenceNum(u32), // from USR with plain value, tag=32777, type=integer
	VsaCwSessionSequenceEnd(usr::CwSessionSequenceEnd), // from USR with Enum value, tag=32778, type=integer
	VsaCwAuthenticationFailCnt(u32), // from USR with plain value, tag=32779, type=integer
	VsaCwClgPartyE164Type(usr::CwClgPartyE164Type), // from USR with Enum value, tag=32780, type=integer
	VsaCwClgPartyE164Number(&'a [u8]), // from USR with plain value, tag=32781, type=string
	VsaCwClgPartyTransProtocol(usr::CwClgPartyTransProtocol), // from USR with Enum value, tag=32782, type=integer
	VsaCwClgPartyTransPort(u32), // from USR with plain value, tag=32783, type=integer
	VsaCwClgPartyTransIp(Ipv4Addr), // from USR with plain value, tag=32784, type=ipaddr
	VsaCwClgPartyTransDns(&'a [u8]), // from USR with plain value, tag=32785, type=string
	VsaCwCldPartyE164Type(usr::CwCldPartyE164Type), // from USR with Enum value, tag=32786, type=integer
	VsaCwCldPartyE164Number(&'a [u8]), // from USR with plain value, tag=32787, type=string
	VsaCwCldPartyTransProtocol(usr::CwCldPartyTransProtocol), // from USR with Enum value, tag=32788, type=integer
	VsaCwCldPartyTransPort(u32), // from USR with plain value, tag=32789, type=integer
	VsaCwCldPartyTransIp(Ipv4Addr), // from USR with plain value, tag=32790, type=ipaddr
	VsaCwCldPartyTransDns(&'a [u8]), // from USR with plain value, tag=32791, type=string
	VsaCwOrigLineIdentifier(u32), // from USR with plain value, tag=32792, type=integer
	VsaCwPstnInterfaceNumber(u32), // from USR with plain value, tag=32793, type=integer
	VsaCwIngrGwayE164Type(usr::CwIngrGwayE164Type), // from USR with Enum value, tag=32794, type=integer
	VsaCwIngrGwayE164Number(&'a [u8]), // from USR with plain value, tag=32795, type=string
	VsaCwIngrGwayTransProtocol(usr::CwIngrGwayTransProtocol), // from USR with Enum value, tag=32796, type=integer
	VsaCwIngrGwayTransPort(u32), // from USR with plain value, tag=32797, type=integer
	VsaCwIngrGwayTransIp(Ipv4Addr), // from USR with plain value, tag=32798, type=ipaddr
	VsaCwIngrGwayTransDns(&'a [u8]), // from USR with plain value, tag=32799, type=string
	VsaCwEgrGwayTransProtocol(usr::CwEgrGwayTransProtocol), // from USR with Enum value, tag=32800, type=integer
	VsaCwEgrGwayTransPort(u32), // from USR with plain value, tag=32801, type=integer
	VsaCwEgrGwayTransIp(Ipv4Addr), // from USR with plain value, tag=32802, type=ipaddr
	VsaCwEgrGwayTransDns(&'a [u8]), // from USR with plain value, tag=32803, type=string
	VsaCwIngrGtkprTransProtocol(usr::CwIngrGtkprTransProtocol), // from USR with Enum value, tag=32804, type=integer
	VsaCwIngrGtkprTransPort(u32), // from USR with plain value, tag=32805, type=integer
	VsaCwIngrGtkprTransIp(Ipv4Addr), // from USR with plain value, tag=32806, type=ipaddr
	VsaCwIngrGtkprTransDns(&'a [u8]), // from USR with plain value, tag=32807, type=string
	VsaCwEgrGtkprTransProtocol(usr::CwEgrGtkprTransProtocol), // from USR with Enum value, tag=32808, type=integer
	VsaCwEgrGtkprTransPort(u32), // from USR with plain value, tag=32809, type=integer
	VsaCwEgrGtkprTransIp(Ipv4Addr), // from USR with plain value, tag=32810, type=ipaddr
	VsaCwEgrGtkprTransDns(&'a [u8]), // from USR with plain value, tag=32811, type=string
	VsaCwCallIdentifier(&'a [u8]), // from USR with plain value, tag=32812, type=string
	VsaCwCallType(usr::CwCallType), // from USR with Enum value, tag=32813, type=integer
	VsaCwCallStartIngrGwSec(&'a [u8]), // from USR with plain value, tag=32814, type=string
	VsaCwCallStartIngrGwMsec(u32), // from USR with plain value, tag=32815, type=integer
	VsaCwCallStartTimeAnsSec(&'a [u8]), // from USR with plain value, tag=32816, type=string
	VsaCwCallStartTimeAnsMsec(u32), // from USR with plain value, tag=32817, type=integer
	VsaCwCallEndTimeSec(&'a [u8]), // from USR with plain value, tag=32818, type=string
	VsaCwCallEndTimeMsec(u32), // from USR with plain value, tag=32819, type=integer
	VsaCwCallDurnConnectDisc(u32), // from USR with plain value, tag=32820, type=integer
	VsaCwCodecType(usr::CwCodecType), // from USR with Enum value, tag=32821, type=integer
	VsaCwCallTerminationCause(usr::CwCallTerminationCause), // from USR with Enum value, tag=32822, type=integer
	VsaCwAudioPacketsSent(u32), // from USR with plain value, tag=32823, type=integer
	VsaCwAudioPacketsReceived(u32), // from USR with plain value, tag=32824, type=integer
	VsaCwAudioPacketsLost(u32), // from USR with plain value, tag=32825, type=integer
	VsaCwAudioPacketsInFrame(u32), // from USR with plain value, tag=32826, type=integer
	VsaCwAudioBytesInFrame(u32), // from USR with plain value, tag=32827, type=integer
	VsaCwAudioSignalInPacket(u32), // from USR with plain value, tag=32828, type=integer
	VsaCwPortIdForCall(u32), // from USR with plain value, tag=32829, type=integer
	VsaCwSlotIdForCall(u32), // from USR with plain value, tag=32830, type=integer
	VsaCwAcctBalanceStartCurr(u32), // from USR with plain value, tag=32831, type=integer
	VsaCwAcctBalanceStartAmt(u32), // from USR with plain value, tag=32832, type=integer
	VsaCwAcctBalanceStartDec(u32), // from USR with plain value, tag=32833, type=integer
	VsaCwAcctBalanceDecrCurr(u32), // from USR with plain value, tag=32834, type=integer
	VsaCwLrqToken(&'a [u8]), // from USR with plain value, tag=32835, type=string
	VsaCwArqToken(&'a [u8]), // from USR with plain value, tag=32836, type=string
	VsaCwTokenStatus(u32), // from USR with plain value, tag=32837, type=integer
	VsaCwSs7DestnPtcodeType(u32), // from USR with plain value, tag=32838, type=integer
	VsaCwSs7DestnPtcodeAddress(u32), // from USR with plain value, tag=32839, type=integer
	VsaCwSs7OrigPtcodeType(u32), // from USR with plain value, tag=32840, type=integer
	VsaCwSs7OrigPtcodeAddress(u32), // from USR with plain value, tag=32841, type=integer
	VsaCwSs7Cic(u32), // from USR with plain value, tag=32842, type=integer
	VsaCwMgcId(u32), // from USR with plain value, tag=32843, type=integer
	VsaCwMgId(u32), // from USR with plain value, tag=32844, type=integer
	VsaCwSignalingProtocol(usr::CwSignalingProtocol), // from USR with Enum value, tag=32845, type=integer
	VsaCwProtocolTransport(usr::CwProtocolTransport), // from USR with Enum value, tag=32846, type=integer
	VsaCwLocalSigTransProtocol(usr::CwLocalSigTransProtocol), // from USR with Enum value, tag=32847, type=integer
	VsaCwLocalSigTransPort(u32), // from USR with plain value, tag=32848, type=integer
	VsaCwLocalSigTransIp(Ipv4Addr), // from USR with plain value, tag=32849, type=ipaddr
	VsaCwLocalSigTransDns(&'a [u8]), // from USR with plain value, tag=32850, type=string
	VsaCwRemoteSigTransProtocol(usr::CwRemoteSigTransProtocol), // from USR with Enum value, tag=32851, type=integer
	VsaCwRemoteSigTransPort(u32), // from USR with plain value, tag=32852, type=integer
	VsaCwRemoteSigTransIp(Ipv4Addr), // from USR with plain value, tag=32853, type=ipaddr
	VsaCwRemoteSigTransDns(&'a [u8]), // from USR with plain value, tag=32854, type=string
	VsaCwLocalMgRtpProtocol(usr::CwLocalMgRtpProtocol), // from USR with Enum value, tag=32855, type=integer
	VsaCwLocalMgRtpPort(u32), // from USR with plain value, tag=32856, type=integer
	VsaCwLocalMgRtpIp(Ipv4Addr), // from USR with plain value, tag=32857, type=ipaddr
	VsaCwLocalMgRtpDns(&'a [u8]), // from USR with plain value, tag=32858, type=string
	VsaCwRemoteMgRtpProtocol(usr::CwRemoteMgRtpProtocol), // from USR with Enum value, tag=32859, type=integer
	VsaCwRemoteMgRtpPort(u32), // from USR with plain value, tag=32860, type=integer
	VsaCwRemoteMgRtpIp(Ipv4Addr), // from USR with plain value, tag=32861, type=ipaddr
	VsaCwRemoteMgRtpDns(&'a [u8]), // from USR with plain value, tag=32862, type=string
	VsaCwCallModel(u32), // from USR with plain value, tag=32863, type=integer
	VsaCwCallPlanId(u32), // from USR with plain value, tag=32864, type=integer
	VsaCwTransCldPartyE164Type(usr::CwTransCldPartyE164Type), // from USR with Enum value, tag=32865, type=integer
	VsaCwTransCldPartyE164Num(&'a [u8]), // from USR with plain value, tag=32866, type=string
	VsaCwOspSourceDevice(&'a [u8]), // from USR with plain value, tag=32867, type=string
	VsaUsrPwUsrIfilterIp(&'a [u8]), // from USR with plain value, tag=36864, type=string
	VsaUsrPwUsrIfilterIpx(&'a [u8]), // from USR with plain value, tag=36865, type=string
	VsaUsrPwUsrOfilterIp(&'a [u8]), // from USR with plain value, tag=36867, type=string
	VsaUsrPwUsrOfilterIpx(&'a [u8]), // from USR with plain value, tag=36868, type=string
	VsaUsrPwUsrOfilterSap(&'a [u8]), // from USR with plain value, tag=36869, type=string
	VsaUsrPwVpnId(&'a [u8]), // from USR with plain value, tag=36870, type=string
	VsaUsrPwVpnName(&'a [u8]), // from USR with plain value, tag=36871, type=string
	VsaUsrPwVpnNeighbor(Ipv4Addr), // from USR with plain value, tag=36872, type=ipaddr
	VsaUsrPwFramedRoutingV2(&'a [u8]), // from USR with plain value, tag=36873, type=string
	VsaUsrPwVpnGateway(&'a [u8]), // from USR with plain value, tag=36874, type=string
	VsaUsrPwTunnelAuthentication(&'a [u8]), // from USR with plain value, tag=36875, type=string
	VsaUsrPwIndex(&'a [u8]), // from USR with plain value, tag=36876, type=string
	VsaUsrPwCutoff(&'a [u8]), // from USR with plain value, tag=36877, type=string
	VsaUsrPwPacket(&'a [u8]), // from USR with plain value, tag=36878, type=string
	VsaUsrPrimaryDnsServer(Ipv4Addr), // from USR with plain value, tag=36879, type=ipaddr
	VsaUsrSecondaryDnsServer(Ipv4Addr), // from USR with plain value, tag=36880, type=ipaddr
	VsaUsrPrimaryNbnsServer(Ipv4Addr), // from USR with plain value, tag=36881, type=ipaddr
	VsaUsrSecondaryNbnsServer(Ipv4Addr), // from USR with plain value, tag=36882, type=ipaddr
	VsaUsrSyslogTap(usr::UsrSyslogTap), // from USR with Enum value, tag=36883, type=integer
	VsaUsrChassisCallSlot(u32), // from USR with plain value, tag=36889, type=integer
	VsaUsrChassisCallSpan(u32), // from USR with plain value, tag=36890, type=integer
	VsaUsrChassisCallChannel(u32), // from USR with plain value, tag=36891, type=integer
	VsaUsrKeypressTimeout(u32), // from USR with plain value, tag=36892, type=integer
	VsaUsrUnauthenticatedTime(u32), // from USR with plain value, tag=36893, type=integer
	VsaUsrConnectSpeed(usr::UsrConnectSpeed), // from USR with Enum value, tag=36899, type=integer
	VsaUsrFramedIpAddressPoolName(&'a [u8]), // from USR with plain value, tag=36900, type=string
	VsaUsrMpEdo(&'a [u8]), // from USR with plain value, tag=36901, type=string
	VsaUsrBearerCapabilities(u32), // from USR with plain value, tag=38912, type=integer
	VsaUsrSpeedOfConnection(usr::UsrSpeedOfConnection), // from USR with Enum value, tag=38913, type=integer
	VsaUsrMaxChannels(u32), // from USR with plain value, tag=38914, type=integer
	VsaUsrChannelExpansion(u32), // from USR with plain value, tag=38915, type=integer
	VsaUsrChannelDecrement(u32), // from USR with plain value, tag=38916, type=integer
	VsaUsrExpansionAlgorithm(usr::UsrExpansionAlgorithm), // from USR with Enum value, tag=38917, type=integer
	VsaUsrCompressionAlgorithm(usr::UsrCompressionAlgorithm), // from USR with Enum value, tag=38918, type=integer
	VsaUsrReceiveAccMap(u32), // from USR with plain value, tag=38919, type=integer
	VsaUsrTransmitAccMap(u32), // from USR with plain value, tag=38920, type=integer
	VsaUsrCompressionResetMode(usr::UsrCompressionResetMode), // from USR with Enum value, tag=38922, type=integer
	VsaUsrMinCompressionSize(u32), // from USR with plain value, tag=38923, type=integer
	VsaUsrIp(u32), // from USR with plain value, tag=38924, type=integer
	VsaUsrIpx(u32), // from USR with plain value, tag=38925, type=integer
	VsaUsrFilterZones(usr::UsrFilterZones), // from USR with Enum value, tag=38926, type=integer
	VsaUsrAppletalk(usr::UsrAppletalk), // from USR with Enum value, tag=38927, type=integer
	VsaUsrBridging(usr::UsrBridging), // from USR with Enum value, tag=38928, type=integer
	VsaUsrSpoofing(usr::UsrSpoofing), // from USR with Enum value, tag=38929, type=integer
	VsaUsrHostType(u32), // from USR with plain value, tag=38930, type=integer
	VsaUsrSendName(&'a [u8]), // from USR with plain value, tag=38931, type=string
	VsaUsrSendPassword(&'a [u8]), // from USR with plain value, tag=38932, type=string
	VsaUsrStartTime(u32), // from USR with plain value, tag=38933, type=integer
	VsaUsrEndTime(u32), // from USR with plain value, tag=38934, type=integer
	VsaUsrSendScript1(&'a [u8]), // from USR with plain value, tag=38935, type=string
	VsaUsrReplyScript1(&'a [u8]), // from USR with plain value, tag=38936, type=string
	VsaUsrSendScript2(&'a [u8]), // from USR with plain value, tag=38937, type=string
	VsaUsrReplyScript2(&'a [u8]), // from USR with plain value, tag=38938, type=string
	VsaUsrSendScript3(&'a [u8]), // from USR with plain value, tag=38939, type=string
	VsaUsrReplyScript3(&'a [u8]), // from USR with plain value, tag=38940, type=string
	VsaUsrSendScript4(&'a [u8]), // from USR with plain value, tag=38941, type=string
	VsaUsrReplyScript4(&'a [u8]), // from USR with plain value, tag=38942, type=string
	VsaUsrSendScript5(&'a [u8]), // from USR with plain value, tag=38943, type=string
	VsaUsrReplyScript5(&'a [u8]), // from USR with plain value, tag=38944, type=string
	VsaUsrSendScript6(&'a [u8]), // from USR with plain value, tag=38945, type=string
	VsaUsrReplyScript6(&'a [u8]), // from USR with plain value, tag=38946, type=string
	VsaUsrTerminalType(&'a [u8]), // from USR with plain value, tag=38947, type=string
	VsaUsrAppletalkNetworkRange(u32), // from USR with plain value, tag=38948, type=integer
	VsaUsrLocalIpAddress(&'a [u8]), // from USR with plain value, tag=38949, type=string
	VsaUsrRoutingProtocol(usr::UsrRoutingProtocol), // from USR with Enum value, tag=38950, type=integer
	VsaUsrModemGroup(u32), // from USR with plain value, tag=38951, type=integer
	VsaUsrModemTrainingTime(u32), // from USR with plain value, tag=38978, type=integer
	VsaUsrInterfaceIndex(u32), // from USR with plain value, tag=38979, type=integer
	VsaUsrMulticastProxy(u32), // from USR with plain value, tag=38989, type=integer
	VsaUsrMulticastForwarding(u32), // from USR with plain value, tag=38992, type=integer
	VsaUsrMpMrru(u32), // from USR with plain value, tag=38959, type=integer
	VsaUsrSapFilterIn(&'a [u8]), // from USR with plain value, tag=36866, type=string
	VsaUsrMic(&'a [u8]), // from USR with plain value, tag=36884, type=string
	VsaUsrLogFilterPackets(&'a [u8]), // from USR with plain value, tag=36887, type=string
	VsaUsrVpnEncrypter(u32), // from USR with plain value, tag=36894, type=integer
	VsaUsrReChapTimeout(u32), // from USR with plain value, tag=36896, type=integer
	VsaUsrTunnelSwitchEndpoint(&'a [u8]), // from USR with plain value, tag=39016, type=string
	VsaUsrIpSaaFilter(u32), // from USR with plain value, tag=39024, type=integer
	VsaInitialModulationType(usr::InitialModulationType), // from USR with Enum value, tag=2339, type=integer
	VsaUsrVtsSessionKey(&'a [u8]), // from USR with plain value, tag=38998, type=string
	VsaUsrOrigNasType(&'a [u8]), // from USR with plain value, tag=38999, type=string
	VsaUsrCallArrivalTime(u32), // from USR with plain value, tag=39000, type=integer
	VsaUsrCallEndTime(u32), // from USR with plain value, tag=39001, type=integer
	VsaUsrTunnelAuthHostname(&'a [u8]), // from USR with plain value, tag=39019, type=string
	VsaUsrAcctReasonCode(u32), // from USR with plain value, tag=39020, type=integer
	VsaUsrSupportsTags(u32), // from USR with plain value, tag=39049, type=integer
	VsaUsrHarcDisconnectCode(usr::UsrHarcDisconnectCode), // from USR with Enum value, tag=39051, type=integer
	VsaUsrRmmieStatus(usr::UsrRmmieStatus), // from USR with Enum value, tag=461, type=integer
	VsaUsrRmmieLastUpdateEvent(usr::UsrRmmieLastUpdateEvent), // from USR with Enum value, tag=2305, type=integer
	VsaUsrRmmieX2Status(usr::UsrRmmieX2Status), // from USR with Enum value, tag=2313, type=integer
	VsaUsrRmmiePlannedDisconnect(usr::UsrRmmiePlannedDisconnect), // from USR with Enum value, tag=2314, type=integer
	VsaUsrVpnGwLocationId(&'a [u8]), // from USR with plain value, tag=36895, type=string
	VsaUsrCcpAlgorithm(usr::UsrCcpAlgorithm), // from USR with Enum value, tag=36897, type=integer
	VsaUsrAccmType(u32), // from USR with plain value, tag=36898, type=integer
	VsaUsrLocalFramedIpAddr(Ipv4Addr), // from USR with plain value, tag=36902, type=ipaddr
	VsaUsrIpxRouting(usr::UsrIpxRouting), // from USR with Enum value, tag=38952, type=integer
	VsaUsrIpxWan(usr::UsrIpxWan), // from USR with Enum value, tag=38953, type=integer
	VsaUsrIpRipPolicies(usr::UsrIpRipPolicies), // from USR with Enum value, tag=38954, type=integer
	VsaUsrIpRipSimpleAuthPassword(&'a [u8]), // from USR with plain value, tag=38955, type=string
	VsaUsrIpRipInputFilter(&'a [u8]), // from USR with plain value, tag=38956, type=string
	VsaUsrIpCallInputFilter(&'a [u8]), // from USR with plain value, tag=38957, type=string
	VsaUsrIpxRipInputFilter(&'a [u8]), // from USR with plain value, tag=38958, type=string
	VsaUsrIpxCallInputFilter(&'a [u8]), // from USR with plain value, tag=38960, type=string
	VsaUsrAtInputFilter(&'a [u8]), // from USR with plain value, tag=38961, type=string
	VsaUsrAtRtmpInputFilter(&'a [u8]), // from USR with plain value, tag=38962, type=string
	VsaUsrAtZipInputFilter(&'a [u8]), // from USR with plain value, tag=38963, type=string
	VsaUsrAtCallInputFilter(&'a [u8]), // from USR with plain value, tag=38964, type=string
	VsaUsrEtBridgeInputFilter(&'a [u8]), // from USR with plain value, tag=38965, type=string
	VsaUsrIpRipOutputFilter(&'a [u8]), // from USR with plain value, tag=38966, type=string
	VsaUsrIpCallOutputFilter(&'a [u8]), // from USR with plain value, tag=38967, type=string
	VsaUsrIpxRipOutputFilter(&'a [u8]), // from USR with plain value, tag=38968, type=string
	VsaUsrIpxCallOutputFilter(&'a [u8]), // from USR with plain value, tag=38969, type=string
	VsaUsrAtOutputFilter(&'a [u8]), // from USR with plain value, tag=38970, type=string
	VsaUsrAtRtmpOutputFilter(&'a [u8]), // from USR with plain value, tag=38971, type=string
	VsaUsrAtZipOutputFilter(&'a [u8]), // from USR with plain value, tag=38972, type=string
	VsaUsrAtCallOutputFilter(&'a [u8]), // from USR with plain value, tag=38973, type=string
	VsaUsrEtBridgeOutputFilter(&'a [u8]), // from USR with plain value, tag=38974, type=string
	VsaUsrEtBridgeCallOutputFilte(&'a [u8]), // from USR with plain value, tag=38975, type=string
	VsaUsrIpDefaultRouteOption(usr::UsrIpDefaultRouteOption), // from USR with Enum value, tag=38976, type=integer
	VsaUsrMpEdoHiper(&'a [u8]), // from USR with plain value, tag=38977, type=string
	VsaUsrTunnelSecurity(usr::UsrTunnelSecurity), // from USR with Enum value, tag=38980, type=integer
	VsaUsrPortTap(u32), // from USR with plain value, tag=38981, type=integer
	VsaUsrPortTapFormat(u32), // from USR with plain value, tag=38982, type=integer
	VsaUsrPortTapOutput(u32), // from USR with plain value, tag=38983, type=integer
	VsaUsrPortTapFacility(u32), // from USR with plain value, tag=38984, type=integer
	VsaUsrPortTapPriority(u32), // from USR with plain value, tag=38985, type=integer
	VsaUsrPortTapAddress(Ipv4Addr), // from USR with plain value, tag=38986, type=ipaddr
	VsaUsrMobileipHomeAgentAddress(Ipv4Addr), // from USR with plain value, tag=38987, type=ipaddr
	VsaUsrTunneledMlpp(u32), // from USR with plain value, tag=38988, type=integer
	VsaUsrMulticastReceive(u32), // from USR with plain value, tag=38990, type=integer
	VsaUsrIgmpQueryInterval(u32), // from USR with plain value, tag=38993, type=integer
	VsaUsrIgmpMaximumResponseTime(u32), // from USR with plain value, tag=38994, type=integer
	VsaUsrIgmpRobustness(u32), // from USR with plain value, tag=38995, type=integer
	VsaUsrIgmpVersion(u32), // from USR with plain value, tag=38996, type=integer
	VsaUsrCallbackType(usr::UsrCallbackType), // from USR with Enum value, tag=39018, type=integer
	VsaUsrRequestType(usr::UsrRequestType), // from USR with Enum value, tag=61441, type=integer
	VsaUsrRmmieNumOfUpdates(u32), // from USR with plain value, tag=462, type=integer
	VsaUsrRmmieManufacturerId(u32), // from USR with plain value, tag=479, type=integer
	VsaUsrRmmieProductCode(&'a [u8]), // from USR with plain value, tag=480, type=string
	VsaUsrRmmieSerialNumber(&'a [u8]), // from USR with plain value, tag=481, type=string
	VsaUsrRmmieFirmwareVersion(&'a [u8]), // from USR with plain value, tag=482, type=string
	VsaUsrRmmieFirmwareBuildDate(&'a [u8]), // from USR with plain value, tag=483, type=string
	VsaUsrCallArrivalInGmt(u32), // from USR with plain value, tag=48722, type=date
	VsaUsrCallConnectInGmt(u32), // from USR with plain value, tag=48721, type=date
	VsaUsrCallTerminateInGmt(u32), // from USR with plain value, tag=48720, type=date
	VsaUsrIds0CallType(u32), // from USR with plain value, tag=48719, type=integer
	VsaUsrCallReferenceNumber(u32), // from USR with plain value, tag=48765, type=integer
	VsaUsrCdmaCallReferenceNumber(u32), // from USR with plain value, tag=387, type=integer
	VsaUsrMobileIpAddress(Ipv4Addr), // from USR with plain value, tag=2190, type=ipaddr
	VsaUsrQnc1ServiceDestination(Ipv4Addr), // from USR with plain value, tag=2292, type=ipaddr
	VsaUsrIwfIpAddress(Ipv4Addr), // from USR with plain value, tag=1012, type=ipaddr
	VsaUsrCalledPartyNumber(&'a [u8]), // from USR with plain value, tag=2192, type=string
	VsaUsrCallingPartyNumber(&'a [u8]), // from USR with plain value, tag=2191, type=string
	VsaUsrCallType(u32), // from USR with plain value, tag=2193, type=integer
	VsaUsrEsn(&'a [u8]), // from USR with plain value, tag=2194, type=string
	VsaUsrIwfCallIdentifier(u32), // from USR with plain value, tag=2195, type=integer
	VsaUsrImsi(&'a [u8]), // from USR with plain value, tag=2196, type=string
	VsaUsrServiceOption(u32), // from USR with plain value, tag=2197, type=integer
	VsaUsrDisconnectCauseIndicator(u32), // from USR with plain value, tag=2198, type=integer
	VsaUsrMobileNumbytesTxed(u32), // from USR with plain value, tag=2199, type=integer
	VsaUsrMobileNumbytesRxed(u32), // from USR with plain value, tag=2200, type=integer
	VsaUsrNumFaxPagesProcessed(u32), // from USR with plain value, tag=2201, type=integer
	VsaUsrCompressionType(u32), // from USR with plain value, tag=2202, type=integer
	VsaUsrCallErrorCode(u32), // from USR with plain value, tag=2203, type=integer
	VsaUsrModemSetupTime(u32), // from USR with plain value, tag=2204, type=integer
	VsaUsrCallConnectingTime(u32), // from USR with plain value, tag=2205, type=integer
	VsaUsrConnectTime(u32), // from USR with plain value, tag=2206, type=integer
	VsaUsrRmmieLastUpdateTime(u32), // from USR with plain value, tag=2304, type=integer
	VsaUsrRmmieRcvTotPwrlvl(u32), // from USR with plain value, tag=2306, type=integer
	VsaUsrRmmieRcvPwrlvl3300Hz(u32), // from USR with plain value, tag=2307, type=integer
	VsaUsrRmmieRcvPwrlvl3750Hz(u32), // from USR with plain value, tag=2308, type=integer
	VsaUsrRmmiePwrlvlNearechoCanc(u32), // from USR with plain value, tag=2309, type=integer
	VsaUsrRmmiePwrlvlFarechoCanc(u32), // from USR with plain value, tag=2310, type=integer
	VsaUsrRmmiePwrlvlNoiseLvl(u32), // from USR with plain value, tag=2311, type=integer
	VsaUsrRmmiePwrlvlXmitLvl(u32), // from USR with plain value, tag=2312, type=integer
	VsaUsrFramedIpxRoute(Ipv4Addr), // from USR with plain value, tag=36903, type=ipaddr
	VsaUsrMpipTunnelOriginator(Ipv4Addr), // from USR with plain value, tag=36904, type=ipaddr
	VsaUsrIgmpRouting(u32), // from USR with plain value, tag=38997, type=integer
	VsaUsrRadMulticastRoutingTtl(u32), // from USR with plain value, tag=39008, type=integer
	VsaUsrRadMulticastRoutingRtlim(u32), // from USR with plain value, tag=39009, type=integer
	VsaUsrRadMulticastRoutingProto(u32), // from USR with plain value, tag=39010, type=integer
	VsaUsrRadMulticastRoutingBound(&'a [u8]), // from USR with plain value, tag=39011, type=string
	VsaUsrRadDvmrpMetric(u32), // from USR with plain value, tag=39012, type=integer
	VsaUsrChatScriptName(&'a [u8]), // from USR with plain value, tag=39013, type=string
	VsaUsrCusrHatScriptRules(&'a [u8]), // from USR with plain value, tag=39014, type=string
	VsaUsrRadLocationType(u32), // from USR with plain value, tag=39015, type=integer
	VsaUsrOspfAddresslessIndex(u32), // from USR with plain value, tag=39017, type=integer
	VsaUsrQosQueuingMehtod(u32), // from USR with plain value, tag=39021, type=integer
	VsaUsrPqDefaultPriority(u32), // from USR with plain value, tag=39022, type=integer
	VsaUsrFqDefaultPriority(u32), // from USR with plain value, tag=39025, type=integer
	VsaUsrIppEnable(u32), // from USR with plain value, tag=39026, type=integer
	VsaUsrPreSharedMnKey(&'a [u8]), // from USR with plain value, tag=39027, type=string
	VsaUsrMipNai(u32), // from USR with plain value, tag=39028, type=integer
	VsaUsrDnisReauthentication(u32), // from USR with plain value, tag=39029, type=integer
	VsaUsrAgent(usr::UsrAgent), // from USR with Enum value, tag=39030, type=integer
	VsaUsrPqParameters(u32), // from USR with plain value, tag=39031, type=integer
	VsaUsrDvmrpPruneLifetime(u32), // from USR with plain value, tag=39032, type=integer
	VsaUsrSpecialXonXoffFlow(u32), // from USR with plain value, tag=39033, type=integer
	VsaUsrDvmrpAdvertisedMetric(u32), // from USR with plain value, tag=39034, type=integer
	VsaUsrDvmrpRetransmitPrunes(u32), // from USR with plain value, tag=39035, type=integer
	VsaUsrDvmrpNonPruners(u32), // from USR with plain value, tag=39036, type=integer
	VsaUsrDvmrpRouteTransit(u32), // from USR with plain value, tag=39037, type=integer
	VsaUsrDvmrpInputFilter(&'a [u8]), // from USR with plain value, tag=39038, type=string
	VsaUsrDvmrpOutputFilter(&'a [u8]), // from USR with plain value, tag=39040, type=string
	VsaUsrPolicyAccess(u32), // from USR with plain value, tag=39041, type=integer
	VsaUsrPolicyConfiguration(u32), // from USR with plain value, tag=39042, type=integer
	VsaUsrPolicyFilename(&'a [u8]), // from USR with plain value, tag=39043, type=string
	VsaUsrPolicyType(u32), // from USR with plain value, tag=39044, type=integer
	VsaUsrMobileSessionId(u32), // from USR with plain value, tag=39045, type=integer
	VsaUsrMobileAccountingType(u32), // from USR with plain value, tag=39046, type=integer
	VsaUsrMobileServiceOption(u32), // from USR with plain value, tag=39047, type=integer
	VsaUsrWallclockTimestamp(u32), // from USR with plain value, tag=39048, type=integer
	VsaUsrDvmrpInitialFlooding(u32), // from USR with plain value, tag=39050, type=integer
	VsaUsrTelnetOptions(u32), // from USR with plain value, tag=39052, type=integer
	VsaUsrCdmaPktdataNetworkId(u32), // from USR with plain value, tag=39053, type=integer
	VsaUsrAuthNextServerAddress(Ipv4Addr), // from USR with plain value, tag=39054, type=ipaddr
	VsaUsrUserPppAodiType(u32), // from USR with plain value, tag=39055, type=integer
	VsaUsrMlpppFragmentationThreshld(u32), // from USR with plain value, tag=39056, type=integer
	VsaUsrUnnumberedLocalIpAddress(Ipv4Addr), // from USR with plain value, tag=39057, type=ipaddr
	VsaUsrTrafficThreshold(u32), // from USR with plain value, tag=39058, type=integer
	VsaUsrKeepAliveInterval(u32), // from USR with plain value, tag=39059, type=integer
	VsaVirtualServerId(u32), // from USR with plain value, tag=39060, type=integer
	VsaUsrX25TrunkProfile(&'a [u8]), // from USR with plain value, tag=39061, type=string
	VsaUsrX25AcctInputSegmentCount(u32), // from USR with plain value, tag=39062, type=integer
	VsaUsrX25AcctOutputSegmentCoun(u32), // from USR with plain value, tag=39063, type=integer
	VsaUsrX25AcctSegmentSize(u32), // from USR with plain value, tag=39064, type=integer
	VsaUsrX25AcctTerminationCode(u32), // from USR with plain value, tag=39065, type=integer
	VsaUsrX25SvcLogicalChannelNumb(u32), // from USR with plain value, tag=39066, type=integer
	VsaUsrNailedBChannelIndicator(u32), // from USR with plain value, tag=39067, type=integer
	VsaUsrX25SvcCallAttributes(u32), // from USR with plain value, tag=39068, type=integer
	VsaUsrInitRegServerAddr(Ipv4Addr), // from USR with plain value, tag=39069, type=ipaddr
	VsaUsrReRegServerAddr(Ipv4Addr), // from USR with plain value, tag=39070, type=ipaddr
	VsaUsrBytesTxRemain(u32), // from USR with plain value, tag=39071, type=integer
	VsaUsrBytesRxRemain(u32), // from USR with plain value, tag=39072, type=integer
	VsaUsrSessionTimeRemain(u32), // from USR with plain value, tag=39073, type=integer
	VsaUsrPrePaidEnabled(usr::UsrPrePaidEnabled), // from USR with Enum value, tag=39074, type=integer
	VsaUsrRegServerProvTimeout(u32), // from USR with plain value, tag=39075, type=integer
	VsaUsrRedirect(u32), // from USR with plain value, tag=39076, type=integer
	VsaUsrVlanTag(u32), // from USR with plain value, tag=39077, type=integer
	VsaUsrRadIpPoolDefinition(&'a [u8]), // from USR with plain value, tag=39078, type=string
	VsaUsrRadNmcCallProgressStatus(u32), // from USR with plain value, tag=39079, type=integer
	VsaUsrRadNmcBlocksRx(u32), // from USR with plain value, tag=39080, type=integer
	VsaUsrTotalBytesRemain(u32), // from USR with plain value, tag=39096, type=integer
	VsaUsrForwardRateLimit(u32), // from USR with plain value, tag=39097, type=integer
	VsaUsrReverseRateLimit(u32), // from USR with plain value, tag=39100, type=integer
	VsaUsrNasType(usr::UsrNasType), // from USR with Enum value, tag=61442, type=integer
	VsaUsrAuthMode(usr::UsrAuthMode), // from USR with Enum value, tag=61443, type=integer
	VsaUtstarcomVlanId(u32), // from UTStarcom with plain value, tag=140, type=integer
	VsaUtstarcomCommittedbandwidth(u32), // from UTStarcom with plain value, tag=142, type=integer
	VsaUtstarcomMaxbandwidth(u32), // from UTStarcom with plain value, tag=143, type=integer
	VsaUtstarcomPriority(u32), // from UTStarcom with plain value, tag=145, type=integer
	VsaUtstarcomErrorReason(u32), // from UTStarcom with plain value, tag=147, type=integer
	VsaUtstarcomPrimarydns(u32), // from UTStarcom with plain value, tag=152, type=integer
	VsaUtstarcomSecondarydns(u32), // from UTStarcom with plain value, tag=153, type=integer
	VsaUtstarcomMaxburstsize(u32), // from UTStarcom with plain value, tag=161, type=integer
	VsaUtstarcomMaxdelay(u32), // from UTStarcom with plain value, tag=162, type=integer
	VsaUtstarcomMaxjitter(u32), // from UTStarcom with plain value, tag=163, type=integer
	VsaUtstarcomDeviceid(&'a [u8]), // from UTStarcom with plain value, tag=165, type=string
	VsaUtstarcomModuleId(u32), // from UTStarcom with plain value, tag=166, type=integer
	VsaUtstarcomPortNo(u32), // from UTStarcom with plain value, tag=167, type=integer
	VsaUtstarcomLogicalPortNo(u32), // from UTStarcom with plain value, tag=168, type=integer
	VsaUtstarcomUniMaxMac(u32), // from UTStarcom with plain value, tag=169, type=integer
	VsaUtstarcomDefaultGateway(u32), // from UTStarcom with plain value, tag=170, type=integer
	VsaUtstarcomCliAccessLevel(u32), // from UTStarcom with plain value, tag=171, type=integer
	VsaUtstarcomActInputOctets(&'a [u8]), // from UTStarcom with plain value, tag=180, type=string
	VsaUtstarcomActOutputOctets(&'a [u8]), // from UTStarcom with plain value, tag=181, type=string
	VsaUtstarcomActInputFrames(&'a [u8]), // from UTStarcom with plain value, tag=182, type=string
	VsaUtstarcomActOutputFrames(&'a [u8]), // from UTStarcom with plain value, tag=183, type=string
	VsaUtstarcomOnuMcFilterEnable(u32), // from UTStarcom with plain value, tag=184, type=integer
	VsaUtstarcomUniAutoNegotiation(u32), // from UTStarcom with plain value, tag=185, type=integer
	VsaUtstarcomUniSpeed(u32), // from UTStarcom with plain value, tag=186, type=integer
	VsaUtstarcomUniDuplex(u32), // from UTStarcom with plain value, tag=187, type=integer
	VsaUtstarcomOnuAdminStatus(u32), // from UTStarcom with plain value, tag=188, type=integer
	VsaUtstarcomOnuFwScUpgrade(u32), // from UTStarcom with plain value, tag=189, type=integer
	VsaVncPppoeCbqRx(u32), // from ValemountNetworks with plain value, tag=1, type=integer
	VsaVncPppoeCbqTx(u32), // from ValemountNetworks with plain value, tag=2, type=integer
	VsaVncPppoeCbqRxFallback(u32), // from ValemountNetworks with plain value, tag=3, type=integer
	VsaVncPppoeCbqTxFallback(u32), // from ValemountNetworks with plain value, tag=4, type=integer
	VsaVncSplash(valemountnetworks::VncSplash), // from ValemountNetworks with Enum value, tag=10, type=integer
	VsaVersanetTerminationCause(versanet::VersanetTerminationCause), // from Versanet with Enum value, tag=1, type=integer
	VsaAcctInterimRecordNumber(u32), // from VerizonWireless with plain value, tag=200, type=integer
	VsaUeInfoType(u32), // from VerizonWireless with plain value, tag=201, type=integer
	VsaUeInfoValue(&'a [u8]), // from VerizonWireless with plain value, tag=202, type=string
	VsaDynamicAddressFlag(u32), // from VerizonWireless with plain value, tag=203, type=integer
	VsaLocalSeqNumber(u32), // from VerizonWireless with plain value, tag=204, type=integer
	VsaTimeFirstUsage(u32), // from VerizonWireless with plain value, tag=205, type=date
	VsaTimeLastUsage(u32), // from VerizonWireless with plain value, tag=206, type=date
	VsaChargingGroupId(&'a [u8]), // from VerizonWireless with plain value, tag=207, type=string
	VsaServiceDataContainerBin(&'a [u8]), // from VerizonWireless with plain value, tag=210, type=octets
	VsaServiceDataContainer(&'a [u8]), // from VerizonWireless with plain value, tag=211, type=tlv
	VsaWaveriderGradeOfService(waverider::WaveriderGradeOfService), // from Waverider with Enum value, tag=1, type=integer
	VsaWaveriderPriorityEnabled(waverider::WaveriderPriorityEnabled), // from Waverider with Enum value, tag=2, type=integer
	VsaWaveriderAuthenticationKey(&'a [u8]), // from Waverider with plain value, tag=3, type=string
	VsaWaveriderCurrentPassword(&'a [u8]), // from Waverider with plain value, tag=5, type=string
	VsaWaveriderNewPassword(&'a [u8]), // from Waverider with plain value, tag=6, type=string
	VsaWaveriderRadioFrequency(waverider::WaveriderRadioFrequency), // from Waverider with Enum value, tag=7, type=integer
	VsaWaveriderSnmpReadCommunity(&'a [u8]), // from Waverider with plain value, tag=8, type=string
	VsaWaveriderSnmpWriteCommunity(&'a [u8]), // from Waverider with plain value, tag=9, type=string
	VsaWaveriderSnmpTrapServer(&'a [u8]), // from Waverider with plain value, tag=10, type=string
	VsaWaveriderSnmpContact(&'a [u8]), // from Waverider with plain value, tag=11, type=string
	VsaWaveriderSnmpLocation(&'a [u8]), // from Waverider with plain value, tag=12, type=string
	VsaWaveriderSnmpName(&'a [u8]), // from Waverider with plain value, tag=13, type=string
	VsaWaveriderMaxCustomers(u32), // from Waverider with plain value, tag=14, type=integer
	VsaWaveriderRfPower(waverider::WaveriderRfPower), // from Waverider with Enum value, tag=15, type=integer
	VsaWbAuthTimeLeft(u32), // from Walabi with plain value, tag=1, type=integer
	VsaWbAuthAccumBw(u32), // from Walabi with plain value, tag=2, type=integer
	VsaWbAuthBwQuota(u32), // from Walabi with plain value, tag=3, type=integer
	VsaWbAuthBwCount(u32), // from Walabi with plain value, tag=4, type=integer
	VsaWbAuthUploadLimit(u32), // from Walabi with plain value, tag=5, type=integer
	VsaWbAuthDownloadLimit(u32), // from Walabi with plain value, tag=6, type=integer
	VsaWbAuthLoginTime(u32), // from Walabi with plain value, tag=7, type=integer
	VsaWbAuthLogoutTime(u32), // from Walabi with plain value, tag=8, type=integer
	VsaWbAuthTimeDiff(u32), // from Walabi with plain value, tag=9, type=integer
	VsaWbAuthBwUsage(u32), // from Walabi with plain value, tag=10, type=integer
	VsaWichorusPolicyName(&'a [u8]), // from Wichorus with plain value, tag=1, type=string
	VsaWichorusUserPrivilege(&'a [u8]), // from Wichorus with plain value, tag=2, type=string
	VsaWimaxCapability(&'a [u8]), // from WiMAX with plain value, tag=1, type=tlv
	VsaWimaxDeviceAuthenticationIndicator(u8), // from WiMAX with plain value, tag=2, type=byte
	VsaWimaxGmtTimezoneOffset(&'a [u8]), // from WiMAX with plain value, tag=3, type=signed
	VsaWimaxAaaSessionId(&'a [u8]), // from WiMAX with plain value, tag=4, type=octets
	VsaWimaxMsk(&'a [u8]), // from WiMAX with plain value, tag=5, type=octets
	VsaWimaxHhaIpMip4(Ipv4Addr), // from WiMAX with plain value, tag=6, type=ipaddr
	VsaWimaxHhaIpMip6(Ipv6Addr), // from WiMAX with plain value, tag=7, type=ipv6addr
	VsaWimaxDhcpv4Server(&'a [u8]), // from WiMAX with plain value, tag=8, type=combo-ip
	VsaWimaxDhcpv6Server(&'a [u8]), // from WiMAX with plain value, tag=9, type=combo-ip
	VsaWimaxMnHhaMip4Key(&'a [u8]), // from WiMAX with plain value, tag=10, type=octets
	VsaWimaxMnHhaMip4Spi(u32), // from WiMAX with plain value, tag=11, type=integer
	VsaWimaxMnHhaMip6Key(&'a [u8]), // from WiMAX with plain value, tag=12, type=octets
	VsaWimaxMnHhaMip6Spi(u32), // from WiMAX with plain value, tag=13, type=integer
	VsaWimaxFaRkKey(&'a [u8]), // from WiMAX with plain value, tag=14, type=octets
	VsaWimaxHaRkKey(&'a [u8]), // from WiMAX with plain value, tag=15, type=octets
	VsaWimaxHaRkSpi(u32), // from WiMAX with plain value, tag=16, type=integer
	VsaWimaxHaRkLifetime(u32), // from WiMAX with plain value, tag=17, type=integer
	VsaWimaxRrqHaIp(&'a [u8]), // from WiMAX with plain value, tag=18, type=combo-ip
	VsaWimaxRrqMnHaKey(&'a [u8]), // from WiMAX with plain value, tag=19, type=octets
	VsaWimaxRrqMnHaSpi(u32), // from WiMAX with plain value, tag=20, type=integer
	VsaWimaxSessionContinue(u32), // from WiMAX with plain value, tag=21, type=integer
	VsaWimaxBeginningOfSession(u32), // from WiMAX with plain value, tag=22, type=integer
	VsaWimaxIpTechnology(wimax::WimaxIpTechnology), // from WiMAX with Enum value, tag=23, type=integer
	VsaWimaxHotlineIndicator(&'a [u8]), // from WiMAX with plain value, tag=24, type=string
	VsaWimaxPrepaidIndicator(u8), // from WiMAX with plain value, tag=25, type=byte
	VsaWimaxPdfid(u16), // from WiMAX with plain value, tag=26, type=short
	VsaWimaxSdfid(u16), // from WiMAX with plain value, tag=27, type=short
	VsaWimaxPacketFlowDescriptor(&'a [u8]), // from WiMAX with plain value, tag=28, type=tlv
	VsaWimaxQosDescriptor(&'a [u8]), // from WiMAX with plain value, tag=29, type=tlv
	VsaWimaxUplinkGrantedQos(&'a [u8]), // from WiMAX with plain value, tag=30, type=tlv
	VsaWimaxControlPacketsIn(u32), // from WiMAX with plain value, tag=31, type=integer
	VsaWimaxControlOctetsIn(u32), // from WiMAX with plain value, tag=32, type=integer
	VsaWimaxControlPacketsOut(u32), // from WiMAX with plain value, tag=33, type=integer
	VsaWimaxControlOctetsOut(u32), // from WiMAX with plain value, tag=34, type=integer
	VsaWimaxPpac(&'a [u8]), // from WiMAX with plain value, tag=35, type=tlv
	VsaWimaxSessionTerminationCapability(wimax::WimaxSessionTerminationCapability), // from WiMAX with Enum value, tag=36, type=integer
	VsaWimaxPpaq(&'a [u8]), // from WiMAX with plain value, tag=37, type=tlv
	VsaWimaxPrepaidTariffSwitching(&'a [u8]), // from WiMAX with plain value, tag=38, type=tlv
	VsaWimaxActiveTimeDuration(u32), // from WiMAX with plain value, tag=39, type=integer
	VsaWimaxDhcpRk(&'a [u8]), // from WiMAX with plain value, tag=40, type=octets
	VsaWimaxDhcpRkKeyId(u32), // from WiMAX with plain value, tag=41, type=integer
	VsaWimaxDhcpRkLifetime(u32), // from WiMAX with plain value, tag=42, type=integer
	VsaWimaxDhcpMsgServerIp(Ipv4Addr), // from WiMAX with plain value, tag=43, type=ipaddr
	VsaWimaxIdleModeTransition(u8), // from WiMAX with plain value, tag=44, type=byte
	VsaWimaxNapId(&'a [u8]), // from WiMAX with plain value, tag=45, type=octets
	VsaWimaxBsId(&'a [u8]), // from WiMAX with plain value, tag=46, type=octets
	VsaWimaxLocation(&'a [u8]), // from WiMAX with plain value, tag=47, type=octets
	VsaWimaxAcctInputPacketsGigaword(u32), // from WiMAX with plain value, tag=48, type=integer
	VsaWimaxAcctOutputPacketsGigaword(u32), // from WiMAX with plain value, tag=49, type=integer
	VsaWimaxUplinkFlowDescription(&'a [u8]), // from WiMAX with plain value, tag=50, type=string
	VsaWimaxBluCoaIpv6(Ipv6Addr), // from WiMAX with plain value, tag=51, type=ipv6addr
	VsaWimaxDnsServer(&'a [u8]), // from WiMAX with plain value, tag=52, type=combo-ip
	VsaWimaxHotlineProfileId(&'a [u8]), // from WiMAX with plain value, tag=53, type=string
	VsaWimaxHttpRedirectionRule(&'a [u8]), // from WiMAX with plain value, tag=54, type=string
	VsaWimaxIpRedirectionRule(&'a [u8]), // from WiMAX with plain value, tag=55, type=string
	VsaWimaxHotlineSessionTimer(u32), // from WiMAX with plain value, tag=56, type=integer
	VsaWimaxNspId(&'a [u8]), // from WiMAX with plain value, tag=57, type=octets
	VsaWimaxHaRkKeyRequested(wimax::WimaxHaRkKeyRequested), // from WiMAX with Enum value, tag=58, type=integer
	VsaWimaxCountType(u8), // from WiMAX with plain value, tag=59, type=byte
	VsaWimaxDmActionCode(wimax::WimaxDmActionCode), // from WiMAX with Enum value, tag=60, type=integer
	VsaWimaxFaRkSpi(u32), // from WiMAX with plain value, tag=61, type=integer
	VsaWimaxDownlinkFlowDescription(&'a [u8]), // from WiMAX with plain value, tag=62, type=string
	VsaWimaxDownlinkGrantedQos(&'a [u8]), // from WiMAX with plain value, tag=63, type=tlv
	VsaWimaxVhaIpMip4(Ipv4Addr), // from WiMAX with plain value, tag=64, type=ipaddr
	VsaWimaxVhaIpMip6(Ipv6Addr), // from WiMAX with plain value, tag=65, type=ipv6addr
	VsaWimaxVhaMip4Key(&'a [u8]), // from WiMAX with plain value, tag=66, type=octets
	VsaWimaxVhaRkKey(&'a [u8]), // from WiMAX with plain value, tag=67, type=octets
	VsaWimaxVhaRkSpi(u32), // from WiMAX with plain value, tag=68, type=integer
	VsaWimaxVhaRkLifetime(u32), // from WiMAX with plain value, tag=69, type=integer
	VsaWimaxMnVhaMip6Key(&'a [u8]), // from WiMAX with plain value, tag=70, type=octets
	VsaWimaxMnVhaMip4Spi(u32), // from WiMAX with plain value, tag=71, type=integer
	VsaWimaxMnVhaMip6Spi(u32), // from WiMAX with plain value, tag=72, type=integer
	VsaWimaxVdhcpv4Server(Ipv4Addr), // from WiMAX with plain value, tag=73, type=ipaddr
	VsaWimaxVdhcpv6Server(Ipv6Addr), // from WiMAX with plain value, tag=74, type=ipv6addr
	VsaWimaxVdhcpRk(&'a [u8]), // from WiMAX with plain value, tag=75, type=octets
	VsaWimaxVdhcpRkKeyId(u32), // from WiMAX with plain value, tag=76, type=integer
	VsaWimaxVdhcpRkLifetime(u32), // from WiMAX with plain value, tag=77, type=integer
	VsaWisprLocationId(&'a [u8]), // from WISPr with plain value, tag=1, type=string
	VsaWisprLocationName(&'a [u8]), // from WISPr with plain value, tag=2, type=string
	VsaWisprLogoffUrl(&'a [u8]), // from WISPr with plain value, tag=3, type=string
	VsaWisprRedirectionUrl(&'a [u8]), // from WISPr with plain value, tag=4, type=string
	VsaWisprBandwidthMinUp(u32), // from WISPr with plain value, tag=5, type=integer
	VsaWisprBandwidthMinDown(u32), // from WISPr with plain value, tag=6, type=integer
	VsaWisprBandwidthMaxUp(u32), // from WISPr with plain value, tag=7, type=integer
	VsaWisprBandwidthMaxDown(u32), // from WISPr with plain value, tag=8, type=integer
	VsaWisprSessionTerminateTime(&'a [u8]), // from WISPr with plain value, tag=9, type=string
	VsaWisprSessionTerminateEndOfDay(&'a [u8]), // from WISPr with plain value, tag=10, type=string
	VsaWisprBillingClassOfService(&'a [u8]), // from WISPr with plain value, tag=11, type=string
	VsaXediaDnsServer(Ipv4Addr), // from Xedia with plain value, tag=1, type=ipaddr
	VsaXediaNetbiosServer(Ipv4Addr), // from Xedia with plain value, tag=2, type=ipaddr
	VsaXediaAddressPool(&'a [u8]), // from Xedia with plain value, tag=3, type=string
	VsaXediaPppEchoInterval(u32), // from Xedia with plain value, tag=4, type=integer
	VsaXediaSshPrivileges(u32), // from Xedia with plain value, tag=5, type=integer
	VsaXediaClientAccessNetwork(&'a [u8]), // from Xedia with plain value, tag=6, type=string
	VsaXediaClientFirewallSetting(u32), // from Xedia with plain value, tag=7, type=integer
	VsaXediaSavePassword(u32), // from Xedia with plain value, tag=8, type=integer
	VsaXylanAuthGroup(u32), // from Xylan with plain value, tag=1, type=integer
	VsaXylanSlotPort(&'a [u8]), // from Xylan with plain value, tag=2, type=string
	VsaXylanTimeOfDay(&'a [u8]), // from Xylan with plain value, tag=3, type=string
	VsaXylanClientIpAddr(Ipv4Addr), // from Xylan with plain value, tag=4, type=ipaddr
	VsaXylanGroupDesc(&'a [u8]), // from Xylan with plain value, tag=5, type=string
	VsaXylanPortDesc(&'a [u8]), // from Xylan with plain value, tag=6, type=string
	VsaXylanProfilNumb(u32), // from Xylan with plain value, tag=7, type=integer
	VsaXylanAuthGroupProtocol(&'a [u8]), // from Xylan with plain value, tag=8, type=string
	VsaXylanAsaAccess(&'a [u8]), // from Xylan with plain value, tag=9, type=string
	VsaXylanEndUserProfile(u32), // from Xylan with plain value, tag=10, type=integer
	VsaXylanAccessPriv(xylan::XylanAccessPriv), // from Xylan with Enum value, tag=16, type=integer
	VsaXylanNmsGroup(&'a [u8]), // from Xylan with plain value, tag=20, type=string
	VsaXylanNmsFirstName(&'a [u8]), // from Xylan with plain value, tag=21, type=string
	VsaXylanNmsLastName(&'a [u8]), // from Xylan with plain value, tag=22, type=string
	VsaXylanNmsDescription(&'a [u8]), // from Xylan with plain value, tag=23, type=string
	VsaXylanAccePrivR1(&'a [u8]), // from Xylan with plain value, tag=33, type=octets
	VsaXylanAccePrivR2(&'a [u8]), // from Xylan with plain value, tag=34, type=octets
	VsaXylanAccePrivW1(&'a [u8]), // from Xylan with plain value, tag=35, type=octets
	VsaXylanAccePrivW2(&'a [u8]), // from Xylan with plain value, tag=36, type=octets
	VsaXylanAccePrivG1(&'a [u8]), // from Xylan with plain value, tag=37, type=octets
	VsaXylanAccePrivG2(&'a [u8]), // from Xylan with plain value, tag=38, type=octets
	VsaXylanAccePrivFR1(&'a [u8]), // from Xylan with plain value, tag=39, type=octets
	VsaXylanAccePrivFR2(&'a [u8]), // from Xylan with plain value, tag=40, type=octets
	VsaXylanAccePrivFW1(&'a [u8]), // from Xylan with plain value, tag=41, type=octets
	VsaXylanAccePrivFW2(&'a [u8]), // from Xylan with plain value, tag=42, type=octets
	VsaYubikeyKey(&'a [u8]), // from Yubico with plain value, tag=1, type=octets
	VsaYubikeyPublicId(&'a [u8]), // from Yubico with plain value, tag=2, type=string
	VsaYubikeyPrivateId(&'a [u8]), // from Yubico with plain value, tag=3, type=octets
	VsaYubikeyCounter(u32), // from Yubico with plain value, tag=4, type=integer
	VsaYubikeyTimestamp(u32), // from Yubico with plain value, tag=5, type=integer
	VsaYubikeyRandom(u32), // from Yubico with plain value, tag=6, type=integer
	VsaYubikeyOtp(&'a [u8]), // from Yubico with plain value, tag=7, type=string
	VsaZeusZxtmGroup(&'a [u8]), // from Zeus with plain value, tag=1, type=string
	VsaZteClientDnsPri(&'a [u8]), // from ZTE with plain value, tag=1, type=string
	VsaZteClientDnsSec(&'a [u8]), // from ZTE with plain value, tag=2, type=string
	VsaZteContextName(&'a [u8]), // from ZTE with plain value, tag=4, type=string
	VsaZteTunnelMaxSessions(u32), // from ZTE with plain value, tag=21, type=integer
	VsaZteTunnelMaxTunnels(u32), // from ZTE with plain value, tag=22, type=integer
	VsaZteTunnelWindow(u32), // from ZTE with plain value, tag=24, type=integer
	VsaZteTunnelRetransmit(u32), // from ZTE with plain value, tag=25, type=integer
	VsaZteTunnelCmdTimeout(u32), // from ZTE with plain value, tag=26, type=integer
	VsaZtePppoeUrl(&'a [u8]), // from ZTE with plain value, tag=27, type=string
	VsaZtePppoeMotm(&'a [u8]), // from ZTE with plain value, tag=28, type=string
	VsaZteTunnelAlgorithm(u32), // from ZTE with plain value, tag=31, type=integer
	VsaZteTunnelDeadtime(u32), // from ZTE with plain value, tag=32, type=integer
	VsaZteMcastSend(u32), // from ZTE with plain value, tag=33, type=integer
	VsaZteMcastReceive(u32), // from ZTE with plain value, tag=34, type=integer
	VsaZteMcastMaxgroups(u32), // from ZTE with plain value, tag=35, type=integer
	VsaZteAccessType(u32), // from ZTE with plain value, tag=74, type=integer
	VsaZteQosType(u32), // from ZTE with plain value, tag=81, type=integer
	VsaZteQosProfileDown(&'a [u8]), // from ZTE with plain value, tag=82, type=string
	VsaZteRateCtrlScrDown(u32), // from ZTE with plain value, tag=83, type=integer
	VsaZteRateCtrlBurstDown(u32), // from ZTE with plain value, tag=84, type=integer
	VsaZteRateCtrlPcr(u32), // from ZTE with plain value, tag=86, type=integer
	VsaZteTcpSynRate(u32), // from ZTE with plain value, tag=88, type=integer
	VsaZteRateCtrlScrUp(u32), // from ZTE with plain value, tag=89, type=integer
	VsaZtePriorityLevel(u32), // from ZTE with plain value, tag=90, type=integer
	VsaZteRateCtrlBurstUp(u32), // from ZTE with plain value, tag=91, type=integer
	VsaZteRateCtrlBurstMaxDown(u32), // from ZTE with plain value, tag=92, type=integer
	VsaZteRateCtrlBurstMaxUp(u32), // from ZTE with plain value, tag=93, type=integer
	VsaZteQosProfileUp(&'a [u8]), // from ZTE with plain value, tag=94, type=string
	VsaZteTcpLimitNum(u32), // from ZTE with plain value, tag=95, type=integer
	VsaZteTcpLimitMode(u32), // from ZTE with plain value, tag=96, type=integer
	VsaZteIgmpServiceProfileNum(u32), // from ZTE with plain value, tag=97, type=integer
	VsaZtePppSserviceType(u32), // from ZTE with plain value, tag=101, type=integer
	VsaZteSwPrivilege(u32), // from ZTE with plain value, tag=104, type=integer
	VsaZteAccessDomain(&'a [u8]), // from ZTE with plain value, tag=151, type=string
	VsaZteVpnId(&'a [u8]), // from ZTE with plain value, tag=190, type=string
	VsaZteRateBustDpir(u32), // from ZTE with plain value, tag=191, type=integer
	VsaZteRateBustUpir(u32), // from ZTE with plain value, tag=192, type=integer
	VsaZteRateCtrlPbsDown(u32), // from ZTE with plain value, tag=202, type=integer
	VsaZteRateCtrlPbsUp(u32), // from ZTE with plain value, tag=203, type=integer
	VsaZteRateCtrlScrUpV6(u32), // from ZTE with plain value, tag=228, type=integer
	VsaZteRateCtrlBurstUpV6(u32), // from ZTE with plain value, tag=229, type=integer
	VsaZteRateCtrlBurstMaxUpV6(u32), // from ZTE with plain value, tag=230, type=integer
	VsaZteRateCtrlPbsUpV6(u32), // from ZTE with plain value, tag=231, type=integer
	VsaZteQosProfileUpV6(&'a [u8]), // from ZTE with plain value, tag=232, type=string
	VsaZteRateCtrlScrDownV6(u32), // from ZTE with plain value, tag=233, type=integer
	VsaZteRateCtrlBurstDownV6(u32), // from ZTE with plain value, tag=234, type=integer
	VsaZteRateCtrlBurstMaxDownV6(u32), // from ZTE with plain value, tag=235, type=integer
	VsaZteRateCtrlPbsDownV6(u32), // from ZTE with plain value, tag=236, type=integer
	VsaZteQosProfileDownV6(&'a [u8]), // from ZTE with plain value, tag=237, type=string
	VsaZyxelPrivilegeAvpair(&'a [u8]), // from Zyxel with plain value, tag=3, type=string
	VsaZyxelCallbackOption(zyxel::ZyxelCallbackOption), // from Zyxel with Enum value, tag=192, type=integer
	VsaZyxelCallbackPhoneSource(zyxel::ZyxelCallbackPhoneSource), // from Zyxel with Enum value, tag=193, type=integer
	Unknown(u8, &'a [u8]),
	VsaUnknown(u32, u8, &'a [u8]),
}

