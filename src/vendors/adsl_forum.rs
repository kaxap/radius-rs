/// Definitions for vendor ADSL-Forum, vendor value 3561
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => value!(i, Attribute::VsaAdslAgentCircuitId(i)),
		2 => value!(i, Attribute::VsaAdslAgentRemoteId(i)),
		129 => map!{i, be_u32, |v| Attribute::VsaActualDataRateUpstream(v)},
		130 => map!{i, be_u32, |v| Attribute::VsaActualDataRateDownstream(v)},
		131 => map!{i, be_u32, |v| Attribute::VsaMinimumDataRateUpstream(v)},
		132 => map!{i, be_u32, |v| Attribute::VsaMinimumDataRateDownstream(v)},
		133 => map!{i, be_u32, |v| Attribute::VsaAttainableDataRateUpstream(v)},
		134 => map!{i, be_u32, |v| Attribute::VsaAttainableDataRateDownstream(v)},
		135 => map!{i, be_u32, |v| Attribute::VsaMaximumDataRateUpstream(v)},
		136 => map!{i, be_u32, |v| Attribute::VsaMaximumDataRateDownstream(v)},
		137 => map!{i, be_u32, |v| Attribute::VsaMinimumDataRateUpstreamLowPower(v)},
		138 => map!{i, be_u32, |v| Attribute::VsaMinimumDataRateDownstreamLowPower(v)},
		139 => map!{i, be_u32, |v| Attribute::VsaMaximumInterleavingDelayUpstream(v)},
		140 => map!{i, be_u32, |v| Attribute::VsaActualInterleavingDelayUpstream(v)},
		141 => map!{i, be_u32, |v| Attribute::VsaMaximumInterleavingDelayDownstream(v)},
		142 => map!{i, be_u32, |v| Attribute::VsaActualInterleavingDelayDownstream(v)},
		144 => value!(i, Attribute::VsaAccessLoopEncapsulation(i)),
		252 => value!(i, Attribute::VsaIwfSession(i)),
        _ => value!(i, Attribute::VsaUnknown(3561, typ, i)),
    }
}
