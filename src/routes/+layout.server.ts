import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, request }) => {
	const config = (cookies.get('config') && JSON.parse(cookies.get('config') as string)) || {};
	const acceptLanguage = request.headers.get('accept-language') || 'en';
	const prefersColorScheme =
		config?.theme || request.headers.get('sec-ch-prefers-color-scheme') || 'dark';

	return {
		acceptLanguage,
		prefersColorScheme
	};
};
