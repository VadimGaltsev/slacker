use crate::request_data::{LayoutBlock, AttachmentType, Channel};
use serde::{Serialize, Deserialize};
use crate::methods::SlackRequest;
use crate::PostMessageResponse;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct PostMessage {
    pub text: String,
    pub channel: String,
    pub blocks: Option<Vec<LayoutBlock>>,
    pub as_user: Option<bool>,
    pub attachment: Option<Vec<AttachmentType>>,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub link_names: Option<bool>,
    pub mrkdwn: Option<bool>,
    pub parse: Option<String>,
    pub reply_broadcast: Option<bool>,
    pub thread_ts: Option<String>,
    pub unfurl_links: Option<bool>,
    pub unfurl_media: Option<bool>,
    pub username: Option<String>,
    pub response_type: Option<MessageVisibility>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MessageVisibility {
    InChannel,
    Ephemeral,
}

impl Default for MessageVisibility {
    fn default() -> Self {
        MessageVisibility::Ephemeral
    }
}

impl PostMessage {
    pub fn new(message: &str) -> PostMessage {
        PostMessage {
            text: message.to_owned(),
            ..Default::default()
        }
    }

    pub fn set_response_type(mut self, visibility: MessageVisibility) -> Self {
        self.response_type = Some(visibility);
        self
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

impl From<PostMessage> for Vec<LayoutBlock> {
    fn from(message: PostMessage) -> Self {
        message.blocks.unwrap()
    }
}

impl SlackRequest<PostMessageResponse> for PostMessage {
    const METHOD_NAME: &'static str = "chat.postMessage";
}
