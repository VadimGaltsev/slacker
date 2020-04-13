use serde::{Serialize, Deserialize};
use serde::export::fmt::Display;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Error {
    ok: bool,
    error: String
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("Error cause: {}", self.error))
    }
}