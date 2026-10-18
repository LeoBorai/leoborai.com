# Design

## Context

Observed state (see proposal.md for motivation):

- `package.json` has `@sveltejs/kit ^2.70.2`, `adapter-cloudflare ^7.2.9`, `svelte ^5.56.8` (already Svelte 5), `typescript ^5.9.3`, `vite ^8.2.0`, `vite-plugin-svelte ^7.2.0`, and `overrides` `wrangler: 4.117.0`, `cookie: ^0.7.2`.
- Registry: `kit@3.1.0` (peers: `vite ^8.0.12`, `svelte ^5.57.1`, `typescript ^6.0.0`, `vite-plugin-svelte ^7`; engines `node >=22.17`), `adapter-cloudflare@8.0.1` (peer `wrangler ^4.118.0`, `kit ^3.0.0-next.0`). Latest `typescript` is 7.0.2, outside Kit's peer range; latest `wrangler` is 4.151.1.
- Lockfile is `bun.lock` (git-tracked). CI (`deploy.yml`, `deploy-staging.yml`) uses `oven-sh/setup-bun`, `bun i`, `bun run build`, then installs Node 22 for `wrangler-action`. `.github/dependabot.yml` already targets `npm`. `.npmrc` has `engine-strict=true`. Local Node is 22.22.1, npm 9.2.0.
- Kit 3 breaking changes that touch this repo (from the Kit CHANGELOG 3.0.0): TypeScript 6 minimum; `cookie` v2 (ASCII-only names) and default cookie `path` `'/'`; Svelte config options must be passed through the Vite plugin; `$app/paths` drops `base`/`assets`/`resolveRoute`; every error goes through `handleError`; tsconfig written to `node_modules/$app/tsconfig`; `204` responses have no body.
- Repo usage: `src/routes/+error.svelte` imports `$app/stores`; the cookie API is used in `src/routes/api/v0/config/+server.ts` (sets `path: '/'` already) and `src/routes/+layout.server.ts`; no `$app/paths`, hooks, or form actions are used. `svelte.config.js` sets `preprocess`, `kit.adapter`, `extensions`.

## Goals / Non-Goals

**Goals:**

- npm is the only supported package manager; `npm ci` is reproducible from a committed `package-lock.json`.
- Kit 3 + Svelte 5 (latest 5.x) build, type-check, lint, and deploy to Cloudflare Pages as before.
- `npm audit` reports no unresolved high/critical findings in production or build dependencies.

**Non-Goals:**

- No feature, styling, or content changes; no migration of Svelte components to new syntax beyond what Kit 3 requires.
- No adding a test suite or changing the Rust workspace.
- No move to TypeScript 7 (blocked by Kit peer range).

## Decisions

- **Pin TypeScript to `^6`.** Kit 3 requires it. Alternative: TS 7 with `--legacy-peer-deps`; rejected as unsupported.
- **Remove both `overrides` first, add back only on evidence.** They exist to force `wrangler`/`cookie` versions that now conflict with Kit 3/adapter 8. Alternative: keep and bump; rejected because adapter 8 declares `wrangler` as a peer and `cookie` is a Kit-internal major bump. Because `wrangler` is a peer, it must be added explicitly as a devDependency (`^4.118.0`, latest 4.x) so `npm ci` and CI resolve it.
- **Use the official migration path**: bump deps, then run `npx sv migrate` only if a Kit 3 migration is offered; otherwise fix by hand from the CHANGELOG. Verify with `npm run check`, `npm run lint`, `npm run build`.
- **Migrate `$app/stores` to `$app/state`** in `+error.svelte` (`page` becomes a rune-style object; `$page.status` -> `page.status`). It is the Svelte 5 idiom and avoids reliance on a deprecated module.
- **Upgrade npm in CI via `actions/setup-node` with `node-version: 22` (>=22.17) and `cache: npm`**, moved before install so the same Node runs install, build, and deploy; `npm ci` replaces `bun i`. Add `engines.node: ">=22.17"` to `package.json` (works with existing `engine-strict=true`). Local npm 9.2.0 is old; lockfile v3 is fine, but note Node 22 ships npm 10.
- **Dependency safety**: run `npm outdated`, `npm audit`, and `npm audit signatures`; update to latest within peer ranges; review release notes only for majors. Alternative of blanket `npm update` alone is insufficient because majors (kit, adapter, typescript) need manual bumps. Pin exact versions via the lockfile, not by removing caret ranges.
- **Keep Dependabot as is** (already `npm`).
- **Docs/config**: update `CLAUDE.md` (commands, package manager, workflows), `Justfile` `fmt`, `.prettierignore` (keep `package-lock.json` ignored; drop bun entries), and `README.md` if it mentions bun.

## Risks / Trade-offs

- [Kit 3 breaks the Cloudflare build or runtime (adapter 8, cookie v2)] -> Run `npm run build` and `npm run preview` locally, smoke-test `/`, `/notes`, `/notes/<slug>`, `/skills`, theme toggle (`PATCH /api/v0/config`), and `/api/v0/resume`; deploy to staging first via push to `main`.
- [Existing `config` cookie values break under cookie v2] -> The cookie name `config` is ASCII and the value is JSON; verify the read/write round trip in preview.
- [mdsvex 0.12.8 untested with Kit 3 / vite-plugin-svelte 7 `svelte` config passing] -> Confirm `.svx` notes and skills render; if the Vite-plugin config rule applies, move `preprocess`/`extensions` accordingly.
- [`npm audit` findings with no upstream fix] -> Add a targeted `overrides` entry and record why in the commit; otherwise document accepted residual risk.
- [Contributors with bun-installed `node_modules`] -> Delete `node_modules` and `bun.lock`, run `npm ci`.
- [Lockfile churn or platform-specific optional deps (e.g. `@rollup`/`esbuild` binaries) missing when generated on a different OS] -> Generate the lockfile with a clean `rm -rf node_modules && npm install`, and confirm `npm ci` passes in CI.

## Migration Plan

1. Branch; delete `bun.lock` and `node_modules`.
2. Edit `package.json` versions, `engines`, overrides; `npm install` to create `package-lock.json`.
3. Fix Kit 3 breakages; run check, lint, build, preview smoke test.
4. Update CI, Justfile, docs; push to `main` for staging deploy and verify.
5. Tag for production only after staging passes.

Rollback: revert the commit (restores `bun.lock` and the old CI); Cloudflare Pages keeps previous deployments for instant rollback.
