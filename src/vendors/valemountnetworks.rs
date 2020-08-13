/// Definitions for vendor ValemountNetworks, vendor value 16313
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VncSplash(pub u32);
 
#[allow(non_upper_case_globals)]
impl VncSplash {
	pub const Show: VncSplash = VncSplash(1);
	pub const NoShow: VncSplash = VncSplash(0);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaVncPppoeCbqRx(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaVncPppoeCbqTx(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaVncPppoeCbqRxFallback(v)},
		4 => map!{i, be_u32, |v| Attribute::VsaVncPppoeCbqTxFallback(v)},
		10 => map! {i, be_u32, |v| Attribute::VsaVncSplash(VncSplash(v))},
        _ => value!(i, Attribute::VsaUnknown(16313, typ, i)),
    }
}
