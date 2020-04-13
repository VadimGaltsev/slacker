use serde::{Serialize, Deserialize};
use crate::LayoutBlock;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Message {
    pub(crate) r#type: String,
    pub(crate) sub_type: String,
    pub(crate) text: String,
    pub(crate) ts: String,
    pub(crate) channel: String,
    pub(crate) blocks: Vec<LayoutBlock>,
    pub(crate) id: u32,
    pub(crate) is_im: bool,
    pub(crate) user: String,
    pub(crate) username: String,
    pub(crate) bot_id: String,
    pub(crate) created: u64,
    pub(crate) is_user_deleted: bool,
    pub(crate) attachments: Vec<Attachment>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Attachment {
    text: String,
    id: u32,
    fallback: String,
}
