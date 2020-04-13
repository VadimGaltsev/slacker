#![allow(dead_code)]

use crate::request_data::composite_object::{ConfirmationDialogObject, TextObject, OptionObject, OptionObjectGroup, Text};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlockElement {
    Image {
        #[serde(flatten)]
        image: Image
    },
    Button {
        #[serde(flatten)]
        button: Button
    },
    StaticSelect {
        #[serde(flatten)]
        static_select: StaticSelect
    },
    ExternalSelect {
        #[serde(flatten)]
        external_static: ExternalStatic
    },
    UsersSelect {
        placeholder: TextObject,
        action_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        initial_user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        confirm: Option<ConfirmationDialogObject>,
    },
    ConversationsSelect {
        placeholder: TextObject,
        action_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        initial_conversation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        confirm: Option<ConfirmationDialogObject>,
    },
    ChannelsSelect {
        placeholder: TextObject,
        action_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        initial_channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        confirm: Option<ConfirmationDialogObject>,
    },
    Overflow {
        action_id: String,
        options: Vec<OptionObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        confirm: Option<ConfirmationDialogObject>,
    },
    #[serde(rename = "datepicker")]
    DatePicker {
        action_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        placeholder: Option<TextObject>,
        #[serde(skip_serializing_if = "Option::is_none")]
        initial_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        confirm: Option<ConfirmationDialogObject>,
    },
    PlainText {
        #[serde(flatten)]
        text: Text
    },
    Mrkdwn {
        #[serde(flatten)]
        text: Text
    },
}

//todo change all for blocks and remove builders and make constructor
impl BlockElement {
    pub fn new_image(image_url: String, alt_text: String) -> Image {
        Image {
            image_url,
            alt_text,
        }
    }

    pub fn new_button<T: Into<TextObject>>(text: T, action_id: String) -> Button {
        Button {
            text: text.into(),
            action_id,
            url: None,
            value: None,
            style: None,
            confirm: None,
        }
    }

    pub fn new_static_select<T: Into<TextObject>>(placeholder: T, action_id: String) -> StaticSelect {
        StaticSelect {
            placeholder: placeholder.into(),
            action_id,
            options: vec![],
            option_groups: None,
            initial_option: None,
            confirm: None,
        }
    }

    pub fn new_static_select_with_options(
        placeholder: TextObject,
        action_id: String,
        options: Vec<OptionObject>,
    ) -> StaticSelect {
        StaticSelect {
            placeholder,
            action_id,
            options,
            option_groups: None,
            initial_option: None,
            confirm: None,
        }
    }

    pub fn new_external_select(placeholder: TextObject, action_id: String) -> ExternalStatic {
        ExternalStatic {
            placeholder,
            action_id,
            initial_option: None,
            min_query_length: None,
            confirm: None,
        }
    }

    pub fn new_channel_select<T: Into<TextObject>>(text: T, action_id: String) -> BlockElement {
        BlockElement::ChannelsSelect {
            placeholder: text.into(),
            action_id,
            initial_channel: None,
            confirm: None,
        }
    }

    pub fn new_text_element<T: Into<TextObject>>(text: T) -> Text {
        text.into().into()
    }

    pub fn new_mrkdwn_text_element(text: &str) -> BlockElement {
        BlockElement::Mrkdwn {
            text: Text {
                text: text.to_string(),
                emoji: None,
                verbatim: None,
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    image_url: String,
    alt_text: String,
}

impl Into<BlockElement> for Image {
    fn into(self) -> BlockElement {
        BlockElement::Image {
            image: self
        }
    }
}

impl Image {
    pub fn set_image_url(mut self, image_url: String) -> Image {
        self.image_url = image_url;
        self
    }

    pub fn set_alt_text(mut self, alt_text: String) -> Image {
        self.alt_text = alt_text;
        self
    }

    pub fn build(self) -> BlockElement {
        BlockElement::Image {
            image: self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Button {
    text: TextObject,
    action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialogObject>,
}

impl Into<BlockElement> for Button {
    fn into(self) -> BlockElement {
        BlockElement::Button {
            button: self
        }
    }
}

impl Button {
    pub fn set_text(mut self, text: TextObject) -> Button {
        self.text = text;
        self
    }

    pub fn set_action_id(mut self, action_id: String) -> Button {
        self.action_id = action_id;
        self
    }

    pub fn set_url(mut self, url: String) -> Button {
        self.url = Some(url);
        self
    }

    pub fn set_value(mut self, action_id: String) -> Button {
        self.value = Some(action_id);
        self
    }

    pub fn set_style(mut self, style: String) -> Button {
        self.style = Some(style);
        self
    }

    pub fn set_confirm(mut self, confirm: ConfirmationDialogObject) -> Button {
        self.confirm = Some(confirm);
        self
    }

    pub fn build(self) -> BlockElement {
        BlockElement::Button {
            button: self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaticSelect {
    placeholder: TextObject,
    action_id: String,
    options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    option_groups: Option<Vec<OptionObjectGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_option: Option<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialogObject>,
}

impl Into<BlockElement> for StaticSelect {
    fn into(self) -> BlockElement {
        BlockElement::StaticSelect {
            static_select: self
        }
    }
}

impl StaticSelect {
    pub fn set_placeholder(mut self, placeholder: TextObject) -> StaticSelect {
        self.placeholder = placeholder;
        self
    }

    pub fn set_action_id(mut self, action_id: String) -> StaticSelect {
        self.action_id = action_id;
        self
    }

    pub fn set_options<T: Into<OptionObject>>(mut self, options: Vec<T>) -> StaticSelect {
        self.options = options.into_iter().map(|e| e.into()).collect();
        self
    }

    pub fn add_option(mut self, option: OptionObject) -> StaticSelect {
        self.options.push(option);
        self
    }

    pub fn set_option_groups(mut self, option_groups: Vec<OptionObjectGroup>) -> StaticSelect {
        self.option_groups = Some(option_groups);
        self
    }

    pub fn add_option_group(mut self, option_group: OptionObjectGroup) -> StaticSelect {
        if let Some(ref mut vec) = self.option_groups {
            vec.push(option_group)
        } else {
            self.option_groups = Some(vec![option_group])
        }
        self
    }

    pub fn set_initial_option(mut self, initial_option: OptionObject) -> StaticSelect {
        self.initial_option = Some(initial_option);
        self
    }

    pub fn set_confirm(mut self, confirm: ConfirmationDialogObject) -> StaticSelect {
        self.confirm = Some(confirm);
        self
    }

    pub fn build(self) -> BlockElement {
        BlockElement::StaticSelect {
            static_select: self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalStatic {
    placeholder: TextObject,
    action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_option: Option<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    //todo check is correct?
    min_query_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialogObject>,
}

impl Into<BlockElement> for ExternalStatic {
    fn into(self) -> BlockElement {
        BlockElement::ExternalSelect {
            external_static: self
        }
    }
}


impl ExternalStatic {
    pub fn set_placeholder(mut self, placeholder: TextObject) -> ExternalStatic {
        self.placeholder = placeholder;
        self
    }

    pub fn set_action_id(mut self, action_id: String) -> ExternalStatic {
        self.action_id = action_id;
        self
    }

    pub fn set_min_query_length(mut self, min_query_length: u32) -> ExternalStatic {
        self.min_query_length = Some(min_query_length);
        self
    }

    pub fn set_initial_option(mut self, initial_option: OptionObject) -> ExternalStatic {
        self.initial_option = Some(initial_option);
        self
    }

    pub fn set_confirm(mut self, confirm: ConfirmationDialogObject) -> ExternalStatic {
        self.confirm = Some(confirm);
        self
    }

    pub fn build(self) -> BlockElement {
        BlockElement::ExternalSelect {
            external_static: self
        }
    }
}

impl Into<BlockElement> for Text {
    fn into(self) -> BlockElement {
        BlockElement::PlainText {
            text: self
        }
    }
}
