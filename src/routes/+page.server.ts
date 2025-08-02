import { GH_API_TOKEN } from '$env/static/private';
import { GitHub, type FetchFn } from '$lib/services/GitHub';

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const res = await fetch('/notes/index.json');
	const contributionsCalendar = fetchContributionsCalendar(fetch);

	if (res.ok) {
		const notes: Domain.Note[] = await res.json();

		notes.sort((a, b) => new Date(b.meta.date).getTime() - new Date(a.meta.date).getTime());
		notes.length = Math.min(notes.length, 5);

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

async function fetchContributionsCalendar(fetch: FetchFn) {
	const ghService = new GitHub(fetch, GH_API_TOKEN);

	try {
		return await ghService.getContributionsCalendar('LeoBorai');
	} catch (error) {
		console.error('Error fetching contributions calendar:', error);
		return {
			totalContributions: 0,
			contributions: []
		};
	}
}
