#![allow(dead_code)]
use crate::request_data::layout_blocks::LayoutBlock;
use crate::request_data::composite_object::TextObject;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldObject {
    title: Option<String>,
    value: Option<String>,
    short: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum AttachmentType {
    FieldReference {
        blocks: Option<Vec<LayoutBlock>>,
        color: Option<String>
    },
    LegacyField {
        author_icon: Option<LayoutBlock>,
        author_link: Option<TextObject>,
        author_name: Option<TextObject>,
        fallback: Option<String>,
        fields: Option<Vec<FieldObject>>,
        footer: Option<LayoutBlock>,
        footer_icon: Option<LayoutBlock>,
        image_url: Option<LayoutBlock>,
        mrkdwn_in: Option<TextObject>,
        pretext: Option<LayoutBlock>,
        text: Option<LayoutBlock>,
        thumb_url: Option<LayoutBlock>,
        title: Option<LayoutBlock>,
        title_link: Option<LayoutBlock>,
        ts: Option<LayoutBlock>
    }
}