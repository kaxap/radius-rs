/// Definitions for vendor Aptilo, vendor value 13209
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AptiloLimitMode(pub u32);
 
#[allow(non_upper_case_globals)]
impl AptiloLimitMode {
	pub const Relative: AptiloLimitMode = AptiloLimitMode(0);
	pub const Absolute: AptiloLimitMode = AptiloLimitMode(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AptiloDeniedCause(pub u32);
 
#[allow(non_upper_case_globals)]
impl AptiloDeniedCause {
	pub const UserNotFound: AptiloDeniedCause = AptiloDeniedCause(1);
	pub const WrongPassword: AptiloDeniedCause = AptiloDeniedCause(2);
	pub const NoZoneAccess: AptiloDeniedCause = AptiloDeniedCause(3);
	pub const InactiveAccessNode: AptiloDeniedCause = AptiloDeniedCause(4);
	pub const InconsistentAccessNode: AptiloDeniedCause = AptiloDeniedCause(5);
	pub const DisabledAccount: AptiloDeniedCause = AptiloDeniedCause(6);
	pub const NoAccessprofile: AptiloDeniedCause = AptiloDeniedCause(7);
	pub const InternalError: AptiloDeniedCause = AptiloDeniedCause(8);
	pub const RealmError: AptiloDeniedCause = AptiloDeniedCause(9);
	pub const NoCredits: AptiloDeniedCause = AptiloDeniedCause(10);
	pub const MaxSession: AptiloDeniedCause = AptiloDeniedCause(11);
	pub const RemoteServerReject: AptiloDeniedCause = AptiloDeniedCause(12);
	pub const RealmInactive: AptiloDeniedCause = AptiloDeniedCause(14);
	pub const OpaqueFailed: AptiloDeniedCause = AptiloDeniedCause(15);
	pub const ServiceClosed: AptiloDeniedCause = AptiloDeniedCause(16);
	pub const LdapFailed: AptiloDeniedCause = AptiloDeniedCause(17);
	pub const InactiveAccount: AptiloDeniedCause = AptiloDeniedCause(18);
	pub const ExpiredAccount: AptiloDeniedCause = AptiloDeniedCause(19);
	pub const IncompleteAccount: AptiloDeniedCause = AptiloDeniedCause(20);
	pub const LicenseLimitReached: AptiloDeniedCause = AptiloDeniedCause(21);
	pub const UnsupportedService: AptiloDeniedCause = AptiloDeniedCause(22);
	pub const RulesetReject: AptiloDeniedCause = AptiloDeniedCause(23);
	pub const RulesetFailed: AptiloDeniedCause = AptiloDeniedCause(24);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AptiloAuthType(pub u32);
 
#[allow(non_upper_case_globals)]
impl AptiloAuthType {
	pub const User: AptiloAuthType = AptiloAuthType(0);
	pub const Auto: AptiloAuthType = AptiloAuthType(1);
	pub const IpRequest: AptiloAuthType = AptiloAuthType(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AptiloNasCapabilities(pub u32);
 
#[allow(non_upper_case_globals)]
impl AptiloNasCapabilities {
	pub const CoaLogin: AptiloNasCapabilities = AptiloNasCapabilities(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AptiloDebugOption(pub u32);
 
#[allow(non_upper_case_globals)]
impl AptiloDebugOption {
	pub const SimulateEapTls: AptiloDebugOption = AptiloDebugOption(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaAptiloSubnetName(i)),
		2 => map!{i, be_u32, |v| Attribute::VsaAptiloOctetsLimit(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaAptiloGigawordsLimit(v)},
		4 => map!{i, be_u32, |v| Attribute::VsaAptiloInputOctetsLimit(v)},
		5 => map!{i, be_u32, |v| Attribute::VsaAptiloInputGigawordsLimit(v)},
		6 => map!{i, be_u32, |v| Attribute::VsaAptiloOutputOctetsLimit(v)},
		7 => map!{i, be_u32, |v| Attribute::VsaAptiloOutputGigawordsLimit(v)},
		8 => map! {i, be_u32, |v| Attribute::VsaAptiloLimitMode(AptiloLimitMode(v))},
		9 => value!(i, Attribute::VsaAptiloApcId(i)),
		10 => value!(i, Attribute::VsaAptiloOpaqueKey(i)),
		11 => map! {i, be_u32, |v| Attribute::VsaAptiloDeniedCause(AptiloDeniedCause(v))},
		12 => map!{i, be_u32, |v| Attribute::VsaAptiloRealmId(v)},
		13 => map!{i, be_u32, |v| Attribute::VsaAptiloApId(v)},
		14 => map!{i, be_u32, |v| Attribute::VsaAptiloUserId(v)},
		15 => value!(i, Attribute::VsaAptiloZone(i)),
		16 => value!(i, Attribute::VsaAptiloFirstName(i)),
		17 => value!(i, Attribute::VsaAptiloLastName(i)),
		18 => value!(i, Attribute::VsaAptiloPhone(i)),
		19 => value!(i, Attribute::VsaAptiloEmail(i)),
		20 => value!(i, Attribute::VsaAptiloOrganization(i)),
		21 => value!(i, Attribute::VsaAptiloAccessProfile(i)),
		22 => map!{i, be_u32, |v| Attribute::VsaAptiloRealmConcurrentLogin(v)},
		23 => map!{i, be_u32, |v| Attribute::VsaAptiloAuthResult(v)},
		24 => value!(i, Attribute::VsaAptiloHotlineIndicator(i)),
		25 => map!{i, be_u32, |v| Attribute::VsaAptiloUserType(v)},
		26 => map!{i, be_u32, |v| Attribute::VsaAptiloExclusiveCount(v)},
		27 => map!{i, be_u32, |v| Attribute::VsaAptiloDurationQuota(v)},
		28 => value!(i, Attribute::VsaAptiloVolumeQuota(i)),
		29 => value!(i, Attribute::VsaAptiloRxVolumeQuota(i)),
		30 => value!(i, Attribute::VsaAptiloTxVolumeQuota(i)),
		31 => map!{i, be_u32, |v| Attribute::VsaAptiloResourceQuota(v)},
		32 => value!(i, Attribute::VsaAptiloQuotaId(i)),
		33 => map!{i, be_u32, |v| Attribute::VsaAptiloRxLimit(v)},
		34 => map!{i, be_u32, |v| Attribute::VsaAptiloTxLimit(v)},
		35 => map!{i, be_u32, |v| Attribute::VsaAptiloTrxLimit(v)},
		36 => map!{i, be_u32, |v| Attribute::VsaAptiloBwMinUp(v)},
		37 => map!{i, be_u32, |v| Attribute::VsaAptiloBwMaxUp(v)},
		38 => map!{i, be_u32, |v| Attribute::VsaAptiloBwMinDown(v)},
		39 => map!{i, be_u32, |v| Attribute::VsaAptiloBwMaxDown(v)},
		40 => value!(i, Attribute::VsaAptiloServiceProfile(i)),
		41 => value!(i, Attribute::VsaAptiloAutomaticService(i)),
		42 => map! {i, be_u32, |v| Attribute::VsaAptiloAuthType(AptiloAuthType(v))},
		43 => map! {i, be_u32, |v| Attribute::VsaAptiloNasCapabilities(AptiloNasCapabilities(v))},
		44 => value!(i, Attribute::VsaAptiloService(i)),
		45 => map!{i, be_u32, |v| Attribute::VsaAptiloServiceProfileId(v)},
		50 => map!{i, be_u32, |v| Attribute::VsaAptiloAuthParam(v)},
		53 => map!{i, be_u32, |v| Attribute::VsaAptiloAccessProfileId(v)},
		56 => value!(i, Attribute::VsaAptiloNasModel(i)),
		57 => map! {i, be_u32, |v| Attribute::VsaAptiloDebugOption(AptiloDebugOption(v))},
		58 => value!(i, Attribute::VsaAptiloSessionId(i)),
		59 => value!(i, Attribute::VsaAptiloPrepaidCapabilities(i)),
		60 => value!(i, Attribute::VsaAptiloOctetsQuota(i)),
		61 => value!(i, Attribute::VsaAptiloOctetsThreshold(i)),
		62 => map!{i, be_u32, |v| Attribute::VsaAptiloResourceThreshold(v)},
		63 => map!{i, be_u32, |v| Attribute::VsaAptiloDurationThreshold(v)},
		64 => value!(i, Attribute::VsaAptiloOctetsBalance(i)),
		65 => map!{i, be_u32, |v| Attribute::VsaAptiloResourceBalance(v)},
		66 => map!{i, be_u32, |v| Attribute::VsaAptiloDurationBalance(v)},
		67 => value!(i, Attribute::VsaAptiloOctetsUsed(i)),
		68 => map!{i, be_u32, |v| Attribute::VsaAptiloResourceUsed(v)},
		69 => map!{i, be_u32, |v| Attribute::VsaAptiloDurationUsed(v)},
		70 => value!(i, Attribute::VsaAptiloOctetsRequest(i)),
		71 => map!{i, be_u32, |v| Attribute::VsaAptiloResourceRequest(v)},
		72 => map!{i, be_u32, |v| Attribute::VsaAptiloDurationRequest(v)},
		73 => value!(i, Attribute::VsaAptiloQosIndicator(i)),
		74 => value!(i, Attribute::VsaAptiloCircuitId(i)),
		75 => value!(i, Attribute::VsaAptiloRemoteId(i)),
		76 => value!(i, Attribute::VsaAptiloLocationName(i)),
		231 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaAptiloKeyIpv61(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		232 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaAptiloKeyIpv62(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		233 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaAptiloKeyIpv63(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		234 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaAptiloKeyIpv64(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		235 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaAptiloKeyIpv65(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		236 => value!(i, Attribute::VsaAptiloKeyOctets1(i)),
		237 => value!(i, Attribute::VsaAptiloKeyOctets2(i)),
		238 => value!(i, Attribute::VsaAptiloKeyOctets3(i)),
		239 => value!(i, Attribute::VsaAptiloKeyOctets4(i)),
		240 => value!(i, Attribute::VsaAptiloKeyOctets5(i)),
		241 => value!(i, Attribute::VsaAptiloKeyString1(i)),
		242 => value!(i, Attribute::VsaAptiloKeyString2(i)),
		243 => value!(i, Attribute::VsaAptiloKeyString3(i)),
		244 => value!(i, Attribute::VsaAptiloKeyString4(i)),
		245 => value!(i, Attribute::VsaAptiloKeyString5(i)),
		246 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAptiloKeyIp1(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		247 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAptiloKeyIp2(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		248 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAptiloKeyIp3(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		249 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAptiloKeyIp4(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		250 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAptiloKeyIp5(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		251 => map!{i, be_u32, |v| Attribute::VsaAptiloKeyInteger1(v)},
		252 => map!{i, be_u32, |v| Attribute::VsaAptiloKeyInteger2(v)},
		253 => map!{i, be_u32, |v| Attribute::VsaAptiloKeyInteger3(v)},
		254 => map!{i, be_u32, |v| Attribute::VsaAptiloKeyInteger4(v)},
		255 => map!{i, be_u32, |v| Attribute::VsaAptiloKeyInteger5(v)},
        _ => value!(i, Attribute::VsaUnknown(13209, typ, i)),
    }
}
