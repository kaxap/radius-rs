/// Definitions for vendor ALU-AAA, vendor value 831
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AluAaaClientErrorAction(pub u32);
 
#[allow(non_upper_case_globals)]
impl AluAaaClientErrorAction {
	pub const Ignore: AluAaaClientErrorAction = AluAaaClientErrorAction(1);
	pub const Disconnect: AluAaaClientErrorAction = AluAaaClientErrorAction(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AluAaaDeltaSession(pub u32);
 
#[allow(non_upper_case_globals)]
impl AluAaaDeltaSession {
	pub const False: AluAaaDeltaSession = AluAaaDeltaSession(0);
	pub const True: AluAaaDeltaSession = AluAaaDeltaSession(1);
}


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaAluAaaAccessRule(i)),
		2 => value!(i, Attribute::VsaAluAaaAvPair(i)),
		3 => map!{i, be_u32, |v| Attribute::VsaAluAaaGsmTripletsNeeded(v)},
		4 => value!(i, Attribute::VsaAluAaaGsmTriplet(i)),
		5 => map!{i, be_u32, |v| Attribute::VsaAluAaaAkaQuintetsNeeded(v)},
		6 => value!(i, Attribute::VsaAluAaaAkaQuintet(i)),
		7 => value!(i, Attribute::VsaAluAaaAkaRand(i)),
		8 => value!(i, Attribute::VsaAluAaaAkaAuts(i)),
		9 => value!(i, Attribute::VsaAluAaaServiceProfile(i)),
		10 => map!{i, be_u8, |v| Attribute::VsaAluAaaLawfulInterceptStatus(v)},
		11 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAluAaaDfCcAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		12 => map!{i, be_u16, |v| Attribute::VsaAluAaaDfCcPort(v)},
		13 => value!(i, Attribute::VsaAluAaaClientProgram(i)),
		14 => map! {i, be_u32, |v| Attribute::VsaAluAaaClientErrorAction(AluAaaClientErrorAction(v))},
		15 => value!(i, Attribute::VsaAluAaaClientOs(i)),
		16 => value!(i, Attribute::VsaAluAaaClientVersion(i)),
		17 => value!(i, Attribute::VsaAluAaaNonce(i)),
		18 => value!(i, Attribute::VsaAluAaaFemtoPublicKeyHash(i)),
		19 => value!(i, Attribute::VsaAluAaaFemtoAssociatedUserName(i)),
		100 => value!(i, Attribute::VsaAluAaaString0(i)),
		101 => value!(i, Attribute::VsaAluAaaString1(i)),
		102 => value!(i, Attribute::VsaAluAaaString2(i)),
		103 => value!(i, Attribute::VsaAluAaaString3(i)),
		104 => map!{i, be_u32, |v| Attribute::VsaAluAaaInteger0(v)},
		105 => map!{i, be_u32, |v| Attribute::VsaAluAaaInteger1(v)},
		106 => map!{i, be_u32, |v| Attribute::VsaAluAaaInteger2(v)},
		107 => map!{i, be_u32, |v| Attribute::VsaAluAaaInteger3(v)},
		108 => value!(i, Attribute::VsaAluAaaAddress0(i)),
		109 => value!(i, Attribute::VsaAluAaaAddress1(i)),
		110 => value!(i, Attribute::VsaAluAaaAddress2(i)),
		111 => value!(i, Attribute::VsaAluAaaAddress3(i)),
		112 => value!(i, Attribute::VsaAluAaaValue0(i)),
		113 => value!(i, Attribute::VsaAluAaaValue1(i)),
		114 => value!(i, Attribute::VsaAluAaaValue2(i)),
		115 => value!(i, Attribute::VsaAluAaaValue3(i)),
		116 => value!(i, Attribute::VsaAluAaaKey0(i)),
		117 => value!(i, Attribute::VsaAluAaaKey1(i)),
		118 => value!(i, Attribute::VsaAluAaaKey2(i)),
		119 => value!(i, Attribute::VsaAluAaaKey3(i)),
		120 => value!(i, Attribute::VsaAluAaaOpaque0(i)),
		121 => value!(i, Attribute::VsaAluAaaOpaque1(i)),
		122 => value!(i, Attribute::VsaAluAaaOpaque2(i)),
		123 => value!(i, Attribute::VsaAluAaaOpaque3(i)),
		124 => value!(i, Attribute::VsaAluAaaEval0(i)),
		125 => value!(i, Attribute::VsaAluAaaEval1(i)),
		126 => value!(i, Attribute::VsaAluAaaEval2(i)),
		127 => value!(i, Attribute::VsaAluAaaEval3(i)),
		128 => value!(i, Attribute::VsaAluAaaExec0(i)),
		129 => value!(i, Attribute::VsaAluAaaExec1(i)),
		130 => value!(i, Attribute::VsaAluAaaExec2(i)),
		131 => value!(i, Attribute::VsaAluAaaExec3(i)),
		199 => value!(i, Attribute::VsaAluAaaOriginalReceiptTime(i)),
		201 => value!(i, Attribute::VsaAluAaaReplyMessage(i)),
		202 => value!(i, Attribute::VsaAluAaaCalledStationId(i)),
		203 => map!{i, take!(4), |v:&[u8]| Attribute::VsaAluAaaNasIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		204 => map!{i, be_u32, |v| Attribute::VsaAluAaaNasPort(v)},
		205 => value!(i, Attribute::VsaAluAaaOldState(i)),
		206 => value!(i, Attribute::VsaAluAaaNewState(i)),
		207 => value!(i, Attribute::VsaAluAaaEvent(i)),
		208 => map!{i, be_u32, |v| Attribute::VsaAluAaaOldTimestamp(v)},
		209 => map!{i, be_u32, |v| Attribute::VsaAluAaaNewTimestamp(v)},
		210 => map! {i, be_u32, |v| Attribute::VsaAluAaaDeltaSession(AluAaaDeltaSession(v))},
		211 => value!(i, Attribute::VsaAluAaaCivicLocation(i)),
		212 => value!(i, Attribute::VsaAluAaaGeospatialLocation(i)),
        _ => value!(i, Attribute::VsaUnknown(831, typ, i)),
    }
}
