use super::NFClient;

mod ausf;

use ausf::AusfClient;

pub struct NfClients {
    ausf_client: AusfClient,
}