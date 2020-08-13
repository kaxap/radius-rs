pub enum ErxIngressStatistics {
    Disable = 0,
    Enable = 1,
}

pub enum ErxEgressStatistics {
    Disable = 0,
    Enable = 1,
}

pub enum ErxAtmServiceCategory {
    Ubr = 1,
    Ubrpcr = 2,
    Nrtvbr = 3,
    Cbr = 4,
}

pub enum ErxCliAllowAllVrAccess {
    Disable = 0,
    Enable = 1,
}

pub enum ErxSaValidate {
    Disable = 0,
    Enable = 1,
}

pub enum ErxIgmpEnable {
    Disable = 0,
    Enable = 1,
}

pub enum ErxQosProfileInterfaceType {
    Ip = 1,
    Atm = 2,
    Hdlc = 3,
    Ethernet = 4,
    ServerPort = 5,
    Atm1483 = 6,
    FrameRelay = 7,
    MplsMinor = 8,
    Cbf = 9,
    IpTunnel = 10,
    VlanSub = 11,
    PppoeSub = 12,
}

pub enum ErxTunnelNasPortMethod {
    None = 0,
    CiscoClid = 1,
}

pub enum ErxPppAuthProtocol {
    None = 0,
    Pap = 1,
    Chap = 2,
    PapChap = 3,
    ChapPap = 4,
}

pub enum ErxBearerType {
    None = 0,
    Analog = 1,
    Digital = 2,
}

pub enum ErxLiAction {
    Off = 0,
    On = 1,
    Noop = 2,
}

pub enum ErxDfBit {
    DontIgnoreDfBit = 0,
    IgnoreDfBit = 1,
}

pub enum ErxMldVersion {
    V1 = 1,
    V2 = 2,
}

pub enum ErxIgmpVersion {
    V1 = 1,
    V2 = 2,
    V3 = 3,
}

pub enum ErxServiceStatistics {
    Disabled = 0,
    Time = 1,
    TimeVolume = 2,
}

pub enum ErxL2TpResynchMethod {
    Disable = 0,
    Failover = 1,
    SilentFailover = 2,
    FailoverWithSilentBackup = 3,
}

pub enum ErxTunnelTxSpeedMethod {
    StaticLayer2 = 1,
    DynamicLayer2 = 2,
    Qos = 3,
    Actual = 4,
}

pub enum ErxIgmpImmediateLeave {
    Disabled = 0,
    Enabled = 1,
}

pub enum ErxMldImmediateLeave {
    Disabled = 0,
    Enabled = 1,
}

pub enum ErxIpBlockMulticast {
    Disabled = 0,
    Enabled = 1,
}

pub enum ErxIgmpExplicitTracking {
    Disabled = 0,
    Enabled = 1,
}

pub enum ErxIgmpNoTrackingV2Grps {
    Disabled = 0,
    Enabled = 1,
}

pub enum ErxMldExplicitTracking {
    Disabled = 0,
    Enabled = 1,
}

pub enum ErxMldNoTrackingV1Grps {
    Disabled = 0,
    Enabled = 1,
}

pub enum ErxDslLineState {
    Showtime = 1,
    Idle = 2,
    Silent = 3,
}

pub enum ErxDslType {
    Adsl1 = 1,
    Adsl2 = 2,
    Adsl2Plus = 3,
    Vdsl1 = 4,
    Vdsl2 = 5,
    Sdsl = 6,
    Unknown = 7,
}

pub enum ErxPppMonitorIngressOnly {
    Disabled = 0,
    Enabled = 1,
}
