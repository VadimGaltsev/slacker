#![allow(dead_code)]
use crate::request_data::layout_blocks::LayoutBlock;
use crate::request_data::secondary_attachments::AttachmentType;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePayload {
    text: String,
    blocks: Option<Vec<LayoutBlock>>,
    attachments: Option<Vec<AttachmentType>>,
    thread_rs: Option<String>,
    mrkdwn: Option<bool>,
}

#[cfg(test)]
mod test {
    use crate::request_data::message_payloads::MessagePayload;
    use crate::request_data::layout_blocks::LayoutBlock;
    use crate::request_data::block_elements::BlockElement;

    #[test]
    fn should_serialize_message_payloads() {
//        let payload = MessagePayload {
//            text: "".to_string(),
//            blocks: Some(vec![LayoutBlock::Context {
//                elements: vec![BlockElement::Image {
//                    image_url: "URL".to_owned(),
//                    alt_text: "altertext".to_owned(),
//                }],
//                block_id: Some("ID".to_owned()),
//            }]),
//            attachments: None,
//            thread_rs: Some("thread?".to_owned()),
//            mrkdwn: Some(true)
//        };
//        let json = serde_json::to_string(&payload).unwrap();
//        let expected = r#"{"text":"","blocks":[{"type":"context","elements":[{"type":"image",
//        "image_url":"URL","alt_text":"altertext"}],
//        "block_id":"ID"}],"attachments":null,"thread_rs":"thread?","mrkdwn":true}"#
//            .replace("\n", "")
//            .replace(" ", "");
//        assert_eq!(expected, json)
    }
}