<script lang="ts">
	let {
		title,
		date,
		slug
	}: {
		title: string;
		date: string;
		slug: string;
	} = $props();

	const formatted = $derived(
		new Date(date).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			timeZone: 'UTC'
		})
	);
</script>

<a
	href="/notes/{slug}"
	class="group -mx-3 flex items-center gap-6 px-3 py-3 transition-colors hover:bg-neutral-100/60 dark:hover:bg-neutral-900/40"
>
	<time class="w-28 shrink-0 font-mono text-xs text-neutral-500" datetime={date}>{formatted}</time>
	<span class="text-accent truncate text-sm font-medium transition-opacity group-hover:opacity-80"
		>{title}</span
	>
</a>
