use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InfiniSyncConfig {
    info: Info,
    configuration: Configuration,
    logger: Logger,
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    version: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Configuration {
    smf_name: String,
    sbi: Sbi,
    service_name_list: Vec<String>,
    snssai_infos: Vec<SnssaiInfo>,
    plmn_list: Option<Vec<Plmn>>,
    locality: String,
    pfcp: PfcpConfig,
    userplane_information: UserplaneInformation,
    t3591: RetransmissionTimer,
    t3592: RetransmissionTimer,
    nrf_uri: String,
    // urr_period: Option<u32>,
    // urr_threshold: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sbi {
    scheme: String,
    register_ip_v4: String,
    binding_ip_v4: String,
    port: u16,
    tls: TlsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct TlsConfig {
    key: String,
    pem: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SnssaiInfo {
    s_nssai: Snssai,
    dnn_infos: Vec<DnnInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Snssai {
    sst: u8,
    sd: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DnnInfo {
    dnn: String,
    dns: DnsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct DnsConfig {
    ipv4: String,
    ipv6: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Plmn {
    mcc: String,
    mnc: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PfcpConfig {
    node_id: String,
    listen_addr: String,
    external_addr: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserplaneInformation {
    up_nodes: std::collections::HashMap<String, UpNode>,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpNode {
    #[serde(rename = "type")]
    node_type: String,
    node_id: Option<String>,
    addr: Option<String>,
    s_nssai_upf_infos: Option<Vec<SnssaiUpfInfo>>,
    interfaces: Option<Vec<Interface>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SnssaiUpfInfo {
    s_nssai: Snssai,
    dnn_upf_info_list: Vec<DnnUpfInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DnnUpfInfo {
    dnn: String,
    pools: Vec<CidrPool>,
    static_pools: Vec<CidrPool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CidrPool {
    cidr: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Interface {
    interface_type: String,
    endpoints: Vec<String>,
    network_instances: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
    #[serde(rename = "A")]
    a: String,
    #[serde(rename = "B")]
    b: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RetransmissionTimer {
    enable: bool,
    expire_time: String,
    max_retry_times: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Logger {
    enable: bool,
    level: String,
    report_caller: bool,
}
