use crate::methods::SlackRequest;
use serde::Serialize;
use crate::UserInfoResponse;

#[derive(Serialize)]
pub struct GetUserInfo(pub String);

impl SlackRequest<UserInfoResponse> for GetUserInfo {
    const METHOD_NAME: &'static str = "users.info";

    fn get_params(&self) -> String {
        format!("?user={}", self.0)
    }
}


/*
"user": {
        "id": "UNCAY1XJR",
        "team_id": "TNBSH08UC",
        "name": "samokatanet",
        "deleted": false,
        "color": "9f69e7",
        "real_name": "vaiklol",
        "tz": "Europe\/Moscow",
        "tz_label": "Moscow Time",
        "tz_offset": 10800,
        "profile": {
            "title": "",
            "phone": "",
            "skype": "",
            "real_name": "vaiklol",
            "real_name_normalized": "vaiklol",
            "display_name": "",
            "display_name_normalized": "",
            "status_text": "",
            "status_emoji": "",
            "status_expiration": 0,
            "avatar_hash": "ga4e74d0ebd8",
            "first_name": "vaiklol",
            "last_name": "",
            "image_24": "https:\/\/secure.gravatar.com\/avatar\/3a4e74d0ebd85e98df6a5cce1a1b0ff1.jpg?s=24&d=https%3A%2F%2Fa.slack-edge.com%2Fdf10d%2Fimg%2Favatars%2Fava_0000-24.png",
            "image_32": "https:\/\/secure.gravatar.com\/avatar\/3a4e74d0ebd85e98df6a5cce1a1b0ff1.jpg?s=32&d=https%3A%2F%2Fa.slack-edge.com%2Fdf10d%2Fimg%2Favatars%2Fava_0000-32.png",
            "image_48": "https:\/\/secure.gravatar.com\/avatar\/3a4e74d0ebd85e98df6a5cce1a1b0ff1.jpg?s=48&d=https%3A%2F%2Fa.slack-edge.com%2Fdf10d%2Fimg%2Favatars%2Fava_0000-48.png",
            "image_72": "https:\/\/secure.gravatar.com\/avatar\/3a4e74d0ebd85e98df6a5cce1a1b0ff1.jpg?s=72&d=https%3A%2F%2Fa.slack-edge.com%2Fdf10d%2Fimg%2Favatars%2Fava_0000-72.png",
            "image_192": "https:\/\/secure.gravatar.com\/avatar\/3a4e74d0ebd85e98df6a5cce1a1b0ff1.jpg?s=192&d=https%3A%2F%2Fa.slack-edge.com%2Fdf10d%2Fimg%2Favatars%2Fava_0000-192.png",
            "image_512": "https:\/\/secure.gravatar.com\/avatar\/3a4e74d0ebd85e98df6a5cce1a1b0ff1.jpg?s=512&d=https%3A%2F%2Fa.slack-edge.com%2Fdf10d%2Fimg%2Favatars%2Fava_0000-512.png",
            "status_text_canonical": "",
            "team": "TNBSH08UC"
        },
        "is_admin": true,
        "is_owner": true,
        "is_primary_owner": true,
        "is_restricted": false,
        "is_ultra_restricted": false,
        "is_bot": false,
        "is_app_user": false,
        "updated": 1569066664,
        "has_2fa": false
    }
*/