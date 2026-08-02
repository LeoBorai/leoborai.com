import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const res = await fetch('/skills/index.json');

	if (res.ok) {
		const skills: Domain.Skill[] = await res.json();

		skills.sort((a, b) => a.meta.name.localeCompare(b.meta.name));

		return {
			skills
		};
	}

	return {
		skills: []
	};
};
