import { GH_API_TOKEN } from '$env/static/private';
import { GitHub } from '$lib/services/GitHub';

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const res = await fetch('/notes/index.json');
	const ghService = new GitHub(fetch, GH_API_TOKEN);

	if (res.ok) {
		const notes: Domain.Note[] = await res.json();

		notes.sort((a, b) => new Date(b.meta.date).getTime() - new Date(a.meta.date).getTime());
		notes.length = Math.min(notes.length, 5);

		const contributionsCalendar = await ghService.getContributionsCalendar('LeoBorai');

		return {
			notes,
			contributionsCalendar
		};
	}

	return {
		notes: [],
		contributionsCalendar: {
			totalContributions: 0,
			contributions: []
		}
	};
};
