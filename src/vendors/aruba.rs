/// Definitions for vendor Aruba, vendor value 14823
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArubaAirgroupDeviceType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ArubaAirgroupDeviceType {
	pub const PersonalDevice: ArubaAirgroupDeviceType = ArubaAirgroupDeviceType(1);
	pub const SharedDevice: ArubaAirgroupDeviceType = ArubaAirgroupDeviceType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArubaAirgroupVersion(pub u32);
 
#[allow(non_upper_case_globals)]
impl ArubaAirgroupVersion {
	pub const AirgroupV1: ArubaAirgroupVersion = ArubaAirgroupVersion(1);
	pub const AirgroupV2: ArubaAirgroupVersion = ArubaAirgroupVersion(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaArubaUserRole(i)),
		2 => map!{i, be_u32, |v| Attribute::VsaArubaUserVlan(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaArubaPrivAdminUser(v)},
		4 => value!(i, Attribute::VsaArubaAdminRole(i)),
		5 => value!(i, Attribute::VsaArubaEssidName(i)),
		6 => value!(i, Attribute::VsaArubaLocationId(i)),
		7 => value!(i, Attribute::VsaArubaPortIdentifier(i)),
		8 => value!(i, Attribute::VsaArubaMmsUserTemplate(i)),
		9 => value!(i, Attribute::VsaArubaNamedUserVlan(i)),
		10 => value!(i, Attribute::VsaArubaApGroup(i)),
		11 => value!(i, Attribute::VsaArubaFramedIpv6Address(i)),
		12 => value!(i, Attribute::VsaArubaDeviceType(i)),
		13 => value!(i, Attribute::VsaArubaApName(i)),
		14 => map!{i, be_u32, |v| Attribute::VsaArubaNoDhcpFingerprint(v)},
		15 => value!(i, Attribute::VsaArubaMdpsDeviceUdid(i)),
		16 => value!(i, Attribute::VsaArubaMdpsDeviceImei(i)),
		17 => value!(i, Attribute::VsaArubaMdpsDeviceIccid(i)),
		18 => map!{i, be_u32, |v| Attribute::VsaArubaMdpsMaxDevices(v)},
		19 => value!(i, Attribute::VsaArubaMdpsDeviceName(i)),
		20 => value!(i, Attribute::VsaArubaMdpsDeviceProduct(i)),
		21 => value!(i, Attribute::VsaArubaMdpsDeviceVersion(i)),
		22 => value!(i, Attribute::VsaArubaMdpsDeviceSerial(i)),
		23 => value!(i, Attribute::VsaArubaCppmRole(i)),
		24 => value!(i, Attribute::VsaArubaAirgroupUserName(i)),
		25 => value!(i, Attribute::VsaArubaAirgroupSharedUser(i)),
		26 => value!(i, Attribute::VsaArubaAirgroupSharedRole(i)),
		27 => map! {i, be_u32, |v| Attribute::VsaArubaAirgroupDeviceType(ArubaAirgroupDeviceType(v))},
		28 => value!(i, Attribute::VsaArubaAuthSurvivability(i)),
		29 => value!(i, Attribute::VsaArubaAsUserName(i)),
		30 => value!(i, Attribute::VsaArubaAsCredentialHash(i)),
		31 => value!(i, Attribute::VsaArubaWorkspaceAppName(i)),
		32 => value!(i, Attribute::VsaArubaMdpsProvisioningSettings(i)),
		33 => value!(i, Attribute::VsaArubaMdpsDeviceProfile(i)),
		34 => map!{i, take!(4), |v:&[u8]| Attribute::VsaArubaApIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		35 => value!(i, Attribute::VsaArubaAirgroupSharedGroup(i)),
		36 => value!(i, Attribute::VsaArubaUserGroup(i)),
		37 => value!(i, Attribute::VsaArubaNetworkSsoToken(i)),
		38 => map! {i, be_u32, |v| Attribute::VsaArubaAirgroupVersion(ArubaAirgroupVersion(v))},
		39 => map!{i, be_u32, |v| Attribute::VsaArubaAuthSurvmethod(v)},
		40 => map!{i, be_u32, |v| Attribute::VsaArubaPortBounceHost(v)},
		41 => map!{i, take!(4), |v:&[u8]| Attribute::VsaArubaCaleaServerIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		42 => value!(i, Attribute::VsaArubaAdminPath(i)),
        _ => value!(i, Attribute::VsaUnknown(14823, typ, i)),
    }
}
