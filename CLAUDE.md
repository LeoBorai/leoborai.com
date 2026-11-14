# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Leo Borai's personal website/blog: a static site written in Rust with Dioxus (`dioxus-ssr`) and generated at build time into `dist/`, then deployed to Cloudflare Pages. No Node.js, npm or `dx` CLI is needed anywhere.

## Commands

```bash
cargo run --release            # generate the site into ./dist
cargo run --release -- --out X # generate into ./X
just serve                             # build + serve ./dist on :8080
cargo fmt --all
```

There is no test suite. `TAILWINDCSS_BIN=/path/to/tailwindcss` makes `build.rs` use a local Tailwind binary instead of downloading one.

## Architecture

- Single crate `site` (no workspace); the generator binary.
  - `build.rs` (repo root) downloads the pinned, sha256-verified Tailwind standalone binary (bundles the forms + typography plugins), compiles `assets/app.css` (scans `src/` and `assets/site.js` for classes) to `$OUT_DIR/app.css`, which `main.rs` embeds.
  - `src/components/{atoms,molecules,organisms}` – atomic Dioxus components; `src/pages/` – one file per page, `Root` (in `pages/mod.rs`) renders chrome + one `Page` variant. Rendered with `dioxus_ssr::render` (pure SSG, no WASM/hydration).
  - `src/markdown.rs` – `pulldown-cmark` + `syntect`/`two-face` highlighting. Code blocks emit `<pre class="shiki">` with `--shiki-light/--shiki-dark` vars that `assets/app.css` switches on `.dark`.
  - `src/content.rs` – loads `content/notes/*.md` (drops `published: false`, newest first) and `content/skills/*.md`.
  - `src/data.rs` + `assets/{social,projects,uses}.json` – static site data.
  - `assets/site.js` – the only client JS (theme toggle, mobile menu, Berlin clock, copy-as-MD, canvas effects). Inlined theme bootstrap lives in `main.rs` (`THEME_SCRIPT`).
- `src/notes.rs`, `src/skills.rs` – front matter types/parsers (`parse_with_body`). `NoteMetadata` fields: title, description, icon, date, preview_image_url, published, categories.
- `content/` – Markdown sources. Adding a note = adding a file; there is no index to regenerate.
- `static/` – copied verbatim into `dist/` (fonts, images, favicons, note images).

Output is `dist/<route>/index.html` (+ `404.html`, `_headers` for cache rules, `assets/*.{hash}.{css,js}`).

Theme is stored in `localStorage` (`theme`), falling back to `prefers-color-scheme`.

## Deployment

`.github/workflows/`: `deploy-staging.yml` (push to `main`) and `deploy.yml` (`v*` tags) run `cargo run --release -- --out dist` then `wrangler pages deploy ./dist` via `cloudflare/wrangler-action`. `audit.yml` runs `cargo audit` on PRs, pushes to `main` and weekly.

## Style

Rust 2024, `cargo fmt`. Tailwind utility classes live inline in the components.
