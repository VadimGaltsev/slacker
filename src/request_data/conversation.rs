#![allow(dead_code)]
use serde_json::Value;
use crate::request_data::channel::{Topic, Purpose};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct ConversationType {
    ok: bool,
    channel: ConversationChannel,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct ConversationChannel {
    id: String,
    name: String,
    is_channel: bool,
    is_group: bool,
    is_im: bool,
    created: u64,
    creator: String,
    is_archived: bool,
    is_general: bool,
    unlinked: u32,
    name_normalized: String,
    is_read_only: bool,
    is_shared: bool,
    is_ext_shared: bool,
    is_org_shared: bool,
    pending_shared: Vec<Value>,
    is_pending_ext_shared: bool,
    is_member: bool,
    is_private: bool,
    is_mpim: bool,
    last_read: String,
    topic: Topic,
    purpose: Purpose,
    previous_names: Vec<String>,
    num_members: u32,
    locale: String,
}