use std::path::Path;

use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, TextContent, schema_utils::CallToolError},
};
use serde::{Deserialize, Serialize};

use crate::fs_service::FileSystemService;

/// Type of filter to apply when filtering file lines.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum FilterType {
    /// Filter using regular expression pattern
    #[serde(rename = "regex")]
    Regex,
    /// Filter by matching keywords (comma-separated)
    #[serde(rename = "keywords")]
    Keywords,
    /// Filter by specific line numbers or ranges (e.g., "1-5,10,15-20")
    #[serde(rename = "lines")]
    Lines,
}

/// Options for customizing the filtering behavior.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FilterOptions {
    /// If true, the search is case-insensitive (default: true for keywords, false for regex)
    #[serde(rename = "caseInsensitive", skip_serializing_if = "Option::is_none")]
    pub case_insensitive: Option<bool>,
    /// Whether to include line numbers in the output (default: true)
    #[serde(rename = "includeLineNumbers", skip_serializing_if = "Option::is_none")]
    pub include_line_numbers: Option<bool>,
    /// Number of context lines to show before each match (default: 0)
    #[serde(rename = "contextBefore", skip_serializing_if = "Option::is_none")]
    pub context_before: Option<u32>,
    /// Number of context lines to show after each match (default: 0)
    #[serde(rename = "contextAfter", skip_serializing_if = "Option::is_none")]
    pub context_after: Option<u32>,
    /// For keywords: match whole words only (default: false)
    #[serde(rename = "wholeWords", skip_serializing_if = "Option::is_none")]
    pub whole_words: Option<bool>,
    /// For keywords: require all keywords to match (AND logic) vs any keyword (OR logic) (default: false = OR)
    #[serde(rename = "matchAll", skip_serializing_if = "Option::is_none")]
    pub match_all: Option<bool>,
    /// Maximum number of results to return (default: unlimited)
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,
    /// If true, ^ and $ match line boundaries instead of string boundaries (default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    /// If true, the dot (.) matches newlines as well (default: false)
    #[serde(rename = "dotAll", skip_serializing_if = "Option::is_none")]
    pub dot_all: Option<bool>,
}

impl Default for FilterOptions {
    fn default() -> Self {
        Self {
            case_insensitive: None,
            include_line_numbers: Some(true),
            context_before: Some(0),
            context_after: Some(0),
            whole_words: Some(false),
            match_all: Some(false),
            max_results: None,
            multiline: None,
            dot_all: None,
        }
    }
}

// filter_file_lines
#[mcp_tool(
    name = "filter_file_lines",
    title="Filter file lines",
    description = concat!(
        "Filters lines from a text file based on different criteria (regex, keywords, or line numbers).",
        "Supports three filter types: ",
        "'regex' for regular expression patterns, ",
        "'keywords' for comma-separated keyword matching, ",
        "'lines' for specific line numbers/ranges (e.g., '1-5,10,15-20'). ",
        "Additional options include context lines, case sensitivity, and line numbering. ",
        "Only works within allowed directories."
    ),
    destructive_hint = false,
    idempotent_hint = true,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FilterFileLines {
    /// The path of the file to filter.
    pub path: String,
    /// Type of filter to apply: 'regex', 'keywords', or 'lines'.
    pub filter_type: FilterType,
    /// Filter criteria based on the filter type:
    /// - For 'regex': a regular expression pattern
    /// - For 'keywords': comma-separated keywords
    /// - For 'lines': line numbers or ranges (e.g., "1-5,10,15-20")
    pub criteria: String,
    /// Optional filtering options for customization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<FilterOptions>,
    /// Optional line range to restrict filtering (format: "start-end" or "start:end")
    #[serde(rename = "lineRange", skip_serializing_if = "Option::is_none")]
    pub line_range: Option<String>,
}

impl FilterFileLines {
    pub async fn run_tool(
        params: Self,
        context: &FileSystemService,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let result = context
            .filter_file_lines(
                Path::new(&params.path),
                params.filter_type,
                &params.criteria,
                params.options,
                params.line_range,
            )
            .await
            .map_err(CallToolError::new)?;

        Ok(CallToolResult::text_content(vec![TextContent::from(
            result,
        )]))
    }
}
