use std::fmt;

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct Error(pub(crate) Box<str>);

pub(crate) fn with_context(msg: std::fmt::Arguments<'_>) -> Error {
    Error(format!("{msg}").into())
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
