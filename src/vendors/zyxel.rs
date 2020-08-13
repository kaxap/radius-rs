/// Definitions for vendor Zyxel, vendor value 890
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZyxelCallbackPhoneSource(pub u32);
 
#[allow(non_upper_case_globals)]
impl ZyxelCallbackPhoneSource {
	pub const Preconfigured: ZyxelCallbackPhoneSource = ZyxelCallbackPhoneSource(0);
	pub const User: ZyxelCallbackPhoneSource = ZyxelCallbackPhoneSource(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZyxelCallbackOption(pub u32);
 
#[allow(non_upper_case_globals)]
impl ZyxelCallbackOption {
	pub const None: ZyxelCallbackOption = ZyxelCallbackOption(0);
	pub const Optional: ZyxelCallbackOption = ZyxelCallbackOption(1);
	pub const Mandatory: ZyxelCallbackOption = ZyxelCallbackOption(2);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		3 => value!(i, Attribute::VsaZyxelPrivilegeAvpair(i)),
		192 => map! {i, be_u32, |v| Attribute::VsaZyxelCallbackOption(ZyxelCallbackOption(v))},
		193 => map! {i, be_u32, |v| Attribute::VsaZyxelCallbackPhoneSource(ZyxelCallbackPhoneSource(v))},
        _ => value!(i, Attribute::VsaUnknown(890, typ, i)),
    }
}
