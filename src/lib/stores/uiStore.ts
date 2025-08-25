import { saveConfig } from '$lib/utils/config';
import { writable, type Readable } from 'svelte/store';

const PREFERS_COLOR_SCHEME_DARK_MEDIA_QUERY = '(prefers-color-scheme: dark)';

export type UIStoreMethods = {
	setDarkColorScheme(): void;
	setLightColorScheme(): void;
	syncPreferredScheme(prefer?: string): void;
};

export enum ColorScheme {
	Dark = 'dark',
	Light = 'light'
}

export type UIStore = {
	colorScheme: ColorScheme;
};

/**
 * Returns the preferred color scheme based on user's operative system
 */
export function getPreferredScheme(): ColorScheme {
	if (typeof window !== 'undefined') {
		return window?.matchMedia?.(PREFERS_COLOR_SCHEME_DARK_MEDIA_QUERY)?.matches
			? ColorScheme.Dark
			: ColorScheme.Light;
	}

	return ColorScheme.Dark;
}

export function createUIStore() {
	const { subscribe, update } = writable({
		isSidebarOpen: false,
		colorScheme: getPreferredScheme()
	});

	const syncPreferredScheme = (prefer: string | undefined) => {
		if (prefer === ColorScheme.Dark) {
			setDarkColorScheme();
			return;
		}

		if (prefer === ColorScheme.Light) {
			setLightColorScheme();
			return;
		}

		const preferredScheme = getPreferredScheme();

		if (preferredScheme === ColorScheme.Dark) {
			setDarkColorScheme();
		} else {
			setLightColorScheme();
		}
	};

	const setDarkColorScheme = () => {
		if (typeof document !== 'undefined') {
			document.documentElement.classList.add('dark');

			saveConfig({
				theme: ColorScheme.Dark
			});

			update((current) => ({
				...current,
				colorScheme: ColorScheme.Dark
			}));
		}
	};

	const setLightColorScheme = () => {
		if (typeof document !== 'undefined') {
			document.documentElement.classList.remove('dark');

			saveConfig({
				theme: ColorScheme.Light
			});

			update((current) => ({
				...current,
				colorScheme: ColorScheme.Light
			}));
		}
	};

	return {
		subscribe,
		setDarkColorScheme,
		setLightColorScheme,
		syncPreferredScheme
	};
}

const uiStore = createUIStore() as unknown as Readable<UIStore> & UIStoreMethods;
export default uiStore;
