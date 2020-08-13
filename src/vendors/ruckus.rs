/// Definitions for vendor Ruckus, vendor value 25053
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuckusSelectionMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl RuckusSelectionMode {
	pub const Subscribed: RuckusSelectionMode = RuckusSelectionMode(0);
	pub const Sentbyms: RuckusSelectionMode = RuckusSelectionMode(1);
	pub const Chosenbysgsn: RuckusSelectionMode = RuckusSelectionMode(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuckusApnResolutionReq(pub u32);
 
#[allow(non_upper_case_globals)]
impl RuckusApnResolutionReq {
	pub const Notrequired: RuckusApnResolutionReq = RuckusApnResolutionReq(0);
	pub const Required: RuckusApnResolutionReq = RuckusApnResolutionReq(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuckusStatus(pub u32);
 
#[allow(non_upper_case_globals)]
impl RuckusStatus {
	pub const Success: RuckusStatus = RuckusStatus(0);
	pub const Failure: RuckusStatus = RuckusStatus(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuckusAuthType(pub u32);
 
#[allow(non_upper_case_globals)]
impl RuckusAuthType {
	pub const PppSim: RuckusAuthType = RuckusAuthType(1);
	pub const Dummyimsi: RuckusAuthType = RuckusAuthType(2);
	pub const Softsim: RuckusAuthType = RuckusAuthType(3);
	pub const Radiussim: RuckusAuthType = RuckusAuthType(4);
	pub const Postpaid: RuckusAuthType = RuckusAuthType(5);
	pub const Prepaid: RuckusAuthType = RuckusAuthType(6);
	pub const Localradius: RuckusAuthType = RuckusAuthType(7);
	pub const Proxyradius: RuckusAuthType = RuckusAuthType(8);
	pub const Voucher: RuckusAuthType = RuckusAuthType(9);
	pub const EapSim: RuckusAuthType = RuckusAuthType(10);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuckusSessionType(pub u32);
 
#[allow(non_upper_case_globals)]
impl RuckusSessionType {
	pub const Ttg: RuckusSessionType = RuckusSessionType(2);
	pub const LocalBreakout: RuckusSessionType = RuckusSessionType(3);
	pub const LocalBreakoutAp: RuckusSessionType = RuckusSessionType(4);
	pub const L3Gre: RuckusSessionType = RuckusSessionType(5);
	pub const L2Gre: RuckusSessionType = RuckusSessionType(6);
	pub const Qinql3: RuckusSessionType = RuckusSessionType(7);
	pub const Pmip: RuckusSessionType = RuckusSessionType(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuckusNasType(pub u32);
 
#[allow(non_upper_case_globals)]
impl RuckusNasType {
	pub const Scg: RuckusNasType = RuckusNasType(1);
	pub const Others: RuckusNasType = RuckusNasType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuckusAccountingStatus(pub u32);
 
#[allow(non_upper_case_globals)]
impl RuckusAccountingStatus {
	pub const AccountingOn: RuckusAccountingStatus = RuckusAccountingStatus(1);
	pub const AccountingOff: RuckusAccountingStatus = RuckusAccountingStatus(0);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaRuckusUserGroups(i)),
		2 => map!{i, be_u32, |v| Attribute::VsaRuckusStaRssi(v)},
		3 => value!(i, Attribute::VsaRuckusSsid(i)),
		4 => map!{i, be_u32, |v| Attribute::VsaRuckusWlanId(v)},
		5 => value!(i, Attribute::VsaRuckusLocation(i)),
		6 => map!{i, be_u32, |v| Attribute::VsaRuckusGracePeriod(v)},
		7 => map!{i, be_u32, |v| Attribute::VsaRuckusScgCbladeIp(v)},
		8 => map!{i, be_u32, |v| Attribute::VsaRuckusScgDbladeIp(v)},
		9 => map!{i, be_u32, |v| Attribute::VsaRuckusVlanId(v)},
		10 => map!{i, be_u32, |v| Attribute::VsaRuckusStaExpiration(v)},
		11 => value!(i, Attribute::VsaRuckusStaUuid(i)),
		12 => map!{i, be_u32, |v| Attribute::VsaRuckusAcceptEnhancementReason(v)},
		13 => value!(i, Attribute::VsaRuckusStaInnerId(i)),
		14 => value!(i, Attribute::VsaRuckusBssid(i)),
		101 => value!(i, Attribute::VsaRuckusTriplets(i)),
		102 => value!(i, Attribute::VsaRuckusImsi(i)),
		103 => value!(i, Attribute::VsaRuckusMsisdn(i)),
		104 => value!(i, Attribute::VsaRuckusApnNi(i)),
		105 => value!(i, Attribute::VsaRuckusQos(i)),
		106 => map! {i, be_u32, |v| Attribute::VsaRuckusSelectionMode(RuckusSelectionMode(v))},
		107 => map! {i, be_u32, |v| Attribute::VsaRuckusApnResolutionReq(RuckusApnResolutionReq(v))},
		108 => value!(i, Attribute::VsaRuckusStartTime(i)),
		109 => map! {i, be_u32, |v| Attribute::VsaRuckusNasType(RuckusNasType(v))},
		110 => map! {i, be_u32, |v| Attribute::VsaRuckusStatus(RuckusStatus(v))},
		111 => value!(i, Attribute::VsaRuckusApnOi(i)),
		112 => map! {i, be_u32, |v| Attribute::VsaRuckusAuthType(RuckusAuthType(v))},
		113 => value!(i, Attribute::VsaRuckusGnUserName(i)),
		114 => value!(i, Attribute::VsaRuckusBrandCode(i)),
		115 => value!(i, Attribute::VsaRuckusPolicyName(i)),
		116 => map!{i, take!(4), |v:&[u8]| Attribute::VsaRuckusClientLocalIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		117 => map!{i, take!(4), |v:&[u8]| Attribute::VsaRuckusSgsnIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		118 => value!(i, Attribute::VsaRuckusChargingCharac(i)),
		119 => value!(i, Attribute::VsaRuckusPdpType(i)),
		120 => value!(i, Attribute::VsaRuckusDynamicAddressFlag(i)),
		121 => value!(i, Attribute::VsaRuckusChchSelectionMode(i)),
		122 => map!{i, take!(4), |v:&[u8]| Attribute::VsaRuckusAaaIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		123 => map!{i, be_u32, |v| Attribute::VsaRuckusCdrType(v)},
		124 => value!(i, Attribute::VsaRuckusSgsnNumber(i)),
		125 => map! {i, be_u32, |v| Attribute::VsaRuckusSessionType(RuckusSessionType(v))},
		126 => map! {i, be_u32, |v| Attribute::VsaRuckusAccountingStatus(RuckusAccountingStatus(v))},
		127 => value!(i, Attribute::VsaRuckusZoneId(i)),
		128 => value!(i, Attribute::VsaRuckusAuthServerId(i)),
		129 => value!(i, Attribute::VsaRuckusUtpId(i)),
		130 => value!(i, Attribute::VsaRuckusAreaCode(i)),
		131 => value!(i, Attribute::VsaRuckusCellIdentifier(i)),
		132 => value!(i, Attribute::VsaRuckusWisprRedirectPolicy(i)),
		133 => map!{i, be_u32, |v| Attribute::VsaRuckusEthProfileId(v)},
		134 => value!(i, Attribute::VsaRuckusZoneName(i)),
		135 => value!(i, Attribute::VsaRuckusWlanName(i)),
        _ => value!(i, Attribute::VsaUnknown(25053, typ, i)),
    }
}
