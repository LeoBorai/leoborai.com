# Tasks

## 1. Switch to npm

- [x] 1.1 Delete `bun.lock` and `node_modules`; verify `git status` shows `bun.lock` removed
- [x] 1.2 Add `engines.node: ">=22.17"` to `package.json`; verify `npm install` honors it (`engine-strict=true`)
- [x] 1.3 Run `npm install` to create `package-lock.json`; verify the file exists and `npm ci` succeeds from a clean `node_modules`
- [x] 1.4 Replace bun references in `Justfile` (`just fmt` -> `npm run format`), `.prettierignore`, `README.md`; verify `grep -rIni bun --exclude-dir=node_modules --exclude-dir=.git --exclude-dir=target --exclude-dir=openspec --exclude-dir=src/notes .` has no tooling hits

## 2. Upgrade dependencies

- [x] 2.1 Run `npm outdated` and `npm audit` and record the baseline; verify output is captured in the commit/PR notes
- [x] 2.2 Bump `@sveltejs/kit` to `^3.1.0`, `@sveltejs/adapter-cloudflare` to `^8.0.1`, `svelte` to latest 5.x (>=5.57.1), `typescript` to `^6`, and add `wrangler` `^4.118.0` as devDependency; verify `npm install` resolves with no peer-dependency errors
- [x] 2.3 Remove the `wrangler` and `cookie` entries from `overrides` (drop the block if empty); verify `npm ls wrangler cookie` shows single, compatible versions
- [x] 2.4 Bump the remaining dependencies to latest within peer ranges (vite, vite-plugin-svelte, tailwind, eslint, prettier, svelte-check, etc.); verify `npm outdated` lists only intentionally held packages (TypeScript 7)
- [x] 2.5 Run `npm audit` and `npm audit signatures`; fix findings by upgrading, or add a commented `overrides` entry; verify no unresolved high/critical findings remain

## 3. Fix Kit 3 breakages

- [x] 3.1 Review `svelte.config.js`, `vite.config.ts`, `tsconfig.json` against the Kit 3 changelog (Svelte options via the Vite plugin, tsconfig generated in `node_modules/$app/tsconfig`); verify `npm run check` reports no config errors
- [x] 3.2 Replace `$app/stores` with `$app/state` in `src/routes/+error.svelte`; verify the 404 page shows status and message
- [x] 3.3 Confirm cookie handling in `src/routes/api/v0/config/+server.ts` and `src/routes/+layout.server.ts` works under cookie v2; verify `PATCH /api/v0/config` sets `config` and the theme persists after reload
- [x] 3.4 Fix any remaining type or lint errors from TS 6 / ESLint upgrades; verify `npm run check` and `npm run lint` exit 0

## 4. Verify build and runtime

- [x] 4.1 Run `npm run build`; verify `.svelte-kit/cloudflare` is produced without errors
- [x] 4.2 Run `npm run preview` and smoke-test `/`, `/notes`, `/notes/<slug>`, `/skills`, `/skills/<slug>`, `/projects`, `/uses`, an unknown URL (404), theme toggle, and `/api/v0/resume`; verify each responds correctly and `.svx` content renders
- [ ] 4.3 Confirm `cargo run -p cli -- notes` still regenerates `static/notes/index.json` with no diff; verify `git diff static/notes/index.json` is empty

## 5. CI and docs

- [ ] 5.1 Update `.github/workflows/deploy.yml` and `deploy-staging.yml`: `actions/setup-node` (Node 22, `cache: npm`) before install, `npm ci`, `npm run build`, remove `oven-sh/setup-bun`; verify the staging workflow succeeds on push to `main`
- [x] 5.2 Update `CLAUDE.md` (package manager, commands, deployment sections, Kit 3 / Node >=22.17 notes); verify no stale bun mentions remain
- [ ] 5.3 Verify the staging deployment loads and the theme/resume endpoints work before tagging a production release

## Workflow follow-up

- Archive the change after the staging deployment is verified.
