/// Definitions for vendor 3GPP, vendor value 10415
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThreeGppPdpType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ThreeGppPdpType {
	pub const Ipv4: ThreeGppPdpType = ThreeGppPdpType(0);
	pub const Ppp: ThreeGppPdpType = ThreeGppPdpType(1);
	pub const Ipv6: ThreeGppPdpType = ThreeGppPdpType(2);
	pub const Ipv4V6: ThreeGppPdpType = ThreeGppPdpType(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThreeGppRatType(pub u8);
 
#[allow(non_upper_case_globals)]
impl ThreeGppRatType {
	pub const Reserved: ThreeGppRatType = ThreeGppRatType(0);
	pub const Utran: ThreeGppRatType = ThreeGppRatType(1);
	pub const Geran: ThreeGppRatType = ThreeGppRatType(2);
	pub const Wlan: ThreeGppRatType = ThreeGppRatType(3);
	pub const Gan: ThreeGppRatType = ThreeGppRatType(4);
	pub const HspaEvolution: ThreeGppRatType = ThreeGppRatType(5);
	pub const Eutran: ThreeGppRatType = ThreeGppRatType(6);
	pub const Virtual: ThreeGppRatType = ThreeGppRatType(7);
	pub const Ieee80216E: ThreeGppRatType = ThreeGppRatType(101);
	pub const ThreeGpp2Ehrpd: ThreeGppRatType = ThreeGppRatType(102);
	pub const ThreeGpp2Hrpd: ThreeGppRatType = ThreeGppRatType(103);
	pub const ThreeGpp21Xrtt: ThreeGppRatType = ThreeGppRatType(104);
	pub const ThreeGpp2Umb: ThreeGppRatType = ThreeGppRatType(105);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ThreeGppAllocateIpType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ThreeGppAllocateIpType {
	pub const DoNotAllocateIpv4Ipv6: ThreeGppAllocateIpType = ThreeGppAllocateIpType(0);
	pub const AllocateIpv4: ThreeGppAllocateIpType = ThreeGppAllocateIpType(1);
	pub const AllocateIpv6Prefix: ThreeGppAllocateIpType = ThreeGppAllocateIpType(2);
	pub const AllocateIpv4Ipv6Prefix: ThreeGppAllocateIpType = ThreeGppAllocateIpType(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaThreeGppImsi(i)),
		2 => map!{i, be_u32, |v| Attribute::VsaThreeGppChargingId(v)},
		3 => map! {i, be_u32, |v| Attribute::VsaThreeGppPdpType(ThreeGppPdpType(v))},
		4 => map!{i, take!(4), |v:&[u8]| Attribute::VsaThreeGppChargingGatewayAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		5 => value!(i, Attribute::VsaThreeGppGprsNegotiatedQosProfile(i)),
		6 => map!{i, take!(4), |v:&[u8]| Attribute::VsaThreeGppSgsnAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		7 => map!{i, take!(4), |v:&[u8]| Attribute::VsaThreeGppGgsnAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		8 => value!(i, Attribute::VsaThreeGppImsiMccMnc(i)),
		9 => value!(i, Attribute::VsaThreeGppGgsnMccMnc(i)),
		10 => value!(i, Attribute::VsaThreeGppNsapi(i)),
		11 => value!(i, Attribute::VsaThreeGppSessionStopIndicator(i)),
		12 => value!(i, Attribute::VsaThreeGppSelectionMode(i)),
		13 => value!(i, Attribute::VsaThreeGppChargingCharacteristics(i)),
		14 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaThreeGppChargingGatewayIpv6Address(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		15 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaThreeGppSgsnIpv6Address(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		16 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaThreeGppGgsnIpv6Address(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		17 => value!(i, Attribute::VsaThreeGppIpv6DnsServers(i)),
		18 => value!(i, Attribute::VsaThreeGppSgsnMccMnc(i)),
		19 => value!(i, Attribute::VsaThreeGppTeardownIndicator(i)),
		20 => value!(i, Attribute::VsaThreeGppImeisv(i)),
		21 => map! {i, be_u8, |v| Attribute::VsaThreeGppRatType(ThreeGppRatType(v))},
		22 => value!(i, Attribute::VsaThreeGppUserLocationInfo(i)),
		23 => value!(i, Attribute::VsaThreeGppMsTimezone(i)),
		24 => value!(i, Attribute::VsaThreeGppCamelChargingInfo(i)),
		25 => value!(i, Attribute::VsaThreeGppPacketFilter(i)),
		26 => map!{i, be_u8, |v| Attribute::VsaThreeGppNegotiatedDscp(v)},
		27 => map! {i, be_u32, |v| Attribute::VsaThreeGppAllocateIpType(ThreeGppAllocateIpType(v))},
        _ => value!(i, Attribute::VsaUnknown(10415, typ, i)),
    }
}
