/// Definitions for vendor Eltex, vendor value 35265
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EltexDisconnectCodeLocal(pub u32);
 
#[allow(non_upper_case_globals)]
impl EltexDisconnectCodeLocal {
	pub const UserAnswer: EltexDisconnectCodeLocal = EltexDisconnectCodeLocal(1);
	pub const IncompleteNumber: EltexDisconnectCodeLocal = EltexDisconnectCodeLocal(2);
	pub const UnassignedNumber: EltexDisconnectCodeLocal = EltexDisconnectCodeLocal(3);
	pub const UnsuccessfulOtherCause: EltexDisconnectCodeLocal = EltexDisconnectCodeLocal(4);
	pub const UserBusy: EltexDisconnectCodeLocal = EltexDisconnectCodeLocal(5);
	pub const OutOfOrder: EltexDisconnectCodeLocal = EltexDisconnectCodeLocal(6);
	pub const NoAnswer: EltexDisconnectCodeLocal = EltexDisconnectCodeLocal(7);
	pub const UnavailableTrunk: EltexDisconnectCodeLocal = EltexDisconnectCodeLocal(8);
	pub const AccessDenied: EltexDisconnectCodeLocal = EltexDisconnectCodeLocal(9);
	pub const UnavailableVoiceChannel: EltexDisconnectCodeLocal = EltexDisconnectCodeLocal(10);
	pub const RadiusServerUnavailable: EltexDisconnectCodeLocal = EltexDisconnectCodeLocal(11);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaEltexAvpair(i)),
		11 => map! {i, be_u32, |v| Attribute::VsaEltexDisconnectCodeLocal(EltexDisconnectCodeLocal(v))},
        _ => value!(i, Attribute::VsaUnknown(35265, typ, i)),
    }
}
