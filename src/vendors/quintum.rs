/// Definitions for vendor Quintum, vendor value 6618
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaQuintumAvpair(i)),
		2 => value!(i, Attribute::VsaQuintumNasPort(i)),
		23 => value!(i, Attribute::VsaQuintumH323RemoteAddress(i)),
		24 => value!(i, Attribute::VsaQuintumH323ConfId(i)),
		25 => value!(i, Attribute::VsaQuintumH323SetupTime(i)),
		26 => value!(i, Attribute::VsaQuintumH323CallOrigin(i)),
		27 => value!(i, Attribute::VsaQuintumH323CallType(i)),
		28 => value!(i, Attribute::VsaQuintumH323ConnectTime(i)),
		29 => value!(i, Attribute::VsaQuintumH323DisconnectTime(i)),
		30 => value!(i, Attribute::VsaQuintumH323DisconnectCause(i)),
		31 => value!(i, Attribute::VsaQuintumH323VoiceQuality(i)),
		33 => value!(i, Attribute::VsaQuintumH323GwId(i)),
		35 => value!(i, Attribute::VsaQuintumH323IncomingConfId(i)),
		101 => value!(i, Attribute::VsaQuintumH323CreditAmount(i)),
		102 => value!(i, Attribute::VsaQuintumH323CreditTime(i)),
		103 => value!(i, Attribute::VsaQuintumH323ReturnCode(i)),
		104 => value!(i, Attribute::VsaQuintumH323PromptId(i)),
		105 => value!(i, Attribute::VsaQuintumH323TimeAndDay(i)),
		106 => value!(i, Attribute::VsaQuintumH323RedirectNumber(i)),
		107 => value!(i, Attribute::VsaQuintumH323PreferredLang(i)),
		108 => value!(i, Attribute::VsaQuintumH323RedirectIpAddress(i)),
		109 => value!(i, Attribute::VsaQuintumH323BillingModel(i)),
		110 => value!(i, Attribute::VsaQuintumH323CurrencyType(i)),
		230 => value!(i, Attribute::VsaQuintumTrunkidIn(i)),
		231 => value!(i, Attribute::VsaQuintumTrunkidOut(i)),
        _ => value!(i, Attribute::VsaUnknown(6618, typ, i)),
    }
}
