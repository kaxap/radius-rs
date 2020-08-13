/// Definitions for vendor UTStarcom, vendor value 7064
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		140 => map!{i, be_u32, |v| Attribute::VsaUtstarcomVlanId(v)},
		142 => map!{i, be_u32, |v| Attribute::VsaUtstarcomCommittedbandwidth(v)},
		143 => map!{i, be_u32, |v| Attribute::VsaUtstarcomMaxbandwidth(v)},
		145 => map!{i, be_u32, |v| Attribute::VsaUtstarcomPriority(v)},
		147 => map!{i, be_u32, |v| Attribute::VsaUtstarcomErrorReason(v)},
		152 => map!{i, be_u32, |v| Attribute::VsaUtstarcomPrimarydns(v)},
		153 => map!{i, be_u32, |v| Attribute::VsaUtstarcomSecondarydns(v)},
		161 => map!{i, be_u32, |v| Attribute::VsaUtstarcomMaxburstsize(v)},
		162 => map!{i, be_u32, |v| Attribute::VsaUtstarcomMaxdelay(v)},
		163 => map!{i, be_u32, |v| Attribute::VsaUtstarcomMaxjitter(v)},
		165 => value!(i, Attribute::VsaUtstarcomDeviceid(i)),
		166 => map!{i, be_u32, |v| Attribute::VsaUtstarcomModuleId(v)},
		167 => map!{i, be_u32, |v| Attribute::VsaUtstarcomPortNo(v)},
		168 => map!{i, be_u32, |v| Attribute::VsaUtstarcomLogicalPortNo(v)},
		169 => map!{i, be_u32, |v| Attribute::VsaUtstarcomUniMaxMac(v)},
		170 => map!{i, be_u32, |v| Attribute::VsaUtstarcomDefaultGateway(v)},
		171 => map!{i, be_u32, |v| Attribute::VsaUtstarcomCliAccessLevel(v)},
		180 => value!(i, Attribute::VsaUtstarcomActInputOctets(i)),
		181 => value!(i, Attribute::VsaUtstarcomActOutputOctets(i)),
		182 => value!(i, Attribute::VsaUtstarcomActInputFrames(i)),
		183 => value!(i, Attribute::VsaUtstarcomActOutputFrames(i)),
		184 => map!{i, be_u32, |v| Attribute::VsaUtstarcomOnuMcFilterEnable(v)},
		185 => map!{i, be_u32, |v| Attribute::VsaUtstarcomUniAutoNegotiation(v)},
		186 => map!{i, be_u32, |v| Attribute::VsaUtstarcomUniSpeed(v)},
		187 => map!{i, be_u32, |v| Attribute::VsaUtstarcomUniDuplex(v)},
		188 => map!{i, be_u32, |v| Attribute::VsaUtstarcomOnuAdminStatus(v)},
		189 => map!{i, be_u32, |v| Attribute::VsaUtstarcomOnuFwScUpgrade(v)},
        _ => value!(i, Attribute::VsaUnknown(7064, typ, i)),
    }
}
