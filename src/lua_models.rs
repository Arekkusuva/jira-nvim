use std::mem;

use mlua::prelude::{LuaUserData, LuaUserDataFields, LuaUserDataMethods};

use crate::formatters;
use crate::jira::transport::{
    CodeBlockAttrs, EmojiAttrs, HeadingAtrs, InlineCardAttrs, Issue, IssueDescriptionNode,
    IssueTransition, MarkNode, MediaAttrs, MediaSingleAttrs, MentionAttrs, OrderedListAttrs,
    PanelAttrs, TableAttrs, TableCellAttrs,
};

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
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("take_description", |_, this, _: ()| {
            Ok(match this.fields.description.as_mut() {
                Some(desc) => {
                    let mut tokens = Vec::with_capacity(num_of_tokens(desc));
                    nodes_to_tokens(&mut tokens, desc);
                    Some(tokens)
                }
                None => None,
            })
        })
    }
}

fn num_of_tokens(node: &IssueDescriptionNode) -> usize {
    use IssueDescriptionNode::*;
    match node {
        Doc { content, .. }
        | Paragraph { content, .. }
        | CodeBlock { content, .. }
        | Blockquote { content }
        | BulletList { content }
        | Heading { content, .. }
        | ListItem { content }
        | MediaGroup { content }
        | MediaSingle { content, .. }
        | OrderedList { content, .. }
        | Panel { content, .. }
        | Table { content, .. }
        | TableCell { content, .. }
        | TableHeader { content, .. }
        | TableRow { content } => {
            let mut num = 2;
            for content_node in content {
                num += num_of_tokens(content_node);
            }
            num
        }
        _ => 1,
    }
}

fn nodes_to_tokens(target: &mut Vec<DescriptionToken>, node: &mut IssueDescriptionNode) {
    use IssueDescriptionNode::*;
    match node {
        Doc { content, .. } => {
            target.push(DescriptionToken::DocBegin);
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::DocEnd);
        }
        Paragraph { content } => {
            target.push(DescriptionToken::ParagraphBegin);
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::ParagraphEnd);
        }
        Text { text, marks } => {
            target.push(DescriptionToken::Text {
                text: text.clone(),
                marks: mem::replace(marks, Vec::new()),
            });
        }
        CodeBlock { attrs, content } => {
            target.push(DescriptionToken::CodeBlockBegin {
                attrs: attrs.take(),
            });
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::CodeBlockEnd);
        }
        Blockquote { content } => {
            target.push(DescriptionToken::BlockquoteBegin);
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::BlockquoteEnd);
        }
        BulletList { content } => {
            target.push(DescriptionToken::BulletListBegin);
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::BulletListEnd);
        }
        Emoji { attrs } => {
            target.push(DescriptionToken::Emoji {
                attrs: attrs.clone(),
            });
        }
        HardBreak => {
            target.push(DescriptionToken::HardBreak);
        }
        Heading { attrs, content } => {
            target.push(DescriptionToken::HeadingBegin {
                attrs: attrs.clone(),
            });
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::HeadingEnd);
        }
        InlineCard { attrs } => {
            target.push(DescriptionToken::InlineCard {
                attrs: attrs.clone(),
            });
        }
        ListItem { content } => {
            target.push(DescriptionToken::ListItemBegin);
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::ListItemEnd);
        }
        MediaGroup { content } => {
            target.push(DescriptionToken::MediaGroupBegin);
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::MediaGroupEnd);
        }
        MediaSingle { attrs, content } => {
            target.push(DescriptionToken::MediaSingleBegin {
                attrs: attrs.clone(),
            });
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::MediaSingleEnd);
        }
        Media { attrs } => {
            target.push(DescriptionToken::Media {
                attrs: attrs.clone(),
            });
        }
        Mention { attrs } => {
            target.push(DescriptionToken::Mention {
                attrs: attrs.clone(),
            });
        }
        OrderedList { attrs, content } => {
            target.push(DescriptionToken::OrderedListBegin {
                attrs: attrs.clone(),
            });
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::OrderedListEnd);
        }
        Panel { attrs, content } => {
            target.push(DescriptionToken::PanelBegin {
                attrs: attrs.clone(),
            });
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::PanelEnd);
        }
        Rule => {
            target.push(DescriptionToken::Rule);
        }
        Table { attrs, content } => {
            target.push(DescriptionToken::TableBegin {
                attrs: attrs.clone(),
            });
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::TableEnd);
        }
        TableCell { attrs, content } => {
            target.push(DescriptionToken::TableCellBegin {
                attrs: attrs.clone(),
            });
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::TableCellEnd);
        }
        TableHeader { attrs, content } => {
            target.push(DescriptionToken::TableHeaderBegin {
                attrs: attrs.clone(),
            });
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::TableHeaderEnd);
        }
        TableRow { content } => {
            target.push(DescriptionToken::TableRowBegin);
            for content_node in content {
                nodes_to_tokens(target, content_node);
            }
            target.push(DescriptionToken::TableRowEnd);
        }
        BlockCard => {
            target.push(DescriptionToken::BlockCard);
        }
    }
}

pub enum DescriptionToken {
    DocBegin,
    DocEnd,
    ParagraphBegin,
    ParagraphEnd,
    Text {
        text: String,
        marks: Vec<MarkNode>,
    },
    CodeBlockBegin {
        attrs: Option<CodeBlockAttrs>,
    },
    CodeBlockEnd,
    BlockquoteBegin,
    BlockquoteEnd,
    BulletListBegin,
    BulletListEnd,
    Emoji {
        attrs: EmojiAttrs,
    },
    HardBreak,
    HeadingBegin {
        attrs: HeadingAtrs,
    },
    HeadingEnd,
    InlineCard {
        attrs: InlineCardAttrs,
    },
    ListItemBegin,
    ListItemEnd,
    MediaGroupBegin,
    MediaGroupEnd,
    MediaSingleBegin {
        attrs: MediaSingleAttrs,
    },
    MediaSingleEnd,
    Media {
        attrs: MediaAttrs,
    },
    Mention {
        attrs: MentionAttrs,
    },
    OrderedListBegin {
        attrs: Option<OrderedListAttrs>,
    },
    OrderedListEnd,
    PanelBegin {
        attrs: PanelAttrs,
    },
    PanelEnd,
    Rule,
    TableBegin {
        attrs: Option<TableAttrs>,
    },
    TableEnd,
    TableCellBegin {
        attrs: Option<TableCellAttrs>,
    },
    TableCellEnd,
    TableHeaderBegin {
        attrs: Option<TableCellAttrs>, // the same as table cell
    },
    TableHeaderEnd,
    TableRowBegin,
    TableRowEnd,
    BlockCard, // unsupported
}

impl LuaUserData for DescriptionToken {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("type_id", |_, this| {
            use DescriptionToken::*;
            Ok(match this {
                DocBegin => 1,
                DocEnd => 2,
                ParagraphBegin => 3,
                ParagraphEnd => 4,
                Text { .. } => 5,
                CodeBlockBegin { .. } => 6,
                CodeBlockEnd => 7,
                BlockquoteBegin => 8,
                BlockquoteEnd => 9,
                BulletListBegin => 10,
                BulletListEnd => 11,
                Emoji { .. } => 12,
                HardBreak => 13,
                HeadingBegin { .. } => 14,
                HeadingEnd => 15,
                InlineCard { .. } => 16,
                ListItemBegin => 17,
                ListItemEnd => 18,
                MediaGroupBegin => 19,
                MediaGroupEnd => 20,
                MediaSingleBegin { .. } => 21,
                MediaSingleEnd => 22,
                Media { .. } => 23,
                Mention { .. } => 24,
                OrderedListBegin { .. } => 25,
                OrderedListEnd => 26,
                PanelBegin { .. } => 27,
                PanelEnd => 28,
                Rule => 29,
                TableBegin { .. } => 30,
                TableEnd => 31,
                TableCellBegin { .. } => 32,
                TableCellEnd => 33,
                TableHeaderBegin { .. } => 34,
                TableHeaderEnd => 35,
                TableRowBegin => 36,
                TableRowEnd => 37,
                BlockCard => 38,
            })
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("text", |_, this, _: ()| {
            Ok(match this {
                DescriptionToken::Text { text, .. } => Some(text.clone()),
                _ => None,
            })
        })
    }
}

impl LuaUserData for IssueTransition {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("transition_id", |_, this| Ok(this.id.clone()));
        fields.add_field_method_get("name", |_, this| Ok(this.to.name.clone()));
    }
}
