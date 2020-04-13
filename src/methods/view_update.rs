use crate::{SlackRequest, LayoutBlock, TextObject, View, PostMessageResponse};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct ViewUpdate {
    view: View,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
}

impl ViewUpdate {
    pub fn new(
        view: View,
    ) -> ViewUpdate {
        ViewUpdate {
            view: View::new_with_id(
                view.callback_id.unwrap_or(Default::default()).as_str(),
                view.title,
                view.blocks,
            ),
            external_id: None,
            hash: None,
            view_id: None,
        }
    }

    pub fn add_submit<T: Into<TextObject>>(mut self, text: T) -> ViewUpdate {
        self.view = self.view.add_submit(text);
        self
    }
}

impl SlackRequest<PostMessageResponse> for ViewUpdate {
    const METHOD_NAME: &'static str = "views.update";
}
