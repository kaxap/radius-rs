/// Definitions for vendor Cisco-ASA, vendor value 3076
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaAuthorizationRequired(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaAuthorizationRequired {
	pub const No: AsaAuthorizationRequired = AsaAuthorizationRequired(0);
	pub const Yes: AsaAuthorizationRequired = AsaAuthorizationRequired(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaAuthorizationType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaAuthorizationType {
	pub const None: AsaAuthorizationType = AsaAuthorizationType(0);
	pub const Radius: AsaAuthorizationType = AsaAuthorizationType(1);
	pub const Ldap: AsaAuthorizationType = AsaAuthorizationType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaCiscoIpPhoneBypass(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaCiscoIpPhoneBypass {
	pub const Disabled: AsaCiscoIpPhoneBypass = AsaCiscoIpPhoneBypass(0);
	pub const Enabled: AsaCiscoIpPhoneBypass = AsaCiscoIpPhoneBypass(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaCiscoLeapBypass(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaCiscoLeapBypass {
	pub const Disabled: AsaCiscoLeapBypass = AsaCiscoLeapBypass(0);
	pub const Enabled: AsaCiscoLeapBypass = AsaCiscoLeapBypass(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaClienttype(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaClienttype {
	pub const CiscoVpnClientIkev1: AsaClienttype = AsaClienttype(1);
	pub const AnyconnectClientSslVpn: AsaClienttype = AsaClienttype(2);
	pub const ClientlessSslVpn: AsaClienttype = AsaClienttype(3);
	pub const CutThroughProxy: AsaClienttype = AsaClienttype(4);
	pub const L2TpOrIpsecSslVpn: AsaClienttype = AsaClienttype(5);
	pub const AnyconnectClientIpsecVpnIkev2: AsaClienttype = AsaClienttype(6);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaExtendedAuthenticationOnRekey(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaExtendedAuthenticationOnRekey {
	pub const Disabled: AsaExtendedAuthenticationOnRekey = AsaExtendedAuthenticationOnRekey(0);
	pub const Enabled: AsaExtendedAuthenticationOnRekey = AsaExtendedAuthenticationOnRekey(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIeProxyBypassLocal(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIeProxyBypassLocal {
	pub const None: AsaIeProxyBypassLocal = AsaIeProxyBypassLocal(0);
	pub const Local: AsaIeProxyBypassLocal = AsaIeProxyBypassLocal(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIeProxyServerPolicy(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIeProxyServerPolicy {
	pub const NoModify: AsaIeProxyServerPolicy = AsaIeProxyServerPolicy(1);
	pub const NoProxy: AsaIeProxyServerPolicy = AsaIeProxyServerPolicy(2);
	pub const AutoDetect: AsaIeProxyServerPolicy = AsaIeProxyServerPolicy(3);
	pub const UseConcentratorSetting: AsaIeProxyServerPolicy = AsaIeProxyServerPolicy(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIkeKeepAlives(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIkeKeepAlives {
	pub const Disabled: AsaIkeKeepAlives = AsaIkeKeepAlives(0);
	pub const Enabled: AsaIkeKeepAlives = AsaIkeKeepAlives(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaAllowNetworkExtensionMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaAllowNetworkExtensionMode {
	pub const Disabled: AsaAllowNetworkExtensionMode = AsaAllowNetworkExtensionMode(0);
	pub const Enabled: AsaAllowNetworkExtensionMode = AsaAllowNetworkExtensionMode(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaInterceptDhcpConfigureMsg(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaInterceptDhcpConfigureMsg {
	pub const Disabled: AsaInterceptDhcpConfigureMsg = AsaInterceptDhcpConfigureMsg(0);
	pub const Enabled: AsaInterceptDhcpConfigureMsg = AsaInterceptDhcpConfigureMsg(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecAllowPasswdStore(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecAllowPasswdStore {
	pub const Disabled: AsaIpsecAllowPasswdStore = AsaIpsecAllowPasswdStore(0);
	pub const Enabled: AsaIpsecAllowPasswdStore = AsaIpsecAllowPasswdStore(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecAuthentication(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecAuthentication {
	pub const None: AsaIpsecAuthentication = AsaIpsecAuthentication(0);
	pub const Radius: AsaIpsecAuthentication = AsaIpsecAuthentication(1);
	pub const LdapAuthorizationOnly: AsaIpsecAuthentication = AsaIpsecAuthentication(2);
	pub const NtDomain: AsaIpsecAuthentication = AsaIpsecAuthentication(3);
	pub const Sdi: AsaIpsecAuthentication = AsaIpsecAuthentication(4);
	pub const Internal: AsaIpsecAuthentication = AsaIpsecAuthentication(5);
	pub const RadiusWithExpiry: AsaIpsecAuthentication = AsaIpsecAuthentication(6);
	pub const KerberosOrActiveDirectory: AsaIpsecAuthentication = AsaIpsecAuthentication(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecAuthOnRekey(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecAuthOnRekey {
	pub const Disabled: AsaIpsecAuthOnRekey = AsaIpsecAuthOnRekey(0);
	pub const Enabled: AsaIpsecAuthOnRekey = AsaIpsecAuthOnRekey(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecBackupServers(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecBackupServers {
	pub const UseClientConfiguredList: AsaIpsecBackupServers = AsaIpsecBackupServers(1);
	pub const DisableAndClearClientList: AsaIpsecBackupServers = AsaIpsecBackupServers(2);
	pub const UseBackupServerList: AsaIpsecBackupServers = AsaIpsecBackupServers(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecClientFirewallFilterOptional(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecClientFirewallFilterOptional {
	pub const Required: AsaIpsecClientFirewallFilterOptional = AsaIpsecClientFirewallFilterOptional(0);
	pub const Optional: AsaIpsecClientFirewallFilterOptional = AsaIpsecClientFirewallFilterOptional(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecIkePeerIdCheck(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecIkePeerIdCheck {
	pub const Required: AsaIpsecIkePeerIdCheck = AsaIpsecIkePeerIdCheck(1);
	pub const IfSupportedByPeerCertificate: AsaIpsecIkePeerIdCheck = AsaIpsecIkePeerIdCheck(2);
	pub const DoNotCheck: AsaIpsecIkePeerIdCheck = AsaIpsecIkePeerIdCheck(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecIpCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecIpCompression {
	pub const Disabled: AsaIpsecIpCompression = AsaIpsecIpCompression(0);
	pub const Enabled: AsaIpsecIpCompression = AsaIpsecIpCompression(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecModeConfig(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecModeConfig {
	pub const Disabled: AsaIpsecModeConfig = AsaIpsecModeConfig(0);
	pub const Enabled: AsaIpsecModeConfig = AsaIpsecModeConfig(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecOverUdp(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecOverUdp {
	pub const Disabled: AsaIpsecOverUdp = AsaIpsecOverUdp(0);
	pub const Enabled: AsaIpsecOverUdp = AsaIpsecOverUdp(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecRequiredClientFirewallCapability(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecRequiredClientFirewallCapability {
	pub const None: AsaIpsecRequiredClientFirewallCapability = AsaIpsecRequiredClientFirewallCapability(0);
	pub const PolicyRemotelyDefined: AsaIpsecRequiredClientFirewallCapability = AsaIpsecRequiredClientFirewallCapability(1);
	pub const PolicyPushed: AsaIpsecRequiredClientFirewallCapability = AsaIpsecRequiredClientFirewallCapability(2);
	pub const PolicyFromServer: AsaIpsecRequiredClientFirewallCapability = AsaIpsecRequiredClientFirewallCapability(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecSplitTunnelingPolicy(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecSplitTunnelingPolicy {
	pub const NoSplitTunneling: AsaIpsecSplitTunnelingPolicy = AsaIpsecSplitTunnelingPolicy(0);
	pub const SplitTunneling: AsaIpsecSplitTunnelingPolicy = AsaIpsecSplitTunnelingPolicy(1);
	pub const LocalLanPermitted: AsaIpsecSplitTunnelingPolicy = AsaIpsecSplitTunnelingPolicy(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaIpsecTunnelType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaIpsecTunnelType {
	pub const LanToLan: AsaIpsecTunnelType = AsaIpsecTunnelType(1);
	pub const RemoteAccess: AsaIpsecTunnelType = AsaIpsecTunnelType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaL2TpMppcCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaL2TpMppcCompression {
	pub const Disabled: AsaL2TpMppcCompression = AsaL2TpMppcCompression(0);
	pub const Enabled: AsaL2TpMppcCompression = AsaL2TpMppcCompression(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaNacEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaNacEnable {
	pub const No: AsaNacEnable = AsaNacEnable(0);
	pub const Yes: AsaNacEnable = AsaNacEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaPerfectForwardSecrecyEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaPerfectForwardSecrecyEnable {
	pub const No: AsaPerfectForwardSecrecyEnable = AsaPerfectForwardSecrecyEnable(0);
	pub const Yes: AsaPerfectForwardSecrecyEnable = AsaPerfectForwardSecrecyEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaPptpMppcCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaPptpMppcCompression {
	pub const Disabled: AsaPptpMppcCompression = AsaPptpMppcCompression(0);
	pub const Enabled: AsaPptpMppcCompression = AsaPptpMppcCompression(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaRequiredClientFirewallVendorCode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaRequiredClientFirewallVendorCode {
	pub const CiscoCic: AsaRequiredClientFirewallVendorCode = AsaRequiredClientFirewallVendorCode(1);
	pub const ZoneLabs: AsaRequiredClientFirewallVendorCode = AsaRequiredClientFirewallVendorCode(2);
	pub const Networkice: AsaRequiredClientFirewallVendorCode = AsaRequiredClientFirewallVendorCode(3);
	pub const Sygate: AsaRequiredClientFirewallVendorCode = AsaRequiredClientFirewallVendorCode(4);
	pub const CiscoIpsa: AsaRequiredClientFirewallVendorCode = AsaRequiredClientFirewallVendorCode(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaRequiredIndividualUserAuth(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaRequiredIndividualUserAuth {
	pub const Disabled: AsaRequiredIndividualUserAuth = AsaRequiredIndividualUserAuth(0);
	pub const Enabled: AsaRequiredIndividualUserAuth = AsaRequiredIndividualUserAuth(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaRequireHwClientAuth(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaRequireHwClientAuth {
	pub const Disabled: AsaRequireHwClientAuth = AsaRequireHwClientAuth(0);
	pub const Enabled: AsaRequireHwClientAuth = AsaRequireHwClientAuth(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaSessionsubtype(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaSessionsubtype {
	pub const None: AsaSessionsubtype = AsaSessionsubtype(0);
	pub const Clientless: AsaSessionsubtype = AsaSessionsubtype(1);
	pub const Client: AsaSessionsubtype = AsaSessionsubtype(2);
	pub const ClientOnly: AsaSessionsubtype = AsaSessionsubtype(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaSessiontype(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaSessiontype {
	pub const None: AsaSessiontype = AsaSessiontype(0);
	pub const AnyconnectClientSslVpn: AsaSessiontype = AsaSessiontype(1);
	pub const AnyconnectClientIpsecVpnOrIkev2: AsaSessiontype = AsaSessiontype(2);
	pub const ClientlessSslVpn: AsaSessiontype = AsaSessiontype(3);
	pub const ClientlessEmailProxy: AsaSessiontype = AsaSessiontype(4);
	pub const CiscoVpnClientOrIkev1: AsaSessiontype = AsaSessiontype(5);
	pub const Ikev1LanToLan: AsaSessiontype = AsaSessiontype(6);
	pub const Ikev2LanToLan: AsaSessiontype = AsaSessiontype(7);
	pub const VpnLoadBalancing: AsaSessiontype = AsaSessiontype(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaSmartTunnelAuto(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaSmartTunnelAuto {
	pub const Disabled: AsaSmartTunnelAuto = AsaSmartTunnelAuto(0);
	pub const Enabled: AsaSmartTunnelAuto = AsaSmartTunnelAuto(1);
	pub const Autostart: AsaSmartTunnelAuto = AsaSmartTunnelAuto(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaStripRealm(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaStripRealm {
	pub const Disabled: AsaStripRealm = AsaStripRealm(0);
	pub const Enabled: AsaStripRealm = AsaStripRealm(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaSvcAsk(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaSvcAsk {
	pub const Disabled: AsaSvcAsk = AsaSvcAsk(0);
	pub const Enabled: AsaSvcAsk = AsaSvcAsk(1);
	pub const EnableDefaultService: AsaSvcAsk = AsaSvcAsk(3);
	pub const EnableDefaultClientless: AsaSvcAsk = AsaSvcAsk(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaSvcDtls(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaSvcDtls {
	pub const False: AsaSvcDtls = AsaSvcDtls(0);
	pub const True: AsaSvcDtls = AsaSvcDtls(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaUseClientAddress(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaUseClientAddress {
	pub const Disabled: AsaUseClientAddress = AsaUseClientAddress(0);
	pub const Enabled: AsaUseClientAddress = AsaUseClientAddress(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnApplyAcl(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnApplyAcl {
	pub const Disabled: AsaWebvpnApplyAcl = AsaWebvpnApplyAcl(0);
	pub const Enabled: AsaWebvpnApplyAcl = AsaWebvpnApplyAcl(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnCitrixMetaframeEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnCitrixMetaframeEnable {
	pub const Disabled: AsaWebvpnCitrixMetaframeEnable = AsaWebvpnCitrixMetaframeEnable(0);
	pub const Enabled: AsaWebvpnCitrixMetaframeEnable = AsaWebvpnCitrixMetaframeEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnFileAccessEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnFileAccessEnable {
	pub const Disabled: AsaWebvpnFileAccessEnable = AsaWebvpnFileAccessEnable(0);
	pub const Enabled: AsaWebvpnFileAccessEnable = AsaWebvpnFileAccessEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnFileServerBrowsingEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnFileServerBrowsingEnable {
	pub const Disabled: AsaWebvpnFileServerBrowsingEnable = AsaWebvpnFileServerBrowsingEnable(0);
	pub const Enabled: AsaWebvpnFileServerBrowsingEnable = AsaWebvpnFileServerBrowsingEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnFileServerEntryEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnFileServerEntryEnable {
	pub const Disabled: AsaWebvpnFileServerEntryEnable = AsaWebvpnFileServerEntryEnable(0);
	pub const Enabled: AsaWebvpnFileServerEntryEnable = AsaWebvpnFileServerEntryEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnHiddenShares(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnHiddenShares {
	pub const None: AsaWebvpnHiddenShares = AsaWebvpnHiddenShares(0);
	pub const Visible: AsaWebvpnHiddenShares = AsaWebvpnHiddenShares(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnHttpCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnHttpCompression {
	pub const Off: AsaWebvpnHttpCompression = AsaWebvpnHttpCompression(0);
	pub const DeflateCompression: AsaWebvpnHttpCompression = AsaWebvpnHttpCompression(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnPortForwardingEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnPortForwardingEnable {
	pub const Disabled: AsaWebvpnPortForwardingEnable = AsaWebvpnPortForwardingEnable(0);
	pub const Enabled: AsaWebvpnPortForwardingEnable = AsaWebvpnPortForwardingEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnPortForwardingExchangeProxyEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnPortForwardingExchangeProxyEnable {
	pub const Disabled: AsaWebvpnPortForwardingExchangeProxyEnable = AsaWebvpnPortForwardingExchangeProxyEnable(0);
	pub const Enabled: AsaWebvpnPortForwardingExchangeProxyEnable = AsaWebvpnPortForwardingExchangeProxyEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnPortForwardingHttpProxy(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnPortForwardingHttpProxy {
	pub const Disabled: AsaWebvpnPortForwardingHttpProxy = AsaWebvpnPortForwardingHttpProxy(0);
	pub const Enabled: AsaWebvpnPortForwardingHttpProxy = AsaWebvpnPortForwardingHttpProxy(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnsmartCardRemovalDisconnect(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnsmartCardRemovalDisconnect {
	pub const Disabled: AsaWebvpnsmartCardRemovalDisconnect = AsaWebvpnsmartCardRemovalDisconnect(0);
	pub const Enabled: AsaWebvpnsmartCardRemovalDisconnect = AsaWebvpnsmartCardRemovalDisconnect(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnSmartTunnelAutoStart(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnSmartTunnelAutoStart {
	pub const Disabled: AsaWebvpnSmartTunnelAutoStart = AsaWebvpnSmartTunnelAutoStart(0);
	pub const Enabled: AsaWebvpnSmartTunnelAutoStart = AsaWebvpnSmartTunnelAutoStart(1);
	pub const Autostart: AsaWebvpnSmartTunnelAutoStart = AsaWebvpnSmartTunnelAutoStart(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnSslVpnClientEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnSslVpnClientEnable {
	pub const Disabled: AsaWebvpnSslVpnClientEnable = AsaWebvpnSslVpnClientEnable(0);
	pub const Enabled: AsaWebvpnSslVpnClientEnable = AsaWebvpnSslVpnClientEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnSslVpnClientKeepInstallation(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnSslVpnClientKeepInstallation {
	pub const Disabled: AsaWebvpnSslVpnClientKeepInstallation = AsaWebvpnSslVpnClientKeepInstallation(0);
	pub const Enabled: AsaWebvpnSslVpnClientKeepInstallation = AsaWebvpnSslVpnClientKeepInstallation(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnSslVpnClientRequired(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnSslVpnClientRequired {
	pub const Disabled: AsaWebvpnSslVpnClientRequired = AsaWebvpnSslVpnClientRequired(0);
	pub const Enabled: AsaWebvpnSslVpnClientRequired = AsaWebvpnSslVpnClientRequired(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnSvcDtlsEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnSvcDtlsEnable {
	pub const Disabled: AsaWebvpnSvcDtlsEnable = AsaWebvpnSvcDtlsEnable(0);
	pub const Enabled: AsaWebvpnSvcDtlsEnable = AsaWebvpnSvcDtlsEnable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnSvcRekeyMethod(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnSvcRekeyMethod {
	pub const Off: AsaWebvpnSvcRekeyMethod = AsaWebvpnSvcRekeyMethod(0);
	pub const Ssl: AsaWebvpnSvcRekeyMethod = AsaWebvpnSvcRekeyMethod(1);
	pub const NewTunnel: AsaWebvpnSvcRekeyMethod = AsaWebvpnSvcRekeyMethod(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnSvcCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnSvcCompression {
	pub const Off: AsaWebvpnSvcCompression = AsaWebvpnSvcCompression(0);
	pub const DeflateCompression: AsaWebvpnSvcCompression = AsaWebvpnSvcCompression(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsaWebvpnUrlEntryEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl AsaWebvpnUrlEntryEnable {
	pub const Disabled: AsaWebvpnUrlEntryEnable = AsaWebvpnUrlEntryEnable(0);
	pub const Enabled: AsaWebvpnUrlEntryEnable = AsaWebvpnUrlEntryEnable(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		2 => map!{i, be_u32, |v| Attribute::VsaAsaSimultaneousLogins(v)},
		5 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAsaPrimaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		6 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAsaSecondaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		7 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAsaPrimaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		8 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAsaSecondaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		9 => map!{i, be_u32, |v| Attribute::VsaAsaSepCardAssignment(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaAsaTunnelingProtocols(v)},
		12 => value!(i, Attribute::VsaAsaIpsecSecAssociation(i)),
		13 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecAuthentication(AsaIpsecAuthentication(v))},
		15 => value!(i, Attribute::VsaAsaBanner1(i)),
		16 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecAllowPasswdStore(AsaIpsecAllowPasswdStore(v))},
		17 => map! {i, be_u32, |v| Attribute::VsaAsaUseClientAddress(AsaUseClientAddress(v))},
		20 => map!{i, be_u32, |v| Attribute::VsaAsaPptpEncryption(v)},
		21 => map!{i, be_u32, |v| Attribute::VsaAsaL2TpEncryption(v)},
		25 => value!(i, Attribute::VsaAsaGroupPolicy(i)),
		27 => value!(i, Attribute::VsaAsaIpsecSplitTunnelList(i)),
		28 => value!(i, Attribute::VsaAsaIpsecDefaultDomain(i)),
		29 => value!(i, Attribute::VsaAsaIpsecSplitDnsNames(i)),
		30 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecTunnelType(AsaIpsecTunnelType(v))},
		31 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecModeConfig(AsaIpsecModeConfig(v))},
		34 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecOverUdp(AsaIpsecOverUdp(v))},
		35 => map!{i, be_u32, |v| Attribute::VsaAsaIpsecOverUdpPort(v)},
		36 => value!(i, Attribute::VsaAsaBanner2(i)),
		37 => map! {i, be_u32, |v| Attribute::VsaAsaPptpMppcCompression(AsaPptpMppcCompression(v))},
		38 => map! {i, be_u32, |v| Attribute::VsaAsaL2TpMppcCompression(AsaL2TpMppcCompression(v))},
		39 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecIpCompression(AsaIpsecIpCompression(v))},
		40 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecIkePeerIdCheck(AsaIpsecIkePeerIdCheck(v))},
		41 => map! {i, be_u32, |v| Attribute::VsaAsaIkeKeepAlives(AsaIkeKeepAlives(v))},
		42 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecAuthOnRekey(AsaIpsecAuthOnRekey(v))},
		45 => map! {i, be_u32, |v| Attribute::VsaAsaRequiredClientFirewallVendorCode(AsaRequiredClientFirewallVendorCode(v))},
		46 => map!{i, be_u32, |v| Attribute::VsaAsaRequiredClientFirewallProductCode(v)},
		47 => value!(i, Attribute::VsaAsaRequiredClientFirewallDescription(i)),
		48 => map! {i, be_u32, |v| Attribute::VsaAsaRequireHwClientAuth(AsaRequireHwClientAuth(v))},
		49 => map! {i, be_u32, |v| Attribute::VsaAsaRequiredIndividualUserAuth(AsaRequiredIndividualUserAuth(v))},
		50 => map!{i, be_u32, |v| Attribute::VsaAsaAuthenticatedUserIdleTimeout(v)},
		51 => map! {i, be_u32, |v| Attribute::VsaAsaCiscoIpPhoneBypass(AsaCiscoIpPhoneBypass(v))},
		55 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecSplitTunnelingPolicy(AsaIpsecSplitTunnelingPolicy(v))},
		56 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecRequiredClientFirewallCapability(AsaIpsecRequiredClientFirewallCapability(v))},
		57 => value!(i, Attribute::VsaAsaIpsecClientFirewallFilterName(i)),
		58 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecClientFirewallFilterOptional(AsaIpsecClientFirewallFilterOptional(v))},
		59 => map! {i, be_u32, |v| Attribute::VsaAsaIpsecBackupServers(AsaIpsecBackupServers(v))},
		60 => value!(i, Attribute::VsaAsaIpsecBackupServerList(i)),
		61 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAsaDhcpNetworkScope(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		62 => map! {i, be_u32, |v| Attribute::VsaAsaInterceptDhcpConfigureMsg(AsaInterceptDhcpConfigureMsg(v))},
		63 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAsaMsClientSubnetMask(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		64 => map! {i, be_u32, |v| Attribute::VsaAsaAllowNetworkExtensionMode(AsaAllowNetworkExtensionMode(v))},
		65 => map! {i, be_u32, |v| Attribute::VsaAsaAuthorizationType(AsaAuthorizationType(v))},
		66 => map! {i, be_u32, |v| Attribute::VsaAsaAuthorizationRequired(AsaAuthorizationRequired(v))},
		67 => value!(i, Attribute::VsaAsaAuthorizationDnField(i)),
		68 => map!{i, be_u32, |v| Attribute::VsaAsaIkeKeepaliveConfidenceInterval(v)},
		69 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnContentFilterParameters(v)},
		70 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnHtmlFilter(v)},
		71 => value!(i, Attribute::VsaAsaWebvpnUrlList(i)),
		72 => value!(i, Attribute::VsaAsaWebvpnPortForwardingList(i)),
		73 => value!(i, Attribute::VsaAsaWebvpnAccessList(i)),
		74 => value!(i, Attribute::VsaAsaWebvpnHttpProxyIpAddress(i)),
		75 => map! {i, be_u32, |v| Attribute::VsaAsaCiscoLeapBypass(AsaCiscoLeapBypass(v))},
		76 => value!(i, Attribute::VsaAsaWebvpnDefaultHomepage(i)),
		77 => value!(i, Attribute::VsaAsaClientTypeVersionLimiting(i)),
		78 => value!(i, Attribute::VsaAsaWebvpnGroupBasedHttpOrHttpsProxyExceptionList(i)),
		79 => value!(i, Attribute::VsaAsaWebvpnPortForwardingName(i)),
		80 => value!(i, Attribute::VsaAsaIeProxyServer(i)),
		81 => map! {i, be_u32, |v| Attribute::VsaAsaIeProxyServerPolicy(AsaIeProxyServerPolicy(v))},
		82 => value!(i, Attribute::VsaAsaIeProxyExceptionList(i)),
		83 => map! {i, be_u32, |v| Attribute::VsaAsaIeProxyBypassLocal(AsaIeProxyBypassLocal(v))},
		84 => map!{i, be_u32, |v| Attribute::VsaAsaIkeKeepaliveRetryInterval(v)},
		85 => value!(i, Attribute::VsaAsaTunnelGroupLock(i)),
		86 => value!(i, Attribute::VsaAsaAccessListInbound(i)),
		87 => value!(i, Attribute::VsaAsaAccessListOutbound(i)),
		88 => map! {i, be_u32, |v| Attribute::VsaAsaPerfectForwardSecrecyEnable(AsaPerfectForwardSecrecyEnable(v))},
		89 => map! {i, be_u32, |v| Attribute::VsaAsaNacEnable(AsaNacEnable(v))},
		90 => map!{i, be_u32, |v| Attribute::VsaAsaNacStatusQueryTimer(v)},
		91 => map!{i, be_u32, |v| Attribute::VsaAsaNacRevalidationTimer(v)},
		92 => value!(i, Attribute::VsaAsaNacDefaultAcl(i)),
		93 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnUrlEntryEnable(AsaWebvpnUrlEntryEnable(v))},
		94 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnFileAccessEnable(AsaWebvpnFileAccessEnable(v))},
		95 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnFileServerEntryEnable(AsaWebvpnFileServerEntryEnable(v))},
		96 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnFileServerBrowsingEnable(AsaWebvpnFileServerBrowsingEnable(v))},
		97 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnPortForwardingEnable(AsaWebvpnPortForwardingEnable(v))},
		98 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnPortForwardingExchangeProxyEnable(AsaWebvpnPortForwardingExchangeProxyEnable(v))},
		99 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnPortForwardingHttpProxy(AsaWebvpnPortForwardingHttpProxy(v))},
		101 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnCitrixMetaframeEnable(AsaWebvpnCitrixMetaframeEnable(v))},
		102 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnApplyAcl(AsaWebvpnApplyAcl(v))},
		103 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnSslVpnClientEnable(AsaWebvpnSslVpnClientEnable(v))},
		104 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnSslVpnClientRequired(AsaWebvpnSslVpnClientRequired(v))},
		105 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnSslVpnClientKeepInstallation(AsaWebvpnSslVpnClientKeepInstallation(v))},
		107 => map!{i, be_u32, |v| Attribute::VsaAsaSvcKeepalive(v)},
		108 => map!{i, be_u32, |v| Attribute::VsaAsaSvcDpdIntervalClient(v)},
		109 => map!{i, be_u32, |v| Attribute::VsaAsaSvcDpdIntervalGateway(v)},
		110 => map!{i, be_u32, |v| Attribute::VsaAsaSvcRekeyTime(v)},
		111 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnSvcRekeyMethod(AsaWebvpnSvcRekeyMethod(v))},
		112 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnSvcCompression(AsaWebvpnSvcCompression(v))},
		113 => value!(i, Attribute::VsaAsaWebvpnCustomization(i)),
		114 => value!(i, Attribute::VsaAsaWebvpnSsoServerName(i)),
		116 => value!(i, Attribute::VsaAsaWebvpnDenyMessage(i)),
		120 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnHttpCompression(AsaWebvpnHttpCompression(v))},
		121 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnKeepaliveIgnore(v)},
		122 => map! {i, be_u32, |v| Attribute::VsaAsaExtendedAuthenticationOnRekey(AsaExtendedAuthenticationOnRekey(v))},
		123 => map! {i, be_u32, |v| Attribute::VsaAsaSvcDtls(AsaSvcDtls(v))},
		124 => value!(i, Attribute::VsaAsaWebvpnAutoHttpSignon(i)),
		125 => map!{i, be_u32, |v| Attribute::VsaAsaSvcMtu(v)},
		126 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnHiddenShares(AsaWebvpnHiddenShares(v))},
		127 => value!(i, Attribute::VsaAsaSvcModules(i)),
		128 => value!(i, Attribute::VsaAsaSvcProfiles(i)),
		131 => map! {i, be_u32, |v| Attribute::VsaAsaSvcAsk(AsaSvcAsk(v))},
		132 => map!{i, be_u32, |v| Attribute::VsaAsaSvcAskTimeout(v)},
		133 => value!(i, Attribute::VsaAsaIeProxyPacUrl(i)),
		135 => map! {i, be_u32, |v| Attribute::VsaAsaStripRealm(AsaStripRealm(v))},
		136 => value!(i, Attribute::VsaAsaSmartTunnel(i)),
		137 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnActivexRelay(v)},
		138 => map! {i, be_u32, |v| Attribute::VsaAsaSmartTunnelAuto(AsaSmartTunnelAuto(v))},
		139 => value!(i, Attribute::VsaAsaSmartTunnelAutoSignonEnable(i)),
		140 => map!{i, be_u32, |v| Attribute::VsaAsaVlan(v)},
		141 => value!(i, Attribute::VsaAsaNacSettings(i)),
		145 => value!(i, Attribute::VsaAsaMemberOf(i)),
		146 => value!(i, Attribute::VsaAsaTunnelgroupname(i)),
		148 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnIdleTimeoutAlertInterval(v)},
		149 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnSessionTimeoutAlertInterval(v)},
		150 => map! {i, be_u32, |v| Attribute::VsaAsaClienttype(AsaClienttype(v))},
		151 => map! {i, be_u32, |v| Attribute::VsaAsaSessiontype(AsaSessiontype(v))},
		152 => map! {i, be_u32, |v| Attribute::VsaAsaSessionsubtype(AsaSessionsubtype(v))},
		157 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnDownloadMaxSize(v)},
		158 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnUploadMaxSize(v)},
		159 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnPostMaxSize(v)},
		160 => value!(i, Attribute::VsaAsaWebvpnUserStorage(i)),
		161 => value!(i, Attribute::VsaAsaWebvpnStorageObjects(i)),
		162 => value!(i, Attribute::VsaAsaWebvpnStorageKey(i)),
		163 => value!(i, Attribute::VsaAsaWebvpnVdi(i)),
		217 => value!(i, Attribute::VsaAsaAddressPools(i)),
		218 => value!(i, Attribute::VsaAsaIpv6AddressPools(i)),
		219 => value!(i, Attribute::VsaAsaIpv6VpnFilter(i)),
		220 => map!{i, be_u32, |v| Attribute::VsaAsaPrivilegeLevel(v)},
		221 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnUnixUserId(v)},
		222 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnUnixGroupId(v)},
		223 => value!(i, Attribute::VsaAsaWebvpnMacroSubstitutionValue1(i)),
		224 => value!(i, Attribute::VsaAsaWebvpnMacroSubstitutionValue2(i)),
		225 => map! {i, be_u32, |v| Attribute::VsaAsaWebvpnsmartCardRemovalDisconnect(AsaWebvpnsmartCardRemovalDisconnect(v))},
		227 => value!(i, Attribute::VsaAsaWebvpnSmartTunnelTunnelPolicy(i)),
		228 => map!{i, be_u32, |v| Attribute::VsaAsaWebvpnHomePageUseSmartTunnel(v)},
        _ => value!(i, Attribute::VsaUnknown(3076, typ, i)),
    }
}
