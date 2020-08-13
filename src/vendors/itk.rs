/// Definitions for vendor ITK, vendor value 1195
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		100 => map!{i, take!(4), |v:&[u8]| Attribute::VsaItkAuthServIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		101 => map!{i, be_u32, |v| Attribute::VsaItkAuthServProt(v)},
		102 => map!{i, be_u32, |v| Attribute::VsaItkProviderId(v)},
		103 => map!{i, be_u32, |v| Attribute::VsaItkUsergroup(v)},
		104 => value!(i, Attribute::VsaItkBanner(i)),
		105 => value!(i, Attribute::VsaItkUsernamePrompt(i)),
		106 => value!(i, Attribute::VsaItkPasswordPrompt(i)),
		107 => value!(i, Attribute::VsaItkWelcomeMessage(i)),
		108 => value!(i, Attribute::VsaItkPrompt(i)),
		109 => map!{i, be_u32, |v| Attribute::VsaItkIpPool(v)},
		110 => map!{i, take!(4), |v:&[u8]| Attribute::VsaItkTunnelIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		111 => map!{i, be_u32, |v| Attribute::VsaItkTunnelProt(v)},
		112 => map!{i, take!(4), |v:&[u8]| Attribute::VsaItkAcctServIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		113 => map!{i, be_u32, |v| Attribute::VsaItkAcctServProt(v)},
		114 => value!(i, Attribute::VsaItkFilterRule(i)),
		115 => map!{i, be_u32, |v| Attribute::VsaItkChannelBinding(v)},
		116 => map!{i, be_u32, |v| Attribute::VsaItkStartDelay(v)},
		117 => value!(i, Attribute::VsaItkNasName(i)),
		118 => map!{i, be_u32, |v| Attribute::VsaItkIsdnProt(v)},
		119 => map!{i, be_u32, |v| Attribute::VsaItkPppAuthType(v)},
		120 => map!{i, be_u32, |v| Attribute::VsaItkDialoutType(v)},
		121 => map!{i, take!(4), |v:&[u8]| Attribute::VsaItkFtpAuthIp(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		122 => value!(i, Attribute::VsaItkUsersDefaultEntry(i)),
		123 => value!(i, Attribute::VsaItkUsersDefaultPw(i)),
		124 => value!(i, Attribute::VsaItkAuthReqType(i)),
		125 => map!{i, be_u32, |v| Attribute::VsaItkModemPoolId(v)},
		126 => value!(i, Attribute::VsaItkModemInitString(i)),
		127 => map!{i, be_u32, |v| Attribute::VsaItkPppClientServerMode(v)},
		128 => value!(i, Attribute::VsaItkPppCompressionProt(i)),
		129 => value!(i, Attribute::VsaItkUsername(i)),
		130 => value!(i, Attribute::VsaItkDestNo(i)),
		131 => value!(i, Attribute::VsaItkDdi(i)),
        _ => value!(i, Attribute::VsaUnknown(1195, typ, i)),
    }
}
