use crate::{SlackRequest, LayoutBlock, TextObject, View, PostMessageResponse};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct ViewOpen {
    trigger_id: String,
    view: View,
}

impl ViewOpen {
    pub fn new<T: Into<LayoutBlock>, Text: Into<TextObject>>(
        trigger_id: String,
        title: Text,
        blocks: Vec<T>,
    ) -> ViewOpen {
        ViewOpen {
            trigger_id,
            view: View::new(title, blocks),
        }
    }

    pub fn new_with_id<T: Into<LayoutBlock>, Text: Into<TextObject>>(
        trigger_id: String,
        view_id: &str,
        title: Text,
        blocks: Vec<T>,
    ) -> ViewOpen {
        ViewOpen {
            trigger_id,
            view: View::new_with_id(view_id, title, blocks),
        }
    }

    pub fn add_submit<T: Into<TextObject>>(mut self, text: T) -> ViewOpen {
        self.view = self.view.add_submit(text);
        self
    }
}

impl SlackRequest<PostMessageResponse> for ViewOpen {
    const METHOD_NAME: &'static str = "views.open";
}
