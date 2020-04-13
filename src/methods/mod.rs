mod post_message;
mod dialog_open;
mod update_message;
mod get_user_info;
mod view_open;
mod view_update;
mod view_push;
use serde::Serialize;

pub use {
    post_message::*,
    dialog_open::*,
    update_message::*,
    get_user_info::*,
    view_open::*,
    view_update::*,
    view_push::*,
};

pub trait SlackRequest<Response>: Serialize {
    const METHOD_NAME: &'static str;

    fn get_params(&self) -> String {
        String::new()
    }
}