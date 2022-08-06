import adapter from '@sveltejs/adapter-cloudflare';
import { mdsvex } from 'mdsvex';
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
export default {
  extensions: ['.svelte', '.svx'],
  preprocess: [
    preprocess({
      sourceMap: true,
      postcss: true,
    }),
    mdsvex({
      layout: './src/routes/notes/_components/Layout.svelte',
    }),
  ],
  kit: {
    adapter: adapter(),
  },
};
