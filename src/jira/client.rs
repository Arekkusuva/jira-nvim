use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;

use crate::jira::error::{JiraError, JiraResult};
use crate::jira::models;

const PAGE_SIZE: usize = 50;

#[derive(Clone)]
struct BasicCredentinals {
    token: String,
    split_at: usize,
}

impl BasicCredentinals {
    fn new(token: &str) -> JiraResult<Self> {
        let split_at = token.find(":").ok_or(JiraError::MalformedToken)?;

        Ok(Self {
            token: token.into(),
            split_at,
        })
    }

    #[inline]
    fn user(&self) -> &str {
        &self.token[..self.split_at]
    }

    #[inline]
    fn password(&self) -> &str {
        &self.token[self.split_at + 1..]
    }
}

#[derive(Clone)]
pub struct JiraClient {
    host: String,
    credentinals: BasicCredentinals,
}

/// A client for [Jira REST API](https://docs.atlassian.com/jira/REST/latest).
impl JiraClient {
    pub fn new(host: &str, token: &str) -> JiraResult<Self> {
        Ok(Self {
            host: host.into(),
            credentinals: BasicCredentinals::new(token)?,
        })
    }
}

impl JiraClient {
    pub fn query(&self, mut jql: &str) -> JiraResult<Vec<models::Issue>> {
        if jql.len() > 1 && jql.starts_with(|c| c == '\'' || c == '"') {
            // trim quotes
            jql = &jql[1..jql.len() - 1]
        }
        let query = models::RequestQuery {
            jql,
            max_results: PAGE_SIZE,
        };

        let res = reqwest::blocking::Client::new()
            .get(&format!("{}/rest/api/latest/search", &self.host))
            .basic_auth(self.credentinals.user(), Some(self.credentinals.password()))
            .header(CONTENT_TYPE, "application/json")
            .query(&query)
            .send()?;

        match res.status() {
            StatusCode::OK => (),
            StatusCode::UNAUTHORIZED => return Err(JiraError::Unauthorized),
            StatusCode::FORBIDDEN => return Err(JiraError::PermissionDenied),
            status if status.is_client_error() => {
                return Err(JiraError::BadRequest(
                    res.json::<models::ErrorResponse>()
                        .map_or(None, |err| Some(err)),
                ))
            }
            status => {
                return Err(JiraError::UnexpectedStatus(
                    status,
                    res.json::<models::ErrorResponse>()
                        .map_or(None, |err| Some(err)),
                ))
            }
        };

        let body = res.json::<models::SearchResult>()?;
        return Ok(body.issues);
    }
}
