/// Definitions for vendor Cisco-VPN3000, vendor value 3076

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000AllowAlphaOnlyPasswords(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000AllowAlphaOnlyPasswords {
	pub const Disallow: Cvpn3000AllowAlphaOnlyPasswords = Cvpn3000AllowAlphaOnlyPasswords(0);
	pub const Allow: Cvpn3000AllowAlphaOnlyPasswords = Cvpn3000AllowAlphaOnlyPasswords(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000SepCardAssignment(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000SepCardAssignment {
	pub const Sep1: Cvpn3000SepCardAssignment = Cvpn3000SepCardAssignment(1);
	pub const Sep2: Cvpn3000SepCardAssignment = Cvpn3000SepCardAssignment(2);
	pub const Sep3: Cvpn3000SepCardAssignment = Cvpn3000SepCardAssignment(4);
	pub const Sep4: Cvpn3000SepCardAssignment = Cvpn3000SepCardAssignment(8);
	pub const AnySep: Cvpn3000SepCardAssignment = Cvpn3000SepCardAssignment(15);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000PriorityOnSep(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000PriorityOnSep {
	pub const High: Cvpn3000PriorityOnSep = Cvpn3000PriorityOnSep(1);
	pub const MedHigh: Cvpn3000PriorityOnSep = Cvpn3000PriorityOnSep(2);
	pub const Medium: Cvpn3000PriorityOnSep = Cvpn3000PriorityOnSep(3);
	pub const MedLow: Cvpn3000PriorityOnSep = Cvpn3000PriorityOnSep(4);
	pub const Low: Cvpn3000PriorityOnSep = Cvpn3000PriorityOnSep(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000TunnelingProtocols(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000TunnelingProtocols {
	pub const Pptp: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(1);
	pub const L2Tp: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(2);
	pub const Ipsec: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(4);
	pub const L2TpOrIpsec: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(8);
	pub const Webvpn: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(16);
	pub const PptpAndL2Tp: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(3);
	pub const PptpAndIpsec: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(5);
	pub const L2TpAndIpsec: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(6);
	pub const PptpL2TpIpsec: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(7);
	pub const PptpAndL2TpOverIpsec: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(9);
	pub const L2TpAndL2TpOverIpsec: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(10);
	pub const PptpL2TpL2TpOverIpsec: Cvpn3000TunnelingProtocols = Cvpn3000TunnelingProtocols(11);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecAuthentication(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecAuthentication {
	pub const None: Cvpn3000IpsecAuthentication = Cvpn3000IpsecAuthentication(0);
	pub const Radius: Cvpn3000IpsecAuthentication = Cvpn3000IpsecAuthentication(1);
	pub const Ldap: Cvpn3000IpsecAuthentication = Cvpn3000IpsecAuthentication(2);
	pub const Ntdomain: Cvpn3000IpsecAuthentication = Cvpn3000IpsecAuthentication(3);
	pub const Sdi: Cvpn3000IpsecAuthentication = Cvpn3000IpsecAuthentication(4);
	pub const Internal: Cvpn3000IpsecAuthentication = Cvpn3000IpsecAuthentication(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecAllowPasswdStore(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecAllowPasswdStore {
	pub const Disallow: Cvpn3000IpsecAllowPasswdStore = Cvpn3000IpsecAllowPasswdStore(0);
	pub const Allow: Cvpn3000IpsecAllowPasswdStore = Cvpn3000IpsecAllowPasswdStore(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000UseClientAddress(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000UseClientAddress {
	pub const Disallow: Cvpn3000UseClientAddress = Cvpn3000UseClientAddress(0);
	pub const Allow: Cvpn3000UseClientAddress = Cvpn3000UseClientAddress(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000PptpMinAuthProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000PptpMinAuthProtocol {
	pub const Pap: Cvpn3000PptpMinAuthProtocol = Cvpn3000PptpMinAuthProtocol(1);
	pub const Chap: Cvpn3000PptpMinAuthProtocol = Cvpn3000PptpMinAuthProtocol(2);
	pub const EapMd5: Cvpn3000PptpMinAuthProtocol = Cvpn3000PptpMinAuthProtocol(4);
	pub const EapGtc: Cvpn3000PptpMinAuthProtocol = Cvpn3000PptpMinAuthProtocol(8);
	pub const EapTls: Cvpn3000PptpMinAuthProtocol = Cvpn3000PptpMinAuthProtocol(16);
	pub const Mschapv1: Cvpn3000PptpMinAuthProtocol = Cvpn3000PptpMinAuthProtocol(32);
	pub const Mschapv2: Cvpn3000PptpMinAuthProtocol = Cvpn3000PptpMinAuthProtocol(64);
	pub const Default: Cvpn3000PptpMinAuthProtocol = Cvpn3000PptpMinAuthProtocol(102);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000L2TpMinAuthProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000L2TpMinAuthProtocol {
	pub const Pap: Cvpn3000L2TpMinAuthProtocol = Cvpn3000L2TpMinAuthProtocol(1);
	pub const Chap: Cvpn3000L2TpMinAuthProtocol = Cvpn3000L2TpMinAuthProtocol(2);
	pub const EapMd5: Cvpn3000L2TpMinAuthProtocol = Cvpn3000L2TpMinAuthProtocol(4);
	pub const EapGtc: Cvpn3000L2TpMinAuthProtocol = Cvpn3000L2TpMinAuthProtocol(8);
	pub const EapTls: Cvpn3000L2TpMinAuthProtocol = Cvpn3000L2TpMinAuthProtocol(16);
	pub const Mschapv1: Cvpn3000L2TpMinAuthProtocol = Cvpn3000L2TpMinAuthProtocol(32);
	pub const Mschapv2: Cvpn3000L2TpMinAuthProtocol = Cvpn3000L2TpMinAuthProtocol(64);
	pub const Default: Cvpn3000L2TpMinAuthProtocol = Cvpn3000L2TpMinAuthProtocol(102);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000PptpEncryption(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000PptpEncryption {
	pub const PptpRequired: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(1);
	pub const Pptp40Bit: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(2);
	pub const Pptp128: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(4);
	pub const PptpStatelessRequired: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(8);
	pub const Pptp40EncryptionReq: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(3);
	pub const Pptp128EncryptionReq: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(5);
	pub const Pptp40Or128: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(6);
	pub const Pptp40Or128EncryReq: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(7);
	pub const PptpEncStatelessReq: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(9);
	pub const Pptp40StatelessReq: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(10);
	pub const Pptp40EncOrStatelessReq: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(11);
	pub const Pptp128StatelessReq: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(12);
	pub const Pptp128EncOrStatelessReq: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(13);
	pub const Pptp40Or128StatelessReq: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(14);
	pub const Pptp40Or128EncOrStatlsReq: Cvpn3000PptpEncryption = Cvpn3000PptpEncryption(15);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000L2TpEncryption(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000L2TpEncryption {
	pub const L2TpRequired: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(1);
	pub const L2Tp40Bit: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(2);
	pub const L2Tp128: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(4);
	pub const L2TpStatelessReq: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(8);
	pub const L2Tp40EncryptionReq: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(3);
	pub const L2Tp128EncryptionReq: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(5);
	pub const L2Tp40Or128: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(6);
	pub const L2Tp40Or128EncryReq: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(7);
	pub const L2TpEncStatelessReq: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(9);
	pub const L2Tp40StatelessReq: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(10);
	pub const L2Tp40EncOrStatelessReq: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(11);
	pub const L2Tp128StatelessReq: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(12);
	pub const L2Tp128EncOrStatelessReq: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(13);
	pub const L2Tp40Or128StatelessReq: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(14);
	pub const L2Tp40Or128EncOrStatlsReq: Cvpn3000L2TpEncryption = Cvpn3000L2TpEncryption(15);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000AuthServerType(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000AuthServerType {
	pub const FirstActiveServer: Cvpn3000AuthServerType = Cvpn3000AuthServerType(0);
	pub const Radius: Cvpn3000AuthServerType = Cvpn3000AuthServerType(1);
	pub const Ldap: Cvpn3000AuthServerType = Cvpn3000AuthServerType(2);
	pub const Nt: Cvpn3000AuthServerType = Cvpn3000AuthServerType(3);
	pub const Sdi: Cvpn3000AuthServerType = Cvpn3000AuthServerType(4);
	pub const Internal: Cvpn3000AuthServerType = Cvpn3000AuthServerType(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecLtlKeepalives(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecLtlKeepalives {
	pub const Off: Cvpn3000IpsecLtlKeepalives = Cvpn3000IpsecLtlKeepalives(0);
	pub const On: Cvpn3000IpsecLtlKeepalives = Cvpn3000IpsecLtlKeepalives(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecTunnelType(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecTunnelType {
	pub const LanToLan: Cvpn3000IpsecTunnelType = Cvpn3000IpsecTunnelType(1);
	pub const RemoteAccess: Cvpn3000IpsecTunnelType = Cvpn3000IpsecTunnelType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecModeConfig(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecModeConfig {
	pub const On: Cvpn3000IpsecModeConfig = Cvpn3000IpsecModeConfig(1);
	pub const Off: Cvpn3000IpsecModeConfig = Cvpn3000IpsecModeConfig(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecUserGroupLock(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecUserGroupLock {
	pub const Off: Cvpn3000IpsecUserGroupLock = Cvpn3000IpsecUserGroupLock(0);
	pub const On: Cvpn3000IpsecUserGroupLock = Cvpn3000IpsecUserGroupLock(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecOverUdp(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecOverUdp {
	pub const Off: Cvpn3000IpsecOverUdp = Cvpn3000IpsecOverUdp(0);
	pub const On: Cvpn3000IpsecOverUdp = Cvpn3000IpsecOverUdp(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000PptpMppcCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000PptpMppcCompression {
	pub const Off: Cvpn3000PptpMppcCompression = Cvpn3000PptpMppcCompression(0);
	pub const On: Cvpn3000PptpMppcCompression = Cvpn3000PptpMppcCompression(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000L2TpMppcCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000L2TpMppcCompression {
	pub const Off: Cvpn3000L2TpMppcCompression = Cvpn3000L2TpMppcCompression(0);
	pub const On: Cvpn3000L2TpMppcCompression = Cvpn3000L2TpMppcCompression(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecIpCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecIpCompression {
	pub const Off: Cvpn3000IpsecIpCompression = Cvpn3000IpsecIpCompression(0);
	pub const On: Cvpn3000IpsecIpCompression = Cvpn3000IpsecIpCompression(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecIkePeerIdcheck(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecIkePeerIdcheck {
	pub const Required: Cvpn3000IpsecIkePeerIdcheck = Cvpn3000IpsecIkePeerIdcheck(1);
	pub const IfSupportedByCertifiate: Cvpn3000IpsecIkePeerIdcheck = Cvpn3000IpsecIkePeerIdcheck(2);
	pub const DoNotCheck: Cvpn3000IpsecIkePeerIdcheck = Cvpn3000IpsecIkePeerIdcheck(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IkeKeepAlives(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IkeKeepAlives {
	pub const Off: Cvpn3000IkeKeepAlives = Cvpn3000IkeKeepAlives(0);
	pub const On: Cvpn3000IkeKeepAlives = Cvpn3000IkeKeepAlives(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecAuthOnRekey(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecAuthOnRekey {
	pub const Off: Cvpn3000IpsecAuthOnRekey = Cvpn3000IpsecAuthOnRekey(0);
	pub const On: Cvpn3000IpsecAuthOnRekey = Cvpn3000IpsecAuthOnRekey(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000ReqrdClientFwVendorCode(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000ReqrdClientFwVendorCode {
	pub const CiscoWithCic: Cvpn3000ReqrdClientFwVendorCode = Cvpn3000ReqrdClientFwVendorCode(1);
	pub const ZoneLabs: Cvpn3000ReqrdClientFwVendorCode = Cvpn3000ReqrdClientFwVendorCode(2);
	pub const NetworkIce: Cvpn3000ReqrdClientFwVendorCode = Cvpn3000ReqrdClientFwVendorCode(3);
	pub const Sygate: Cvpn3000ReqrdClientFwVendorCode = Cvpn3000ReqrdClientFwVendorCode(4);
	pub const CiscoWithCsa: Cvpn3000ReqrdClientFwVendorCode = Cvpn3000ReqrdClientFwVendorCode(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000RequireHwClientAuth(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000RequireHwClientAuth {
	pub const No: Cvpn3000RequireHwClientAuth = Cvpn3000RequireHwClientAuth(0);
	pub const Yes: Cvpn3000RequireHwClientAuth = Cvpn3000RequireHwClientAuth(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000RequireIndividualUserAuth(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000RequireIndividualUserAuth {
	pub const No: Cvpn3000RequireIndividualUserAuth = Cvpn3000RequireIndividualUserAuth(0);
	pub const Yes: Cvpn3000RequireIndividualUserAuth = Cvpn3000RequireIndividualUserAuth(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000CiscoIpPhoneBypass(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000CiscoIpPhoneBypass {
	pub const No: Cvpn3000CiscoIpPhoneBypass = Cvpn3000CiscoIpPhoneBypass(0);
	pub const Yes: Cvpn3000CiscoIpPhoneBypass = Cvpn3000CiscoIpPhoneBypass(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecSplitTunnelingPolicy(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecSplitTunnelingPolicy {
	pub const TunnelEverything: Cvpn3000IpsecSplitTunnelingPolicy = Cvpn3000IpsecSplitTunnelingPolicy(0);
	pub const OnlyTunnelListedNetworks: Cvpn3000IpsecSplitTunnelingPolicy = Cvpn3000IpsecSplitTunnelingPolicy(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecReqrdClientFwCap(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecReqrdClientFwCap {
	pub const None: Cvpn3000IpsecReqrdClientFwCap = Cvpn3000IpsecReqrdClientFwCap(0);
	pub const PolicyDefinedByRemoteFwAyt: Cvpn3000IpsecReqrdClientFwCap = Cvpn3000IpsecReqrdClientFwCap(1);
	pub const PolicyPushedCpp: Cvpn3000IpsecReqrdClientFwCap = Cvpn3000IpsecReqrdClientFwCap(2);
	pub const PolicyFromServer: Cvpn3000IpsecReqrdClientFwCap = Cvpn3000IpsecReqrdClientFwCap(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecClientFwFilterOpt(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecClientFwFilterOpt {
	pub const Required: Cvpn3000IpsecClientFwFilterOpt = Cvpn3000IpsecClientFwFilterOpt(0);
	pub const Optional: Cvpn3000IpsecClientFwFilterOpt = Cvpn3000IpsecClientFwFilterOpt(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000IpsecBackupServers(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000IpsecBackupServers {
	pub const UserClientConfiguredList: Cvpn3000IpsecBackupServers = Cvpn3000IpsecBackupServers(1);
	pub const DisableAndClearClientList: Cvpn3000IpsecBackupServers = Cvpn3000IpsecBackupServers(2);
	pub const UseBackupServerList: Cvpn3000IpsecBackupServers = Cvpn3000IpsecBackupServers(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000MsClientIcptDhcpConfMsg(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000MsClientIcptDhcpConfMsg {
	pub const No: Cvpn3000MsClientIcptDhcpConfMsg = Cvpn3000MsClientIcptDhcpConfMsg(0);
	pub const Yes: Cvpn3000MsClientIcptDhcpConfMsg = Cvpn3000MsClientIcptDhcpConfMsg(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000AllowNetworkExtensionMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000AllowNetworkExtensionMode {
	pub const No: Cvpn3000AllowNetworkExtensionMode = Cvpn3000AllowNetworkExtensionMode(0);
	pub const Yes: Cvpn3000AllowNetworkExtensionMode = Cvpn3000AllowNetworkExtensionMode(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000WebvpnContentFilterParameters(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000WebvpnContentFilterParameters {
	pub const Javaandactivex: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(1);
	pub const Javascript: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(2);
	pub const JavaandactivexJavascript: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(3);
	pub const Images: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(4);
	pub const ImagesJavaandactivex: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(5);
	pub const ImagesJavascript: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(6);
	pub const ImagesJavascriptJavaandactivex: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(7);
	pub const Cookies: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(8);
	pub const CookiesJavaandactivex: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(9);
	pub const CookiesJavascript: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(10);
	pub const CookiesJavascriptJavaandactivex: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(11);
	pub const CookiesImages: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(12);
	pub const CookiesImagesJavaandactivex: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(13);
	pub const CookiesImagesJavascript: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(14);
	pub const CookiesImagesJavascriptJavaandactivex: Cvpn3000WebvpnContentFilterParameters = Cvpn3000WebvpnContentFilterParameters(15);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000StripRealm(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000StripRealm {
	pub const False: Cvpn3000StripRealm = Cvpn3000StripRealm(0);
	pub const True: Cvpn3000StripRealm = Cvpn3000StripRealm(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cvpn3000SmartTunnelAuto(pub u32);
 
#[allow(non_upper_case_globals)]
impl Cvpn3000SmartTunnelAuto {
	pub const Disabled: Cvpn3000SmartTunnelAuto = Cvpn3000SmartTunnelAuto(0);
	pub const Enabled: Cvpn3000SmartTunnelAuto = Cvpn3000SmartTunnelAuto(1);
	pub const Auto: Cvpn3000SmartTunnelAuto = Cvpn3000SmartTunnelAuto(2);
}

