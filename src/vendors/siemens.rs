/// Definitions for vendor Siemens, vendor value 4329
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaSiemensUrlRedirection(i)),
		2 => value!(i, Attribute::VsaSiemensApName(i)),
		3 => value!(i, Attribute::VsaSiemensApSerial(i)),
		4 => value!(i, Attribute::VsaSiemensVnsName(i)),
		5 => value!(i, Attribute::VsaSiemensSsid(i)),
		6 => value!(i, Attribute::VsaSiemensBssMac(i)),
		7 => value!(i, Attribute::VsaSiemensPolicyName(i)),
		8 => value!(i, Attribute::VsaSiemensTopologyName(i)),
		9 => value!(i, Attribute::VsaSiemensIngressRcName(i)),
		10 => value!(i, Attribute::VsaSiemensEgressRcName(i)),
        _ => value!(i, Attribute::VsaUnknown(4329, typ, i)),
    }
}
