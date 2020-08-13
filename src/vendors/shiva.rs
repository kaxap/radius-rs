/// Definitions for vendor Shiva, vendor value 166
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShivaCompression(pub u32);
 
#[allow(non_upper_case_globals)]
impl ShivaCompression {
	pub const None: ShivaCompression = ShivaCompression(0);
	pub const Negotiate: ShivaCompression = ShivaCompression(1);
	pub const Spider: ShivaCompression = ShivaCompression(2);
	pub const Predictor: ShivaCompression = ShivaCompression(3);
	pub const Stac: ShivaCompression = ShivaCompression(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShivaCircuitType(pub u32);
 
#[allow(non_upper_case_globals)]
impl ShivaCircuitType {
	pub const Primary: ShivaCircuitType = ShivaCircuitType(1);
	pub const SecondaryBackup: ShivaCircuitType = ShivaCircuitType(2);
	pub const SecondaryAugment: ShivaCircuitType = ShivaCircuitType(3);
	pub const SecondarySwitch: ShivaCircuitType = ShivaCircuitType(4);
	pub const Listener: ShivaCircuitType = ShivaCircuitType(5);
	pub const Radius: ShivaCircuitType = ShivaCircuitType(6);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShivaTypeOfService(pub u32);
 
#[allow(non_upper_case_globals)]
impl ShivaTypeOfService {
	pub const Analog: ShivaTypeOfService = ShivaTypeOfService(1);
	pub const DigitizedAnalog: ShivaTypeOfService = ShivaTypeOfService(2);
	pub const Digital: ShivaTypeOfService = ShivaTypeOfService(3);
	pub const DigitalV110: ShivaTypeOfService = ShivaTypeOfService(4);
	pub const DigitalV120: ShivaTypeOfService = ShivaTypeOfService(5);
	pub const DigitalLeasedLine: ShivaTypeOfService = ShivaTypeOfService(6);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShivaLinkProtocol(pub u32);
 
#[allow(non_upper_case_globals)]
impl ShivaLinkProtocol {
	pub const Hdlc: ShivaLinkProtocol = ShivaLinkProtocol(1);
	pub const Arav1: ShivaLinkProtocol = ShivaLinkProtocol(2);
	pub const Arav2: ShivaLinkProtocol = ShivaLinkProtocol(3);
	pub const Shell: ShivaLinkProtocol = ShivaLinkProtocol(4);
	pub const Aalap: ShivaLinkProtocol = ShivaLinkProtocol(5);
	pub const Slip: ShivaLinkProtocol = ShivaLinkProtocol(6);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShivaConnectReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl ShivaConnectReason {
	pub const Remote: ShivaConnectReason = ShivaConnectReason(1);
	pub const Dialback: ShivaConnectReason = ShivaConnectReason(2);
	pub const VirtualConnection: ShivaConnectReason = ShivaConnectReason(3);
	pub const BandwidthOnDemand: ShivaConnectReason = ShivaConnectReason(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShivaDisconnectReason(pub u32);
 
#[allow(non_upper_case_globals)]
impl ShivaDisconnectReason {
	pub const Remote: ShivaDisconnectReason = ShivaDisconnectReason(1);
	pub const Error: ShivaDisconnectReason = ShivaDisconnectReason(2);
	pub const IdleTimeout: ShivaDisconnectReason = ShivaDisconnectReason(3);
	pub const SessionTimeout: ShivaDisconnectReason = ShivaDisconnectReason(4);
	pub const AdminDisconnect: ShivaDisconnectReason = ShivaDisconnectReason(5);
	pub const Dialback: ShivaDisconnectReason = ShivaDisconnectReason(6);
	pub const VirtualConnection: ShivaDisconnectReason = ShivaDisconnectReason(7);
	pub const BandwidthOnDemand: ShivaDisconnectReason = ShivaDisconnectReason(8);
	pub const FailedAuthentication: ShivaDisconnectReason = ShivaDisconnectReason(9);
	pub const Preempted: ShivaDisconnectReason = ShivaDisconnectReason(10);
	pub const Blocked: ShivaDisconnectReason = ShivaDisconnectReason(11);
	pub const TariffManagement: ShivaDisconnectReason = ShivaDisconnectReason(12);
	pub const Backup: ShivaDisconnectReason = ShivaDisconnectReason(13);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShivaFunction(pub u32);
 
#[allow(non_upper_case_globals)]
impl ShivaFunction {
	pub const Unknown: ShivaFunction = ShivaFunction(0);
	pub const Dialin: ShivaFunction = ShivaFunction(1);
	pub const Dialout: ShivaFunction = ShivaFunction(2);
	pub const LanToLan: ShivaFunction = ShivaFunction(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaShivaUserAttributes(i)),
		30 => map! {i, be_u32, |v| Attribute::VsaShivaCompression(ShivaCompression(v))},
		31 => map!{i, be_u32, |v| Attribute::VsaShivaDialbackDelay(v)},
		32 => map!{i, be_u32, |v| Attribute::VsaShivaCallDurnTrap(v)},
		33 => map!{i, be_u32, |v| Attribute::VsaShivaBandwidthTrap(v)},
		34 => map!{i, be_u32, |v| Attribute::VsaShivaMinimumCall(v)},
		35 => value!(i, Attribute::VsaShivaDefaultHost(i)),
		36 => value!(i, Attribute::VsaShivaMenuName(i)),
		37 => value!(i, Attribute::VsaShivaUserFlags(i)),
		38 => value!(i, Attribute::VsaShivaTermtype(i)),
		39 => value!(i, Attribute::VsaShivaBreakKey(i)),
		40 => value!(i, Attribute::VsaShivaFwdKey(i)),
		41 => value!(i, Attribute::VsaShivaBakKey(i)),
		42 => map!{i, be_u32, |v| Attribute::VsaShivaDialTimeout(v)},
		43 => value!(i, Attribute::VsaShivaLatPort(i)),
		44 => map!{i, be_u32, |v| Attribute::VsaShivaMaxVcs(v)},
		45 => map!{i, be_u32, |v| Attribute::VsaShivaDhcpLeasetime(v)},
		46 => value!(i, Attribute::VsaShivaLatGroups(i)),
		60 => map!{i, be_u32, |v| Attribute::VsaShivaRtcTimestamp(v)},
		61 => map! {i, be_u32, |v| Attribute::VsaShivaCircuitType(ShivaCircuitType(v))},
		90 => value!(i, Attribute::VsaShivaCalledNumber(i)),
		91 => value!(i, Attribute::VsaShivaCallingNumber(i)),
		92 => value!(i, Attribute::VsaShivaCustomerId(i)),
		93 => map! {i, be_u32, |v| Attribute::VsaShivaTypeOfService(ShivaTypeOfService(v))},
		94 => map!{i, be_u32, |v| Attribute::VsaShivaLinkSpeed(v)},
		95 => map!{i, be_u32, |v| Attribute::VsaShivaLinksInBundle(v)},
		96 => map!{i, be_u32, |v| Attribute::VsaShivaCompressionType(v)},
		97 => map! {i, be_u32, |v| Attribute::VsaShivaLinkProtocol(ShivaLinkProtocol(v))},
		98 => map!{i, be_u32, |v| Attribute::VsaShivaNetworkProtocols(v)},
		99 => map!{i, be_u32, |v| Attribute::VsaShivaSessionId(v)},
		100 => map! {i, be_u32, |v| Attribute::VsaShivaDisconnectReason(ShivaDisconnectReason(v))},
		101 => map!{i, take!(4), |v:&[u8]| Attribute::VsaShivaAcctServSwitch(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		102 => map!{i, be_u32, |v| Attribute::VsaShivaEventFlags(v)},
		103 => map! {i, be_u32, |v| Attribute::VsaShivaFunction(ShivaFunction(v))},
		104 => map! {i, be_u32, |v| Attribute::VsaShivaConnectReason(ShivaConnectReason(v))},
        _ => value!(i, Attribute::VsaUnknown(166, typ, i)),
    }
}
