/// Definitions for vendor Nomadix, vendor value 3309
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NomadixIpUpsell(pub u32);
 
#[allow(non_upper_case_globals)]
impl NomadixIpUpsell {
	pub const Privatepool: NomadixIpUpsell = NomadixIpUpsell(0);
	pub const Publicpool: NomadixIpUpsell = NomadixIpUpsell(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaNomadixBwUp(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaNomadixBwDown(v)},
		3 => value!(i, Attribute::VsaNomadixUrlRedirection(i)),
		4 => map! {i, be_u32, |v| Attribute::VsaNomadixIpUpsell(NomadixIpUpsell(v))},
		5 => value!(i, Attribute::VsaNomadixExpiration(i)),
		6 => value!(i, Attribute::VsaNomadixSubnet(i)),
		7 => map!{i, be_u32, |v| Attribute::VsaNomadixMaxbytesup(v)},
		8 => map!{i, be_u32, |v| Attribute::VsaNomadixMaxbytesdown(v)},
		9 => map!{i, be_u32, |v| Attribute::VsaNomadixEndofsession(v)},
		10 => value!(i, Attribute::VsaNomadixLogoffUrl(i)),
		11 => map!{i, be_u32, |v| Attribute::VsaNomadixNetVlan(v)},
		12 => value!(i, Attribute::VsaNomadixConfigUrl(i)),
		13 => value!(i, Attribute::VsaNomadixGoodbyeUrl(i)),
		14 => value!(i, Attribute::VsaNomadixQosPolicy(i)),
		17 => map!{i, be_u32, |v| Attribute::VsaNomadixSmtpRedirect(v)},
		18 => value!(i, Attribute::VsaNomadixCentralizedMgmt(i)),
		19 => map!{i, be_u32, |v| Attribute::VsaNomadixGroupPolicyId(v)},
		20 => map!{i, be_u32, |v| Attribute::VsaNomadixGroupBwMaxUp(v)},
		21 => map!{i, be_u32, |v| Attribute::VsaNomadixGroupBwMaxDown(v)},
        _ => value!(i, Attribute::VsaUnknown(3309, typ, i)),
    }
}
