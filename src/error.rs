use std::fmt;
use std::sync::Arc;

use mlua::prelude::LuaError;

use crate::jira::transport;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SetupFailed,
    MalformedToken,
    Unauthorized,
    PermissionDenied,
    BadRequest(Option<transport::ErrorResponse>),
    UnexpectedStatus(reqwest::StatusCode, Option<transport::ErrorResponse>),
    HttpClient(reqwest::Error),
    Lua(LuaError),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            SetupFailed => write!(f, "Lib wasn't setup or error occured during setup"),
            MalformedToken => write!(f, "Malformed token"),
            Unauthorized => write!(f, "Unauthorized"),
            PermissionDenied => write!(f, "Permission denied"),
            BadRequest(ref err) => match err.as_ref().map(transport::ErrorResponse::any) {
                Some(any) => write!(f, "Bad request with erorr: ({})", any),
                None => write!(f, "Bad request"),
            },
            UnexpectedStatus(status, ref err) => {
                match err.as_ref().map(transport::ErrorResponse::any) {
                    Some(any) => write!(
                        f,
                        "Unexpected response status {} with error: ({})",
                        status.as_str(),
                        any,
                    ),
                    None => write!(f, "Unexpected response status {}", status.as_str(),),
                }
            }
            HttpClient(ref err) => <reqwest::Error as fmt::Display>::fmt(err, f),
            Lua(ref err) => <LuaError as fmt::Display>::fmt(err, f),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::HttpClient(err)
    }
}

impl From<LuaError> for Error {
    fn from(err: LuaError) -> Self {
        Self::Lua(err)
    }
}

impl Into<LuaError> for Error {
    fn into(self) -> LuaError {
        use Error::*;
        match self {
            Lua(err) => err,
            err => LuaError::ExternalError(Arc::new(err)),
        }
    }
}
