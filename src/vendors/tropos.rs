/// Definitions for vendor Tropos, vendor value 14529
use nom::IResult;
#[allow(unused_imports)]
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
#[allow(unused_imports)]
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::radius::*;


pub fn parse(i: &[u8], typ: u8) -> IResult<&[u8], Attribute> {
    match typ {
		1 => map!{i, be_u32, |v| Attribute::VsaTroposUnicastCipher(v)},
		2 => map!{i, be_u32, |v| Attribute::VsaTroposLayer2InputOctets(v)},
		3 => map!{i, be_u32, |v| Attribute::VsaTroposLayer2OutputOctets(v)},
		4 => map!{i, be_u32, |v| Attribute::VsaTroposLayer2InputFrames(v)},
		5 => map!{i, be_u32, |v| Attribute::VsaTroposLayer2OutputFrames(v)},
		6 => map!{i, be_u32, |v| Attribute::VsaTroposLayer2InputDrops(v)},
		7 => value!(i, Attribute::VsaTroposNoiseFloor(i)),
		8 => value!(i, Attribute::VsaTroposNoiseUpperBound(i)),
		9 => value!(i, Attribute::VsaTroposRelease(i)),
		11 => value!(i, Attribute::VsaTroposSecondaryIp(i)),
		12 => map!{i, be_u32, |v| Attribute::VsaTroposTerminateCause(v)},
		13 => map!{i, be_u32, |v| Attribute::VsaTroposAverageRssi(v)},
		15 => value!(i, Attribute::VsaTroposChannel(i)),
		16 => map!{i, be_u32, |v| Attribute::VsaTroposRetriesSent(v)},
		17 => map!{i, be_u32, |v| Attribute::VsaTroposRetryBits(v)},
		18 => value!(i, Attribute::VsaTroposRatesSent(i)),
		19 => value!(i, Attribute::VsaTroposRatesReceived(i)),
		21 => map!{i, be_u32, |v| Attribute::VsaTroposRoutedTime(v)},
		22 => map!{i, be_u32, |v| Attribute::VsaTroposRoutlessSince(v)},
		23 => value!(i, Attribute::VsaTroposCapabilityInfo(i)),
		24 => map!{i, be_u32, |v| Attribute::VsaTroposInputCap(v)},
		25 => map!{i, be_u32, |v| Attribute::VsaTroposOutputCap(v)},
		26 => map!{i, be_u32, |v| Attribute::VsaTroposClassMult(v)},
		27 => value!(i, Attribute::VsaTroposCellName(i)),
		28 => value!(i, Attribute::VsaTroposCellLocation(i)),
		29 => value!(i, Attribute::VsaTroposSerialNumber(i)),
		30 => value!(i, Attribute::VsaTroposLatitude(i)),
		31 => value!(i, Attribute::VsaTroposLongitude(i)),
        _ => value!(i, Attribute::VsaUnknown(14529, typ, i)),
    }
}
