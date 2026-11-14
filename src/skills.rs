//! AI Agent Skills types and utility functions

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
}

impl SkillMetadata {
    /// Parses a file, returning its metadata and the Markdown body
    /// (everything after the YAML front matter).
    pub fn parse_with_body(s: &str) -> Result<(Self, String), crate::frontmatter::Error> {
        crate::frontmatter::parse(s)
    }
}
