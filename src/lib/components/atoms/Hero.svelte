<script lang="ts">
	import Container from './Container.svelte';

	let { title, description }: { title: string; description: string } = $props();
</script>

<Container class="py-10">
	<h1 class="text-3xl font-bold text-neutral-900 sm:text-4xl dark:text-neutral-100">{title}</h1>
	<p class="mt-4 max-w-2xl text-neutral-600 dark:text-neutral-400">{description}</p>
</Container>
