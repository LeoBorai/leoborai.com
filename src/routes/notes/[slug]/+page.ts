import { error } from '@sveltejs/kit';

export async function load({ params: { slug } }) {
	try {
		const post = await import(`../../../notes/${slug}.svx`);

		return {
			metadata: post.metadata,
			content: post.default
		};
	} catch (error) {
		// Failed to find the requested language, try to find a fallback language
	}

	throw error(404, 'Not found');
}
