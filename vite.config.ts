import tailwindcss from '@tailwindcss/vite';
import adapter from '@sveltejs/adapter-cloudflare';
import { sveltekit } from '@sveltejs/kit/vite';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { mdsvex } from 'mdsvex';
import { createHighlighter } from 'shiki';
import { defineConfig } from 'vite';

const LANGS = [
	'bash',
	'diff',
	'dockerfile',
	'go',
	'graphql',
	'html',
	'javascript',
	'json',
	'powershell',
	'rust',
	'sql',
	'svelte',
	'toml',
	'typescript',
	'yaml'
];

const ALIASES: Record<string, string> = { js: 'javascript', ts: 'typescript', yml: 'yaml' };

const highlighter = await createHighlighter({
	themes: ['github-light', 'github-dark'],
	langs: LANGS
});

/** Build-time highlighting; light/dark colors are CSS variables switched in app.css. */
const highlight = (code: string, lang?: string) => {
	const resolved = ALIASES[lang ?? ''] ?? lang ?? '';
	const html = highlighter.codeToHtml(code, {
		lang: LANGS.includes(resolved) ? resolved : 'text',
		themes: { light: 'github-light', dark: 'github-dark' },
		defaultColor: false
	});
	// mdsvex emits Svelte markup: escape characters that are special in a template literal
	const escaped = html.replace(/[\\`$]/g, (c) => `\\${c}`);
	return `{@html \`${escaped}\`}`;
};

export default defineConfig({
	plugins: [
		tailwindcss(),
		sveltekit({
			preprocess: [vitePreprocess(), mdsvex({ highlight: { highlighter: highlight } })],
			extensions: ['.svelte', '.svx'],
			adapter: adapter()
		})
	]
});
