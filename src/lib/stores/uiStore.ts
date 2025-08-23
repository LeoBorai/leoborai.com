import { writable, type Readable } from 'svelte/store';

const PREFERS_COLOR_SCHEME_DARK_MEDIA_QUERY = '(prefers-color-scheme: dark)';

export type UIStoreMethods = {
	setDarkColorScheme(): void;
	setLightColorScheme(): void;
	syncPreferredScheme(): void;
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
		const cached = localStorage.getItem('preferredColorScheme');

		if (cached) {
			return cached === ColorScheme.Dark ? ColorScheme.Dark : ColorScheme.Light;
		}

		return window?.matchMedia?.(PREFERS_COLOR_SCHEME_DARK_MEDIA_QUERY)?.matches
			? ColorScheme.Dark
			: ColorScheme.Light;
	}

	return ColorScheme.Light;
}

export function createUIStore() {
	const { subscribe, update } = writable({
		isSidebarOpen: false,
		colorScheme: getPreferredScheme()
	});

	const syncPreferredScheme = () => {
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
			localStorage.setItem('preferredColorScheme', ColorScheme.Dark);

			update((current) => ({
				...current,
				colorScheme: ColorScheme.Dark
			}));
		}
	};

	const setLightColorScheme = () => {
		if (typeof document !== 'undefined') {
			document.documentElement.classList.remove('dark');
			localStorage.setItem('preferredColorScheme', ColorScheme.Light);

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
