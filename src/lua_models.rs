use mlua::prelude::{LuaUserData, LuaUserDataFields};

use crate::formatters;
use crate::jira::transport::{Issue, IssueTransition};

impl LuaUserData for Issue {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("issue_id", |_, this| Ok(this.id.clone()));
        fields.add_field_method_get("issue_key", |_, this| Ok(this.key.clone()));
        fields.add_field_method_get("created_at", |_, this| {
            Ok(this.fields.created_at.to_rfc3339())
        });
        fields.add_field_method_get("created_ago", |_, this| {
            Ok(formatters::diff_ago(this.fields.created_at))
        });
        fields.add_field_method_get("summary", |_, this| Ok(this.fields.summary.clone()));
        fields.add_field_method_get("issue_type", |_, this| {
            Ok(this.fields.issue_type.name.clone())
        });
        fields.add_field_method_get("status", |_, this| {
            Ok(this.fields.status.category.name.clone())
        });
        fields.add_field_method_get("is_subtask", |_, this| {
            Ok(this.fields.issue_type.is_subtask)
        });
        fields.add_field_method_get("priority", |_, this| Ok(this.fields.priority.name.clone()));
        fields.add_field_method_get("labels", |_, this| {
            Ok(formatters::list(&this.fields.labels))
        });
        fields.add_field_method_get("assignee_name", |_, this| {
            Ok(match this.fields.assignee.as_ref() {
                Some(a) => a.name.clone(),
                None => "None".into(),
            })
        });
        // fields.add_field_method_get("description", |_, this| {
        //     Ok(this
        //         .fields
        //         .description
        //         .as_ref()
        //         .map(parse_description_tokens))
        // });
    }
}

// pub enum LuaDescriptionToken {
//     Text(String),
// }

// fn parse_description_tokens(description: &String) -> Vec<LuaDescriptionToken> {
//     let mut tokens = Vec::new();
//     for line in description.lines() {
//         tokens.push(LuaDescriptionToken::Text(line.into()));
//     }
//     tokens
// }

// impl LuaUserData for LuaDescriptionToken {
//     fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
//         fields.add_field_method_get("is_text", |_, _| Ok(true));
//         fields.add_field_method_get("text", |_, this| {
//             Ok(match this {
//                 LuaDescriptionToken::Text(text) => text.clone(),
//             })
//         });
//     }
// }

impl LuaUserData for IssueTransition {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("transition_id", |_, this| Ok(this.id.clone()));
        fields.add_field_method_get("name", |_, this| Ok(this.to.name.clone()));
    }
}
