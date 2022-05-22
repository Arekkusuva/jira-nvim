use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

//----------------------------------------
// Error
//----------------------------------------

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "errorMessages")]
    pub error_messages: Option<Vec<String>>,
    #[serde(default)]
    pub status: usize,
}

impl ErrorResponse {
    pub fn any(&self) -> String {
        if let Some(msgs) = &self.error_messages {
            format!("error_code: {}, erorr: {}", self.status, msgs[0].clone())
        } else {
            format!("error_code: {}", self.status)
        }
    }
}

//----------------------------------------
// Transitions
//----------------------------------------

#[derive(Serialize, Debug)]
pub struct NewTransition<'a> {
    #[serde(rename = "transition")]
    pub target: TargetTransition<'a>,
}

#[derive(Serialize, Debug)]
pub struct TargetTransition<'a> {
    pub id: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct IssueTransitions {
    pub transitions: Vec<IssueTransition>,
}

#[derive(Deserialize, Debug)]
pub struct IssueTransition {
    pub id: String,
    pub to: IssueTransitionTo,
}

#[derive(Deserialize, Debug)]
pub struct IssueTransitionTo {
    pub name: String,
}

//----------------------------------------
// Query
//----------------------------------------

#[derive(Serialize, Debug)]
pub struct RequestQuery<'a> {
    pub jql: &'a str,
    #[serde(rename = "maxResults")]
    pub max_results: usize,
    pub fields: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    pub total: u64,
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    #[serde(rename = "startAt")]
    pub start_at: u64,
    pub expand: Option<String>,
    pub issues: Vec<Issue>,
}

#[derive(Deserialize, Debug)]
pub struct Issue {
    #[serde(rename = "self")]
    pub link: String,
    pub key: String,
    pub id: String,
    pub fields: Fields,
}

#[derive(Deserialize, Debug)]
pub struct Fields {
    #[serde(rename = "created")]
    pub created_at: DateTime<Utc>,
    pub summary: String,
    #[serde(rename = "issuetype")]
    pub issue_type: IssueType,
    pub priority: IssuePriority,
    pub labels: Vec<String>,
    pub status: IssueStatus,
    pub assignee: Option<IssueAssignee>,
}

#[derive(Deserialize, Debug)]
pub struct IssueType {
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub name: String,
    #[serde(rename = "subtask")]
    pub is_subtask: bool,
}

#[derive(Deserialize, Debug)]
pub struct IssuePriority {
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct IssueStatus {
    #[serde(rename = "statusCategory")]
    pub category: IssueCategory,
}

#[derive(Deserialize, Debug)]
pub struct IssueCategory {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct IssueAssignee {
    #[serde(rename = "displayName")]
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IssueContentType {
    None,
    Doc,
    Paragraph,
    Text,
}

impl Default for IssueContentType {
    fn default() -> Self {
        Self::None
    }
}
