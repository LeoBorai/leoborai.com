//! Loads `content/notes/*.md` and `content/skills/*.md` and renders them to HTML.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use notes::NoteMetadata;
use skills::SkillMetadata;

use crate::markdown::Markdown;

#[derive(Clone, PartialEq)]
pub struct NoteEntry {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
    pub html: String,
}

#[derive(Clone, PartialEq)]
pub struct SkillEntry {
    pub slug: String,
    pub name: String,
    pub description: String,
    /// Raw Markdown without front matter (for the "Copy as MD" button)
    pub markdown: String,
    pub html: String,
}

fn markdown_files(dir: &Path) -> Result<Vec<(String, String)>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();

        if path.extension().is_some_and(|ext| ext == "md") {
            let slug = path
                .file_stem()
                .and_then(|s| s.to_str())
                .context("non UTF-8 file name")?
                .to_string();

            files.push((slug, fs::read_to_string(&path)?));
        }
    }

    Ok(files)
}

/// Published notes, newest first.
pub fn load_notes(root: &Path, md: &Markdown) -> Result<Vec<NoteEntry>> {
    let mut entries = Vec::new();

    for (slug, source) in markdown_files(&root.join("content/notes"))? {
        let (meta, body) = NoteMetadata::parse_with_body(&source)
            .with_context(|| format!("parsing note `{slug}`"))?;

        if !meta.published {
            continue;
        }

        entries.push(NoteEntry {
            html: md.render(&body),
            slug,
            title: meta.title,
            description: meta.description,
            date: meta.date,
        });
    }

    entries.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.slug.cmp(&b.slug)));

    Ok(entries)
}

/// Skills, sorted by name.
pub fn load_skills(root: &Path, md: &Markdown) -> Result<Vec<SkillEntry>> {
    let mut entries = Vec::new();

    for (slug, source) in markdown_files(&root.join("content/skills"))? {
        let (meta, body) = SkillMetadata::parse_with_body(&source)
            .with_context(|| format!("parsing skill `{slug}`"))?;

        entries.push(SkillEntry {
            html: md.render(&body),
            markdown: body,
            slug,
            name: meta.name,
            description: meta.description,
        });
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(entries)
}
