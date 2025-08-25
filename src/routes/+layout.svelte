<script lang="ts">
	import Footer from '$lib/components/atoms/Footer.svelte';
	import Navigation from '$lib/components/atoms/Navigation.svelte';
	import ThemeToggle from '$lib/components/atoms/ThemeToggle.svelte';
	import uiStore from '$lib/stores/uiStore';

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

<div class="flex w-full">
	<div class="fixed inset-0 flex justify-center sm:px-8">
		<div class="flex w-full max-w-7xl lg:px-8">
			<div
				class="w-full bg-white ring-1 ring-zinc-100 dark:bg-zinc-900 dark:ring-zinc-300/20"
			></div>
		</div>
	</div>
	<div class="relative flex w-full flex-col">
		<header
			class="pointer-events-none relative z-50 flex flex-none flex-col"
			style="height:var(--header-height);margin-bottom:var(--header-mb)"
		>
			<div class="order-last mt-[calc(--spacing(16)-(--spacing(3)))]"></div>
			<div class="top-0 z-10 h-16 pt-6" style="position:var(--header-position)">
				<div
					class="top-(--header-top,--spacing(6)) w-full sm:px-8"
					style="position:var(--header-inner-position)"
				>
					<div class="mx-auto w-full max-w-7xl lg:px-8">
						<div class="relative px-4 sm:px-8 lg:px-12">
							<div class="mx-auto max-w-2xl lg:max-w-5xl">
								<div class="relative flex gap-4">
									<div class="flex flex-1"></div>
									<div class="flex flex-1 justify-end md:justify-center">
										<Navigation />
									</div>
									<div class="flex justify-end md:flex-1">
										<div class="pointer-events-auto">
											<ThemeToggle />
										</div>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</header>
		<div class="flex-none" style="height:var(--content-offset)"></div>
		<main class="flex-auto">
			{@render children()}
		</main>
		<Footer />
	</div>
</div>
