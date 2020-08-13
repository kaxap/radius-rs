/// Definitions for vendor Local-Web, vendor value 19220
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		192 => value!(i, Attribute::VsaLocalWebClientIp(i)),
		193 => value!(i, Attribute::VsaLocalWebBorderRouter(i)),
		200 => map!{i, be_u32, |v| Attribute::VsaLocalWebTxLimit(v)},
		201 => map!{i, be_u32, |v| Attribute::VsaLocalWebRxLimit(v)},
		210 => map!{i, be_u32, |v| Attribute::VsaLocalWebAcctTime(v)},
		211 => map!{i, be_u32, |v| Attribute::VsaLocalWebAcctDuration(v)},
		212 => map!{i, be_u32, |v| Attribute::VsaLocalWebAcctInterimTxBytes(v)},
		213 => map!{i, be_u32, |v| Attribute::VsaLocalWebAcctInterimRxBytes(v)},
		214 => map!{i, be_u32, |v| Attribute::VsaLocalWebAcctInterimTxGigawords(v)},
		215 => map!{i, be_u32, |v| Attribute::VsaLocalWebAcctInterimRxGigawords(v)},
		216 => map!{i, be_u32, |v| Attribute::VsaLocalWebAcctInterimTxMgmt(v)},
		217 => map!{i, be_u32, |v| Attribute::VsaLocalWebAcctInterimRxMgmt(v)},
		230 => map!{i, be_u32, |v| Attribute::VsaLocalWebAcctTxMgmt(v)},
		231 => map!{i, be_u32, |v| Attribute::VsaLocalWebAcctRxMgmt(v)},
		240 => map!{i, be_u32, |v| Attribute::VsaLocalWebReauthCounter(v)},
        _ => value!(i, Attribute::VsaUnknown(19220, typ, i)),
    }
}
