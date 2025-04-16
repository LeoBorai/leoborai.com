use std::cmp::Ordering;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;

use proto::{NoteMetadata, NotesIndex, RichNoteMetadata};

const NOTES_DIR: &str = "crates/www/assets/notes";

pub trait NotesIndexExt {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Result<NotesIndex>;
    fn save_to_file(&self, path: PathBuf) -> Result<()>;
}

impl NotesIndexExt for NotesIndex {
    fn new() -> Result<Self> {
        let mut index: Vec<RichNoteMetadata> = Vec::new();
        let entries = list_entries()?;

        tracing::info!("Found {} entries", entries.len());

        for entry in entries {
            if let Some(meta) = find_note(&entry)? {
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

        Ok(NotesIndex(index))
    }

    fn save_to_file(&self, path: PathBuf) -> Result<()> {
        let index = serde_json::to_string_pretty(&self)?;
        let path = path.join("notes_index.json");

        std::fs::write(&path, index)?;
        tracing::info!("Saved index to file at {}", path.display());

        Ok(())
    }
}

fn list_entries() -> Result<Vec<PathBuf>> {
    let dir = read_dir(NOTES_DIR)?;

    Ok(dir
        .into_iter()
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    return None;
                }

                if let Some(ext) = path.extension() {
                    if ext != "md" {
                        return None;
                    }
                }

                Some(entry.path())
            } else {
                None
            }
        })
        .collect())
}

fn find_note(entry: &PathBuf) -> Result<Option<NoteMetadata>> {
    tracing::info!("Reading note from {:?}", entry);
    if entry.exists() {
        let file = read_to_string(entry)?;
        let note = NoteMetadata::from_str(&file)?;

        return Ok(Some(note));
    }

    Ok(None)
}
