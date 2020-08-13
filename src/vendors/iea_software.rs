/// Definitions for vendor IEA-Software, vendor value 24023
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AmMirroring(pub u32);
 
#[allow(non_upper_case_globals)]
impl AmMirroring {
	pub const Disabled: AmMirroring = AmMirroring(0);
	pub const FullPacketLocalPcap: AmMirroring = AmMirroring(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AmDisconnectAccess(pub u32);
 
#[allow(non_upper_case_globals)]
impl AmDisconnectAccess {
	pub const AllowAll: AmDisconnectAccess = AmDisconnectAccess(0);
	pub const DenyRadius: AmDisconnectAccess = AmDisconnectAccess(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaAmInterruptHtmlfile(i)),
		2 => map!{i, be_u32, |v| Attribute::VsaAmInterruptInterval(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaAmInterruptTimeout(v)},
		4 => value!(i, Attribute::VsaAmStatusHtmlfile(i)),
		5 => map!{i, be_u32, |v| Attribute::VsaAmHttpProxyPort(v)},
		6 => value!(i, Attribute::VsaAmAckHtmlfile(i)),
		7 => value!(i, Attribute::VsaAmNakHtmlfile(i)),
		8 => value!(i, Attribute::VsaAmBandwidthPool(i)),
		9 => map!{i, be_u32, |v| Attribute::VsaAmBandwidthPoolMaxUp(v)},
		10 => map!{i, be_u32, |v| Attribute::VsaAmBandwidthPoolMaxDown(v)},
		11 => map! {i, be_u32, |v| Attribute::VsaAmMirroring(AmMirroring(v))},
		12 => map! {i, be_u32, |v| Attribute::VsaAmDisconnectAccess(AmDisconnectAccess(v))},
        _ => value!(i, Attribute::VsaUnknown(24023, typ, i)),
    }
}
