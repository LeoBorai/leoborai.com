/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
	const res = await fetch('/notes/index.json');

	if (res.ok) {
		const notes = await res.json();

		return {
			notes
		};
	}

	return {
		notes: []
	};
}
