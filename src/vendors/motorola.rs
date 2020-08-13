/// Definitions for vendor Motorola, vendor value 161
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MotorolaCanopyHpenable(pub u32);
 
#[allow(non_upper_case_globals)]
impl MotorolaCanopyHpenable {
	pub const Disable: MotorolaCanopyHpenable = MotorolaCanopyHpenable(0);
	pub const Enable: MotorolaCanopyHpenable = MotorolaCanopyHpenable(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MotorolaCanopyVllearnen(pub u32);
 
#[allow(non_upper_case_globals)]
impl MotorolaCanopyVllearnen {
	pub const Disable: MotorolaCanopyVllearnen = MotorolaCanopyVllearnen(0);
	pub const Enable: MotorolaCanopyVllearnen = MotorolaCanopyVllearnen(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MotorolaCanopyVlframes(pub u32);
 
#[allow(non_upper_case_globals)]
impl MotorolaCanopyVlframes {
	pub const All: MotorolaCanopyVlframes = MotorolaCanopyVlframes(0);
	pub const Tagged: MotorolaCanopyVlframes = MotorolaCanopyVlframes(1);
	pub const Untagged: MotorolaCanopyVlframes = MotorolaCanopyVlframes(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MotorolaCanopyVlsmmgpass(pub u32);
 
#[allow(non_upper_case_globals)]
impl MotorolaCanopyVlsmmgpass {
	pub const Enable: MotorolaCanopyVlsmmgpass = MotorolaCanopyVlsmmgpass(1);
	pub const Disable: MotorolaCanopyVlsmmgpass = MotorolaCanopyVlsmmgpass(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MotorolaCanopyUserlevel(pub u32);
 
#[allow(non_upper_case_globals)]
impl MotorolaCanopyUserlevel {
	pub const Tech: MotorolaCanopyUserlevel = MotorolaCanopyUserlevel(1);
	pub const Install: MotorolaCanopyUserlevel = MotorolaCanopyUserlevel(2);
	pub const Admin: MotorolaCanopyUserlevel = MotorolaCanopyUserlevel(3);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyLpulcir(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyLpdlcir(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyHpulcir(v)},
		4 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyHpdlcir(v)},
		5 => map! {i, be_u32, |v| Attribute::VsaMotorolaCanopyHpenable(MotorolaCanopyHpenable(v))},
		6 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyUlbr(v)},
		7 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyUlbl(v)},
		8 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyDlbr(v)},
		9 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyDlbl(v)},
		14 => map! {i, be_u32, |v| Attribute::VsaMotorolaCanopyVllearnen(MotorolaCanopyVllearnen(v))},
		15 => map! {i, be_u32, |v| Attribute::VsaMotorolaCanopyVlframes(MotorolaCanopyVlframes(v))},
		16 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyVlidset(v)},
		20 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyVlageto(v)},
		21 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyVligvid(v)},
		22 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyVlmgvid(v)},
		23 => map! {i, be_u32, |v| Attribute::VsaMotorolaCanopyVlsmmgpass(MotorolaCanopyVlsmmgpass(v))},
		24 => map!{i, be_u32, |v| Attribute::VsaMotorolaCanopyBcastmir(v)},
		50 => map! {i, be_u32, |v| Attribute::VsaMotorolaCanopyUserlevel(MotorolaCanopyUserlevel(v))},
		224 => value!(i, Attribute::VsaMotorolaCanopySharedSecret(i)),
		225 => value!(i, Attribute::VsaMotorolaCanopySuldr(i)),
		226 => value!(i, Attribute::VsaMotorolaCanopySdldr(i)),
		227 => value!(i, Attribute::VsaMotorolaCanopyUlba(i)),
		228 => value!(i, Attribute::VsaMotorolaCanopyDlba(i)),
		229 => value!(i, Attribute::VsaMotorolaCanopyEnable(i)),
		230 => value!(i, Attribute::VsaMotorolaCanopyLpsuldr(i)),
		231 => value!(i, Attribute::VsaMotorolaCanopyLpsdldr(i)),
		232 => value!(i, Attribute::VsaMotorolaCanopyHpcenable(i)),
		233 => value!(i, Attribute::VsaMotorolaCanopyHpsuldr(i)),
		234 => value!(i, Attribute::VsaMotorolaCanopyHpsdldr(i)),
		235 => value!(i, Attribute::VsaMotorolaCanopyHigherbw(i)),
		236 => value!(i, Attribute::VsaMotorolaCanopyCirenable(i)),
		10 => map!{i, take!(4), |v:&[u8]| Attribute::VsaMotorolaWimaxMipMnHomeAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		11 => value!(i, Attribute::VsaMotorolaWimaxMipKey(i)),
		12 => map!{i, be_u32, |v| Attribute::VsaMotorolaWimaxMipSpi(v)},
		13 => map!{i, take!(4), |v:&[u8]| Attribute::VsaMotorolaWimaxMnHa(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		30 => value!(i, Attribute::VsaMotorolaWimaxNetworkDomainName(i)),
		31 => map!{i, take!(4), |v:&[u8]| Attribute::VsaMotorolaWimaxEmsAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		32 => value!(i, Attribute::VsaMotorolaWimaxProvisioningServer(i)),
		34 => value!(i, Attribute::VsaMotorolaWimaxNtpServer(i)),
		35 => value!(i, Attribute::VsaMotorolaWimaxHoSvcClass(i)),
		60 => value!(i, Attribute::VsaMotorolaWimaxMaximumTotalBandwidth(i)),
		61 => value!(i, Attribute::VsaMotorolaWimaxMaximumCommitBandwidth(i)),
		63 => value!(i, Attribute::VsaMotorolaWimaxConvergenceSublayer(i)),
		64 => value!(i, Attribute::VsaMotorolaWimaxServiceFlows(i)),
		65 => value!(i, Attribute::VsaMotorolaWimaxVlanId(i)),
		80 => value!(i, Attribute::VsaMotorolaAccountingMessage(i)),
        _ => value!(i, Attribute::VsaUnknown(161, typ, i)),
    }
}
