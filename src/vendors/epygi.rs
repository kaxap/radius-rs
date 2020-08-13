/// Definitions for vendor Epygi, vendor value 16459
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EpygiCalltype(pub u32);
 
#[allow(non_upper_case_globals)]
impl EpygiCalltype {
	pub const Internal: EpygiCalltype = EpygiCalltype(0);
	pub const Sip: EpygiCalltype = EpygiCalltype(1);
	pub const Hdot323: EpygiCalltype = EpygiCalltype(2);
	pub const Fxo: EpygiCalltype = EpygiCalltype(3);
	pub const T1E1Cas: EpygiCalltype = EpygiCalltype(4);
	pub const T1E1Ccs: EpygiCalltype = EpygiCalltype(5);
	pub const IsdnPri: EpygiCalltype = EpygiCalltype(6);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EpygiInterfacename(pub u32);
 
#[allow(non_upper_case_globals)]
impl EpygiInterfacename {
	pub const Ethernet: EpygiInterfacename = EpygiInterfacename(0);
	pub const Fxo: EpygiInterfacename = EpygiInterfacename(1);
	pub const T1E1User: EpygiInterfacename = EpygiInterfacename(2);
	pub const T1E1Network: EpygiInterfacename = EpygiInterfacename(3);
	pub const Isdn: EpygiInterfacename = EpygiInterfacename(4);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EpygiCallredirectreason(pub u32);
 
#[allow(non_upper_case_globals)]
impl EpygiCallredirectreason {
	pub const NoReason: EpygiCallredirectreason = EpygiCallredirectreason(0);
	pub const CallForwardUncondit: EpygiCallredirectreason = EpygiCallredirectreason(1);
	pub const CallForwardBusy: EpygiCallredirectreason = EpygiCallredirectreason(2);
	pub const CallForwardNoanswer: EpygiCallredirectreason = EpygiCallredirectreason(3);
	pub const CallTranfer: EpygiCallredirectreason = EpygiCallredirectreason(4);
	pub const CallPark: EpygiCallredirectreason = EpygiCallredirectreason(5);
	pub const CallPickup: EpygiCallredirectreason = EpygiCallredirectreason(6);
	pub const ManyextensionRinging: EpygiCallredirectreason = EpygiCallredirectreason(7);
	pub const HuntGroup: EpygiCallredirectreason = EpygiCallredirectreason(8);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EpygiCalldisconnectreason(pub u32);
 
#[allow(non_upper_case_globals)]
impl EpygiCalldisconnectreason {
	pub const CallIsRedirected: EpygiCalldisconnectreason = EpygiCalldisconnectreason(0);
	pub const CallOriginOnhook: EpygiCalldisconnectreason = EpygiCalldisconnectreason(1);
	pub const CallTeminOnhook: EpygiCalldisconnectreason = EpygiCalldisconnectreason(2);
	pub const DisconectedByCac: EpygiCalldisconnectreason = EpygiCalldisconnectreason(3);
	pub const Other: EpygiCalldisconnectreason = EpygiCalldisconnectreason(4);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaEpygiAvpair(i)),
		2 => value!(i, Attribute::VsaEpygiNasPort(i)),
		23 => value!(i, Attribute::VsaEpygiH323RemoteAddress(i)),
		24 => value!(i, Attribute::VsaEpygiH323ConfId(i)),
		25 => value!(i, Attribute::VsaEpygiH323SetupTime(i)),
		26 => value!(i, Attribute::VsaEpygiH323CallOrigin(i)),
		27 => value!(i, Attribute::VsaEpygiH323CallType(i)),
		28 => value!(i, Attribute::VsaEpygiH323ConnectTime(i)),
		29 => value!(i, Attribute::VsaEpygiH323DisconnectTime(i)),
		30 => value!(i, Attribute::VsaEpygiH323DisconnectCause(i)),
		31 => value!(i, Attribute::VsaEpygiH323VoiceQuality(i)),
		33 => value!(i, Attribute::VsaEpygiH323GwId(i)),
		35 => value!(i, Attribute::VsaEpygiH323IncomingConfId(i)),
		101 => value!(i, Attribute::VsaEpygiH323CreditAmount(i)),
		102 => value!(i, Attribute::VsaEpygiH323CreditTime(i)),
		103 => value!(i, Attribute::VsaEpygiH323ReturnCode(i)),
		104 => value!(i, Attribute::VsaEpygiH323PromptId(i)),
		105 => value!(i, Attribute::VsaEpygiH323TimeAndDay(i)),
		106 => value!(i, Attribute::VsaEpygiH323RedirectNumber(i)),
		107 => value!(i, Attribute::VsaEpygiH323PreferredLang(i)),
		108 => value!(i, Attribute::VsaEpygiH323RedirectIpAddress(i)),
		109 => value!(i, Attribute::VsaEpygiH323BillingModel(i)),
		110 => value!(i, Attribute::VsaEpygiH323Currency(i)),
		150 => value!(i, Attribute::VsaEpygiRegexpdate(i)),
		151 => value!(i, Attribute::VsaEpygiFiadid(i)),
		152 => value!(i, Attribute::VsaEpygiPortid(i)),
		153 => value!(i, Attribute::VsaEpygiAccesstype(i)),
		154 => value!(i, Attribute::VsaEpygiCallinfo(i)),
		170 => value!(i, Attribute::VsaEpygiOrigcallid(i)),
		171 => value!(i, Attribute::VsaEpygiParentcallid(i)),
		172 => map! {i, be_u32, |v| Attribute::VsaEpygiCalltype(EpygiCalltype(v))},
		173 => value!(i, Attribute::VsaEpygiDevicename(i)),
		174 => map! {i, be_u32, |v| Attribute::VsaEpygiInterfacename(EpygiInterfacename(v))},
		175 => map!{i, be_u32, |v| Attribute::VsaEpygiInterfacenumber(v)},
		176 => map!{i, be_u32, |v| Attribute::VsaEpygiTimeslotnumber(v)},
		177 => map!{i, be_u32, |v| Attribute::VsaEpygiOrigipaddr(v)},
		178 => map!{i, be_u32, |v| Attribute::VsaEpygiDestipaddr(v)},
		179 => map!{i, be_u32, |v| Attribute::VsaEpygiOrigipport(v)},
		180 => map!{i, be_u32, |v| Attribute::VsaEpygiDestipport(v)},
		181 => value!(i, Attribute::VsaEpygiCallingpartynumber(i)),
		182 => value!(i, Attribute::VsaEpygiCalledpartynumber(i)),
		183 => map!{i, be_u32, |v| Attribute::VsaEpygiDatetimeorigination(v)},
		184 => map!{i, be_u32, |v| Attribute::VsaEpygiDatetimeconnect(v)},
		185 => map!{i, be_u32, |v| Attribute::VsaEpygiDatetimedisconnect(v)},
		186 => map!{i, be_u32, |v| Attribute::VsaEpygiDuration(v)},
		187 => map!{i, be_u32, |v| Attribute::VsaEpygiOutsourcertpIp(v)},
		188 => map!{i, be_u32, |v| Attribute::VsaEpygiOutdestrtpIp(v)},
		189 => map!{i, be_u32, |v| Attribute::VsaEpygiInsourcertpIp(v)},
		190 => map!{i, be_u32, |v| Attribute::VsaEpygiIndestrtpIp(v)},
		191 => map!{i, be_u32, |v| Attribute::VsaEpygiOutsourcertpPort(v)},
		192 => map!{i, be_u32, |v| Attribute::VsaEpygiOutdestrtpPort(v)},
		193 => map!{i, be_u32, |v| Attribute::VsaEpygiInsourcertpPort(v)},
		194 => map!{i, be_u32, |v| Attribute::VsaEpygiIndestrtpPort(v)},
		195 => map! {i, be_u32, |v| Attribute::VsaEpygiCallredirectreason(EpygiCallredirectreason(v))},
		196 => map! {i, be_u32, |v| Attribute::VsaEpygiCalldisconnectreason(EpygiCalldisconnectreason(v))},
		197 => map!{i, be_u32, |v| Attribute::VsaEpygiOutrtpPayload(v)},
		198 => map!{i, be_u32, |v| Attribute::VsaEpygiOutrtpPacketsize(v)},
		199 => map!{i, be_u32, |v| Attribute::VsaEpygiOutrtpPackets(v)},
		200 => map!{i, be_u32, |v| Attribute::VsaEpygiOutrtpOctets(v)},
		201 => map!{i, be_u32, |v| Attribute::VsaEpygiInrtpPayload(v)},
		202 => map!{i, be_u32, |v| Attribute::VsaEpygiInrtpPacketsize(v)},
		203 => map!{i, be_u32, |v| Attribute::VsaEpygiInrtpPackets(v)},
		204 => map!{i, be_u32, |v| Attribute::VsaEpygiInrtpOctets(v)},
		205 => map!{i, be_u32, |v| Attribute::VsaEpygiInrtpPacketslost(v)},
		206 => map!{i, be_u32, |v| Attribute::VsaEpygiInrtpPacketsdupl(v)},
		207 => map!{i, be_u32, |v| Attribute::VsaEpygiInrtpJitter(v)},
		208 => map!{i, be_u32, |v| Attribute::VsaEpygiInrtpLatency(v)},
        _ => value!(i, Attribute::VsaUnknown(16459, typ, i)),
    }
}
