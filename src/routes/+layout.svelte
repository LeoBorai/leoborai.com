<script lang="ts">
	import Footer from '#lib/components/atoms/Footer.svelte';
	import Header from '#lib/components/atoms/Header.svelte';
	import PixelField from '#lib/components/atoms/PixelField.svelte';
	import RailGlow from '#lib/components/atoms/RailGlow.svelte';
	import uiStore from '#lib/stores/uiStore.ts';

	import '../app.css';

	let { children, data } = $props();

	uiStore.syncPreferredScheme(data.prefersColorScheme);
</script>

<svelte:head>
	<title>Leo Borai | Rust Software Engineer</title>
	<meta
		name="description"
		content="Leo Borai is a Rust Software Engineer with expertise in building efficient and reliable software solutions."
	/>
	<meta
		name="keywords"
		content="Rust, Software Engineer, Leo Borai, programming, software development, GitHub"
	/>
	<meta name="author" content="Leo Borai" />
	<meta property="og:title" content="Leo Borai | Rust Software Engineer" />
	<meta
		property="og:description"
		content="Explore the work and experience of Leo Borai, a Rust Software Engineer dedicated to creating high-quality software."
	/>
	<meta property="og:url" content="https://github.com/LeoBorai" />
	<meta property="og:image" content="https://github.com/LeoBorai.png" />
	<meta property="og:type" content="profile" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="Leo Borai | Rust Software Engineer" />
	<meta
		name="twitter:description"
		content="Discover the projects and skills of Leo Borai, a Rust Software Engineer."
	/>
	<meta name="twitter:image" content="https://github.com/LeoBorai.png" />
	<meta name="viewport" content="width=device-width, initial-scale=1.0" />
	<link rel="canonical" href="https://github.com/LeoBorai" />
	<link rel="icon" href="/favicon.ico" type="image/x-icon" />
</svelte:head>

<div class="dotted-background fixed inset-0 -z-10" aria-hidden="true"></div>
<div
	class="pointer-events-none fixed inset-0 z-0 hidden [mask-image:linear-gradient(#000,transparent_min(95vh,900px))] xl:block"
	aria-hidden="true"
>
	<div
		class="absolute inset-y-0 left-0 w-[calc(50%-40rem+1px)] border-r border-neutral-200 dark:border-neutral-800"
	></div>
	<div
		class="absolute inset-y-0 right-0 w-[calc(50%-40rem+1px)] border-l border-neutral-200 dark:border-neutral-800"
	></div>
</div>

<PixelField />

<RailGlow />

<div class="relative min-h-screen text-neutral-900 dark:text-neutral-100">
	<Header />
	<main class="relative z-10 pt-20">
		{@render children()}
	</main>
	<Footer />
</div>
