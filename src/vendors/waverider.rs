/// Definitions for vendor Waverider, vendor value 2979
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaveriderGradeOfService(pub u32);
 
#[allow(non_upper_case_globals)]
impl WaveriderGradeOfService {
	pub const Be: WaveriderGradeOfService = WaveriderGradeOfService(1);
	pub const Bronze: WaveriderGradeOfService = WaveriderGradeOfService(2);
	pub const Silver: WaveriderGradeOfService = WaveriderGradeOfService(3);
	pub const Gold: WaveriderGradeOfService = WaveriderGradeOfService(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaveriderPriorityEnabled(pub u32);
 
#[allow(non_upper_case_globals)]
impl WaveriderPriorityEnabled {
	pub const Disabled: WaveriderPriorityEnabled = WaveriderPriorityEnabled(0);
	pub const Enabled: WaveriderPriorityEnabled = WaveriderPriorityEnabled(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaveriderRadioFrequency(pub u32);
 
#[allow(non_upper_case_globals)]
impl WaveriderRadioFrequency {
	pub const Auto: WaveriderRadioFrequency = WaveriderRadioFrequency(1);
	pub const Nomadic: WaveriderRadioFrequency = WaveriderRadioFrequency(2);
	pub const F9050: WaveriderRadioFrequency = WaveriderRadioFrequency(3);
	pub const F9116: WaveriderRadioFrequency = WaveriderRadioFrequency(4);
	pub const F9184: WaveriderRadioFrequency = WaveriderRadioFrequency(5);
	pub const F9250: WaveriderRadioFrequency = WaveriderRadioFrequency(6);
	pub const F9084: WaveriderRadioFrequency = WaveriderRadioFrequency(7);
	pub const F9150: WaveriderRadioFrequency = WaveriderRadioFrequency(8);
	pub const F9216: WaveriderRadioFrequency = WaveriderRadioFrequency(9);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaveriderRfPower(pub u32);
 
#[allow(non_upper_case_globals)]
impl WaveriderRfPower {
	pub const P15: WaveriderRfPower = WaveriderRfPower(1);
	pub const P16: WaveriderRfPower = WaveriderRfPower(2);
	pub const P17: WaveriderRfPower = WaveriderRfPower(3);
	pub const P18: WaveriderRfPower = WaveriderRfPower(4);
	pub const P19: WaveriderRfPower = WaveriderRfPower(5);
	pub const P20: WaveriderRfPower = WaveriderRfPower(6);
	pub const P21: WaveriderRfPower = WaveriderRfPower(7);
	pub const P22: WaveriderRfPower = WaveriderRfPower(8);
	pub const P23: WaveriderRfPower = WaveriderRfPower(9);
	pub const P24: WaveriderRfPower = WaveriderRfPower(10);
	pub const P25: WaveriderRfPower = WaveriderRfPower(11);
	pub const P26: WaveriderRfPower = WaveriderRfPower(12);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaWaveriderGradeOfService(WaveriderGradeOfService(v))},
		2 => map! {i, be_u32, |v| Attribute::VsaWaveriderPriorityEnabled(WaveriderPriorityEnabled(v))},
		3 => value!(i, Attribute::VsaWaveriderAuthenticationKey(i)),
		5 => value!(i, Attribute::VsaWaveriderCurrentPassword(i)),
		6 => value!(i, Attribute::VsaWaveriderNewPassword(i)),
		7 => map! {i, be_u32, |v| Attribute::VsaWaveriderRadioFrequency(WaveriderRadioFrequency(v))},
		8 => value!(i, Attribute::VsaWaveriderSnmpReadCommunity(i)),
		9 => value!(i, Attribute::VsaWaveriderSnmpWriteCommunity(i)),
		10 => value!(i, Attribute::VsaWaveriderSnmpTrapServer(i)),
		11 => value!(i, Attribute::VsaWaveriderSnmpContact(i)),
		12 => value!(i, Attribute::VsaWaveriderSnmpLocation(i)),
		13 => value!(i, Attribute::VsaWaveriderSnmpName(i)),
		14 => map!{i, be_u32, |v| Attribute::VsaWaveriderMaxCustomers(v)},
		15 => map! {i, be_u32, |v| Attribute::VsaWaveriderRfPower(WaveriderRfPower(v))},
        _ => value!(i, Attribute::VsaUnknown(2979, typ, i)),
    }
}
