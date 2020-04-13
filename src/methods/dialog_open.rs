use serde::{Serialize, Deserialize};
use crate::methods::SlackRequest;
use serde_json::Value;
use crate::request_data::Dialog;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct DialogOpen {
    trigger_id: String,
    dialog: Dialog,
}

impl DialogOpen {
    pub fn new(trigger_id: &str, dialog: Dialog) -> DialogOpen {
        DialogOpen {
            trigger_id: trigger_id.to_string(),
            dialog,
        }
    }
}

impl SlackRequest<Value> for DialogOpen {
    const METHOD_NAME: &'static str = "dialog.open";
}
