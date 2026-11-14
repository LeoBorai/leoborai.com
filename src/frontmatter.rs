//! Minimal YAML front matter splitter.

use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing `---` delimited YAML front matter")]
    Missing,
    #[error("invalid YAML front matter: {0}")]
    Yaml(#[from] serde_yaml_ng::Error),
}

/// Parses a leading `--- ... ---` YAML block, returning it and the remaining body.
pub fn parse<T: DeserializeOwned>(source: &str) -> Result<(T, String), Error> {
    let source = source.trim_start_matches('\u{feff}').replace("\r\n", "\n");
    let rest = source.strip_prefix("---\n").ok_or(Error::Missing)?;

    let (yaml, body) = match rest.find("\n---\n") {
        Some(end) => (&rest[..end], &rest[end + 5..]),
        None => match rest.strip_suffix("\n---") {
            Some(yaml) => (yaml, ""),
            None => return Err(Error::Missing),
        },
    };

    Ok((
        serde_yaml_ng::from_str(yaml)?,
        body.trim_start_matches('\n').to_string(),
    ))
}
