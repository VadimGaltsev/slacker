#![allow(dead_code)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ConfirmationDialogObject {
    pub title: String,
    pub text: String,
    pub confirm: String,
    pub deny: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextObject {
    PlainText {
        #[serde(flatten)]
        text: Text
    },
    Mrkdwn {
        #[serde(flatten)]
        text: Text
    },
}

impl Default for TextObject {
    fn default() -> Self {
        TextObject::PlainText {
            text: Default::default()
        }
    }
}

impl Into<TextObject> for Text {
    fn into(self) -> TextObject {
        match self.emoji {
            Some(ref _is_emoji) => TextObject::PlainText { text: self },
            _ => TextObject::Mrkdwn { text: self }
        }
    }
}

impl Into<Text> for TextObject {
    fn into(self) -> Text {
        match self {
            TextObject::PlainText { text } => text,
            TextObject::Mrkdwn { text } => text,
        }
    }
}

impl Into<TextObject> for &str {
    fn into(self) -> TextObject {
        TextObject::new_plain_text(self).into()
    }
}

impl TextObject {
    pub fn new_plain_text(text: &str) -> Text {
        let text = text.to_owned();
        Text {
            text,
            emoji: Some(true),
            verbatim: None,
        }
    }

    pub fn new_mrkdwn_text(text: &str) -> Text {
        let text = text.to_owned();
        Text {
            text,
            emoji: None,
            verbatim: Some(true),
        }
    }

    pub fn new_text(text: &str) -> Text {
        let text = text.to_owned();
        Text {
            text,
            emoji: None,
            verbatim: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct Text {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbatim: Option<bool>,
}

impl Text {
    pub fn set_text(mut self, text: String) -> Text {
        self.text = text;
        self
    }

    pub fn set_emoji(mut self, is_emoji: bool) -> Text {
        self.verbatim = None;
        self.emoji = Some(is_emoji);
        self
    }

    pub fn set_verbatim(mut self, is_verbatim: bool) -> Text {
        self.emoji = None;
        self.verbatim = Some(is_verbatim);
        self
    }

    pub fn build(self) -> TextObject {
        if self.emoji.unwrap_or(false) {
            TextObject::PlainText {
                text: self
            }
        } else {
            TextObject::Mrkdwn {
                text: self
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionObject {
    pub text: TextObject,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl OptionObject {
    pub fn new(label: &str) -> Self {
        OptionObject{
            text: label.into(),
            value: label.to_owned(),
            url: None
        }
    }
}

impl Into<OptionObject> for &str {
    fn into(self) -> OptionObject {
        OptionObject::new(self)
    }
}

impl Into<OptionObject> for String {
    fn into(self) -> OptionObject {
        OptionObject::new(&self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionObjectGroup {
    pub label: TextObject,
    pub options: Vec<OptionObject>,
}