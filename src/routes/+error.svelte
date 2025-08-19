<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	let description = 'An unexpected error occurred. Please try again later.';

	onMount(() => {
		if ($page.data && $page.data.error) {
			description = $page.data.error.message || description;
		}
	});
</script>

<div class="mt-9 sm:px-8">
	<div class="mx-auto w-full max-w-7xl lg:px-8">
		<div class="relative px-4 sm:px-8 lg:px-12">
			<div class="mx-auto max-w-2xl lg:max-w-5xl">
				<div class="max-w-2xl">
					<div class="max-w-2xl">
						<h1
							class="text-4xl font-bold tracking-tight text-zinc-800 sm:text-5xl dark:text-zinc-100"
						>
							{$page.error?.message}
						</h1>
						<p class="mt-6 text-base text-zinc-600 dark:text-zinc-400">
							{description}
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
