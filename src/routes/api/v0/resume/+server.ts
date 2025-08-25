import { TELEGRAM_BOT, TELEGRAM_CHAT } from '$env/static/private';

export async function POST({ fetch, request }) {
	const API_URL = new URL(`https://api.telegram.org/bot${TELEGRAM_BOT}/sendMessage`);
	const { email } = await request.json();
	const MESSAGE = `<b>Resume Request</b>
📧&nbsp;<b>Email</b>: ${email}`;

	await fetch(API_URL, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			chat_id: TELEGRAM_CHAT,
			text: MESSAGE,
			parse_mode: 'HTML'
		})
	});
}
