use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataWarpConfig {
    version: String,
    description: String,
    pfcp: Pfcp,
    gtpu: Gtpu,
    dnn_list: Vec<Dnn>,
    logger: Logger,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pfcp {
    addr: String,
    node_id: String,
    retrans_timeout: String,
    max_retrans: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gtpu {
    forwarder: String,
    if_list: Vec<Interface>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Interface {
    addr: String,
    #[serde(rename = "type")]
    interface_type: String,
    // Uncomment or add more fields as necessary
    // name: Option<String>,
    // if_name: Option<String>,
    // mtu: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dnn {
    dnn: String,
    cidr: String,
    // Uncomment or add more fields as necessary
    // nat_if_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Logger {
    enable: bool,
    level: String,
    report_caller: bool,
}