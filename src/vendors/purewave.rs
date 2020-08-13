/// Definitions for vendor Purewave, vendor value 21074
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PurewaveCsType(pub u32);
 
#[allow(non_upper_case_globals)]
impl PurewaveCsType {
	pub const EthernetCs: PurewaveCsType = PurewaveCsType(1);
	pub const Ipv4Cs: PurewaveCsType = PurewaveCsType(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PurewaveMaxDownlinkRate(pub u32);
 
#[allow(non_upper_case_globals)]
impl PurewaveMaxDownlinkRate {
	pub const Qpsk1Or2: PurewaveMaxDownlinkRate = PurewaveMaxDownlinkRate(3);
	pub const Qpsk3Or4: PurewaveMaxDownlinkRate = PurewaveMaxDownlinkRate(4);
	pub const Qam161Or2: PurewaveMaxDownlinkRate = PurewaveMaxDownlinkRate(5);
	pub const Qam163Or4: PurewaveMaxDownlinkRate = PurewaveMaxDownlinkRate(6);
	pub const Qam641Or2: PurewaveMaxDownlinkRate = PurewaveMaxDownlinkRate(7);
	pub const Qam642Or3: PurewaveMaxDownlinkRate = PurewaveMaxDownlinkRate(8);
	pub const Qam643Or4: PurewaveMaxDownlinkRate = PurewaveMaxDownlinkRate(9);
	pub const Qam645Or6: PurewaveMaxDownlinkRate = PurewaveMaxDownlinkRate(10);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PurewaveMaxUplinkRate(pub u32);
 
#[allow(non_upper_case_globals)]
impl PurewaveMaxUplinkRate {
	pub const Qpsk1Or2: PurewaveMaxUplinkRate = PurewaveMaxUplinkRate(3);
	pub const Qpsk3Or4: PurewaveMaxUplinkRate = PurewaveMaxUplinkRate(4);
	pub const Qam161Or2: PurewaveMaxUplinkRate = PurewaveMaxUplinkRate(5);
	pub const Qam163Or4: PurewaveMaxUplinkRate = PurewaveMaxUplinkRate(6);
	pub const Qam641Or2: PurewaveMaxUplinkRate = PurewaveMaxUplinkRate(7);
	pub const Qam642Or3: PurewaveMaxUplinkRate = PurewaveMaxUplinkRate(8);
	pub const Qam643Or4: PurewaveMaxUplinkRate = PurewaveMaxUplinkRate(9);
	pub const Qam645Or6: PurewaveMaxUplinkRate = PurewaveMaxUplinkRate(10);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaPurewaveClientProfile(v)},
		2 => map! {i, be_u32, |v| Attribute::VsaPurewaveCsType(PurewaveCsType(v))},
		3 => map! {i, be_u32, |v| Attribute::VsaPurewaveMaxDownlinkRate(PurewaveMaxDownlinkRate(v))},
		4 => map! {i, be_u32, |v| Attribute::VsaPurewaveMaxUplinkRate(PurewaveMaxUplinkRate(v))},
		5 => map!{i, take!(4), |v:&[u8]| Attribute::VsaPurewaveIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		6 => map!{i, take!(4), |v:&[u8]| Attribute::VsaPurewaveIpNetmask(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		7 => map!{i, be_u32, |v| Attribute::VsaPurewaveServiceEnable(v)},
        _ => value!(i, Attribute::VsaUnknown(21074, typ, i)),
    }
}
