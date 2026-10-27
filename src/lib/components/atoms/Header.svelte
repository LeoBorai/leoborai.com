<script lang="ts">
	import { page } from '$app/state';
	import { NAV_LINKS, SITE } from '#lib/config/site.ts';

	import ThemeToggle from './ThemeToggle.svelte';

	let menuOpen = $state(false);

	const isActive = (href: string) =>
		href === '/' ? page.url.pathname === '/' : page.url.pathname.startsWith(href);

	const dir = (href: string) => (href === '/' ? '~' : `~${href}/`);
</script>

{#snippet brand()}
	<a href="/" class="flex items-center font-mono text-sm font-semibold tracking-tight">
		<span class="text-accent">sys@leo</span><span
			class="text-neutral-400">:</span
		><span class="text-neutral-500">~</span><span class="text-neutral-400">$</span>
	</a>
{/snippet}

<header class="fixed inset-x-0 top-0 z-50 px-3 pt-3 font-mono sm:px-4">
	<div
		class="mx-auto max-w-5xl rounded-2xl border border-neutral-200/80 bg-white/70 shadow-lg shadow-neutral-900/5 backdrop-blur-xl dark:border-neutral-800/80 dark:bg-neutral-950/70 dark:shadow-black/30"
	>
		<div class="hidden h-14 items-center gap-2 px-3 md:flex">
			<div class="flex items-center rounded-xl bg-neutral-100 px-3 py-1.5 dark:bg-neutral-900">
				{@render brand()}
			</div>
			<nav class="flex flex-1 items-center justify-end gap-1" aria-label="Primary">
				{#each NAV_LINKS as link (link.href)}
					<a
						href={link.href}
						aria-current={isActive(link.href) ? 'page' : undefined}
						class="rounded-lg px-3 py-1.5 font-mono text-xs whitespace-nowrap transition-colors {isActive(
							link.href
						)
							? 'text-accent bg-accent/10 ring-accent/20 ring-1'
							: 'text-neutral-500 hover:bg-neutral-100 hover:text-neutral-900 dark:text-neutral-400 dark:hover:bg-neutral-800/60 dark:hover:text-neutral-100'}"
						>{dir(link.href)}</a
					>
				{/each}
			</nav>
			<ThemeToggle />
		</div>

		<div class="flex h-14 items-center justify-between px-4 md:hidden">
			{@render brand()}
			<div class="flex h-full items-center">
				<ThemeToggle />
				<button
					type="button"
					class="flex h-8 w-8 cursor-pointer items-center justify-center text-neutral-500 hover:text-neutral-900 dark:hover:text-neutral-100"
					aria-expanded={menuOpen}
					aria-controls="mobile-menu"
					aria-label={menuOpen ? 'Close menu' : 'Open menu'}
					onclick={() => (menuOpen = !menuOpen)}
				>
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="1.5"
						class="h-4 w-4"
						aria-hidden="true"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d={menuOpen ? 'M6 18 18 6M6 6l12 12' : 'M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5'}
						/>
					</svg>
				</button>
			</div>
		</div>

		{#if menuOpen}
			<nav
				id="mobile-menu"
				aria-label="Mobile"
				class="flex flex-col gap-1 border-t border-neutral-200/80 p-2 md:hidden dark:border-neutral-800/80"
			>
				{#each NAV_LINKS as link (link.href)}
					<a
						href={link.href}
						onclick={() => (menuOpen = false)}
						aria-current={isActive(link.href) ? 'page' : undefined}
						class="rounded-lg px-4 py-2.5 font-mono text-xs {isActive(link.href)
							? 'text-accent bg-accent/5'
							: 'text-neutral-500 dark:text-neutral-400'}">{dir(link.href)}</a
					>
				{/each}
			</nav>
		{/if}
	</div>
</header>
