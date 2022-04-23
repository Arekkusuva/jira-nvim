use reqwest::blocking::Response;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;

use crate::error::{Error, Result};
use crate::jira::transport;

const PAGE_SIZE: usize = 50;

trait ResponseExt: Sized {
    fn if_status(self, expected: StatusCode) -> Result<Self>;

    #[inline]
    fn assert_status(self, expected: StatusCode) -> Result<()> {
        Self::if_status(self, expected)?;
        Ok(())
    }
}

impl ResponseExt for Response {
    fn if_status(self, expected: StatusCode) -> Result<Response> {
        match self.status() {
            status if status == expected => Ok(self),
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::FORBIDDEN => Err(Error::PermissionDenied),
            status if status.is_client_error() => Err(Error::BadRequest(
                self.json::<transport::ErrorResponse>()
                    .map_or(None, |err| Some(err)),
            )),
            status => Err(Error::UnexpectedStatus(
                status,
                self.json::<transport::ErrorResponse>()
                    .map_or(None, |err| Some(err)),
            )),
        }
    }
}

#[derive(Clone)]
struct BasicCredentinals {
    token: String,
    split_at: usize,
}

impl BasicCredentinals {
    fn new(token: &str) -> Result<Self> {
        let split_at = token.find(":").ok_or(Error::MalformedToken)?;

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

/// A client for [Jira REST API](https://docs.atlassian.com/jira/REST/latest).
#[derive(Clone)]
pub struct JiraClient {
    host: String,
    credentinals: BasicCredentinals,
}

impl JiraClient {
    pub fn new(host: &str, token: &str) -> Result<Self> {
        Ok(Self {
            host: host.into(),
            credentinals: BasicCredentinals::new(token)?,
        })
    }
}

// TODO: Use specific version
impl JiraClient {
    pub fn query(&self, mut jql: &str) -> Result<Vec<transport::Issue>> {
        if jql.len() > 1 && jql.starts_with(|c| c == '\'' || c == '"') {
            // trim quotes
            jql = &jql[1..jql.len() - 1]
        }

        Ok(reqwest::blocking::Client::new()
            .get(&format!("{}/rest/api/latest/search", &self.host))
            .basic_auth(self.credentinals.user(), Some(self.credentinals.password()))
            .query(&transport::RequestQuery {
                jql,
                max_results: PAGE_SIZE,
                fields: Some(
                    "created,summary,issuetype,priority,labels,status,assignee,description",
                ),
            })
            .send()?
            .if_status(StatusCode::OK)?
            .json::<transport::SearchResult>()?
            .issues)
    }

    pub fn issue_transitions(&self, issue_key: &str) -> Result<Vec<transport::IssueTransition>> {
        Ok(reqwest::blocking::Client::new()
            .get(&format!(
                "{}/rest/api/latest/issue/{}/transitions",
                &self.host, issue_key,
            ))
            .basic_auth(self.credentinals.user(), Some(self.credentinals.password()))
            .send()?
            .if_status(StatusCode::OK)?
            .json::<transport::IssueTransitions>()?
            .transitions)
    }

    pub fn perform_issue_transition(&self, issue_key: &str, transition_id: &str) -> Result<()> {
        reqwest::blocking::Client::new()
            .post(&format!(
                "{}/rest/api/latest/issue/{}/transitions",
                &self.host, issue_key,
            ))
            .basic_auth(self.credentinals.user(), Some(self.credentinals.password()))
            .header(CONTENT_TYPE, "application/json")
            .json(&transport::NewTransition {
                target: transport::TargetTransition { id: transition_id },
            })
            .send()?
            .assert_status(StatusCode::NO_CONTENT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn test_perform_issue_transition() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/rest/api/latest/issue/test_key/transitions")
                .header("authorization", "Basic dGVzdDp0b2tlbg==")
                .header("Content-Type", "application/json")
                .body(r#"{"transition":{"id":"test_transition_id"}}"#);
            then.status(204);
        });

        let client = JiraClient::new(&server.base_url(), "test:token").unwrap();
        let res = client.perform_issue_transition("test_key", "test_transition_id");
        mock.assert();
        res.expect("Failed to perform transition");
    }
}
