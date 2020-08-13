/// Definitions for vendor Lancom, vendor value 2356
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaLcsTrafficLimit(v)},
		2 => value!(i, Attribute::VsaLcsMacAddress(i)),
		3 => value!(i, Attribute::VsaLcsRedirectionUrl(i)),
		4 => value!(i, Attribute::VsaLcsComment(i)),
		5 => map!{i, be_u32, |v| Attribute::VsaLcsAccountEnd(v)},
		6 => value!(i, Attribute::VsaLcsWpaPassphrase(i)),
		7 => value!(i, Attribute::VsaLcsPbspotusername(i)),
		8 => map!{i, be_u32, |v| Attribute::VsaLcsTxratelimit(v)},
		9 => map!{i, be_u32, |v| Attribute::VsaLcsRxratelimit(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaLcsAccessRights(v)},
		12 => map!{i, be_u32, |v| Attribute::VsaLcsFunctionRights(v)},
		13 => value!(i, Attribute::VsaLcsAdvertisementUrl(i)),
		14 => map!{i, be_u32, |v| Attribute::VsaLcsAdvertisementInterval(v)},
		15 => map!{i, be_u32, |v| Attribute::VsaLcsTrafficLimitGigawords(v)},
		16 => value!(i, Attribute::VsaLcsOrigNasIdentifier(i)),
		17 => map!{i, take!(4), |v:&[u8]| Attribute::VsaLcsOrigNasIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		18 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::VsaLcsOrigNasIpv6Address(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		19 => value!(i, Attribute::VsaLcsIkev2LocalPassword(i)),
		20 => value!(i, Attribute::VsaLcsIkev2RemotePassword(i)),
		21 => map!{i, take!(4), |v:&[u8]| Attribute::VsaLcsDnsServerIpv4Address(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		22 => value!(i, Attribute::VsaLcsVpnIpv4Rule(i)),
		23 => value!(i, Attribute::VsaLcsVpnIpv6Rule(i)),
		24 => map!{i, be_u32, |v| Attribute::VsaLcsRoutingTag(v)},
		25 => value!(i, Attribute::VsaLcsIkev2Ipv4Route(i)),
		26 => value!(i, Attribute::VsaLcsIkev2Ipv6Route(i)),
        _ => value!(i, Attribute::VsaUnknown(2356, typ, i)),
    }
}
