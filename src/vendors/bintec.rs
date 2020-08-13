/// Definitions for vendor BinTec, vendor value 272
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		224 => value!(i, Attribute::VsaBintecBiboppptable(i)),
		225 => value!(i, Attribute::VsaBintecBibodialtable(i)),
		226 => value!(i, Attribute::VsaBintecIpextiftable(i)),
		227 => value!(i, Attribute::VsaBintecIproutetable(i)),
		228 => value!(i, Attribute::VsaBintecIpextrttable(i)),
		229 => value!(i, Attribute::VsaBintecIpnatpresettable(i)),
		230 => value!(i, Attribute::VsaBintecIpxcirctable(i)),
		231 => value!(i, Attribute::VsaBintecRipcirctable(i)),
		232 => value!(i, Attribute::VsaBintecSapcirctable(i)),
		233 => value!(i, Attribute::VsaBintecIpxstaticroutetable(i)),
		234 => value!(i, Attribute::VsaBintecIpxstaticservtable(i)),
		235 => value!(i, Attribute::VsaBintecOspfiftable(i)),
		236 => value!(i, Attribute::VsaBintecPppextiftable(i)),
		237 => value!(i, Attribute::VsaBintecIpfiltertable(i)),
		238 => value!(i, Attribute::VsaBintecIpqostable(i)),
		239 => value!(i, Attribute::VsaBintecQosiftable(i)),
		240 => value!(i, Attribute::VsaBintecQospolicytable(i)),
        _ => value!(i, Attribute::VsaUnknown(272, typ, i)),
    }
}
