//! Notes types and utility functions

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Icon {
    Docker,
    Git,
    Rust,
    Python,
    Svelte,
    Gcp,
    TypeScript,
    Dev,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteMetadata {
    pub title: String,
    pub description: String,
    pub icon: Icon,
    pub date: NaiveDate,
    pub preview_image_url: String,
    pub published: bool,
    pub categories: Vec<String>,
}

impl NoteMetadata {
    /// Parses a file, returning its metadata and the Markdown body
    /// (everything after the YAML front matter).
    pub fn parse_with_body(s: &str) -> Result<(Self, String), crate::frontmatter::Error> {
        crate::frontmatter::parse(s)
    }
}
