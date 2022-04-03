use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::sync::Arc;

use mlua::prelude::LuaError;

use crate::jira::models;

pub type JiraResult<T = ()> = Result<T, JiraError>;

#[derive(Debug)]
pub enum JiraError {
    MalformedToken,
    Unauthorized,
    PermissionDenied,
    BadRequest(Option<models::ErrorResponse>),
    UnexpectedStatus(reqwest::StatusCode, Option<models::ErrorResponse>),
    HttpClient(reqwest::Error),
}

impl Error for JiraError {}

impl Display for JiraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use JiraError::*;
        match self {
            MalformedToken => write!(f, "Malformed token"),
            Unauthorized => write!(f, "Unauthorized"),
            PermissionDenied => write!(f, "Permission denied"),
            BadRequest(ref err) => {
                write!(
                    f,
                    "Bad request: {:?}",
                    err.as_ref().map(models::ErrorResponse::any)
                )
            }
            UnexpectedStatus(status, ref err) => {
                write!(
                    f,
                    "Unexpected response status {}: {:?}",
                    status.as_str(),
                    err.as_ref().map(models::ErrorResponse::any),
                )
            }
            HttpClient(ref err) => <reqwest::Error as Display>::fmt(err, f),
        }
    }
}

impl From<reqwest::Error> for JiraError {
    fn from(err: reqwest::Error) -> Self {
        Self::HttpClient(err)
    }
}

impl Into<LuaError> for JiraError {
    fn into(self) -> LuaError {
        LuaError::ExternalError(Arc::new(self))
    }
}
