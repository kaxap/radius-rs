/// Definitions for vendor Kineto, vendor value 16445
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KwHnbLocInfoMacroCoverageInd(pub u32);
 
#[allow(non_upper_case_globals)]
impl KwHnbLocInfoMacroCoverageInd {
	pub const Geran: KwHnbLocInfoMacroCoverageInd = KwHnbLocInfoMacroCoverageInd(0);
	pub const Utran: KwHnbLocInfoMacroCoverageInd = KwHnbLocInfoMacroCoverageInd(1);
	pub const None: KwHnbLocInfoMacroCoverageInd = KwHnbLocInfoMacroCoverageInd(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KwHnbLocInfoGeoCoordinates(pub u32);
 
#[allow(non_upper_case_globals)]
impl KwHnbLocInfoGeoCoordinates {
	pub const North: KwHnbLocInfoGeoCoordinates = KwHnbLocInfoGeoCoordinates(0);
	pub const South: KwHnbLocInfoGeoCoordinates = KwHnbLocInfoGeoCoordinates(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KwHnbLocInfoAltitudeDirection(pub u32);
 
#[allow(non_upper_case_globals)]
impl KwHnbLocInfoAltitudeDirection {
	pub const Height: KwHnbLocInfoAltitudeDirection = KwHnbLocInfoAltitudeDirection(0);
	pub const Depth: KwHnbLocInfoAltitudeDirection = KwHnbLocInfoAltitudeDirection(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KwUeCapabilities(pub u32);
 
#[allow(non_upper_case_globals)]
impl KwUeCapabilities {
	pub const R99: KwUeCapabilities = KwUeCapabilities(0);
	pub const Rel4: KwUeCapabilities = KwUeCapabilities(1);
	pub const Rel5: KwUeCapabilities = KwUeCapabilities(2);
	pub const Rel6: KwUeCapabilities = KwUeCapabilities(3);
	pub const Rel7: KwUeCapabilities = KwUeCapabilities(4);
	pub const Rel8: KwUeCapabilities = KwUeCapabilities(5);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		2 => value!(i, Attribute::VsaKinetoUmaReleaseIndicator(i)),
		3 => value!(i, Attribute::VsaKinetoUmaApRadioIdentity(i)),
		4 => value!(i, Attribute::VsaKinetoUmaCellIdentity(i)),
		5 => value!(i, Attribute::VsaKinetoUmaLocationAreaIdentification(i)),
		6 => value!(i, Attribute::VsaKinetoUmaCoverageIndicator(i)),
		7 => value!(i, Attribute::VsaKinetoUmaClassmark(i)),
		8 => value!(i, Attribute::VsaKinetoUmaGeographicalLocation(i)),
		9 => value!(i, Attribute::VsaKinetoUmaSgwIpAddress(i)),
		10 => value!(i, Attribute::VsaKinetoUmaSgwFqdn(i)),
		11 => value!(i, Attribute::VsaKinetoUmaRedirectionCounter(i)),
		12 => value!(i, Attribute::VsaKinetoUmaDiscoveryRejectCause(i)),
		17 => value!(i, Attribute::VsaKinetoUmaRrcState(i)),
		21 => value!(i, Attribute::VsaKinetoUmaRegisterRejectCause(i)),
		41 => value!(i, Attribute::VsaKinetoUmaRoutingAreaCode(i)),
		42 => value!(i, Attribute::VsaKinetoUmaApLocation(i)),
		44 => value!(i, Attribute::VsaKinetoUmaLocationStatus(i)),
		49 => value!(i, Attribute::VsaKinetoUmaUtranCellIdentity(i)),
		58 => value!(i, Attribute::VsaKinetoUmaLocationBlacklistIndicator(i)),
		61 => value!(i, Attribute::VsaKinetoUmaApServiceName(i)),
		62 => value!(i, Attribute::VsaKinetoUmaServiceZoneInformation(i)),
		67 => value!(i, Attribute::VsaKinetoUmaServingUncTableIndicator(i)),
		68 => value!(i, Attribute::VsaKinetoUmaRegistrationIndicators(i)),
		69 => value!(i, Attribute::VsaKinetoUmaUmaPlmnList(i)),
		71 => value!(i, Attribute::VsaKinetoUmaRequiredUmaServices(i)),
		73 => value!(i, Attribute::VsaKinetoUma3GCellIdentity(i)),
		96 => value!(i, Attribute::VsaKinetoUmaMsRadioIdentity(i)),
		97 => value!(i, Attribute::VsaKinetoUmaUncIpAddress(i)),
		98 => value!(i, Attribute::VsaKinetoUmaUncFqdn(i)),
		// out of u8 bounds 		65281 => value!(i, Attribute::VsaKinetoUrrTransactionType(i)),
		// out of u8 bounds 		65282 => value!(i, Attribute::VsaKinetoLocationKey(i)),
		// out of u8 bounds 		65283 => value!(i, Attribute::VsaKinetoUpClientRemoteAddress(i)),
		// out of u8 bounds 		65284 => value!(i, Attribute::VsaKinetoHandInControlFlag(i)),
		// out of u8 bounds 		65285 => value!(i, Attribute::VsaKinetoHandOutControlFlag(i)),
		// out of u8 bounds 		65286 => value!(i, Attribute::VsaKinetoBillingRateIndicator(i)),
		// out of u8 bounds 		65289 => value!(i, Attribute::VsaKinetoServiceAreaCode(i)),
		// out of u8 bounds 		65408 => value!(i, Attribute::VsaKwIuhMessageType(i)),
		// out of u8 bounds 		65409 => map!{i, take!(4), |v:&[u8]| Attribute::VsaKwHnbRemoteAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		// out of u8 bounds 		65410 => value!(i, Attribute::VsaKwHnbIdentity(i)),
		// out of u8 bounds 		65411 => map! {i, be_u32, |v| Attribute::VsaKwHnbLocInfoMacroCoverageInd(KwHnbLocInfoMacroCoverageInd(v))},
		// out of u8 bounds 		65412 => value!(i, Attribute::VsaKwHnbLocInfoGeranCellId(i)),
		// out of u8 bounds 		65413 => value!(i, Attribute::VsaKwHnbLocInfoUtranCellId(i)),
		// out of u8 bounds 		65414 => map! {i, be_u32, |v| Attribute::VsaKwHnbLocInfoGeoCoordinates(KwHnbLocInfoGeoCoordinates(v))},
		// out of u8 bounds 		65415 => map! {i, be_u32, |v| Attribute::VsaKwHnbLocInfoAltitudeDirection(KwHnbLocInfoAltitudeDirection(v))},
		// out of u8 bounds 		65416 => value!(i, Attribute::VsaKwHnbLocInfoIpAddress(i)),
		// out of u8 bounds 		65417 => value!(i, Attribute::VsaKwHnbPlmnId(i)),
		// out of u8 bounds 		65418 => value!(i, Attribute::VsaKwHnbCellId(i)),
		// out of u8 bounds 		65419 => value!(i, Attribute::VsaKwHnbLac(i)),
		// out of u8 bounds 		65420 => value!(i, Attribute::VsaKwHnbRac(i)),
		// out of u8 bounds 		65421 => value!(i, Attribute::VsaKwHnbSac(i)),
		// out of u8 bounds 		65422 => value!(i, Attribute::VsaKwHnbCsgId(i)),
		// out of u8 bounds 		65423 => map! {i, be_u32, |v| Attribute::VsaKwUeCapabilities(KwUeCapabilities(v))},
		// out of u8 bounds 		65424 => value!(i, Attribute::VsaKwHnbLocationAreaInd(i)),
		// out of u8 bounds 		65425 => value!(i, Attribute::VsaKwIuhBillingRateIndicator(i)),
		// out of u8 bounds 		65426 => value!(i, Attribute::VsaKwRegistrationRejectCause(i)),
		// out of u8 bounds 		65427 => value!(i, Attribute::VsaKwHnbLocationBlacklistInd(i)),
		// out of u8 bounds 		65428 => value!(i, Attribute::VsaKwHnbCellAccessMode(i)),
		// out of u8 bounds 		65429 => value!(i, Attribute::VsaKwUeMembershipStatus(i)),
        _ => value!(i, Attribute::VsaUnknown(16445, typ, i)),
    }
}
