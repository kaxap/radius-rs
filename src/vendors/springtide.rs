/// Definitions for vendor SpringTide, vendor value 3551
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaStAcctVcConnectionId(i)),
		2 => value!(i, Attribute::VsaStServiceName(i)),
		3 => map!{i, be_u32, |v| Attribute::VsaStServiceDomain(v)},
		4 => value!(i, Attribute::VsaStPolicyName(i)),
		5 => map!{i, take!(4), |v:&[u8]| Attribute::VsaStPrimaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		6 => map!{i, take!(4), |v:&[u8]| Attribute::VsaStSecondaryDnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		7 => map!{i, take!(4), |v:&[u8]| Attribute::VsaStPrimaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		8 => map!{i, take!(4), |v:&[u8]| Attribute::VsaStSecondaryNbnsServer(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		9 => map!{i, be_u32, |v| Attribute::VsaStPhysicalPort(v)},
		10 => map!{i, be_u32, |v| Attribute::VsaStPhysicalSlot(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaStVirtualPathId(v)},
		12 => map!{i, be_u32, |v| Attribute::VsaStVirtualCircuitId(v)},
		13 => value!(i, Attribute::VsaStRealmName(i)),
		14 => map!{i, be_u32, |v| Attribute::VsaStIpsecPfsGroup(v)},
		15 => map!{i, be_u32, |v| Attribute::VsaStIpsecClientFirewall(v)},
		16 => value!(i, Attribute::VsaStIpsecClientSubnet(i)),
        _ => value!(i, Attribute::VsaUnknown(3551, typ, i)),
    }
}
