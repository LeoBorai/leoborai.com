// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	namespace Domain {
		type Note = {
			meta: {
				title: string;
				description: string;
				icon: string;
				date: string;
				preview_image_url: string;
				published: boolean;
				categories: string[];
			};
			slug: string;
		};
	}
}

export {};
