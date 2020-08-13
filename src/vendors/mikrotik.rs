/// Definitions for vendor Mikrotik, vendor value 14988
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MikrotikWirelessEncAlgo(pub u32);
 
#[allow(non_upper_case_globals)]
impl MikrotikWirelessEncAlgo {
	pub const NoEncryption: MikrotikWirelessEncAlgo = MikrotikWirelessEncAlgo(0);
	pub const Four0BitWep: MikrotikWirelessEncAlgo = MikrotikWirelessEncAlgo(1);
	pub const One04BitWep: MikrotikWirelessEncAlgo = MikrotikWirelessEncAlgo(2);
	pub const AesCcm: MikrotikWirelessEncAlgo = MikrotikWirelessEncAlgo(3);
	pub const Tkip: MikrotikWirelessEncAlgo = MikrotikWirelessEncAlgo(4);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaMikrotikRecvLimit(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaMikrotikXmitLimit(v)},
		3 => value!(i, Attribute::VsaMikrotikGroup(i)),
		4 => map!{i, be_u32, |v| Attribute::VsaMikrotikWirelessForward(v)},
		5 => map!{i, be_u32, |v| Attribute::VsaMikrotikWirelessSkipDot1X(v)},
		6 => map! {i, be_u32, |v| Attribute::VsaMikrotikWirelessEncAlgo(MikrotikWirelessEncAlgo(v))},
		7 => value!(i, Attribute::VsaMikrotikWirelessEncKey(i)),
		8 => value!(i, Attribute::VsaMikrotikRateLimit(i)),
		9 => value!(i, Attribute::VsaMikrotikRealm(i)),
		10 => map!{i, take!(4), |v:&[u8]| Attribute::VsaMikrotikHostIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		11 => value!(i, Attribute::VsaMikrotikMarkId(i)),
		12 => value!(i, Attribute::VsaMikrotikAdvertiseUrl(i)),
		13 => map!{i, be_u32, |v| Attribute::VsaMikrotikAdvertiseInterval(v)},
		14 => map!{i, be_u32, |v| Attribute::VsaMikrotikRecvLimitGigawords(v)},
		15 => map!{i, be_u32, |v| Attribute::VsaMikrotikXmitLimitGigawords(v)},
		16 => value!(i, Attribute::VsaMikrotikWirelessPsk(i)),
		17 => map!{i, be_u32, |v| Attribute::VsaMikrotikTotalLimit(v)},
		18 => map!{i, be_u32, |v| Attribute::VsaMikrotikTotalLimitGigawords(v)},
		19 => value!(i, Attribute::VsaMikrotikAddressList(i)),
		20 => value!(i, Attribute::VsaMikrotikWirelessMpkey(i)),
		21 => value!(i, Attribute::VsaMikrotikWirelessComment(i)),
		22 => value!(i, Attribute::VsaMikrotikDelegatedIpv6Pool(i)),
		23 => value!(i, Attribute::VsaMikrotikDhcpOptionSet(i)),
		24 => value!(i, Attribute::VsaMikrotikDhcpOptionParamStr1(i)),
		25 => value!(i, Attribute::VsaMikortikDhcpOptionParamstr2(i)),
		26 => map!{i, be_u32, |v| Attribute::VsaMikrotikWirelessVlanid(v)},
		27 => map!{i, be_u32, |v| Attribute::VsaMikrotikWirelessVlanidType(v)},
		28 => value!(i, Attribute::VsaMikrotikWirelessMinsignal(i)),
		29 => value!(i, Attribute::VsaMikrotikWirelessMaxsignal(i)),
        _ => value!(i, Attribute::VsaUnknown(14988, typ, i)),
    }
}
