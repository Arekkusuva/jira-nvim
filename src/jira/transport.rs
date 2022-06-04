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

#[derive(Serialize, Debug, Default)]
pub struct RequestQuery<'a> {
    pub jql: Option<&'a str>,
    #[serde(rename = "maxResults")]
    pub max_results: Option<usize>,
    pub fields: Option<&'a str>,
}

impl<'a> RequestQuery<'a> {
    #[inline]
    pub fn jql(&mut self, jql: &'a str) -> &mut Self {
        self.jql = Some(jql);
        self
    }

    #[inline]
    pub fn max_results(&mut self, max_results: usize) -> &mut Self {
        self.max_results = Some(max_results);
        self
    }

    #[inline]
    pub fn fields(&mut self, fields: &'a str) -> &mut Self {
        self.fields = Some(fields);
        self
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
    #[serde(rename = "created")]
    pub created_at: DateTime<Utc>,
    pub summary: String,
    #[serde(rename = "issuetype")]
    pub issue_type: IssueType,
    pub priority: IssuePriority,
    pub labels: Vec<String>,
    pub status: IssueStatus,
    pub assignee: Option<IssueAssignee>,
    pub description: Option<IssueDescriptionNode>,
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

//----------------------------------------
// Issue description
//----------------------------------------

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum IssueDescriptionNode {
    // Root note
    Doc {
        version: usize,
        #[serde(default)]
        content: Vec<IssueDescriptionNode>,
    },
    Paragraph {
        #[serde(default)]
        content: Vec<IssueDescriptionNode>,
    },
    Text {
        text: String,
        #[serde(default)]
        marks: Vec<MarkNode>,
    },
    CodeBlock {
        attrs: Option<CodeBlockAttrs>,
        #[serde(default)]
        content: Vec<IssueDescriptionNode>,
    },
    Blockquote {
        #[serde(default)]
        content: Vec<IssueDescriptionNode>,
    },
    BulletList {
        #[serde(default)]
        content: Vec<IssueDescriptionNode>,
    },
    Emoji {
        attrs: EmojiAttrs,
    },
    HardBreak,
    Heading {
        attrs: HeadingAtrs,
        #[serde(default)]
        content: Vec<IssueDescriptionNode>,
    },
    InlineCard {
        attrs: InlineCardAttrs,
    },
    ListItem {
        #[serde(default)]
        content: Vec<IssueDescriptionNode>,
    },
    MediaGroup {
        #[serde(default)]
        content: Vec<IssueDescriptionNode>,
    },
    MediaSingle {
        attrs: MediaSingleAttrs,
        #[serde(default)]
        content: Vec<IssueDescriptionNode>,
    },
    Media {
        attrs: MediaAttrs,
    },
    Mention {
        attrs: MentionAttrs,
    },
    OrderedList {
        attrs: Option<OrderedListAttrs>,
        #[serde(default)]
        content: Vec<IssueDescriptionNode>,
    },
    Panel {
        attrs: PanelAttrs,
        #[serde(default)]
        content: Vec<IssueDescriptionNode>,
    },
    Rule,
    Table {
        attrs: Option<TableAttrs>,
        content: Vec<IssueDescriptionNode>,
    },
    TableCell {
        attrs: Option<TableCellAttrs>,
        content: Vec<IssueDescriptionNode>,
    },
    TableHeader {
        attrs: Option<TableCellAttrs>, // the same as table cell
        content: Vec<IssueDescriptionNode>,
    },
    TableRow {
        content: Vec<IssueDescriptionNode>,
    },
    // TODO: Support
    BlockCard,
}

#[derive(Deserialize, Debug)]
pub struct CodeBlockAttrs {
    pub language: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MarkNode {
    Code,
    Em,
    Link { attrs: LinkAttrs },
    Strike,
    Strong,
    Subsup { attrs: SubsupAttrs },
    TextColor { attrs: ColorAttrs },
    Underline,
}

#[derive(Deserialize, Debug)]
pub struct LinkAttrs {
    pub href: String,
    pub title: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SubsupAttrs {
    Sub,
    Sup,
}

#[derive(Deserialize, Debug)]
pub struct ColorAttrs {
    pub color: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EmojiAttrs {
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub text: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HeadingAtrs {
    pub level: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InlineCardAttrs {
    pub url: String,
    // TODO: pub data
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediaSingleAttrs {
    pub layout: String,
    #[serde(default)]
    pub width: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediaAttrs {
    #[serde(rename = "type")]
    pub typ: MediaType,
    pub id: String,
    pub collection: String,
    #[serde(rename = "occurrenceKey")]
    pub occurrence_key: Option<String>,
    #[serde(default)]
    pub width: usize,
    #[serde(default)]
    pub height: usize,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    File,
    Link,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MentionAttrs {
    pub id: String,
    pub text: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderedListAttrs {
    pub order: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PanelAttrs {
    #[serde(rename = "panelType")]
    pub typ: PanelType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TableAttrs {
    #[serde(rename = "isNumberColumnEnabled", default)]
    pub number_column_enabled: bool,
    #[serde(default)]
    pub layout: TableLayout,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TableLayout {
    Default,
    #[serde(rename = "full-width")]
    FullWidth,
    Wide,
}

impl Default for TableLayout {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TableCellAttrs {
    pub background: Option<String>,
    #[serde(default)]
    pub colspan: usize,
    #[serde(default)]
    pub colwidth: usize,
    #[serde(default)]
    pub rowspan: usize,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PanelType {
    Info,
    Note,
    Warning,
    Success,
    Error,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum NodeAttrs {
    Emoji {
        #[serde(rename = "shortName")]
        short_name: String,
        text: String,
    },
    Heading {
        level: usize,
    },
    InlineCard {
        url: String,
    },
    Mention {
        id: String,
        text: String,
    },
}

#[derive(Deserialize, Debug)]
pub struct IssuDescriptionMark {
    #[serde(default)]
    pub typ: MarkType,
    pub attrs: Option<MarkAttrs>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MarkType {
    None,
    Code,
    Em,
    Link,
    Strike,
    Strong,
    Subsup,
    TextColor,
    Underline,
}

impl Default for MarkType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Deserialize, Debug)]
pub struct MarkAttrs {
    pub href: Option<String>,
    pub title: Option<String>,
    pub color: Option<String>,
}
