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
