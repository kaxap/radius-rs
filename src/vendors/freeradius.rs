/// Definitions for vendor FreeRADIUS, vendor value 11344
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FreeradiusStatisticsType(pub u32);
 
#[allow(non_upper_case_globals)]
impl FreeradiusStatisticsType {
	pub const None: FreeradiusStatisticsType = FreeradiusStatisticsType(0);
	pub const Authentication: FreeradiusStatisticsType = FreeradiusStatisticsType(1);
	pub const Accounting: FreeradiusStatisticsType = FreeradiusStatisticsType(2);
	pub const ProxyAuthentication: FreeradiusStatisticsType = FreeradiusStatisticsType(4);
	pub const ProxyAccounting: FreeradiusStatisticsType = FreeradiusStatisticsType(8);
	pub const Internal: FreeradiusStatisticsType = FreeradiusStatisticsType(0x10);
	pub const Client: FreeradiusStatisticsType = FreeradiusStatisticsType(0x20);
	pub const Server: FreeradiusStatisticsType = FreeradiusStatisticsType(0x40);
	pub const HomeServer: FreeradiusStatisticsType = FreeradiusStatisticsType(0x80);
	pub const AuthAcct: FreeradiusStatisticsType = FreeradiusStatisticsType(0x03);
	pub const ProxyAuthAcct: FreeradiusStatisticsType = FreeradiusStatisticsType(0x0c);
	pub const All: FreeradiusStatisticsType = FreeradiusStatisticsType(0x1f);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FreeradiusStatsServerState(pub u32);
 
#[allow(non_upper_case_globals)]
impl FreeradiusStatsServerState {
	pub const Alive: FreeradiusStatsServerState = FreeradiusStatsServerState(0);
	pub const Zombie: FreeradiusStatsServerState = FreeradiusStatsServerState(1);
	pub const Dead: FreeradiusStatsServerState = FreeradiusStatsServerState(2);
	pub const Idle: FreeradiusStatsServerState = FreeradiusStatsServerState(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, take!(4), |v:&[u8]| Attribute::VsaFreeradiusProxiedTo(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		2 => map!{i, be_u32, |v| Attribute::VsaFreeradiusAcctSessionStartTime(v)},
		127 => map! {i, be_u32, |v| Attribute::VsaFreeradiusStatisticsType(FreeradiusStatisticsType(v))},
		128 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAccessRequests(v)},
		129 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAccessAccepts(v)},
		130 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAccessRejects(v)},
		131 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAccessChallenges(v)},
		132 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAuthResponses(v)},
		133 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAuthDuplicateRequests(v)},
		134 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAuthMalformedRequests(v)},
		135 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAuthInvalidRequests(v)},
		136 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAuthDroppedRequests(v)},
		137 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAuthUnknownTypes(v)},
		138 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAccessRequests(v)},
		139 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAccessAccepts(v)},
		140 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAccessRejects(v)},
		141 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAccessChallenges(v)},
		142 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAuthResponses(v)},
		143 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAuthDuplicateRequests(v)},
		144 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAuthMalformedRequests(v)},
		145 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAuthInvalidRequests(v)},
		146 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAuthDroppedRequests(v)},
		147 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAuthUnknownTypes(v)},
		148 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAccountingRequests(v)},
		149 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAccountingResponses(v)},
		150 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAcctDuplicateRequests(v)},
		151 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAcctMalformedRequests(v)},
		152 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAcctInvalidRequests(v)},
		153 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAcctDroppedRequests(v)},
		154 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalAcctUnknownTypes(v)},
		155 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAccountingRequests(v)},
		156 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAccountingResponses(v)},
		157 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAcctDuplicateRequests(v)},
		158 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAcctMalformedRequests(v)},
		159 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAcctInvalidRequests(v)},
		160 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAcctDroppedRequests(v)},
		161 => map!{i, be_u32, |v| Attribute::VsaFreeradiusTotalProxyAcctUnknownTypes(v)},
		162 => map!{i, be_u32, |v| Attribute::VsaFreeradiusQueueLenInternal(v)},
		163 => map!{i, be_u32, |v| Attribute::VsaFreeradiusQueueLenProxy(v)},
		164 => map!{i, be_u32, |v| Attribute::VsaFreeradiusQueueLenAuth(v)},
		165 => map!{i, be_u32, |v| Attribute::VsaFreeradiusQueueLenAcct(v)},
		166 => map!{i, be_u32, |v| Attribute::VsaFreeradiusQueueLenDetail(v)},
		167 => map!{i, take!(4), |v:&[u8]| Attribute::VsaFreeradiusStatsClientIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		168 => map!{i, be_u32, |v| Attribute::VsaFreeradiusStatsClientNumber(v)},
		169 => map!{i, be_u32, |v| Attribute::VsaFreeradiusStatsClientNetmask(v)},
		170 => map!{i, take!(4), |v:&[u8]| Attribute::VsaFreeradiusStatsServerIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		171 => map!{i, be_u32, |v| Attribute::VsaFreeradiusStatsServerPort(v)},
		172 => map!{i, be_u32, |v| Attribute::VsaFreeradiusStatsServerOutstandingRequests(v)},
		173 => map! {i, be_u32, |v| Attribute::VsaFreeradiusStatsServerState(FreeradiusStatsServerState(v))},
		174 => map!{i, be_u32, |v| Attribute::VsaFreeradiusStatsServerTimeOfDeath(v)},
		175 => map!{i, be_u32, |v| Attribute::VsaFreeradiusStatsServerTimeOfLife(v)},
		176 => map!{i, be_u32, |v| Attribute::VsaFreeradiusStatsStartTime(v)},
		177 => map!{i, be_u32, |v| Attribute::VsaFreeradiusStatsHupTime(v)},
		178 => map!{i, be_u32, |v| Attribute::VsaFreeradiusServerEmaWindow(v)},
		179 => map!{i, be_u32, |v| Attribute::VsaFreeradiusServerEmaUsecWindow1(v)},
		180 => map!{i, be_u32, |v| Attribute::VsaFreeradiusServerEmaUsecWindow10(v)},
		181 => map!{i, be_u32, |v| Attribute::VsaFreeradiusQueuePpsIn(v)},
		182 => map!{i, be_u32, |v| Attribute::VsaFreeradiusQueuePpsOut(v)},
		183 => map!{i, be_u32, |v| Attribute::VsaFreeradiusQueueUsePercentage(v)},
		184 => map!{i, be_u32, |v| Attribute::VsaFreeradiusStatsLastPacketRecv(v)},
		185 => map!{i, be_u32, |v| Attribute::VsaFreeradiusStatsLastPacketSent(v)},
        _ => value!(i, Attribute::VsaUnknown(11344, typ, i)),
    }
}
