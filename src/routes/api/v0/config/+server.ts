import { json } from '@sveltejs/kit';

export async function PATCH({ request, cookies }) {
	const nextConfig = await request.json();

	let config = (cookies.get('config') && JSON.parse(cookies.get('config') as string)) || {};

	config = {
		...config,
		theme: nextConfig?.theme
	};

	cookies.set('config', JSON.stringify(config), {
		path: '/'
	});

	return json(config);
}
