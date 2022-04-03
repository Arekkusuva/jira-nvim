use std::collections::HashMap;

use mlua::prelude::{LuaUserData, LuaUserDataFields};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct RequestQuery<'a> {
    pub jql: &'a str,
    #[serde(rename = "maxResults")]
    pub max_results: usize,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "errorMessages")]
    pub error_messages: Vec<String>,
    pub errors: HashMap<String, String>,
}

impl ErrorResponse {
    pub fn any(&self) -> Option<&str> {
        if !self.error_messages.is_empty() {
            Some(self.error_messages[0].as_str())
        } else if self.errors.is_empty() {
            self.errors.iter().next().map(|(val, _)| val.as_str())
        } else {
            None
        }
    }
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
    summary: String,
    #[serde(rename = "issuetype")]
    issue_type: IssueType,
    priority: IssuePriority,
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

impl LuaUserData for Issue {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("issue_id", |_, this| Ok(this.id.clone()));
        fields.add_field_method_get("summary", |_, this| Ok(this.fields.summary.clone()));
        fields.add_field_method_get("issue_type", |_, this| {
            Ok(this.fields.issue_type.name.clone())
        });
        fields.add_field_method_get("is_subtask", |_, this| {
            Ok(this.fields.issue_type.is_subtask)
        });
        fields.add_field_method_get("priority", |_, this| Ok(this.fields.priority.name.clone()));
    }
}

// TODO: Consider using `ToLua` instead of `LuaUserData`
// impl<'lua> ToLua<'lua> for Issue {
//     fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
//         let table = lua.create_table()?;
//         table.set("issue_id", self.id.clone())?;
//         table.set("summary", self.fields.summary.clone())?;
//         Ok(LuaValue::Table(table))
//     }
// }
