use crate::{LayoutBlock, TextObject};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct View {
    r#type: ViewType,
    pub title: TextObject,
    pub blocks: Vec<LayoutBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ViewState>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct ViewState {
    pub values: Value
}

impl View {
    pub fn new<T: Into<LayoutBlock>, Text: Into<TextObject>>(title: Text, blocks: Vec<T>) -> View {
        View {
            title: title.into(),
            blocks: blocks.into_iter().map(|b| b.into()).collect(),
            ..Default::default()
        }
    }

    pub fn new_with_id<T: Into<LayoutBlock>, Text: Into<TextObject>>(
        view_id: &str,
        title: Text,
        blocks: Vec<T>
    ) -> View {
        View {
            title: title.into(),
            blocks: blocks.into_iter().map(|b| b.into()).collect(),
            callback_id: Some(view_id.to_owned()),
            ..Default::default()
        }
    }

    pub fn add_submit<T: Into<TextObject>>(mut self, text: T) -> View {
        self.submit = Some(text.into());
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ViewType {
    Modal,
    Home,
}

impl Default for ViewType {
    fn default() -> Self {
        ViewType::Modal
    }
}
