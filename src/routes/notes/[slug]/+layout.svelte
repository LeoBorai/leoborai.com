<script lang="ts">
	let { children } = $props();
</script>

<main class="flex flex-col">
	{@render children()}
</main>
