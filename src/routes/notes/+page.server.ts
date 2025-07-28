import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const res = await fetch('/notes/index.json');

	if (res.ok) {
		const notes: Domain.Note[] = await res.json();

		notes.sort((a, b) => new Date(b.meta.date).getTime() - new Date(a.meta.date).getTime());

		return {
			notes
		};
	}

	return {
		notes: []
	};
};
