<script lang="ts">
	export let data: { notes: object[] };
</script>

<div class="flex flex-col items-center justify-center font-semibold">
	{#each data.notes as note}
		<div class="mt-4 w-full max-w-2xl rounded-md border p-4 shadow-sm">
			<h2 class="text-lg font-semibold text-gray-800">{note.title}</h2>
			<p class="text-sm text-gray-600">{note.content}</p>
			<span class="text-xs text-gray-400">{note.meta.title}</span>
		</div>
	{/each}
</div>
