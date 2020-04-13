use serde::{Serialize, Deserialize};
use crate::request_data::Message;
use serde_json::Value;
use crate::View;

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default)]
pub struct PostMessageResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: String,
    pub ts: String,
    pub view: Option<View>,
    pub message: Option<Message>,
    pub response_metadata: Option<Value>,
}

#[cfg(test)]
mod test {
    use crate::PostMessageResponse;

    #[test]
    fn should_deserialize_response() {
        let expected = r#"{
    "ok": true,
    "channel": "CMZHE342E",
    "ts": "1568669623.000400",
    "message": {
        "type": "message",
        "subtype": "bot_message",
        "text": "Hello dear",
        "ts": "1568669623.000400",
        "username": "TestLib",
        "bot_id": "BNCTQ9L2C"
    }
}"#;
        let json = serde_json::from_str::<PostMessageResponse>(expected);
        println!("{:?}", json);
    }
}