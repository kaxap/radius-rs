/// Definitions for vendor Airespace, vendor value 14179
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AirespaceQosLevel(pub u32);
 
#[allow(non_upper_case_globals)]
impl AirespaceQosLevel {
	pub const Bronze: AirespaceQosLevel = AirespaceQosLevel(3);
	pub const Silver: AirespaceQosLevel = AirespaceQosLevel(0);
	pub const Gold: AirespaceQosLevel = AirespaceQosLevel(1);
	pub const Platinum: AirespaceQosLevel = AirespaceQosLevel(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaAirespaceWlanId(v)},
		2 => map! {i, be_u32, |v| Attribute::VsaAirespaceQosLevel(AirespaceQosLevel(v))},
		3 => map!{i, be_u32, |v| Attribute::VsaAirespaceDscp(v)},
		4 => map!{i, be_u32, |v| Attribute::VsaAirespace8021PTag(v)},
		5 => value!(i, Attribute::VsaAirespaceInterfaceName(i)),
		6 => value!(i, Attribute::VsaAirespaceAclName(i)),
		7 => map!{i, be_u32, |v| Attribute::VsaAirespaceDataBandwidthAverageContract(v)},
		8 => map!{i, be_u32, |v| Attribute::VsaAirespaceRealTimeBandwidthAverageContract(v)},
		9 => map!{i, be_u32, |v| Attribute::VsaAirespaceDataBandwidthBurstContract(v)},
		10 => map!{i, be_u32, |v| Attribute::VsaAirespaceRealTimeBandwidthBurstContract(v)},
		11 => value!(i, Attribute::VsaAirespaceGuestRoleName(i)),
		13 => map!{i, be_u32, |v| Attribute::VsaAirespaceDataBandwidthAverageContractUpstream(v)},
		14 => map!{i, be_u32, |v| Attribute::VsaAirespaceRealTimeBandwidthAverageContractUpstream(v)},
		15 => map!{i, be_u32, |v| Attribute::VsaAirespaceDataBandwidthBurstContractUpstream(v)},
		16 => map!{i, be_u32, |v| Attribute::VsaAirespaceRealTimeBandwidthBurstContractUpstream(v)},
        _ => value!(i, Attribute::VsaUnknown(14179, typ, i)),
    }
}
