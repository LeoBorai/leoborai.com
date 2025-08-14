//! Notes types and utility functions

use std::str::FromStr;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use yaml_front_matter::YamlFrontMatter;

/// The format of the date string in the YAML front matter
pub const DATE_STR_FORMAT: &str = "%Y-%m-%d";

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse YAML front matter. {0}")]
    ParseError(String),
}

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

impl FromStr for NoteMetadata {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let front_matter = YamlFrontMatter::parse::<NoteMetadata>(s)
            .map_err(|err| Error::ParseError(err.to_string()))?;
        let note = front_matter.metadata;

        Ok(note)
    }
}
