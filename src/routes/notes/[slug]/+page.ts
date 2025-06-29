import { error } from '@sveltejs/kit';

export async function load({ params }) {
	try {
		const post = await import(`../../../notes/${params.slug}.svx`);
		const content = post.default;

		return {
			content
		};
	} catch {
		// Failed to find the requested language, try to find a fallback language
	}

	throw error(404, 'Not found');
}
