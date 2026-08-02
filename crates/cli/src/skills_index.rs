use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use skills::SkillMetadata;

/// `Cargo.toml` relative path to the directory containing skills
const MDSVEX_DIR: &str = "src/skills";

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillsIndex(Vec<RichSkillMetadata>);

#[derive(Debug, Serialize, Deserialize)]
pub struct RichSkillMetadata {
    pub meta: SkillMetadata,
    pub slug: String,
}

impl SkillsIndex {
    /// Creates a new `RichSkillMetadata` index (`SkillsIndex`) sorted by name.
    pub fn new() -> Result<Self> {
        let mut index: Vec<RichSkillMetadata> = Vec::new();
        let entries = Self::list_entries()?;

        tracing::info!("Found {} entries", entries.len());

        for entry in entries {
            if let Some(meta) = Self::find_skill(&entry)? {
                let Some(os_str_filename) = entry.file_stem() else {
                    tracing::error!("Failed to get filename for {:?}", entry);
                    continue;
                };

                let Some(filename) = os_str_filename.to_str() else {
                    tracing::error!("OsString is not a valid UTF-8 string");
                    continue;
                };

                index.push(RichSkillMetadata {
                    slug: filename.to_string(),
                    meta,
                });
            } else {
                tracing::error!("Failed to find skill in {:?}", entry);
            }
        }

        index.sort_by(|a, b| a.meta.name.cmp(&b.meta.name));

        Ok(Self(index))
    }

    pub fn save_to_file(&self, path: PathBuf) -> Result<()> {
        let index = serde_json::to_string_pretty(&self)?;
        let path = path.join("index.json");

        std::fs::write(&path, index)?;
        tracing::info!("Saved index to file at {}", path.display());

        Ok(())
    }

    /// Lists entries for skills localted at [`MDSVEX_DIR`]
    fn list_entries() -> Result<Vec<PathBuf>> {
        let dir = read_dir(MDSVEX_DIR)?;

        Ok(dir.into_iter().map(|entry| entry.unwrap().path()).collect())
    }

    /// Attempts to find a skill in the given entry
    fn find_skill(entry: &PathBuf) -> Result<Option<SkillMetadata>> {
        if entry.exists() {
            let file = read_to_string(entry)?;
            let skill = SkillMetadata::from_str(&file)?;

            return Ok(Some(skill));
        }

        Ok(None)
    }
}
