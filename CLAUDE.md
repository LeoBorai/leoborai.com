# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Leo Borai's personal website/blog: a SvelteKit 5 app deployed to Cloudflare Pages, plus a small Rust workspace whose CLI generates the static blog-post index consumed by the frontend at runtime. The two halves (`src/`, `crates/`) are coupled only through the generated `static/notes/index.json` file — there is no shared build step that wires them together automatically.

## Commands

Package manager is **bun** (see `bun.lock`, CI uses `bun i`). `.npmrc` sets `engine-strict=true`.

```bash
bun run dev          # start dev server
bun run build        # production build (adapter-cloudflare -> .svelte-kit/cloudflare)
bun run preview      # preview the production build
bun run check        # svelte-kit sync + svelte-check (type checking)
bun run check:watch  # same, watch mode
bun run lint         # prettier --check . && eslint .
bun run format       # prettier --write .  (also `just fmt`)
```

Rust workspace (default member `crates/cli`):

```bash
cargo build
cargo run -p cli -- notes   # regenerate static/notes/index.json from src/notes/*.svx
```

There is no test suite in either half of the repo (no `#[test]` in the Rust crates, no `*.test.*` files on the frontend).

`Justfile`:

- `just dkc` — pulls and runs `ghcr.io/leoborai/dkc:latest`, mounting the repo at `/app` (dev container).
- `just fmt` — runs `bun run format`.

## Architecture

### Notes/blog pipeline

Blog posts are `.svx` (Markdown + Svelte, via mdsvex) files in `src/notes/`, each with YAML front matter parsed by the `notes` crate (`crates/notes/src/lib.rs`) into `NoteMetadata { title, description, icon, date, preview_image_url, published, categories }`.

`cargo run -p cli -- notes` (implemented in `crates/cli/src/notes_index.rs`) reads every file in `src/notes/`, drops unpublished entries, sorts by date descending, and writes the result as `static/notes/index.json`. This index is a **build artifact that must be regenerated manually** whenever notes are added, edited, or (un)published — the SvelteKit build does not do this itself.

At runtime, `src/routes/+page.server.ts` and `src/routes/notes/+page.server.ts` fetch `/notes/index.json` (the static file above) and sort/slice it for display. `src/routes/notes/[slug]/+page.ts` dynamically imports the matching `../../../notes/{slug}.svx` module directly (not via the index) to render a single post, throwing a 404 if the import fails.

The `Domain.Note` type in `src/app.d.ts` is a hand-maintained mirror of the Rust `RichNoteMetadata`/`NoteMetadata` structs — keep both in sync manually when the note schema changes (there is no shared codegen between the Rust and TypeScript sides).

### Theme/config

Theme preference flows through a `config` cookie, not localStorage:

- `src/routes/+layout.server.ts` reads the `config` cookie, falling back to the `sec-ch-prefers-color-scheme` request header.
- `src/lib/stores/uiStore.ts` (`createUIStore`) applies the `dark` class to `document.documentElement` and persists changes via `saveConfig` (`src/lib/utils/config.ts`), which `PATCH`es `src/routes/api/v0/config/+server.ts` to update the cookie.

### Resume request endpoint

`src/routes/api/v0/resume/+server.ts` POSTs a Telegram message (via the Telegram Bot API) whenever a resume is requested, using `TELEGRAM_BOT`/`TELEGRAM_CHAT` env vars (`$env/static/private`). These plus `GH_API_TOKEN` and `VERSION` are injected at build/deploy time in CI, not read from a local `.env` for production builds.

### Components

`src/lib/components/atoms/` holds all Svelte components (flat, no further atomic-design tiers despite the folder name). `src/routes/+layout.svelte` composes `Navigation`, `ThemeToggle`, and `Footer` around `{@render children()}`.

## Deployment

Two GitHub Actions workflows (`.github/workflows/`), both: `bun i` → `bun run build` → `wrangler pages deploy ./.svelte-kit/cloudflare` via `cloudflare/wrangler-action`.

- `deploy-staging.yml`: triggers on push to `main`, deploys to the staging Cloudflare Pages project, `NODE_ENV=development`.
- `deploy.yml`: triggers on `v*` tags, deploys to the production Cloudflare Pages project, `NODE_ENV=production`.

Neither workflow runs the Rust CLI — the notes index committed to `static/notes/index.json` must already be up to date before tagging/pushing.

## Style

- Formatting is enforced by Prettier (`useTabs`, single quotes, no trailing commas, 100 print width) with `prettier-plugin-svelte` and `prettier-plugin-tailwindcss` — run `bun run format` rather than hand-formatting.
- TypeScript strict mode is on; ESLint extends `typescript-eslint` recommended + `eslint-plugin-svelte` recommended, with Prettier conflict rules disabled.
