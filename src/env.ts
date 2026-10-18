import { defineEnvVars } from '@sveltejs/kit/env';

export const variables = defineEnvVars({
	TELEGRAM_BOT: { static: true },
	TELEGRAM_CHAT: { static: true }
});
