<script lang="ts">
	import { page } from '$app/stores';

	import '../app.css';

	let { children } = $props();
	const sections = [
		{ name: 'Inicio', link: '/' },
		{ name: 'Bio', link: '/bio' },
		{ name: 'Notas', link: '/notes' },
		{ name: 'Contacto', link: '/contact' }
	];
</script>

<div class="flex min-h-screen bg-gray-50 sm:grid sm:grid-cols-[10%_auto_10%] sm:p-8">
	<div
		class="mx-auto min-h-screen w-full overflow-hidden bg-white sm:col-start-2 sm:col-end-3 sm:min-h-auto sm:rounded-md sm:shadow-md"
	>
		<header
			class="relative flex h-60 items-center justify-center bg-cover bg-center bg-no-repeat"
			style="background-image: url('/img/heading.jpg');"
		>
			<figure
				class="absolute bottom-0 -mb-6 size-32 overflow-hidden rounded-md border-6 border-white sm:-mb-10"
			>
				<img alt="A portrait of myself" src="/img/whoami.jpeg" class="rounded-md" />
			</figure>
		</header>
		<article class="flex flex-col items-center justify-center pt-10 font-semibold sm:pt-14">
			<h1 class="text-xl font-semibold text-gray-800">Leo Borai</h1>
		</article>
		<ul
			class="flex flex-col items-center justify-center pt-4 text-xs text-gray-600 sm:flex-row sm:space-x-4"
		>
			<li>LinkedIn</li>
			<li>Bluesky</li>
			<li>GitHub</li>
			<li>HuggingFace</li>
			<li>StackOverflow</li>
		</ul>
		<nav class="flex justify-center p-4">
			{#each sections as section}
				<a
					class="mx-4 text-xs text-gray-600 uppercase transition-colors duration-300 hover:underline"
					class:underline={section.link === $page.url.pathname}
					href={section.link}
				>
					{section.name}
				</a>
			{/each}
		</nav>
		<main class="px-4 py-6 sm:px-6 sm:py-9">
			{@render children()}
		</main>
	</div>
</div>
