import { error } from '@sveltejs/kit';

export async function load({ params: { slug } }) {
	try {
		const post = await import(`../../../skills/${slug}.svx`);

		return {
			metadata: post.metadata,
			content: post.default
		};
	} catch {
		// Failed to find the requested skill
	}

	throw error(404, 'Not found');
}
