use std::cmp::Ordering;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use notes::NoteMetadata;

/// `Cargo.toml` relative path to the directory containing notes
const MDSVEX_DIR: &str = "src/notes";

#[derive(Debug, Serialize, Deserialize)]
pub struct NotesIndex(Vec<RichNoteMetadata>);

#[derive(Debug, Serialize, Deserialize)]
pub struct RichNoteMetadata {
    pub meta: NoteMetadata,
    pub slug: String,
}

impl NotesIndex {
    /// Creates a new `RichNoteMetadata` index (`NotesIndex`) sorted by creation
    /// date (from YAML Front Matter's Date).
    pub fn new() -> Result<Self> {
        let mut index: Vec<RichNoteMetadata> = Vec::new();
        let entries = Self::list_entries()?;

        tracing::info!("Found {} entries", entries.len());

        for entry in entries {
            if let Some(meta) = Self::find_note(&entry)? {
                if !meta.published {
                    continue;
                }

                let Some(os_str_filename) = entry.file_stem() else {
                    tracing::error!("Failed to get filename for {:?}", entry);
                    continue;
                };

                let Some(filename) = os_str_filename.to_str() else {
                    tracing::error!("OsString is not a valid UTF-8 string");
                    continue;
                };

                index.push(RichNoteMetadata {
                    slug: filename.to_string(),
                    meta,
                });
            } else {
                tracing::error!("Failed to find note in {:?}", entry);
            }
        }

        index.sort_by(|a, b| {
            if a.meta.date > b.meta.date {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        Ok(Self(index))
    }

    pub fn save_to_file(&self, path: PathBuf) -> Result<()> {
        let index = serde_json::to_string_pretty(&self)?;
        let path = path.join("index.json");

        std::fs::write(&path, index)?;
        tracing::info!("Saved index to file at {}", path.display());

        Ok(())
    }

    /// Lists entries for notes localted at [`MDSVEX_DIR`]
    fn list_entries() -> Result<Vec<PathBuf>> {
        let dir = read_dir(MDSVEX_DIR)?;

        Ok(dir.into_iter().map(|entry| entry.unwrap().path()).collect())
    }

    /// Attempts to find a note in the given entry
    fn find_note(entry: &PathBuf) -> Result<Option<NoteMetadata>> {
        if entry.exists() {
            let file = read_to_string(entry)?;
            let note = NoteMetadata::from_str(&file)?;

            return Ok(Some(note));
        }

        Ok(None)
    }
}
