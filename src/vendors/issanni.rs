/// Definitions for vendor Issanni, vendor value 5948
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IssanniTunnelType(pub u32);
 
#[allow(non_upper_case_globals)]
impl IssanniTunnelType {
	pub const IpIp: IssanniTunnelType = IssanniTunnelType(1);
	pub const Esp: IssanniTunnelType = IssanniTunnelType(2);
	pub const L2Tp: IssanniTunnelType = IssanniTunnelType(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IssanniNatType(pub u32);
 
#[allow(non_upper_case_globals)]
impl IssanniNatType {
	pub const Nat: IssanniNatType = IssanniNatType(1);
	pub const Napt: IssanniNatType = IssanniNatType(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaIssanniSoftflowTemplate(i)),
		2 => value!(i, Attribute::VsaIssanniNatSupport(i)),
		3 => value!(i, Attribute::VsaIssanniRoutingContext(i)),
		4 => value!(i, Attribute::VsaIssanniTunnelName(i)),
		5 => value!(i, Attribute::VsaIssanniIpPoolName(i)),
		6 => value!(i, Attribute::VsaIssanniPppoeUrl(i)),
		7 => value!(i, Attribute::VsaIssanniPppoeMotm(i)),
		8 => value!(i, Attribute::VsaIssanniService(i)),
		9 => map!{i, take!(4), |v:&[u8]| Attribute::VsaIssanniPriDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		10 => map!{i, take!(4), |v:&[u8]| Attribute::VsaIssanniSecDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		11 => map!{i, take!(4), |v:&[u8]| Attribute::VsaIssanniPriNbns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		12 => map!{i, take!(4), |v:&[u8]| Attribute::VsaIssanniSecNbns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		13 => value!(i, Attribute::VsaIssanniTrafficClass(i)),
		14 => map! {i, be_u32, |v| Attribute::VsaIssanniTunnelType(IssanniTunnelType(v))},
		15 => map! {i, be_u32, |v| Attribute::VsaIssanniNatType(IssanniNatType(v))},
		16 => value!(i, Attribute::VsaIssanniQosClass(i)),
		17 => value!(i, Attribute::VsaIssanniInterfaceName(i)),
        _ => value!(i, Attribute::VsaUnknown(5948, typ, i)),
    }
}
