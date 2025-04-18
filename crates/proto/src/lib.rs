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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Icon {
    Docker,
    Git,
    Rust,
    Python,
    Svelte,
    Gcp,
    TypeScript,
    #[default]
    Dev,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NotesIndex(pub Vec<RichNoteMetadata>);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RichNoteMetadata {
    pub meta: NoteMetadata,
    pub slug: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BookshelfBookShopping {
    pub amazon: String,
    pub online: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BookshelfBook {
    pub title: String,
    pub author: String,
    pub isbn_10: String,
    pub isbn_13: String,
    pub publisher: String,
    pub read_on: i32,
    pub review: String,
    pub shopping: BookshelfBookShopping,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub repo_url: String,
    pub extract: String,
    pub website: Option<String>,
    pub tags: Vec<String>,
    pub langs: Vec<String>,
}
