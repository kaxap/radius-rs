use crate::vendors::adsl_forum;
use crate::vendors::threecom;
use crate::vendors::threegpp;
use crate::vendors::threegpp2;
use crate::vendors::acc;
use crate::vendors::acme;
use crate::vendors::actelis;
use crate::vendors::airespace;
use crate::vendors::alcatel;
use crate::vendors::alcatel_lucent_service_router;
use crate::vendors::alu_aaa;
use crate::vendors::alteon;
use crate::vendors::alvarion;
use crate::vendors::apc;
use crate::vendors::aptilo;
use crate::vendors::arbor;
use crate::vendors::aruba;
use crate::vendors::azaire;
use crate::vendors::ascend;
use crate::vendors::bay_networks;
use crate::vendors::bintec;
use crate::vendors::bluecoat;
use crate::vendors::broadsoft;
use crate::vendors::brocade;
use crate::vendors::bskyb;
use crate::vendors::bt;
use crate::vendors::cablelabs;
use crate::vendors::cabletron;
use crate::vendors::camiant;
use crate::vendors::chillispot;
use crate::vendors::cisco;
use crate::vendors::cisco_asa;
use crate::vendors::cisco_bbsm;
use crate::vendors::citrix;
use crate::vendors::clavister;
use crate::vendors::cnergee;
use crate::vendors::colubris;
use crate::vendors::compatible;
use crate::vendors::cosine;
use crate::vendors::dante;
use crate::vendors::dellemc;
use crate::vendors::dlink;
use crate::vendors::digium;
use crate::vendors::dragonwave;
use crate::vendors::efficientip;
use crate::vendors::eltex;
use crate::vendors::epygi;
use crate::vendors::equallogic;
use crate::vendors::ericsson;
use crate::vendors::ericsson_ab;
use crate::vendors::ericsson_packet_core_networks;
use crate::vendors::extreme;
use crate::vendors::f5;
use crate::vendors::fdxtended;
use crate::vendors::freeradius;
use crate::vendors::freeswitch;
use crate::vendors::fortinet;
use crate::vendors::foundry;
use crate::vendors::gandalf;
use crate::vendors::gemtek;
use crate::vendors::h3c;
use crate::vendors::hp;
use crate::vendors::huawei;
use crate::vendors::iea_software;
use crate::vendors::infonet;
use crate::vendors::issanni;
use crate::vendors::itk;
use crate::vendors::ipunplugged;
use crate::vendors::juniper;
use crate::vendors::jradius;
use crate::vendors::karlnet;
use crate::vendors::kineto;
use crate::vendors::lancom;
use crate::vendors::livingston;
use crate::vendors::local_web;
use crate::vendors::lucent;
use crate::vendors::manzara;
use crate::vendors::meinberg;
use crate::vendors::meraki;
use crate::vendors::merit;
use crate::vendors::meru;
use crate::vendors::microsemi;
use crate::vendors::microsoft;
use crate::vendors::mikrotik;
use crate::vendors::motorola;
use crate::vendors::navini;
use crate::vendors::netscreen;
use crate::vendors::networkphysics;
use crate::vendors::nexans;
use crate::vendors::ntua;
use crate::vendors::nokia;
use crate::vendors::nomadix;
use crate::vendors::nortel;
use crate::vendors::packeteer;
use crate::vendors::paloalto;
use crate::vendors::patton;
use crate::vendors::perle;
use crate::vendors::propel;
use crate::vendors::prosoft;
use crate::vendors::proxim;
use crate::vendors::purewave;
use crate::vendors::quiconnect;
use crate::vendors::quintum;
use crate::vendors::redcreek;
use crate::vendors::riverbed;
use crate::vendors::riverstone;
use crate::vendors::roaring_penguin;
use crate::vendors::ruckus;
use crate::vendors::ruggedcom;
use crate::vendors::netborder;
use crate::vendors::shasta;
use crate::vendors::sg;
use crate::vendors::shiva;
use crate::vendors::siemens;
use crate::vendors::slipstream;
use crate::vendors::sonicwall;
use crate::vendors::springtide;
use crate::vendors::starent;
use crate::vendors::surfnet;
use crate::vendors::symbol;
use crate::vendors::telebit;
use crate::vendors::terena;
use crate::vendors::trapeze;
use crate::vendors::travelping;
use crate::vendors::tropos;
use crate::vendors::t_systems_nova;
use crate::vendors::ukerna;
use crate::vendors::unisphere;
use crate::vendors::unix;
use crate::vendors::usr;
use crate::vendors::utstarcom;
use crate::vendors::valemountnetworks;
use crate::vendors::versanet;
use crate::vendors::verizonwireless;
use crate::vendors::waverider;
use crate::vendors::walabi;
use crate::vendors::wichorus;
use crate::vendors::wimax;
use crate::vendors::wispr;
use crate::vendors::xedia;
use crate::vendors::xylan;
use crate::vendors::yubico;
use crate::vendors::zeus;
use crate::vendors::zte;
use crate::vendors::zyxel;

use nom::{IResult, Needed};
use crate::radius::*;
use nom::number::streaming::{be_u64, be_u32, be_u16, be_u8};
use nom::Err;
use std::net::{Ipv4Addr, Ipv6Addr};

pub fn parse_attribute_content(i: &[u8], t: u8) -> IResult<&[u8], Attribute> {
    match t {
		1 => value!(i, Attribute::UserName(i)),
		2 => value!(i, Attribute::UserPassword(i)),
		3 => value!(i, Attribute::ChapPassword(i)),
		4 => map!{i, take!(4), |v:&[u8]| Attribute::NasIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		5 => map!{i, be_u32, |v| Attribute::NasPort(v)},
		6 => map! {i, be_u32, |v| Attribute::ServiceType(ServiceType(v))},
		7 => map! {i, be_u32, |v| Attribute::FramedProtocol(FramedProtocol(v))},
		8 => map!{i, take!(4), |v:&[u8]| Attribute::FramedIpAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		9 => map!{i, take!(4), |v:&[u8]| Attribute::FramedIpNetmask(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		10 => map! {i, be_u32, |v| Attribute::FramedRouting(FramedRouting(v))},
		11 => value!(i, Attribute::FilterId(i)),
		12 => map!{i, be_u32, |v| Attribute::FramedMtu(v)},
		13 => map! {i, be_u32, |v| Attribute::FramedCompression(FramedCompression(v))},
		14 => map!{i, take!(4), |v:&[u8]| Attribute::LoginIpHost(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		15 => map! {i, be_u32, |v| Attribute::LoginService(LoginService(v))},
		16 => map! {i, be_u32, |v| Attribute::LoginTcpPort(LoginTcpPort(v))},
		18 => value!(i, Attribute::ReplyMessage(i)),
		19 => value!(i, Attribute::CallbackNumber(i)),
		20 => value!(i, Attribute::CallbackId(i)),
		22 => value!(i, Attribute::FramedRoute(i)),
		23 => map!{i, take!(4), |v:&[u8]| Attribute::FramedIpxNetwork(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		24 => value!(i, Attribute::State(i)),
		25 => value!(i, Attribute::Class(i)),
		26 => {
            if i.len() < 5 {
                return Err(Err::Incomplete(Needed::Size(5)));
            }

            do_parse! {i,
			vendor_id: be_u32 >>
			typ: be_u8 >>
			len: verify!(be_u8, |val:&u8| *val >= 2) >>
			attr: flat_map!(take!(len-2),call!(parse_vendor_attribute_content, vendor_id, typ)) >>
			( attr )
			}
        }
		27 => map!{i, be_u32, |v| Attribute::SessionTimeout(v)},
		28 => map!{i, be_u32, |v| Attribute::IdleTimeout(v)},
		29 => map! {i, be_u32, |v| Attribute::TerminationAction(TerminationAction(v))},
		30 => value!(i, Attribute::CalledStationId(i)),
		31 => value!(i, Attribute::CallingStationId(i)),
		32 => value!(i, Attribute::NasIdentifier(i)),
		33 => value!(i, Attribute::ProxyState(i)),
		34 => value!(i, Attribute::LoginLatService(i)),
		35 => value!(i, Attribute::LoginLatNode(i)),
		36 => value!(i, Attribute::LoginLatGroup(i)),
		37 => map!{i, be_u32, |v| Attribute::FramedAppletalkLink(v)},
		38 => map!{i, be_u32, |v| Attribute::FramedAppletalkNetwork(v)},
		39 => value!(i, Attribute::FramedAppletalkZone(i)),
		60 => value!(i, Attribute::ChapChallenge(i)),
		61 => map! {i, be_u32, |v| Attribute::NasPortType(NasPortType(v))},
		62 => map!{i, be_u32, |v| Attribute::PortLimit(v)},
		63 => value!(i, Attribute::LoginLatPort(i)),
		40 => map! {i, be_u32, |v| Attribute::AcctStatusType(AcctStatusType(v))},
		41 => map!{i, be_u32, |v| Attribute::AcctDelayTime(v)},
		42 => map!{i, be_u32, |v| Attribute::AcctInputOctets(v)},
		43 => map!{i, be_u32, |v| Attribute::AcctOutputOctets(v)},
		44 => value!(i, Attribute::AcctSessionId(i)),
		45 => map! {i, be_u32, |v| Attribute::AcctAuthentic(AcctAuthentic(v))},
		46 => map!{i, be_u32, |v| Attribute::AcctSessionTime(v)},
		47 => map!{i, be_u32, |v| Attribute::AcctInputPackets(v)},
		48 => map!{i, be_u32, |v| Attribute::AcctOutputPackets(v)},
		49 => map! {i, be_u32, |v| Attribute::AcctTerminateCause(AcctTerminateCause(v))},
		50 => value!(i, Attribute::AcctMultiSessionId(i)),
		51 => map!{i, be_u32, |v| Attribute::AcctLinkCount(v)},
		68 => value!(i, Attribute::AcctTunnelConnection(i)),
		86 => map!{i, be_u32, |v| Attribute::AcctTunnelPacketsLost(v)},
		64 => map! {i, be_u32, |v| Attribute::TunnelType(TunnelType(v))},
		65 => map! {i, be_u32, |v| Attribute::TunnelMediumType(TunnelMediumType(v))},
		66 => value!(i, Attribute::TunnelClientEndpoint(i)),
		67 => value!(i, Attribute::TunnelServerEndpoint(i)),
		69 => value!(i, Attribute::TunnelPassword(i)),
		81 => value!(i, Attribute::TunnelPrivateGroupId(i)),
		82 => value!(i, Attribute::TunnelAssignmentId(i)),
		83 => map!{i, be_u32, |v| Attribute::TunnelPreference(v)},
		90 => value!(i, Attribute::TunnelClientAuthId(i)),
		91 => value!(i, Attribute::TunnelServerAuthId(i)),
		52 => map!{i, be_u32, |v| Attribute::AcctInputGigawords(v)},
		53 => map!{i, be_u32, |v| Attribute::AcctOutputGigawords(v)},
		55 => map!{i, be_u32, |v| Attribute::EventTimestamp(v)},
		70 => value!(i, Attribute::ArapPassword(i)),
		71 => value!(i, Attribute::ArapFeatures(i)),
		72 => map! {i, be_u32, |v| Attribute::ArapZoneAccess(ArapZoneAccess(v))},
		73 => map!{i, be_u32, |v| Attribute::ArapSecurity(v)},
		74 => value!(i, Attribute::ArapSecurityData(i)),
		75 => map!{i, be_u32, |v| Attribute::PasswordRetry(v)},
		76 => map! {i, be_u32, |v| Attribute::Prompt(Prompt(v))},
		77 => value!(i, Attribute::ConnectInfo(i)),
		78 => value!(i, Attribute::ConfigurationToken(i)),
		79 => value!(i, Attribute::EapMessage(i)),
		80 => value!(i, Attribute::MessageAuthenticator(i)),
		84 => value!(i, Attribute::ArapChallengeResponse(i)),
		85 => map!{i, be_u32, |v| Attribute::AcctInterimInterval(v)},
		87 => value!(i, Attribute::NasPortId(i)),
		88 => value!(i, Attribute::FramedPool(i)),
		95 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::NasIpv6Address(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		96 => value!(i, Attribute::FramedInterfaceId(i)),
		97 => value!(i, Attribute::FramedIpv6Prefix(i)),
		98 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::LoginIpv6Host(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		99 => value!(i, Attribute::FramedIpv6Route(i)),
		100 => value!(i, Attribute::FramedIpv6Pool(i)),
		101 => map! {i, be_u32, |v| Attribute::ErrorCause(ErrorCause(v))},
		102 => value!(i, Attribute::EapKeyName(i)),
		89 => value!(i, Attribute::ChargeableUserIdentity(i)),
		56 => map!{i, be_u32, |v| Attribute::EgressVlanid(v)},
		57 => map! {i, be_u32, |v| Attribute::IngressFilters(IngressFilters(v))},
		58 => value!(i, Attribute::EgressVlanName(i)),
		59 => value!(i, Attribute::UserPriorityTable(i)),
		123 => value!(i, Attribute::DelegatedIpv6Prefix(i)),
		92 => value!(i, Attribute::NasFilterRule(i)),
		124 => map!{i, be_u64, |v| Attribute::Mip6FeatureVector(v)},
		125 => value!(i, Attribute::Mip6HomeLinkPrefix(i)),
		126 => value!(i, Attribute::OperatorName(i)),
		127 => value!(i, Attribute::LocationInformation(i)),
		128 => value!(i, Attribute::LocationData(i)),
		129 => value!(i, Attribute::BasicLocationPolicyRules(i)),
		130 => value!(i, Attribute::ExtendedLocationPolicyRules(i)),
		131 => map! {i, be_u32, |v| Attribute::LocationCapable(LocationCapable(v))},
		132 => map! {i, be_u32, |v| Attribute::RequestedLocationInfo(RequestedLocationInfo(v))},
		133 => map! {i, be_u32, |v| Attribute::FramedManagement(FramedManagement(v))},
		134 => map! {i, be_u32, |v| Attribute::ManagementTransportProtection(ManagementTransportProtection(v))},
		135 => value!(i, Attribute::ManagementPolicyId(i)),
		136 => map!{i, be_u32, |v| Attribute::ManagementPrivilegeLevel(v)},
		137 => value!(i, Attribute::PkmSsCert(i)),
		138 => value!(i, Attribute::PkmCaCert(i)),
		139 => value!(i, Attribute::PkmConfigSettings(i)),
		140 => value!(i, Attribute::PkmCryptosuiteList(i)),
		141 => map!{i, be_u16, |v| Attribute::PkmSaid(v)},
		142 => value!(i, Attribute::PkmSaDescriptor(i)),
		143 => value!(i, Attribute::PkmAuthKey(i)),
		144 => value!(i, Attribute::DsLiteTunnelName(i)),
		145 => value!(i, Attribute::MobileNodeIdentifier(i)),
		146 => value!(i, Attribute::ServiceSelection(i)),
		147 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::Pmip6HomeLmaIpv6Address(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		148 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::Pmip6VisitedLmaIpv6Address(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		149 => map!{i, take!(4), |v:&[u8]| Attribute::Pmip6HomeLmaIpv4Address(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		150 => map!{i, take!(4), |v:&[u8]| Attribute::Pmip6VisitedLmaIpv4Address(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		151 => value!(i, Attribute::Pmip6HomeHnPrefix(i)),
		152 => value!(i, Attribute::Pmip6VisitedHnPrefix(i)),
		153 => value!(i, Attribute::Pmip6HomeInterfaceId(i)),
		154 => value!(i, Attribute::Pmip6VisitedInterfaceId(i)),
		155 => value!(i, Attribute::Pmip6HomeIpv4Hoa(i)),
		156 => value!(i, Attribute::Pmip6VisitedIpv4Hoa(i)),
		157 => map!{i, take!(4), |v:&[u8]| Attribute::Pmip6HomeDhcp4ServerAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		158 => map!{i, take!(4), |v:&[u8]| Attribute::Pmip6VisitedDhcp4ServerAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		159 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::Pmip6HomeDhcp6ServerAddress(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		160 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::Pmip6VisitedDhcp6ServerAddress(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		161 => map!{i, take!(4), |v:&[u8]| Attribute::Pmip6HomeIpv4Gateway(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		162 => map!{i, take!(4), |v:&[u8]| Attribute::Pmip6VisitedIpv4Gateway(Ipv4Addr::new(v[0],v[1],v[2],v[3]))},
		163 => map! {i, be_u32, |v| Attribute::EapLowerLayer(EapLowerLayer(v))},
		168 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::FramedIpv6Address(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		169 => map!{i, tuple!(be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16,be_u16), |v| Attribute::DnsServerIpv6Address(Ipv6Addr::new(v.0,v.1,v.2,v.3,v.4,v.5,v.6,v.7))},
		170 => value!(i, Attribute::RouteIpv6Information(i)),
		171 => value!(i, Attribute::DelegatedIpv6PrefixPool(i)),
		172 => value!(i, Attribute::StatefulIpv6AddressPool(i)),
		241 => value!(i, Attribute::ExtendedAttribute1(i)),
		242 => value!(i, Attribute::ExtendedAttribute2(i)),
		243 => value!(i, Attribute::ExtendedAttribute3(i)),
		244 => value!(i, Attribute::ExtendedAttribute4(i)),
		245 => value!(i, Attribute::ExtendedAttribute5(i)),
		246 => value!(i, Attribute::ExtendedAttribute6(i)),
		173 => value!(i, Attribute::Ipv66RdConfiguration(i)),
		164 => value!(i, Attribute::GssAcceptorServiceName(i)),
		165 => value!(i, Attribute::GssAcceptorHostName(i)),
		166 => value!(i, Attribute::GssAcceptorServiceSpecifics(i)),
		167 => value!(i, Attribute::GssAcceptorRealmName(i)),
		250 => map!{i, be_u32, |v| Attribute::CharNoecho(v)},
		94 => value!(i, Attribute::OriginatingLineInfo(i)),
		206 => value!(i, Attribute::DigestResponse(i)),
		207 => value!(i, Attribute::DigestAttributes(i)),
		_ => value!(i, Attribute::Unknown(t, i)),
}
}
pub fn parse_vendor_attribute_content(i: &[u8], vendor_id: u32, typ: u8) -> IResult<&[u8], Attribute> {
    match vendor_id {
		4 => unix::parse(&i, typ),
		5 => acc::parse(&i, typ),
		9 => cisco::parse(&i, typ),
		11 => hp::parse(&i, typ),
		43 => threecom::parse(&i, typ),
		52 => cabletron::parse(&i, typ),
		61 => merit::parse(&i, typ),
		64 => gandalf::parse(&i, typ),
		66 => citrix::parse(&i, typ),
		94 => nokia::parse(&i, typ),
		117 => telebit::parse(&i, typ),
		161 => motorola::parse(&i, typ),
		166 => shiva::parse(&i, typ),
		171 => dlink::parse(&i, typ),
		193 => ericsson::parse(&i, typ),
		255 => compatible::parse(&i, typ),
		266 => nexans::parse(&i, typ),
		272 => bintec::parse(&i, typ),
		307 => livingston::parse(&i, typ),
		311 => microsoft::parse(&i, typ),
		318 => apc::parse(&i, typ),
		388 => symbol::parse(&i, typ),
		429 => usr::parse(&i, typ),
		529 => ascend::parse(&i, typ),
		562 => nortel::parse(&i, typ),
		594 => bt::parse(&i, typ),
		674 => dellemc::parse(&i, typ),
		762 => karlnet::parse(&i, typ),
		800 => xylan::parse(&i, typ),
		831 => alu_aaa::parse(&i, typ),
		838 => xedia::parse(&i, typ),
		841 => proxim::parse(&i, typ),
		890 => zyxel::parse(&i, typ),
		969 => ntua::parse(&i, typ),
		1076 => surfnet::parse(&i, typ),
		1195 => itk::parse(&i, typ),
		1584 => bay_networks::parse(&i, typ),
		1588 => brocade::parse(&i, typ),
		1768 => patton::parse(&i, typ),
		1872 => alteon::parse(&i, typ),
		1916 => extreme::parse(&i, typ),
		1958 => redcreek::parse(&i, typ),
		1966 => perle::parse(&i, typ),
		1991 => foundry::parse(&i, typ),
		2004 => walabi::parse(&i, typ),
		2011 => huawei::parse(&i, typ),
		2180 => versanet::parse(&i, typ),
		2334 => packeteer::parse(&i, typ),
		2352 => ericsson_ab::parse(&i, typ),
		2356 => lancom::parse(&i, typ),
		2440 => efficientip::parse(&i, typ),
		2454 => sg::parse(&i, typ),
		2636 => juniper::parse(&i, typ),
		2979 => waverider::parse(&i, typ),
		3041 => alcatel::parse(&i, typ),
		3076 => cisco_asa::parse(&i, typ),
		3085 => cosine::parse(&i, typ),
		3199 => shasta::parse(&i, typ),
		3224 => netscreen::parse(&i, typ),
		3309 => nomadix::parse(&i, typ),
		3375 => f5::parse(&i, typ),
		3551 => springtide::parse(&i, typ),
		3561 => adsl_forum::parse(&i, typ),
		3902 => zte::parse(&i, typ),
		4329 => siemens::parse(&i, typ),
		4453 => infonet::parse(&i, typ),
		4491 => cablelabs::parse(&i, typ),
		4735 => prosoft::parse(&i, typ),
		4846 => lucent::parse(&i, typ),
		4874 => unisphere::parse(&i, typ),
		5089 => clavister::parse(&i, typ),
		5263 => cisco_bbsm::parse(&i, typ),
		5468 => actelis::parse(&i, typ),
		5535 => threegpp2::parse(&i, typ),
		5567 => riverstone::parse(&i, typ),
		5597 => meinberg::parse(&i, typ),
		5925 => ipunplugged::parse(&i, typ),
		5948 => issanni::parse(&i, typ),
		6431 => broadsoft::parse(&i, typ),
		6504 => navini::parse(&i, typ),
		6527 => alcatel_lucent_service_router::parse(&i, typ),
		6618 => quintum::parse(&i, typ),
		7000 => slipstream::parse(&i, typ),
		7064 => utstarcom::parse(&i, typ),
		7119 => networkphysics::parse(&i, typ),
		7146 => zeus::parse(&i, typ),
		7262 => dragonwave::parse(&i, typ),
		7751 => azaire::parse(&i, typ),
		8164 => starent::parse(&i, typ),
		8741 => sonicwall::parse(&i, typ),
		8744 => colubris::parse(&i, typ),
		9148 => acme::parse(&i, typ),
		9694 => arbor::parse(&i, typ),
		10055 => roaring_penguin::parse(&i, typ),
		10415 => threegpp::parse(&i, typ),
		10529 => gemtek::parse(&i, typ),
		10923 => ericsson_packet_core_networks::parse(&i, typ),
		11344 => freeradius::parse(&i, typ),
		12356 => fortinet::parse(&i, typ),
		12394 => alvarion::parse(&i, typ),
		12740 => equallogic::parse(&i, typ),
		12951 => verizonwireless::parse(&i, typ),
		13209 => aptilo::parse(&i, typ),
		14122 => wispr::parse(&i, typ),
		14179 => airespace::parse(&i, typ),
		14436 => quiconnect::parse(&i, typ),
		14501 => bluecoat::parse(&i, typ),
		14525 => trapeze::parse(&i, typ),
		14529 => tropos::parse(&i, typ),
		14559 => chillispot::parse(&i, typ),
		14823 => aruba::parse(&i, typ),
		14895 => propel::parse(&i, typ),
		14988 => mikrotik::parse(&i, typ),
		15004 => ruggedcom::parse(&i, typ),
		15983 => meru::parse(&i, typ),
		16313 => valemountnetworks::parse(&i, typ),
		16445 => kineto::parse(&i, typ),
		16459 => epygi::parse(&i, typ),
		16787 => t_systems_nova::parse(&i, typ),
		16924 => bskyb::parse(&i, typ),
		17163 => riverbed::parse(&i, typ),
		18681 => travelping::parse(&i, typ),
		19211 => jradius::parse(&i, typ),
		19220 => local_web::parse(&i, typ),
		19382 => manzara::parse(&i, typ),
		21074 => purewave::parse(&i, typ),
		21274 => camiant::parse(&i, typ),
		22736 => digium::parse(&i, typ),
		24023 => iea_software::parse(&i, typ),
		24757 => wimax::parse(&i, typ),
		25053 => ruckus::parse(&i, typ),
		25178 => terena::parse(&i, typ),
		25461 => paloalto::parse(&i, typ),
		25506 => h3c::parse(&i, typ),
		25622 => ukerna::parse(&i, typ),
		27030 => wichorus::parse(&i, typ),
		27262 => dante::parse(&i, typ),
		27880 => freeswitch::parse(&i, typ),
		29671 => meraki::parse(&i, typ),
		34536 => fdxtended::parse(&i, typ),
		35265 => eltex::parse(&i, typ),
		35987 => netborder::parse(&i, typ),
		40676 => microsemi::parse(&i, typ),
		41482 => yubico::parse(&i, typ),
		49426 => cnergee::parse(&i, typ),
        _ => value!(i, Attribute::VsaUnknown(vendor_id, typ, i)),
    }
}
