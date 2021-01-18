use serde::{Serialize, Deserialize};
use std::fmt::{Formatter, Display};

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