<script lang="ts">
	let { children }: { children: import('svelte').Snippet } = $props();
</script>

<span
	class="inline-flex items-center gap-2 border border-green-600/40 bg-green-600/5 px-3 py-1 font-mono text-xs tracking-wider text-green-700 uppercase dark:border-green-500/30 dark:bg-green-500/5 dark:text-green-500"
>
	<span class="relative flex h-2 w-2" aria-hidden="true">
		<span
			class="absolute inline-flex h-full w-full animate-ping rounded-full bg-green-500 opacity-75"
		></span>
		<span class="relative inline-flex h-2 w-2 rounded-full bg-green-600 dark:bg-green-500"></span>
	</span>
	{@render children()}
</span>
