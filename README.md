# leoborai.com

Personal website and blog. A static site written in Rust with [Dioxus](https://dioxuslabs.com) (`dioxus-ssr`),
generated at build time and deployed to Cloudflare Pages. No Node.js, npm or `dx` CLI required.

```bash
cargo run --release          # writes the site to ./dist
cargo run --release -- --out public
just serve                           # build + serve ./dist on :8080
```

- `content/notes/*.md` – blog posts (YAML front matter + Markdown)
- `content/skills/*.md` – AI agent skills
- `static/` – copied verbatim into the output (fonts, images, favicons)
- `src/` – the generator (Dioxus components, Markdown rendering, syntax highlighting, front matter parsing)
- `build.rs` – downloads the pinned Tailwind CSS standalone binary and compiles `assets/app.css`

Adding a note is just dropping a new `.md` file in `content/notes/`; there is no index to regenerate.
