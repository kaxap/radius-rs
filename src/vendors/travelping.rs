/// Definitions for vendor Travelping, vendor value 18681
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TpTlsAuthType(pub u32);
 
#[allow(non_upper_case_globals)]
impl TpTlsAuthType {
	pub const PreSharedKey: TpTlsAuthType = TpTlsAuthType(0);
	pub const X509SubjectCommonname: TpTlsAuthType = TpTlsAuthType(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FramedProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl FramedProtocol {
	pub const TpCapwap: FramedProtocol = FramedProtocol(0x48f90001);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ServiceType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ServiceType {
	pub const TpCapwapWtp: ServiceType = ServiceType(0x48f90001);
	pub const TpCapwapSta: ServiceType = ServiceType(0x48f90002);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaTpGatewayVersion(i)),
		2 => value!(i, Attribute::VsaTpFirmwareVariant(i)),
		3 => value!(i, Attribute::VsaTpFirmwareVersion(i)),
		4 => value!(i, Attribute::VsaTpGatewayConfig(i)),
		5 => value!(i, Attribute::VsaTpEncIv(i)),
		6 => value!(i, Attribute::VsaTpPassword(i)),
		7 => value!(i, Attribute::VsaTpUserAgent(i)),
		8 => map!{i, be_u32, |v| Attribute::VsaTpAuthReply(v)},
		9 => value!(i, Attribute::VsaTpAccessClassId(i)),
		10 => value!(i, Attribute::VsaTpHostName(i)),
		11 => value!(i, Attribute::VsaTpDhcpRequestOptionList(i)),
		12 => value!(i, Attribute::VsaTpDhcpParameterRequestList(i)),
		13 => value!(i, Attribute::VsaTpDhcpVendorClassId(i)),
		14 => value!(i, Attribute::VsaTpDhcpClientId(i)),
		15 => value!(i, Attribute::VsaTpLocationId(i)),
		16 => map!{i, take!(4), |v:&[u8]| Attribute::VsaTpNatIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		17 => value!(i, Attribute::VsaTpZoneId(i)),
		18 => value!(i, Attribute::VsaTpMonitorId(i)),
		19 => value!(i, Attribute::VsaTpRelatedSessionId(i)),
		20 => map!{i, be_u32, |v| Attribute::VsaTpMonitorSessionId(v)},
		21 => map!{i, be_u64, |v| Attribute::VsaTpMaxInputOctets(v)},
		22 => map!{i, be_u64, |v| Attribute::VsaTpMaxOutputOctets(v)},
		23 => map!{i, be_u64, |v| Attribute::VsaTpMaxTotalOctets(v)},
		24 => value!(i, Attribute::VsaTpExitAccessClassId(i)),
		25 => value!(i, Attribute::VsaTpAccessRule(i)),
		26 => value!(i, Attribute::VsaTpAccessGroup(i)),
		27 => value!(i, Attribute::VsaTpNatPoolId(i)),
		28 => map!{i, be_u32, |v| Attribute::VsaTpNatPortStart(v)},
		29 => map!{i, be_u32, |v| Attribute::VsaTpNatPortEnd(v)},
		30 => map!{i, be_u32, |v| Attribute::VsaTpKeepAliveTimeout(v)},
		31 => map! {i, be_u32, |v| Attribute::VsaTpTlsAuthType(TpTlsAuthType(v))},
		32 => value!(i, Attribute::VsaTpTlsPreSharedKey(i)),
		33 => map!{i, be_u32, |v| Attribute::VsaTpCapwapTimestamp(v)},
		34 => value!(i, Attribute::VsaTpCapwapWtpVersion(i)),
		35 => value!(i, Attribute::VsaTpCapwapSessionId(i)),
		36 => map!{i, be_u32, |v| Attribute::VsaTpCapwapRadioId(v)},
		37 => map!{i, be_u32, |v| Attribute::VsaTpCapwapWwanId(v)},
		38 => map!{i, be_u32, |v| Attribute::VsaTpCapwapWwanRat(v)},
		39 => map!{i, be_u32, |v| Attribute::VsaTpCapwapWwanRssi(v)},
		40 => map!{i, be_u32, |v| Attribute::VsaTpCapwapWwanCreg(v)},
		41 => map!{i, be_u32, |v| Attribute::VsaTpCapwapWwanLac(v)},
		42 => map!{i, be_u32, |v| Attribute::VsaTpCapwapWwanLatency(v)},
		43 => map!{i, be_u32, |v| Attribute::VsaTpCapwapWwanMcc(v)},
		44 => map!{i, be_u32, |v| Attribute::VsaTpCapwapWwanMnc(v)},
		45 => map!{i, be_u32, |v| Attribute::VsaTpCapwapWwanCellId(v)},
		46 => map!{i, be_u32, |v| Attribute::VsaTpCapwapPowerSaveIdleTimeout(v)},
		47 => map!{i, be_u32, |v| Attribute::VsaTpCapwapPowerSaveBusyTimeout(v)},
		48 => value!(i, Attribute::VsaTpCapwapSsid(i)),
		49 => map!{i, be_u32, |v| Attribute::VsaTpCapwapMaxWifiClients(v)},
		50 => value!(i, Attribute::VsaTpCapwapWalledGarden(i)),
		51 => value!(i, Attribute::VsaTpCapwapGpsLatitude(i)),
		52 => value!(i, Attribute::VsaTpCapwapGpsLongitude(i)),
		53 => value!(i, Attribute::VsaTpCapwapGpsAltitude(i)),
		54 => value!(i, Attribute::VsaTpCapwapGpsHdop(i)),
		55 => value!(i, Attribute::VsaTpCapwapGpsTimestamp(i)),
		56 => value!(i, Attribute::VsaTpCapwapHardwareVersion(i)),
		57 => value!(i, Attribute::VsaTpCapwapSoftwareVersion(i)),
		58 => value!(i, Attribute::VsaTpCapwapBootVersion(i)),
		59 => value!(i, Attribute::VsaTpCapwapOtherSoftwareVersion(i)),
		60 => map!{i, be_u64, |v| Attribute::VsaTpExcessInputOctets(v)},
		61 => map!{i, be_u64, |v| Attribute::VsaTpExcessOutputOctets(v)},
		62 => map!{i, be_u64, |v| Attribute::VsaTpExcessTotalOctets(v)},
		63 => value!(i, Attribute::VsaTpTraceId(i)),
        _ => value!(i, Attribute::VsaUnknown(18681, typ, i)),
    }
}
