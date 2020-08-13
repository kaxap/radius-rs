/// Definitions for vendor ZTE, vendor value 3902
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaZteClientDnsPri(i)),
		2 => value!(i, Attribute::VsaZteClientDnsSec(i)),
		4 => value!(i, Attribute::VsaZteContextName(i)),
		21 => map!{i, be_u32, |v| Attribute::VsaZteTunnelMaxSessions(v)},
		22 => map!{i, be_u32, |v| Attribute::VsaZteTunnelMaxTunnels(v)},
		24 => map!{i, be_u32, |v| Attribute::VsaZteTunnelWindow(v)},
		25 => map!{i, be_u32, |v| Attribute::VsaZteTunnelRetransmit(v)},
		26 => map!{i, be_u32, |v| Attribute::VsaZteTunnelCmdTimeout(v)},
		27 => value!(i, Attribute::VsaZtePppoeUrl(i)),
		28 => value!(i, Attribute::VsaZtePppoeMotm(i)),
		31 => map!{i, be_u32, |v| Attribute::VsaZteTunnelAlgorithm(v)},
		32 => map!{i, be_u32, |v| Attribute::VsaZteTunnelDeadtime(v)},
		33 => map!{i, be_u32, |v| Attribute::VsaZteMcastSend(v)},
		34 => map!{i, be_u32, |v| Attribute::VsaZteMcastReceive(v)},
		35 => map!{i, be_u32, |v| Attribute::VsaZteMcastMaxgroups(v)},
		74 => map!{i, be_u32, |v| Attribute::VsaZteAccessType(v)},
		81 => map!{i, be_u32, |v| Attribute::VsaZteQosType(v)},
		82 => value!(i, Attribute::VsaZteQosProfileDown(i)),
		83 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlScrDown(v)},
		84 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlBurstDown(v)},
		86 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlPcr(v)},
		88 => map!{i, be_u32, |v| Attribute::VsaZteTcpSynRate(v)},
		89 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlScrUp(v)},
		90 => map!{i, be_u32, |v| Attribute::VsaZtePriorityLevel(v)},
		91 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlBurstUp(v)},
		92 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlBurstMaxDown(v)},
		93 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlBurstMaxUp(v)},
		94 => value!(i, Attribute::VsaZteQosProfileUp(i)),
		95 => map!{i, be_u32, |v| Attribute::VsaZteTcpLimitNum(v)},
		96 => map!{i, be_u32, |v| Attribute::VsaZteTcpLimitMode(v)},
		97 => map!{i, be_u32, |v| Attribute::VsaZteIgmpServiceProfileNum(v)},
		101 => map!{i, be_u32, |v| Attribute::VsaZtePppSserviceType(v)},
		104 => map!{i, be_u32, |v| Attribute::VsaZteSwPrivilege(v)},
		151 => value!(i, Attribute::VsaZteAccessDomain(i)),
		190 => value!(i, Attribute::VsaZteVpnId(i)),
		191 => map!{i, be_u32, |v| Attribute::VsaZteRateBustDpir(v)},
		192 => map!{i, be_u32, |v| Attribute::VsaZteRateBustUpir(v)},
		202 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlPbsDown(v)},
		203 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlPbsUp(v)},
		228 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlScrUpV6(v)},
		229 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlBurstUpV6(v)},
		230 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlBurstMaxUpV6(v)},
		231 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlPbsUpV6(v)},
		232 => value!(i, Attribute::VsaZteQosProfileUpV6(i)),
		233 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlScrDownV6(v)},
		234 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlBurstDownV6(v)},
		235 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlBurstMaxDownV6(v)},
		236 => map!{i, be_u32, |v| Attribute::VsaZteRateCtrlPbsDownV6(v)},
		237 => value!(i, Attribute::VsaZteQosProfileDownV6(i)),
        _ => value!(i, Attribute::VsaUnknown(3902, typ, i)),
    }
}
