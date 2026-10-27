<script lang="ts">
	import { NAV_LINKS, SITE, SOCIAL_LINKS } from '#lib/config/site.ts';
</script>

{#snippet column(title: string)}
	<h3 class="font-mono text-xs tracking-wider text-neutral-500 uppercase">{title}</h3>
{/snippet}

<footer
	class="relative mt-12 w-full border-t border-neutral-200 bg-neutral-50 dark:border-neutral-800 dark:bg-neutral-950"
>
	<div
		class="mx-auto max-w-7xl border-x border-neutral-200 bg-neutral-100/40 px-6 py-12 dark:border-neutral-800 dark:bg-neutral-950/40"
	>
		<div class="grid grid-cols-1 gap-12 md:grid-cols-3">
			<div class="space-y-3">
				{@render column('About')}
				<p class="text-sm text-neutral-500 dark:text-neutral-400">
					{SITE.name} — {SITE.role}. Web Development, Systems Programming and AI Engineering.
				</p>
			</div>

			<div class="space-y-3">
				{@render column('Sitemap')}
				<nav aria-label="Footer" class="flex flex-col space-y-2">
					{#each NAV_LINKS as link (link.href)}
						<a
							href={link.href}
							class="hover:text-accent text-sm text-neutral-500 transition-colors dark:text-neutral-400"
							>{link.label}</a
						>
					{/each}
				</nav>
			</div>

			<div class="space-y-3">
				{@render column('Connect')}
				<nav aria-label="Social" class="flex flex-col space-y-2">
					{#each SOCIAL_LINKS as link (link.label)}
						<a
							href={link.href}
							target="_blank"
							rel="noopener noreferrer"
							class="hover:text-accent inline-flex items-center gap-2 text-sm text-neutral-500 transition-colors dark:text-neutral-400"
						>
							<svg viewBox={link.viewBox} fill="currentColor" class="h-4 w-4" aria-hidden="true"
								><path
									d={link.path}
									fill-rule={link.evenodd ? 'evenodd' : 'nonzero'}
								/>{#each link.layers ?? [] as layer (layer)}<path d={layer} />{/each}</svg
							>
							{link.label}
						</a>
					{/each}
				</nav>
			</div>
		</div>

		<div
			class="mt-12 flex items-center justify-center border-t border-neutral-200 pt-6 dark:border-neutral-800"
		>
			<p class="text-center text-xs text-neutral-500">
				© {new Date().getFullYear()}
				{SITE.name}. All rights reserved.
			</p>
		</div>
	</div>
</footer>
