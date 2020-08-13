/// Definitions for vendor VerizonWireless, vendor value 12951
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		200 => map!{i, be_u32, |v| Attribute::VsaAcctInterimRecordNumber(v)},
		201 => map!{i, be_u32, |v| Attribute::VsaUeInfoType(v)},
		202 => value!(i, Attribute::VsaUeInfoValue(i)),
		203 => map!{i, be_u32, |v| Attribute::VsaDynamicAddressFlag(v)},
		204 => map!{i, be_u32, |v| Attribute::VsaLocalSeqNumber(v)},
		205 => map!{i, be_u32, |v| Attribute::VsaTimeFirstUsage(v)},
		206 => map!{i, be_u32, |v| Attribute::VsaTimeLastUsage(v)},
		207 => value!(i, Attribute::VsaChargingGroupId(i)),
		210 => value!(i, Attribute::VsaServiceDataContainerBin(i)),
		211 => value!(i, Attribute::VsaServiceDataContainer(i)),
        _ => value!(i, Attribute::VsaUnknown(12951, typ, i)),
    }
}
