type Config = {
	theme: 'light' | 'dark';
};

export async function saveConfig(config: Config) {
	await fetch('/api/v0/config', {
		method: 'PATCH',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(config)
	});
}
