use std::net::{Ipv4Addr, Ipv6Addr};

use super::{dns_settings::DnsSettings, DnsCommon, RecordMessage, ToRecordMessage};
use cloudflare_derive::DnsCommon;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, Default, DnsCommon)]
pub struct ARecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Ipv4Addr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DnsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_modified_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_modified_on: Option<String>,
}

impl ToRecordMessage for ARecord {
    fn to_record_message(self) -> RecordMessage {
        RecordMessage::A(self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, DnsCommon)]
pub struct AAAARecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Ipv6Addr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DnsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_modified_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_modified_on: Option<String>,
}

impl ToRecordMessage for AAAARecord {
    fn to_record_message(self) -> RecordMessage {
        RecordMessage::AAAA(self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, DnsCommon)]
pub struct CNAMERecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ToRecordMessage for CNAMERecord {
    fn to_record_message(self) -> RecordMessage {
        RecordMessage::CNAME(self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Meta(Value);

#[derive(Serialize, Deserialize, Debug, Clone, Default, DnsCommon)]
pub struct MXRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxiable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DnsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
}

impl ToRecordMessage for MXRecord {
    fn to_record_message(self) -> RecordMessage {
        RecordMessage::MX(self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, DnsCommon)]
pub struct TXTRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxiable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DnsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
}

impl ToRecordMessage for TXTRecord {
    fn to_record_message(self) -> RecordMessage {
        RecordMessage::TXT(self)
    }
}
