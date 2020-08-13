pub enum PvcEncapsulationType {
    AaaEncapsAtmRaw = 1,
    AaaEncapsAtmRoute1483 = 2,
    AaaEncapsAtmAuto1483 = 3,
    AaaEncapsAtmMulti = 4,
    AaaEncapsAtmBridge1483 = 5,
    AaaEncapsAtmPpp = 6,
    AaaEncapsAtmPppSerial = 7,
    AaaEncapsAtmPppNlpid = 8,
    AaaEncapsAtmPppAuto = 9,
    AaaEncapsAtmPppoe = 10,
    AaaEncapsAtmL2Tp = 11,
    AaaEncapsAtmPppLlc = 12,
    AaaEncapsFrameAuto1490 = 13,
    AaaEncapsFrameMulti = 14,
    AaaEncapsFrameBridge1490 = 15,
    AaaEncapsFramePpp = 16,
    AaaEncapsFramePppAuto = 17,
    AaaEncapsFramePppoe = 18,
    AaaEncapsFrameRoute1490 = 19,
    AaaEncapsFrameL2Tp = 20,
    AaaEncapsL2TpVcMuxed = 21,
    AaaEncapsEth = 22,
    AaaEncapsEthPppoe = 23,
    AaaEncapsEthMulti = 24,
    AaaEncapsEthDot1Q = 25,
    AaaEncapsEthDot1QPppoe = 26,
    AaaEncapsAtmMultiPppoe = 27,
    AaaEncapsAtmMultiIpv6Oe = 28,
    AaaEncapsAtmMultiPppoeNIpv6Oe = 29,
    AaaEncapsEthDot1QTunnel = 30,
    AaaEncapsEthDot1QTunnelPppoe = 31,
}

pub enum PvcCircuitPadding {
    AaaCircuitPadding = 1,
    AaaCircuitNoPadding = 2,
}

pub enum BindType {
    AaaAuthBind = 1,
    AaaBypassBind = 2,
    AaaInterfaceBind = 3,
    AaaSubscribeBind = 4,
    AaaTunnelBind = 5,
    AaaSessionBind = 6,
    AaaQ8021Bind = 7,
    AaaMultiBind = 8,
    AaaDhcpBind = 9,
    AaaMultiBindSub = 10,
    AaaBridgeGroupBind = 11,
    AaaVlanBind = 12,
    AaaVlanGroupBind = 13,
    AaaAutoSubscriberBind = 14,
}

pub enum BindAuthProtocol {
    AaaPppPap = 1,
    AaaPppChap = 2,
    AaaPppChapWait = 3,
    AaaPppChapPap = 4,
    AaaPppChapWaitPap = 5,
    AaaPppEap = 6,
    AaaPppPapChap = 7,
    AaaPppPapChapWait = 8,
}

pub enum SourceValidation {
    Enabled = 1,
    Disabled = 2,
}

pub enum TunnelDomain {
    Enabled = 1,
    Disabled = 2,
}

pub enum TunnelFunction {
    LacOnly = 1,
    LnsOnly = 2,
    LacLns = 3,
}

pub enum TunnelSessionAuth {
    Chap = 1,
    Pap = 2,
    ChapPap = 3,
}

pub enum TunnelGroup {
    Enabled = 1,
    Disabled = 2,
}

pub enum TunnelAlgorithm {
    First = 1,
    LoadBalance = 2,
    Wrr = 3,
}

pub enum McastSend {
    NoSend = 1,
    Send = 2,
    UnsolicitedSend = 3,
}

pub enum McastReceive {
    NoReceive = 1,
    Receive = 2,
}

pub enum TunnelDnis {
    Dnis = 1,
    DnisOnly = 2,
}

pub enum PlatformType {
    Sms = 1,
    Smartedge800 = 2,
    Se400 = 3,
    Se100 = 4,
}

pub enum CircuitProtocolEncap {
    EncapsPppoe = 27,
}

pub enum MediumType {
    Dsl = 11,
    Cable = 12,
    Wireless = 13,
    Satellite = 14,
}

pub enum IpTosField {
    Normal = 0,
    MinCostOnly = 1,
    MaxReliabilityOnly = 2,
    MaxReliabilityPlusMinCost = 3,
    MaxThroughputOnly = 4,
    MaxThroughputPlusMinCost = 5,
    MaxThroughputPlusMaxReliability = 6,
    MaxThroughputPlusMaxReliabilityPlusMinCost = 7,
    MinDelayOnly = 8,
    MinDelayPlusMinCost = 9,
    MinDelayPlusMaxReliability = 10,
    MinDelayPlusMaxReliabilityPlusMinCost = 11,
    MinDelayPlusMaxThroughput = 12,
    MinDelayPlusMaxThroughputPlusMinCost = 13,
    MinDelayPlusMaxThroughputPlusMaxReliability = 14,
    MinDelayPlusMaxThroughputPlusMaxReliabilityPlusMinCost = 15,
}

pub enum LacPortType {
    NasPortType10Bt = 40,
    NasPortType100Bt = 41,
    NasPortTypeDs3Fr = 42,
    NasPortTypeDs3Atm = 43,
    NasPortTypeOc3 = 44,
    NasPortTypeHssi = 45,
    NasPortTypeEia530 = 46,
    NasPortTypeT1 = 47,
    NasPortTypeChanT3 = 48,
    NasPortTypeDs1Fr = 49,
    NasPortTypeE3Atm = 50,
    NasPortTypeImaAtm = 51,
    NasPortTypeDs3Atm2 = 52,
    NasPortTypeOc3Atm2 = 53,
    NasPortType1000Bsx = 54,
    NasPortTypeE1Fr = 55,
    NasPortTypeE1Atm = 56,
    NasPortTypeE3Fr = 57,
    NasPortTypeOc3Pos = 58,
    NasPortTypeOc12Pos = 59,
    NasPortTypePppoe = 60,
}

pub enum LacRealPortType {
    NasPortType10Bt = 40,
    NasPortType100Bt = 41,
    NasPortTypeDs3Fr = 42,
    NasPortTypeDs3Atm = 43,
    NasPortTypeOc3 = 44,
    NasPortTypeHssi = 45,
    NasPortTypeEia530 = 46,
    NasPortTypeT1 = 47,
    NasPortTypeChanT3 = 48,
    NasPortTypeDs1Fr = 49,
    NasPortTypeE3Atm = 50,
    NasPortTypeImaAtm = 51,
    NasPortTypeDs3Atm2 = 52,
    NasPortTypeOc3Atm2 = 53,
    NasPortType1000Bsx = 54,
    NasPortTypeE1Fr = 55,
    NasPortTypeE1Atm = 56,
    NasPortTypeE3Fr = 57,
    NasPortTypeOc3Pos = 58,
    NasPortTypeOc12Pos = 59,
    NasPortTypePppoe = 60,
}

pub enum AcctUpdateReason {
    AaaLoadAcctSessionUp = 1,
    AaaLoadAcctSessionDown = 2,
    AaaLoadAcctPeriodic = 3,
    AaaLoadAcctDynAcEntStart = 4,
    AaaLoadAcctDynAcEntStop = 5,
    AaaLoadAcctDynAcEntTimeout = 6,
    AaaLoadAcctSubscriberReauthor = 7,
    AaaLoadAcctPppIpcpUp = 8,
    AaaLoadAcctPppMpLinkUp = 9,
    AaaLoadAcctDhcpIpAddrGranted = 10,
    AaaLoadAcctDhcpIpAddrReleased = 11,
    AaaLoadAcctAclTimeredAction = 12,
    AaaLoadAcctAclAction = 13,
    AaaLoadAcctCmd = 14,
    AaaLoadAcctTest = 15,
}

pub enum DslLineState {
    Showtime = 1,
    Idle = 2,
    Silent = 3,
}

pub enum DslTransmissionSystem {
    Adsl1 = 1,
    Adsl2 = 2,
    Adsl2Plus = 3,
    Vdsl1 = 4,
    Vdsl2 = 5,
    Sdsl = 6,
    Unknown = 7,
}

pub enum ServiceAction {
    DeActivate = 0,
    ActivateWithAcct = 1,
    ActivateWithoutAcct = 2,
}
