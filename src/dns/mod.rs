pub mod dns_settings;
pub mod record;
pub use record::{AAAARecord, ARecord, CNAMERecord, MXRecord, TXTRecord};
use std::net::IpAddr;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch(DnsCommonEnum)]
pub trait DnsCommon {
    fn get_id(self) -> Option<String>;
    fn set_id(&mut self, v: Option<String>);
    fn get_zone_id(self) -> Option<String>;
    fn set_zone_id(&mut self, v: Option<String>);
    fn get_zone_name(self) -> Option<String>;
    fn set_zone_name(&mut self, v: Option<String>);
    fn get_name(self) -> Option<String>;
    fn set_name(&mut self, v: Option<String>);
}

pub struct ContentError(String);

pub trait DnsIpContent {
    fn get_content(self) -> IpAddr;
    fn set_content(&mut self, addr: IpAddr) -> Result<(), ContentError>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[enum_dispatch]
pub enum RecordMessage {
    A(ARecord),
    AAAA(AAAARecord),
    CNAME(CNAMERecord),
    MX(MXRecord),
    TXT(TXTRecord),
}

pub trait ToRecordMessage {
    fn to_record_message(self) -> RecordMessage;
}

impl RecordMessage {
    pub fn for_update(self) -> Self {
        match self {
            RecordMessage::A(r) => {
                let mut rm = r.clone();
                rm.id = None;
                rm.zone_id = None;
                rm.zone_name = None;
                rm.meta = None;
                RecordMessage::A(rm)
            }
            RecordMessage::AAAA(r) => {
                let mut rm = r.clone();
                rm.id = None;
                rm.zone_id = None;
                rm.zone_name = None;
                rm.meta = None;
                RecordMessage::AAAA(rm)
            }
            RecordMessage::CNAME(r) => {
                let mut rm = r.clone();
                rm.id = None;
                rm.zone_id = None;
                rm.zone_name = None;
                rm.meta = None;
                RecordMessage::CNAME(rm)
            }
            RecordMessage::MX(r) => {
                let mut rm = r.clone();
                rm.id = None;
                rm.zone_id = None;
                rm.zone_name = None;
                rm.meta = None;
                RecordMessage::MX(rm)
            }
            RecordMessage::TXT(r) => {
                let mut rm = r.clone();
                rm.id = None;
                rm.zone_id = None;
                rm.zone_name = None;
                rm.meta = None;
                RecordMessage::TXT(rm)
            }
        }
    }
}
