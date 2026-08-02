<script lang="ts">
	let { content }: { content: string } = $props();

	let copied = $state(false);

	async function copy() {
		await navigator.clipboard.writeText(content);
		copied = true;
		setTimeout(() => (copied = false), 2000);
	}
</script>

<button
	type="button"
	class="group flex cursor-pointer items-center gap-2 rounded-full bg-white/90 px-3 py-2 text-sm font-medium text-zinc-800 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm transition hover:text-emerald-500 dark:bg-zinc-800/90 dark:text-zinc-200 dark:ring-white/10 dark:hover:text-emerald-400 dark:hover:ring-white/20"
	onclick={copy}
>
	<svg viewBox="0 0 16 16" fill="none" aria-hidden="true" class="h-4 w-4 stroke-current"
		><path
			d="M5.75 5.75h5.5a1 1 0 0 1 1 1v5.5a1 1 0 0 1-1 1h-5.5a1 1 0 0 1-1-1v-5.5a1 1 0 0 1 1-1Z"
			stroke-width="1.5"
			stroke-linecap="round"
			stroke-linejoin="round"
		></path><path
			d="M3.75 9.75h-1a1 1 0 0 1-1-1v-5.5a1 1 0 0 1 1-1h5.5a1 1 0 0 1 1 1v1"
			stroke-width="1.5"
			stroke-linecap="round"
			stroke-linejoin="round"
		></path></svg
	>{copied ? 'Copied!' : 'Copy as MD'}</button
>
