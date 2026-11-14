//! Static site generator for leoborai.com.
//!
//! Renders every page with Dioxus (`dioxus-ssr`) and writes the result to
//! `dist/`, ready to be deployed to Cloudflare Pages.
//!
//! ```sh
//! cargo run --release            # from the repository root
//! cargo run --release -- --out public
//! ```

mod components;
mod content;
mod data;
mod frontmatter;
mod markdown;
mod notes;
mod pages;
mod skills;

use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{Datelike, Utc};
use dioxus::prelude::*;

use crate::pages::{Page, Root, RootProps};

const APP_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/app.css"));
const SITE_JS: &str = include_str!("../assets/site.js");

const TITLE: &str = "Leo Borai | Rust Software Engineer";
const DESCRIPTION: &str = "Leo Borai is a Rust Software Engineer with expertise in building efficient and reliable software solutions.";

const CACHE_HEADERS: &str = "/assets/*
  Cache-Control: public, max-age=31536000, immutable

/fonts/*
  Cache-Control: public, max-age=31536000, immutable

/img/*
  Cache-Control: public, max-age=604800

/notes/images/*
  Cache-Control: public, max-age=604800
";

/// Applies the saved theme (falling back to the OS preference) before first paint.
const THEME_SCRIPT: &str = "try{var t=localStorage.getItem('theme');if(t?t==='dark':matchMedia('(prefers-color-scheme: dark)').matches)document.documentElement.classList.add('dark')}catch(e){}";

struct Assets {
    css: String,
    js: String,
}

struct Site {
    out: PathBuf,
    assets: Assets,
    year: i32,
}

fn main() -> Result<()> {
    let root = std::env::current_dir()?;
    let out = parse_out_dir(&root);

    if out.exists() {
        fs::remove_dir_all(&out).with_context(|| format!("cleaning {}", out.display()))?;
    }
    fs::create_dir_all(&out)?;

    copy_dir(&root.join("static"), &out)?;

    let md = markdown::Markdown::new();
    let notes = content::load_notes(&root, &md)?;
    let skills = content::load_skills(&root, &md)?;

    let site = Site {
        assets: write_assets(&out)?,
        year: Utc::now().year(),
        out,
    };

    site.page(
        "/",
        TITLE,
        DESCRIPTION,
        Page::Home {
            notes: notes.iter().take(5).cloned().collect(),
        },
    )?;
    site.page(
        "/notes",
        TITLE,
        DESCRIPTION,
        Page::Notes {
            notes: notes.clone(),
        },
    )?;
    site.page(
        "/skills",
        TITLE,
        DESCRIPTION,
        Page::Skills {
            skills: skills.clone(),
        },
    )?;
    site.page("/projects", TITLE, DESCRIPTION, Page::Projects)?;
    site.page("/uses", TITLE, DESCRIPTION, Page::Uses)?;

    for note in &notes {
        site.page(
            &format!("/notes/{}", note.slug),
            &format!("{} | Leo Borai", note.title),
            &note.description,
            Page::Note { note: note.clone() },
        )?;
    }

    for skill in &skills {
        site.page(
            &format!("/skills/{}", skill.slug),
            &format!("{} | Leo Borai", skill.name),
            &skill.description,
            Page::Skill {
                skill: skill.clone(),
            },
        )?;
    }

    site.write(
        "404.html",
        &site.document("/404", "Not found | Leo Borai", DESCRIPTION, Page::NotFound),
    )?;
    site.write("_headers", CACHE_HEADERS)?;

    println!(
        "Generated {} notes, {} skills -> {}",
        notes.len(),
        skills.len(),
        site.out.display()
    );

    Ok(())
}

fn parse_out_dir(root: &Path) -> PathBuf {
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg == "--out"
            && let Some(dir) = args.next()
        {
            return root.join(dir);
        }
    }

    root.join("dist")
}

fn hash(content: &str) -> String {
    let mut hasher = std::hash::DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:08x}", hasher.finish() as u32)
}

fn write_assets(out: &Path) -> Result<Assets> {
    let dir = out.join("assets");
    fs::create_dir_all(&dir)?;

    let css = format!("app.{}.css", hash(APP_CSS));
    let js = format!("site.{}.js", hash(SITE_JS));

    fs::write(dir.join(&css), APP_CSS)?;
    fs::write(dir.join(&js), SITE_JS)?;

    Ok(Assets {
        css: format!("/assets/{css}"),
        js: format!("/assets/{js}"),
    })
}

fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to)?;

    for entry in fs::read_dir(from).with_context(|| format!("reading {}", from.display()))? {
        let entry = entry?;
        let target = to.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }

    Ok(())
}

fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

impl Site {
    fn write(&self, rel: &str, content: &str) -> Result<()> {
        let path = self.out.join(rel);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&path, content).with_context(|| format!("writing {}", path.display()))
    }

    /// Writes `/some/path` as `some/path/index.html`.
    fn page(&self, path: &str, title: &str, description: &str, page: Page) -> Result<()> {
        let file = if path == "/" {
            "index.html".to_string()
        } else {
            format!("{}/index.html", path.trim_matches('/'))
        };

        self.write(&file, &self.document(path, title, description, page))
    }

    fn document(&self, path: &str, title: &str, description: &str, page: Page) -> String {
        let mut vdom = VirtualDom::new_with_props(
            Root,
            RootProps {
                path: path.to_string(),
                page,
                year: self.year,
            },
        );
        vdom.rebuild_in_place();
        let body = dioxus_ssr::render(&vdom);

        let title = escape_attr(title);
        let description = escape_attr(description);

        format!(
            r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<meta name="description" content="{description}">
<meta name="keywords" content="Rust, Software Engineer, Leo Borai, programming, software development, GitHub">
<meta name="author" content="Leo Borai">
<meta property="og:title" content="{title}">
<meta property="og:description" content="{description}">
<meta property="og:url" content="https://github.com/LeoBorai">
<meta property="og:image" content="https://github.com/LeoBorai.png">
<meta property="og:type" content="profile">
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="{title}">
<meta name="twitter:description" content="{description}">
<meta name="twitter:image" content="https://github.com/LeoBorai.png">
<link rel="canonical" href="https://github.com/LeoBorai">
<link rel="icon" href="/favicon.ico" type="image/x-icon">
<script>{THEME_SCRIPT}</script>
<link rel="stylesheet" href="{css}">
<script src="{js}" defer></script>
</head>
<body class="bg-[#f0eeeb] dark:bg-[#121212]">
{body}
</body>
</html>
"##,
            css = self.assets.css,
            js = self.assets.js,
        )
    }
}
