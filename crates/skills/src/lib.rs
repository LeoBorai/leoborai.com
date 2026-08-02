//! AI Agent Skills types and utility functions

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use yaml_front_matter::YamlFrontMatter;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse YAML front matter. {0}")]
    ParseError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
}

impl FromStr for SkillMetadata {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let front_matter = YamlFrontMatter::parse::<SkillMetadata>(s)
            .map_err(|err| Error::ParseError(err.to_string()))?;
        let skill = front_matter.metadata;

        Ok(skill)
    }
}
