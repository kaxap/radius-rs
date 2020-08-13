/// Definitions for vendor Digium, vendor value 22736
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		101 => value!(i, Attribute::VsaAsteriskAccCode(i)),
		102 => value!(i, Attribute::VsaAsteriskSrc(i)),
		103 => value!(i, Attribute::VsaAsteriskDst(i)),
		104 => value!(i, Attribute::VsaAsteriskDstCtx(i)),
		105 => value!(i, Attribute::VsaAsteriskClid(i)),
		106 => value!(i, Attribute::VsaAsteriskChan(i)),
		107 => value!(i, Attribute::VsaAsteriskDstChan(i)),
		108 => value!(i, Attribute::VsaAsteriskLastApp(i)),
		109 => value!(i, Attribute::VsaAsteriskLastData(i)),
		110 => value!(i, Attribute::VsaAsteriskStartTime(i)),
		111 => value!(i, Attribute::VsaAsteriskAnswerTime(i)),
		112 => value!(i, Attribute::VsaAsteriskEndTime(i)),
		113 => map!{i, be_u32, |v| Attribute::VsaAsteriskDuration(v)},
		114 => map!{i, be_u32, |v| Attribute::VsaAsteriskBillSec(v)},
		115 => value!(i, Attribute::VsaAsteriskDisposition(i)),
		116 => value!(i, Attribute::VsaAsteriskAmaFlags(i)),
		117 => value!(i, Attribute::VsaAsteriskUniqueId(i)),
		118 => value!(i, Attribute::VsaAsteriskUserField(i)),
        _ => value!(i, Attribute::VsaUnknown(22736, typ, i)),
    }
}
