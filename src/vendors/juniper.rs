/// Definitions for vendor Juniper, vendor value 2636
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JuniperCtpviewAppGroup(pub u32);
 
#[allow(non_upper_case_globals)]
impl JuniperCtpviewAppGroup {
	pub const NetView: JuniperCtpviewAppGroup = JuniperCtpviewAppGroup(1);
	pub const NetAdmin: JuniperCtpviewAppGroup = JuniperCtpviewAppGroup(2);
	pub const GlobalAdmin: JuniperCtpviewAppGroup = JuniperCtpviewAppGroup(3);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JuniperCtpGroup(pub u32);
 
#[allow(non_upper_case_globals)]
impl JuniperCtpGroup {
	pub const ReadOnly: JuniperCtpGroup = JuniperCtpGroup(1);
	pub const Admin: JuniperCtpGroup = JuniperCtpGroup(2);
	pub const PrivilegedAdmin: JuniperCtpGroup = JuniperCtpGroup(3);
	pub const Auditor: JuniperCtpGroup = JuniperCtpGroup(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JuniperCtpviewOsGroup(pub u32);
 
#[allow(non_upper_case_globals)]
impl JuniperCtpviewOsGroup {
	pub const WebManager: JuniperCtpviewOsGroup = JuniperCtpviewOsGroup(1);
	pub const SystemAdmin: JuniperCtpviewOsGroup = JuniperCtpviewOsGroup(2);
	pub const Auditor: JuniperCtpviewOsGroup = JuniperCtpviewOsGroup(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaJuniperLocalUserName(i)),
		2 => value!(i, Attribute::VsaJuniperAllowCommands(i)),
		3 => value!(i, Attribute::VsaJuniperDenyCommands(i)),
		4 => value!(i, Attribute::VsaJuniperAllowConfiguration(i)),
		5 => value!(i, Attribute::VsaJuniperDenyConfiguration(i)),
		8 => value!(i, Attribute::VsaJuniperInteractiveCommand(i)),
		9 => value!(i, Attribute::VsaJuniperConfigurationChange(i)),
		10 => value!(i, Attribute::VsaJuniperUserPermissions(i)),
		11 => value!(i, Attribute::VsaJuniperJunosspaceProfile(i)),
		21 => map! {i, be_u32, |v| Attribute::VsaJuniperCtpGroup(JuniperCtpGroup(v))},
		22 => map! {i, be_u32, |v| Attribute::VsaJuniperCtpviewAppGroup(JuniperCtpviewAppGroup(v))},
		23 => map! {i, be_u32, |v| Attribute::VsaJuniperCtpviewOsGroup(JuniperCtpviewOsGroup(v))},
		31 => map!{i, take!(4), |v:&[u8]| Attribute::VsaJuniperPrimaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		32 => map!{i, take!(4), |v:&[u8]| Attribute::VsaJuniperPrimaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		33 => map!{i, take!(4), |v:&[u8]| Attribute::VsaJuniperSecondaryDns(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		34 => map!{i, take!(4), |v:&[u8]| Attribute::VsaJuniperSecondaryWins(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		35 => value!(i, Attribute::VsaJuniperInterfaceId(i)),
		36 => value!(i, Attribute::VsaJuniperIpPoolName(i)),
		37 => map!{i, be_u32, |v| Attribute::VsaJuniperKeepAlive(v)},
		38 => value!(i, Attribute::VsaJuniperCosTrafficControlProfile(i)),
		39 => value!(i, Attribute::VsaJuniperCosParameter(i)),
		40 => map!{i, be_u32, |v| Attribute::VsaJuniperEncapsulationOverhead(v)},
		41 => map!{i, be_u32, |v| Attribute::VsaJuniperCellOverhead(v)},
		42 => map!{i, be_u32, |v| Attribute::VsaJuniperTxConnectSpeed(v)},
		43 => map!{i, be_u32, |v| Attribute::VsaJuniperRxConnectSpeed(v)},
		44 => value!(i, Attribute::VsaJuniperFirewallFilterName(i)),
		45 => value!(i, Attribute::VsaJuniperPolicerParameter(i)),
		46 => value!(i, Attribute::VsaJuniperLocalGroupName(i)),
		47 => value!(i, Attribute::VsaJuniperLocalInterface(i)),
		48 => value!(i, Attribute::VsaJuniperSwitchingFilter(i)),
		49 => value!(i, Attribute::VsaJuniperVoipVlan(i)),
        _ => value!(i, Attribute::VsaUnknown(2636, typ, i)),
    }
}
