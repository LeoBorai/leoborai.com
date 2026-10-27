<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		id,
		label,
		href,
		hrefLabel,
		children
	}: {
		id?: string;
		label: string;
		/** Optional "view all" link shown opposite the label */
		href?: string;
		hrefLabel?: string;
		children: Snippet;
	} = $props();
</script>

<section {id} class="scroll-mt-20 py-6">
	<div class="mb-4 flex items-center justify-between">
		<h2 class="font-mono text-[11px] tracking-[0.2em] text-neutral-500 uppercase">{label}</h2>
		{#if href}
			<a
				{href}
				class="hover:text-accent font-mono text-[10px] tracking-wider text-neutral-500 uppercase transition-colors"
				>{hrefLabel ?? 'View all'} →</a
			>
		{/if}
	</div>
	{@render children()}
</section>
