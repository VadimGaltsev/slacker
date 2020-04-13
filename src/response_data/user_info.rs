use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct UserInfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub user: Option<UserInfo>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct UserInfo {
    pub id: String,
    team_id: String,
    deleted: bool,
    real_name: String,
    tz: String,
    tz_label: String,
    pub profile: Profile,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Profile {
    title: String,
    phone: String,
    read_name: String,
    display_name: String,
    pub image_24: String,
}
