
const PACKET_BYTES: [u8; 471] = [
    0x04, 0xd7, 0x01, 0xd7, 0x3f, 0xe8, 0x1b, 0xb7, 0x4e, 0x34, 0x45, 0x39, 0x72, 0xfb, 0x1e, 0x38,
    0x27, 0xdd, 0x8e, 0xd2, 0x28, 0x06, 0x00, 0x00, 0x00, 0x02, 0x31, 0x06, 0x00, 0x00, 0x00, 0x01,
    0x2a, 0x06, 0x00, 0x00, 0x00, 0x00, 0x2b, 0x06, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x06, 0x00, 0x00,
    0x00, 0x00, 0x30, 0x06, 0x00, 0x00, 0x00, 0x00, 0x33, 0x06, 0x00, 0x00, 0x00, 0x01, 0x37, 0x06,
    0x5e, 0xf9, 0xcf, 0xb9, 0x2d, 0x06, 0x00, 0x00, 0x00, 0x02, 0x29, 0x06, 0x00, 0x00, 0x00, 0x00,
    0x2e, 0x06, 0x00, 0x00, 0x00, 0x9e, 0x32, 0x0a, 0x30, 0x31, 0x38, 0x61, 0x38, 0x38, 0x65, 0x64,
    0x1f, 0x0d, 0x38, 0x38, 0x38, 0x38, 0x39, 0x38, 0x32, 0x37, 0x33, 0x38, 0x33, 0x08, 0x06, 0x0a,
    0x3a, 0xcc, 0x9b, 0x2c, 0x12, 0x35, 0x31, 0x64, 0x33, 0x61, 0x39, 0x30, 0x32, 0x30, 0x31, 0x38,
    0x61, 0x38, 0x38, 0x65, 0x64, 0x07, 0x06, 0x00, 0x00, 0x00, 0x07, 0x1e, 0x13, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x65, 0x74, 0x2e, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x2e, 0x6b, 0x7a, 0x20, 0x05,
    0x55, 0x47, 0x57, 0x06, 0x06, 0x00, 0x00, 0x00, 0x02, 0x3d, 0x06, 0x00, 0x00, 0x00, 0x05, 0xe0,
    0x0a, 0x04, 0x71, 0x07, 0x40, 0x92, 0x34, 0x70, 0xf9, 0xe3, 0x06, 0x41, 0x41, 0xa9, 0x02, 0x1a,
    0x0e, 0x00, 0x00, 0x71, 0xf0, 0x03, 0x08, 0x48, 0x75, 0x61, 0x77, 0x65, 0x69, 0x1a, 0x0c, 0x00,
    0x00, 0x71, 0xf0, 0x04, 0x06, 0x31, 0x39, 0x2e, 0x31, 0x1a, 0x17, 0x00, 0x00, 0x28, 0xaf, 0x01,
    0x11, 0x35, 0x30, 0x31, 0x30, 0x30, 0x30, 0x30, 0x34, 0x32, 0x39, 0x34, 0x33, 0x30, 0x37, 0x39,
    0x1a, 0x0c, 0x00, 0x00, 0x28, 0xaf, 0x02, 0x06, 0x01, 0x8a, 0x88, 0xed, 0x1a, 0x0c, 0x00, 0x00,
    0x28, 0xaf, 0x03, 0x06, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x0c, 0x00, 0x00, 0x28, 0xaf, 0x04, 0x06,
    0x41, 0x41, 0xb7, 0x91, 0x1a, 0x2b, 0x00, 0x00, 0x28, 0xaf, 0x05, 0x25, 0x30, 0x37, 0x2d, 0x32,
    0x33, 0x39, 0x32, 0x31, 0x66, 0x39, 0x31, 0x39, 0x36, 0x66, 0x65, 0x66, 0x65, 0x34, 0x34, 0x34,
    0x38, 0x66, 0x66, 0x66, 0x66, 0x30, 0x30, 0x34, 0x66, 0x30, 0x30, 0x34, 0x66, 0x30, 0x30, 0x1a,
    0x0c, 0x00, 0x00, 0x28, 0xaf, 0x06, 0x06, 0x41, 0x41, 0xb6, 0x02, 0x1a, 0x0c, 0x00, 0x00, 0x28,
    0xaf, 0x07, 0x06, 0x41, 0x41, 0xa9, 0x02, 0x1a, 0x0d, 0x00, 0x00, 0x28, 0xaf, 0x08, 0x07, 0x34,
    0x30, 0x31, 0x37, 0x37, 0x1a, 0x0d, 0x00, 0x00, 0x28, 0xaf, 0x09, 0x07, 0x34, 0x30, 0x31, 0x37,
    0x37, 0x1a, 0x09, 0x00, 0x00, 0x28, 0xaf, 0x0a, 0x03, 0x36, 0x1a, 0x09, 0x00, 0x00, 0x28, 0xaf,
    0x0b, 0x03, 0xff, 0x1a, 0x09, 0x00, 0x00, 0x28, 0xaf, 0x0c, 0x03, 0x30, 0x1a, 0x0c, 0x00, 0x00,
    0x28, 0xaf, 0x0d, 0x06, 0x30, 0x34, 0x30, 0x30, 0x1a, 0x0d, 0x00, 0x00, 0x28, 0xaf, 0x12, 0x07,
    0x35, 0x30, 0x31, 0x30, 0x30, 0x1a, 0x09, 0x00, 0x00, 0x28, 0xaf, 0x15, 0x03, 0x01, 0x1a, 0x10,
    0x00, 0x00, 0x28, 0xaf, 0x16, 0x0a, 0x01, 0x04, 0xf1, 0x77, 0x7b, 0xe9, 0x52, 0xe9, 0x1a, 0x0a,
    0x00, 0x00, 0x28, 0xaf, 0x17, 0x04, 0x42, 0x00, 0x1a, 0x09, 0x00, 0x00, 0x28, 0xaf, 0x1a, 0x03,
    0x00, 0x04, 0x06, 0x41, 0x41, 0xa9, 0x10,
];


#[test]
fn test() {

    use crate::radius::*;
    use crate::vendors::threegpp::{ThreeGppPdpType, ThreeGppRatType};
    use std::str;

    let mut counter = 0;
    let p_res = new(&PACKET_BYTES);
    match p_res {
        Ok(mut data) => {
            while let Some(attr) = data.next() {
                counter += 1;
                match attr {
                    Attribute::AcctStatusType(status_type) => {
                        assert_eq!(status_type, AcctStatusType::Stop)
                    }
                    Attribute::AcctTerminateCause(cause) => {
                        assert_eq!(cause, AcctTerminateCause::UserRequest)
                    }
                    Attribute::AcctInputOctets(octets) => assert_eq!(octets, 0),
                    Attribute::AcctOutputOctets(octets) => assert_eq!(octets, 0),
                    Attribute::AcctInputPackets(octets) => assert_eq!(octets, 0),
                    Attribute::AcctOutputPackets(octets) => assert_eq!(octets, 0),
                    Attribute::AcctLinkCount(c) => assert_eq!(c, 1),
                    Attribute::EventTimestamp(ts) => assert_eq!(ts, 1593429945),
                    Attribute::AcctAuthentic(auth) => assert_eq!(auth, AcctAuthentic::Local),
                    Attribute::AcctDelayTime(t) => assert_eq!(t, 0),
                    Attribute::AcctSessionTime(t) => assert_eq!(t, 158),
                    Attribute::AcctMultiSessionId(id) => {
                        assert_eq!(&id[..], [48, 49, 56, 97, 56, 56, 101, 100])
                    }
                    Attribute::CallingStationId(msisdn) => {
                        assert_eq!(str::from_utf8(msisdn).unwrap(), "88889827383")
                    }
                    Attribute::FramedIpAddress(ipaddr) => {
                        assert_eq!(&ipaddr.to_string(), "10.58.204.155")
                    }
                    Attribute::AcctSessionId(id) => assert_eq!(
                        &id[..],
                        [53, 49, 100, 51, 97, 57, 48, 50, 48, 49, 56, 97, 56, 56, 101, 100]
                    ),
                    Attribute::FramedProtocol(proto) => {
                        assert_eq!(proto, FramedProtocol::GprsPdpContext)
                    }
                    Attribute::CalledStationId(id) => {
                        assert_eq!(str::from_utf8(id).unwrap(), "internet.int32.kz")
                    }
                    Attribute::NasIdentifier(id) => assert_eq!(&id[..], [85, 71, 87]),
                    Attribute::ServiceType(typ) => assert_eq!(typ, ServiceType::FramedUser),
                    Attribute::NasPortType(typ) => assert_eq!(typ, NasPortType::Virtual),
                    Attribute::VsaThreeGppImsi(imsi) => {
                        assert_eq!(str::from_utf8(imsi).unwrap(), "501000042943079")
                    }
                    Attribute::VsaThreeGppChargingId(id) => assert_eq!(id, 25856237),
                    At21tribute::VsaThreeGppPdpType(typ) => assert_eq!(typ, ThreeGppPdpType::Ipv4),
                    Attribute::VsaThreeGppChargingGatewayAddress(addr) => {
                        assert_eq!(&addr.to_string(), "65.65.183.145")
                    }
                    Attribute::VsaThreeGppGprsNegotiatedQosProfile(obj) => {
                        assert!(&obj[..].iter().eq([
                            48, 55, 45, 50, 51, 57, 50, 49, 102, 57, 49, 57, 54, 102, 101, 102,
                            101, 52, 52, 52, 56, 102, 102, 102, 102, 48, 48, 52, 102, 48, 48, 52,
                            102, 48, 48
                        ]
                        .iter()))
                    }
                    Attribute::VsaThreeGppSgsnAddress(sgsn) => {
                        assert_eq!(&sgsn.to_string(), "65.65.182.2")
                    }
                    Attribute::VsaThreeGppGgsnAddress(ggsn) => {
                        assert_eq!(&ggsn.to_string(), "65.65.169.2")
                    }
                    Attribute::VsaThreeGppImsiMccMnc(mccmnc) => {
                        assert_eq!(str::from_utf8(mccmnc).unwrap(), "50100")
                    }
                    Attribute::VsaThreeGppGgsnMccMnc(mccmnc) => {
                        assert_eq!(str::from_utf8(mccmnc).unwrap(), "50100")
                    }
                    Attribute::VsaThreeGppNsapi(obj) => assert_eq!(&obj[..], [54]),
                    Attribute::VsaThreeGppSessionStopIndicator(obj) => assert_eq!(&obj[..], [255]),
                    Attribute::VsaThreeGppSelectionMode(obj) => assert_eq!(&obj[..], [48]),
                    Attribute::VsaThreeGppChargingCharacteristics(obj) => {
                        assert_eq!(&obj[..], [48, 52, 48, 48])
                    }
                    Attribute::VsaThreeGppSgsnMccMnc(obj) => {
                        assert_eq!(&obj[..], [52, 48, 49, 55, 55])
                    }
                    Attribute::VsaThreeGppRatType(o) => assert_eq!(o, ThreeGppRatType::Utran),
                    Attribute::VsaThreeGppUserLocationInfo(o) => {
                        assert_eq!(&o[..], [1, 4, 241, 119, 123, 233, 82, 233])
                    }
                    Attribute::VsaThreeGppMsTimezone(o) => assert_eq!(&o[..], [66, 0]),
                    Attribute::VsaThreeGppNegotiatedDscp(o) => assert_eq!(o, 0),
                    Attribute::NasIpAddress(o) => assert_eq!(&o.to_string(), "65.65.169.16"),
                    Attribute::Unknown(t, p) => match t {
                        224 => assert_eq!(&p[..], [4, 113, 7, 64, 146, 52, 112, 249]),
                        227 => assert_eq!(&p[..], [0x41, 0x41, 0xa9, 0x02]),
                        _ => panic!(
                            "Attribute tag = {}, data = {:?} must not be in this data set",
                            t, p
                        ),
                    },
                    Attribute::VsaUnknown(vendor_id, t, p) => {
                        assert_eq!(vendor_id, 29168);
                        match t {
                            3 => assert_eq!(str::from_utf8(p).unwrap(), "Huawei"),
                            4 => assert_eq!(&p[..], [0x31, 0x39, 0x2e, 0x31]),
                            _ => panic!("VSA Attribute vendor_id = {}, tag = {}, data = {:?} must not be in this data set", vendor_id, t, p),
                        }
                    }
                    a => panic!("Attribute {:?} must not be in this data set", a),
                }
                println!("{:?}", attr)
            }

            match data.result {
                Ok(()) => {
                    assert_eq!(counter, 43);
                    println!("test successful")
                }
                Err(e) => panic!("error while processing {:?}", e),
            }
        }
        Err(e) => panic!("error {:?}", &e),
    }
}
