/// Definitions for vendor WiMAX, vendor value 24757
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WimaxIpTechnology(pub u32);
 
#[allow(non_upper_case_globals)]
impl WimaxIpTechnology {
	pub const Reserved0: WimaxIpTechnology = WimaxIpTechnology(0);
	pub const Reserved1: WimaxIpTechnology = WimaxIpTechnology(1);
	pub const Pmip4: WimaxIpTechnology = WimaxIpTechnology(2);
	pub const Cmip4: WimaxIpTechnology = WimaxIpTechnology(3);
	pub const Cmip6: WimaxIpTechnology = WimaxIpTechnology(4);
	pub const EthernetCs: WimaxIpTechnology = WimaxIpTechnology(5);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WimaxSessionTerminationCapability(pub u32);
 
#[allow(non_upper_case_globals)]
impl WimaxSessionTerminationCapability {
	pub const DynamicAuthorization: WimaxSessionTerminationCapability = WimaxSessionTerminationCapability(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WimaxHaRkKeyRequested(pub u32);
 
#[allow(non_upper_case_globals)]
impl WimaxHaRkKeyRequested {
	pub const No: WimaxHaRkKeyRequested = WimaxHaRkKeyRequested(0);
	pub const Yes: WimaxHaRkKeyRequested = WimaxHaRkKeyRequested(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WimaxDmActionCode(pub u32);
 
#[allow(non_upper_case_globals)]
impl WimaxDmActionCode {
	pub const DeregisterMs: WimaxDmActionCode = WimaxDmActionCode(0);
	pub const SuspendMsTraffic: WimaxDmActionCode = WimaxDmActionCode(1);
	pub const SuspendUserTraffic: WimaxDmActionCode = WimaxDmActionCode(2);
	pub const ResumeTraffic: WimaxDmActionCode = WimaxDmActionCode(3);
	pub const MsTerminate: WimaxDmActionCode = WimaxDmActionCode(4);
	pub const MsIdle: WimaxDmActionCode = WimaxDmActionCode(5);
	pub const MsCompletedIpv6Handover: WimaxDmActionCode = WimaxDmActionCode(6);
	pub const BsSendsResCmd: WimaxDmActionCode = WimaxDmActionCode(0xffff);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaWimaxCapability(i)),
		2 => map!{i, be_u8, |v| Attribute::VsaWimaxDeviceAuthenticationIndicator(v)},
		3 => value!(i, Attribute::VsaWimaxGmtTimezoneOffset(i)),
		4 => value!(i, Attribute::VsaWimaxAaaSessionId(i)),
		5 => value!(i, Attribute::VsaWimaxMsk(i)),
		6 => map!{i, take!(4), |v:&[u8]| Attribute::VsaWimaxHhaIpMip4(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		7 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaWimaxHhaIpMip6(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		8 => value!(i, Attribute::VsaWimaxDhcpv4Server(i)),
		9 => value!(i, Attribute::VsaWimaxDhcpv6Server(i)),
		10 => value!(i, Attribute::VsaWimaxMnHhaMip4Key(i)),
		11 => map!{i, be_u32, |v| Attribute::VsaWimaxMnHhaMip4Spi(v)},
		12 => value!(i, Attribute::VsaWimaxMnHhaMip6Key(i)),
		13 => map!{i, be_u32, |v| Attribute::VsaWimaxMnHhaMip6Spi(v)},
		14 => value!(i, Attribute::VsaWimaxFaRkKey(i)),
		15 => value!(i, Attribute::VsaWimaxHaRkKey(i)),
		16 => map!{i, be_u32, |v| Attribute::VsaWimaxHaRkSpi(v)},
		17 => map!{i, be_u32, |v| Attribute::VsaWimaxHaRkLifetime(v)},
		18 => value!(i, Attribute::VsaWimaxRrqHaIp(i)),
		19 => value!(i, Attribute::VsaWimaxRrqMnHaKey(i)),
		20 => map!{i, be_u32, |v| Attribute::VsaWimaxRrqMnHaSpi(v)},
		21 => map!{i, be_u32, |v| Attribute::VsaWimaxSessionContinue(v)},
		22 => map!{i, be_u32, |v| Attribute::VsaWimaxBeginningOfSession(v)},
		23 => map! {i, be_u32, |v| Attribute::VsaWimaxIpTechnology(WimaxIpTechnology(v))},
		24 => value!(i, Attribute::VsaWimaxHotlineIndicator(i)),
		25 => map!{i, be_u8, |v| Attribute::VsaWimaxPrepaidIndicator(v)},
		26 => map!{i, be_u16, |v| Attribute::VsaWimaxPdfid(v)},
		27 => map!{i, be_u16, |v| Attribute::VsaWimaxSdfid(v)},
		28 => value!(i, Attribute::VsaWimaxPacketFlowDescriptor(i)),
		29 => value!(i, Attribute::VsaWimaxQosDescriptor(i)),
		30 => value!(i, Attribute::VsaWimaxUplinkGrantedQos(i)),
		31 => map!{i, be_u32, |v| Attribute::VsaWimaxControlPacketsIn(v)},
		32 => map!{i, be_u32, |v| Attribute::VsaWimaxControlOctetsIn(v)},
		33 => map!{i, be_u32, |v| Attribute::VsaWimaxControlPacketsOut(v)},
		34 => map!{i, be_u32, |v| Attribute::VsaWimaxControlOctetsOut(v)},
		35 => value!(i, Attribute::VsaWimaxPpac(i)),
		36 => map! {i, be_u32, |v| Attribute::VsaWimaxSessionTerminationCapability(WimaxSessionTerminationCapability(v))},
		37 => value!(i, Attribute::VsaWimaxPpaq(i)),
		38 => value!(i, Attribute::VsaWimaxPrepaidTariffSwitching(i)),
		39 => map!{i, be_u32, |v| Attribute::VsaWimaxActiveTimeDuration(v)},
		40 => value!(i, Attribute::VsaWimaxDhcpRk(i)),
		41 => map!{i, be_u32, |v| Attribute::VsaWimaxDhcpRkKeyId(v)},
		42 => map!{i, be_u32, |v| Attribute::VsaWimaxDhcpRkLifetime(v)},
		43 => map!{i, take!(4), |v:&[u8]| Attribute::VsaWimaxDhcpMsgServerIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		44 => map!{i, be_u8, |v| Attribute::VsaWimaxIdleModeTransition(v)},
		45 => value!(i, Attribute::VsaWimaxNapId(i)),
		46 => value!(i, Attribute::VsaWimaxBsId(i)),
		47 => value!(i, Attribute::VsaWimaxLocation(i)),
		48 => map!{i, be_u32, |v| Attribute::VsaWimaxAcctInputPacketsGigaword(v)},
		49 => map!{i, be_u32, |v| Attribute::VsaWimaxAcctOutputPacketsGigaword(v)},
		50 => value!(i, Attribute::VsaWimaxUplinkFlowDescription(i)),
		51 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaWimaxBluCoaIpv6(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		52 => value!(i, Attribute::VsaWimaxDnsServer(i)),
		53 => value!(i, Attribute::VsaWimaxHotlineProfileId(i)),
		54 => value!(i, Attribute::VsaWimaxHttpRedirectionRule(i)),
		55 => value!(i, Attribute::VsaWimaxIpRedirectionRule(i)),
		56 => map!{i, be_u32, |v| Attribute::VsaWimaxHotlineSessionTimer(v)},
		57 => value!(i, Attribute::VsaWimaxNspId(i)),
		58 => map! {i, be_u32, |v| Attribute::VsaWimaxHaRkKeyRequested(WimaxHaRkKeyRequested(v))},
		59 => map!{i, be_u8, |v| Attribute::VsaWimaxCountType(v)},
		60 => map! {i, be_u32, |v| Attribute::VsaWimaxDmActionCode(WimaxDmActionCode(v))},
		61 => map!{i, be_u32, |v| Attribute::VsaWimaxFaRkSpi(v)},
		62 => value!(i, Attribute::VsaWimaxDownlinkFlowDescription(i)),
		63 => value!(i, Attribute::VsaWimaxDownlinkGrantedQos(i)),
		64 => map!{i, take!(4), |v:&[u8]| Attribute::VsaWimaxVhaIpMip4(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		65 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaWimaxVhaIpMip6(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		66 => value!(i, Attribute::VsaWimaxVhaMip4Key(i)),
		67 => value!(i, Attribute::VsaWimaxVhaRkKey(i)),
		68 => map!{i, be_u32, |v| Attribute::VsaWimaxVhaRkSpi(v)},
		69 => map!{i, be_u32, |v| Attribute::VsaWimaxVhaRkLifetime(v)},
		70 => value!(i, Attribute::VsaWimaxMnVhaMip6Key(i)),
		71 => map!{i, be_u32, |v| Attribute::VsaWimaxMnVhaMip4Spi(v)},
		72 => map!{i, be_u32, |v| Attribute::VsaWimaxMnVhaMip6Spi(v)},
		73 => map!{i, take!(4), |v:&[u8]| Attribute::VsaWimaxVdhcpv4Server(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		74 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaWimaxVdhcpv6Server(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		75 => value!(i, Attribute::VsaWimaxVdhcpRk(i)),
		76 => map!{i, be_u32, |v| Attribute::VsaWimaxVdhcpRkKeyId(v)},
		77 => map!{i, be_u32, |v| Attribute::VsaWimaxVdhcpRkLifetime(v)},
        _ => value!(i, Attribute::VsaUnknown(24757, typ, i)),
    }
}
