/// Definitions for vendor Cabletron, vendor value 52
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CabletronProtocolEnable(pub u32);
 
#[allow(non_upper_case_globals)]
impl CabletronProtocolEnable {
	pub const IpEnable: CabletronProtocolEnable = CabletronProtocolEnable(1);
	pub const BridgeEnable: CabletronProtocolEnable = CabletronProtocolEnable(2);
	pub const IpBrEnable: CabletronProtocolEnable = CabletronProtocolEnable(3);
	pub const BrIpxEnable: CabletronProtocolEnable = CabletronProtocolEnable(6);
	pub const IpBrIpxEnable: CabletronProtocolEnable = CabletronProtocolEnable(7);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CabletronProtocolCallable(pub u32);
 
#[allow(non_upper_case_globals)]
impl CabletronProtocolCallable {
	pub const IpCallable: CabletronProtocolCallable = CabletronProtocolCallable(1);
	pub const BridgeCallable: CabletronProtocolCallable = CabletronProtocolCallable(2);
	pub const IpBrCallable: CabletronProtocolCallable = CabletronProtocolCallable(3);
	pub const BrIpxCallable: CabletronProtocolCallable = CabletronProtocolCallable(6);
	pub const IpBrIpxCallable: CabletronProtocolCallable = CabletronProtocolCallable(7);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		201 => map! {i, be_u32, |v| Attribute::VsaCabletronProtocolEnable(CabletronProtocolEnable(v))},
		202 => map! {i, be_u32, |v| Attribute::VsaCabletronProtocolCallable(CabletronProtocolCallable(v))},
        _ => value!(i, Attribute::VsaUnknown(52, typ, i)),
    }
}
