/// Definitions for vendor Nortel, vendor value 562
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NortelPrivilegeLevel(pub u32);
 
#[allow(non_upper_case_globals)]
impl NortelPrivilegeLevel {
	pub const Voicemailadmin: NortelPrivilegeLevel = NortelPrivilegeLevel(0);
	pub const Contactcenter: NortelPrivilegeLevel = NortelPrivilegeLevel(1);
	pub const Sbainstaller: NortelPrivilegeLevel = NortelPrivilegeLevel(2);
	pub const Sbasystemcoord: NortelPrivilegeLevel = NortelPrivilegeLevel(3);
	pub const Sbasystemcoordbasic: NortelPrivilegeLevel = NortelPrivilegeLevel(4);
	pub const Sbabasic: NortelPrivilegeLevel = NortelPrivilegeLevel(5);
	pub const Security: NortelPrivilegeLevel = NortelPrivilegeLevel(6);
	pub const Cteapp: NortelPrivilegeLevel = NortelPrivilegeLevel(7);
	pub const SbaIpsetregistration: NortelPrivilegeLevel = NortelPrivilegeLevel(8);
	pub const ApplicationBcmmonitor: NortelPrivilegeLevel = NortelPrivilegeLevel(9);
	pub const Cdrapp: NortelPrivilegeLevel = NortelPrivilegeLevel(10);
	pub const Modemlogin: NortelPrivilegeLevel = NortelPrivilegeLevel(11);
	pub const Guestlogin: NortelPrivilegeLevel = NortelPrivilegeLevel(12);
	pub const Admindownload: NortelPrivilegeLevel = NortelPrivilegeLevel(13);
	pub const Exclusiveaccess: NortelPrivilegeLevel = NortelPrivilegeLevel(14);
	pub const Admin: NortelPrivilegeLevel = NortelPrivilegeLevel(15);
	pub const Dataadmin: NortelPrivilegeLevel = NortelPrivilegeLevel(16);
	pub const Remoteaccess: NortelPrivilegeLevel = NortelPrivilegeLevel(17);
	pub const Guest: NortelPrivilegeLevel = NortelPrivilegeLevel(18);
	pub const Voiceadmin: NortelPrivilegeLevel = NortelPrivilegeLevel(19);
	pub const Backupoperator: NortelPrivilegeLevel = NortelPrivilegeLevel(20);
	pub const Remotemonitoring: NortelPrivilegeLevel = NortelPrivilegeLevel(21);
	pub const Softwareupgrade: NortelPrivilegeLevel = NortelPrivilegeLevel(22);
	pub const Alarmviewer: NortelPrivilegeLevel = NortelPrivilegeLevel(24);
	pub const Operationallogs: NortelPrivilegeLevel = NortelPrivilegeLevel(26);
	pub const Diagnosticlogs: NortelPrivilegeLevel = NortelPrivilegeLevel(27);
	pub const Applicationivr: NortelPrivilegeLevel = NortelPrivilegeLevel(28);
	pub const IsdnDialIn: NortelPrivilegeLevel = NortelPrivilegeLevel(30);
	pub const WanDialIn: NortelPrivilegeLevel = NortelPrivilegeLevel(32);
	pub const SystemSerialport: NortelPrivilegeLevel = NortelPrivilegeLevel(36);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		110 => value!(i, Attribute::VsaNortelUserRole(i)),
		166 => map! {i, be_u32, |v| Attribute::VsaNortelPrivilegeLevel(NortelPrivilegeLevel(v))},
		200 => map!{i, be_u32, |v| Attribute::VsaPassportCommandScope(v)},
		201 => map!{i, be_u32, |v| Attribute::VsaPassportCommandImpact(v)},
		202 => map!{i, be_u32, |v| Attribute::VsaPassportCustomerIdentifier(v)},
		203 => map!{i, be_u32, |v| Attribute::VsaPassportAllowedAccess(v)},
		204 => map!{i, be_u32, |v| Attribute::VsaPassportAllowedoutAccess(v)},
		205 => value!(i, Attribute::VsaPassportLoginDirectory(i)),
		206 => map!{i, be_u32, |v| Attribute::VsaPassportTimeoutProtocol(v)},
		207 => value!(i, Attribute::VsaPassportRole(i)),
        _ => value!(i, Attribute::VsaUnknown(562, typ, i)),
    }
}
