import { error } from '@sveltejs/kit';

export async function load({ params: { slug } }) {
	try {
		const post = await import(`../../../skills/${slug}.svx`);
		const raw = await import(`../../../skills/${slug}.svx?raw`);
		const markdown = raw.default.replace(/^---\r?\n[\s\S]*?\r?\n---\r?\n/, '');

		return {
			metadata: post.metadata,
			content: post.default,
			markdown
		};
	} catch {
		// Failed to find the requested skill
	}

	throw error(404, 'Not found');
}
