#![allow(dead_code)]
use serde::{Serialize, Deserialize};
use crate::request_data::message::Message;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChannelType {
    ok: bool,
    channel: Channel,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Channel {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) is_channel: bool,
    pub(crate) created: u64,
    pub(crate) creator: String,
    pub(crate) is_archived: bool,
    pub(crate) is_general: bool,
    pub(crate) name_normalized: String,
    pub(crate) is_shared: bool,
    pub(crate) is_org_shared: bool,
    pub(crate) is_member: bool,
    pub(crate) is_private: bool,
    pub(crate) is_mpim: bool,
    pub(crate) last_read: String,
    pub(crate) latest: Message,
    pub(crate) unread_count: u32,
    pub(crate) unread_count_display: u32,
    pub(crate) members: Vec<String>,
    pub(crate) topic: Topic,
    pub(crate) purpose: Purpose,
    pub(crate) previous_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Topic {
    value: String,
    creator: String,
    last_set: u64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Purpose {
    value: String,
    creator: String,
    last_set: u64,
}