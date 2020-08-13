/// Definitions for vendor infonet, vendor value 4453
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		238 => value!(i, Attribute::VsaInfonetProxy(i)),
		239 => value!(i, Attribute::VsaInfonetConfig(i)),
		240 => value!(i, Attribute::VsaInfonetMcsCountry(i)),
		241 => value!(i, Attribute::VsaInfonetMcsRegion(i)),
		242 => value!(i, Attribute::VsaInfonetMcsOffPeak(i)),
		243 => value!(i, Attribute::VsaInfonetMcsOverflow(i)),
		244 => value!(i, Attribute::VsaInfonetMcsPort(i)),
		245 => value!(i, Attribute::VsaInfonetMcsPortCount(i)),
		247 => value!(i, Attribute::VsaInfonetAccountNumber(i)),
		248 => value!(i, Attribute::VsaInfonetType(i)),
		252 => value!(i, Attribute::VsaInfonetPoolRequest(i)),
		254 => map!{i, be_u32, |v| Attribute::VsaInfonetSurchargeType(v)},
		255 => value!(i, Attribute::VsaInfonetNasLocation(i)),
		246 => value!(i, Attribute::VsaInfonetRandomIpPool(i)),
		249 => value!(i, Attribute::VsaInfonetRealmType(i)),
		250 => value!(i, Attribute::VsaInfonetLoginhostDest(i)),
		251 => value!(i, Attribute::VsaInfonetTunnelDecisionIp(i)),
        _ => value!(i, Attribute::VsaUnknown(4453, typ, i)),
    }
}
