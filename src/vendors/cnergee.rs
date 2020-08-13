/// Definitions for vendor Cnergee, vendor value 49426
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BelrasRedirectPool(pub u32);
 
#[allow(non_upper_case_globals)]
impl BelrasRedirectPool {
	pub const Deleted: BelrasRedirectPool = BelrasRedirectPool(1);
	pub const Disabled: BelrasRedirectPool = BelrasRedirectPool(2);
	pub const Disputes: BelrasRedirectPool = BelrasRedirectPool(3);
	pub const Expired: BelrasRedirectPool = BelrasRedirectPool(4);
	pub const Unknown: BelrasRedirectPool = BelrasRedirectPool(5);
	pub const Exhausted: BelrasRedirectPool = BelrasRedirectPool(6);
	pub const Wrongmac: BelrasRedirectPool = BelrasRedirectPool(7);
	pub const Vlanmismatch: BelrasRedirectPool = BelrasRedirectPool(8);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaBelrasUpSpeedLimit(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaBelrasDownSpeedLimit(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaBelrasQosSpeed(v)},
		4 => value!(i, Attribute::VsaBelrasUser(i)),
		5 => map!{i, take!(4), |v:&[u8]| Attribute::VsaBelrasDhcpRouterIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		6 => map!{i, be_u32, |v| Attribute::VsaBelrasDhcpMask(v)},
		7 => map!{i, be_u32, |v| Attribute::VsaBelrasRedirect(v)},
		8 => map! {i, be_u32, |v| Attribute::VsaBelrasRedirectPool(BelrasRedirectPool(v))},
		9 => value!(i, Attribute::VsaBelrasDhcpOption82(i)),
		10 => map!{i, be_u32, |v| Attribute::VsaBelrasSessionOctetsLimit(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaBelrasOctetsDirection(v)},
		12 => map!{i, be_u32, |v| Attribute::VsaBelrasAkamaiSpeed(v)},
		13 => map!{i, be_u32, |v| Attribute::VsaBelrasCacheSpeed(v)},
		14 => map!{i, be_u32, |v| Attribute::VsaBelrasCacheflySpeed(v)},
		15 => map!{i, be_u32, |v| Attribute::VsaBelrasGgcSpeed(v)},
		16 => map!{i, be_u32, |v| Attribute::VsaBelrasGoogleSpeed(v)},
		17 => map!{i, be_u32, |v| Attribute::VsaBelrasIncapsulaSpeed(v)},
		18 => map!{i, be_u32, |v| Attribute::VsaBelrasLimelightSpeed(v)},
		19 => map!{i, be_u32, |v| Attribute::VsaBelrasOthersSpeed(v)},
		20 => map!{i, be_u32, |v| Attribute::VsaBelrasRediffSpeed(v)},
		21 => map!{i, be_u32, |v| Attribute::VsaBelrasTorrentSpeed(v)},
		22 => map!{i, be_u32, |v| Attribute::VsaBelrasBelcacheSpeed(v)},
		23 => map!{i, be_u32, |v| Attribute::VsaBelrasDhcpLeaseTime(v)},
        _ => value!(i, Attribute::VsaUnknown(49426, typ, i)),
    }
}
