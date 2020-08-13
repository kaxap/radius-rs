/// Definitions for vendor F5, vendor value 3375
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct F5LtmUserRole(pub u32);
 
#[allow(non_upper_case_globals)]
impl F5LtmUserRole {
	pub const Administrator: F5LtmUserRole = F5LtmUserRole(0);
	pub const ResourceAdmin: F5LtmUserRole = F5LtmUserRole(20);
	pub const UserManager: F5LtmUserRole = F5LtmUserRole(40);
	pub const Auditor: F5LtmUserRole = F5LtmUserRole(80);
	pub const Manager: F5LtmUserRole = F5LtmUserRole(100);
	pub const AppEditor: F5LtmUserRole = F5LtmUserRole(300);
	pub const AdvancedOperator: F5LtmUserRole = F5LtmUserRole(350);
	pub const Operator: F5LtmUserRole = F5LtmUserRole(400);
	pub const FirewallManager: F5LtmUserRole = F5LtmUserRole(450);
	pub const FraudProtectionManager: F5LtmUserRole = F5LtmUserRole(480);
	pub const CertificateManager: F5LtmUserRole = F5LtmUserRole(500);
	pub const IruleManager: F5LtmUserRole = F5LtmUserRole(510);
	pub const Guest: F5LtmUserRole = F5LtmUserRole(700);
	pub const WebApplicationSecurityAdministrator: F5LtmUserRole = F5LtmUserRole(800);
	pub const WebApplicationSecurityEditor: F5LtmUserRole = F5LtmUserRole(810);
	pub const AccelerationPolicyEditor: F5LtmUserRole = F5LtmUserRole(850);
	pub const NoAccess: F5LtmUserRole = F5LtmUserRole(900);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct F5LtmUserRoleUniversal(pub u32);
 
#[allow(non_upper_case_globals)]
impl F5LtmUserRoleUniversal {
	pub const Disabled: F5LtmUserRoleUniversal = F5LtmUserRoleUniversal(0);
	pub const Enabled: F5LtmUserRoleUniversal = F5LtmUserRoleUniversal(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct F5LtmUserConsole(pub u32);
 
#[allow(non_upper_case_globals)]
impl F5LtmUserConsole {
	pub const Disabled: F5LtmUserConsole = F5LtmUserConsole(0);
	pub const Enabled: F5LtmUserConsole = F5LtmUserConsole(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map! {i, be_u32, |v| Attribute::VsaF5LtmUserRole(F5LtmUserRole(v))},
		2 => map! {i, be_u32, |v| Attribute::VsaF5LtmUserRoleUniversal(F5LtmUserRoleUniversal(v))},
		3 => value!(i, Attribute::VsaF5LtmUserPartition(i)),
		4 => map! {i, be_u32, |v| Attribute::VsaF5LtmUserConsole(F5LtmUserConsole(v))},
		5 => value!(i, Attribute::VsaF5LtmUserShell(i)),
		10 => map!{i, be_u32, |v| Attribute::VsaF5LtmUserContext1(v)},
		11 => map!{i, be_u32, |v| Attribute::VsaF5LtmUserContext2(v)},
		12 => value!(i, Attribute::VsaF5LtmUserInfo1(i)),
		13 => value!(i, Attribute::VsaF5LtmUserInfo2(i)),
		14 => value!(i, Attribute::VsaF5LtmAuditMsg(i)),
        _ => value!(i, Attribute::VsaUnknown(3375, typ, i)),
    }
}
