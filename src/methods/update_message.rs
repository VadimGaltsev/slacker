use crate::request_data::{LayoutBlock, AttachmentType, Channel};
use serde::{Serialize, Deserialize};
use crate::methods::SlackRequest;
use crate::PostMessageResponse;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateMessage {
    pub text: String,
    pub channel: String,
    pub ts: String,
    pub blocks: Option<Vec<LayoutBlock>>,
    pub as_user: Option<bool>,
    pub attachment: Option<Vec<AttachmentType>>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
}

impl UpdateMessage {
    pub fn new(message: &str, channel: &str, ts: &str) -> UpdateMessage {
        UpdateMessage {
            text: message.to_owned(),
            channel: channel.to_owned(),
            ts: ts.to_owned(),
            ..Default::default()
        }
    }

    pub fn channel(mut self, channel: Channel) -> Self {
        self.channel = channel.id;
        self
    }

    pub fn channel_str(mut self, channel: &str) -> Self {
        self.channel = channel.to_owned();
        self
    }

    pub fn add_block<T: Into<LayoutBlock>>(mut self, block: T) -> Self {
        if let Some(blocks) = &mut self.blocks {
            blocks.push(block.into())
        } else {
            self.blocks = Some(vec![block.into()])
        }
        self
    }

    pub fn with_blocks<T: Into<LayoutBlock>>(mut self, blocks: Vec<T>) -> Self {
        self.blocks = Some(blocks.into_iter().map(|e| e.into()).collect());
        self
    }
}

impl SlackRequest<PostMessageResponse> for UpdateMessage {
    const METHOD_NAME: &'static str = "chat.update";
}
