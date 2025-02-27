use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DnsSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_only: Option<bool>,
}
