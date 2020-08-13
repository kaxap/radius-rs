/// Definitions for vendor UKERNA, vendor value 25622
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		128 => value!(i, Attribute::VsaUkernaGssAcceptorServiceName(i)),
		129 => value!(i, Attribute::VsaUkernaGssAcceptorHostName(i)),
		130 => value!(i, Attribute::VsaUkernaGssAcceptorServiceSpecific(i)),
		131 => value!(i, Attribute::VsaUkernaGssAcceptorRealmName(i)),
		132 => value!(i, Attribute::VsaSamlAaaAssertion(i)),
		135 => value!(i, Attribute::VsaEapChannelBindingMessage(i)),
		136 => value!(i, Attribute::VsaTrustRouterCoi(i)),
		137 => value!(i, Attribute::VsaTrustRouterApc(i)),
		138 => value!(i, Attribute::VsaMoonshotHostTargetedid(i)),
		139 => value!(i, Attribute::VsaMoonshotRealmTargetedid(i)),
		140 => value!(i, Attribute::VsaMoonshotTrCoiTargetedid(i)),
		141 => value!(i, Attribute::VsaMoonshotMstidGssAcceptor(i)),
		142 => value!(i, Attribute::VsaMoonshotMstidNamespace(i)),
		143 => value!(i, Attribute::VsaMoonshotMstidTargetedid(i)),
		144 => value!(i, Attribute::VsaMoonshotOtpSecret(i)),
        _ => value!(i, Attribute::VsaUnknown(25622, typ, i)),
    }
}
