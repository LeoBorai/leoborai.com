<script lang="ts">
	import Footer from '$lib/components/atoms/Footer.svelte';
	import Navigation from '$lib/components/atoms/Navigation.svelte';

	import '../app.css';

	let { children } = $props();
</script>

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
			<div class="top-0 order-last -mb-3 pt-3 sm:px-8" style="position:var(--header-position)">
				<div class="mx-auto w-full max-w-7xl lg:px-8">
					<div class="relative px-4 sm:px-8 lg:px-12">
						<div class="mx-auto max-w-2xl lg:max-w-5xl">
							<div
								class="top-(--avatar-top,--spacing(3)) w-full"
								style="position:var(--header-inner-position)"
							>
								<div class="relative">
									<div
										class="absolute top-3 left-0 h-10 w-10 origin-left rounded-full bg-white/90 p-0.5 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm transition-opacity dark:bg-zinc-800/90 dark:ring-white/10"
										style="opacity:var(--avatar-border-opacity, 0);transform:var(--avatar-border-transform)"
									></div>
									<a
										aria-label="Home"
										class="pointer-events-auto block h-16 w-16 origin-left"
										style="transform:var(--avatar-image-transform)"
										href="/"
										><img
											alt=""
											fetchpriority="high"
											width="512"
											height="512"
											decoding="async"
											data-nimg="1"
											class="h-16 w-16 rounded-full bg-zinc-100 object-cover dark:bg-zinc-800"
											style="color:transparent"
											sizes="4rem"
											src="/img/whoami.jpg"
										/></a
									>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
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
											<!-- <ThemeToggle /> -->
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
