# Proposal

## Why

The project is pinned to bun and SvelteKit 2. SvelteKit 3 is released (`@sveltejs/kit@3.1.0`, `@sveltejs/adapter-cloudflare@8.0.1`), and the current `overrides` (`wrangler@4.117.0`, `cookie@^0.7.2`) block it: adapter-cloudflare 8 needs `wrangler ^4.118.0` and Kit 3 uses `cookie` v2. Moving to npm gives a standard lockfile (`package-lock.json`) and `npm audit` for supply-chain checks. Doing both now avoids reworking the lockfile twice.

## What Changes

- Replace bun with npm: delete `bun.lock`, generate and commit `package-lock.json`, update CI workflows (`oven-sh/setup-bun` -> `actions/setup-node`, `bun i` -> `npm ci`, `bun run build` -> `npm run build`), `Justfile`, `CLAUDE.md`, `.prettierignore`.
- Upgrade `@sveltejs/kit` 2 -> 3, `@sveltejs/adapter-cloudflare` 7 -> 8, `typescript` 5 -> 6 (Kit 3 peer `^6.0.0`; TS 7 is out of range), keep `svelte` 5 (bump to latest 5.x, Kit 3 needs >=5.57.1) and `@sveltejs/vite-plugin-svelte` 7.
- Drop obsolete `overrides` (`cookie`, pinned `wrangler`) and let adapter-cloudflare resolve `wrangler`; re-add an override only if `npm audit` shows a fix is needed.
- Bump all other dependencies to latest compatible versions and resolve `npm audit` findings.
- Fix code affected by Kit 3 breaking changes (see design.md), e.g. `$app/stores` -> `$app/state` in `src/routes/+error.svelte`, `svelte.config.js` options, cookie handling.
- Require Node >=22.17 (Kit 3 engine), declared in `package.json` `engines` and CI.

**BREAKING** (tooling only): contributors must use npm and Node >=22.17; bun no longer works for installs.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. No user-visible behavior changes; the change is tooling and dependencies only, so the change sets `skip_specs: true`.

## Impact

- `package.json`, `package-lock.json` (new), `bun.lock` (removed), `.npmrc`
- `svelte.config.js`, `vite.config.ts`, `tsconfig.json`, `eslint.config.js`
- `src/routes/+error.svelte` and any other Kit 3-affected route files
- `.github/workflows/deploy.yml`, `.github/workflows/deploy-staging.yml`
- `Justfile`, `CLAUDE.md`, `README.md`, `.prettierignore`
- Cloudflare Pages build output (`.svelte-kit/cloudflare`) must still deploy via `wrangler pages deploy`.
