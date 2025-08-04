<script lang="ts">
	import Hero from '$lib/components/atoms/Hero.svelte';
	import NoteEntry from '$lib/components/atoms/NoteEntry.svelte';
	import SocialNetworks from '$lib/components/atoms/SocialNetworks.svelte';
	import Technologies from '$lib/components/atoms/Technologies.svelte';

	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<div class="mt-9 sm:px-8">
	<div class="mx-auto w-full max-w-7xl lg:px-8">
		<div class="relative px-4 sm:px-8 lg:px-12">
			<div class="mx-auto max-w-2xl lg:max-w-5xl">
				<div class="max-w-2xl">
					<Hero
						title="Rust Software Developer doing AI Engineering &amp; Open-Source"
						description="I’m a software developer focused on Rust programming, AI integrations, and open-source projects. I enjoy building efficient and scalable systems that leverage the power of Rust and AI technologies."
					/>
					<SocialNetworks />
				</div>
			</div>
		</div>
	</div>
</div>

<div class="mt-24 sm:px-8 md:mt-28">
	<div class="mx-auto w-full max-w-7xl lg:px-8">
		<div class="relative px-4 sm:px-8 lg:px-12">
			<div class="mx-auto max-w-2xl lg:max-w-5xl">
				<div class="mx-auto grid max-w-xl grid-cols-1 gap-y-20 lg:max-w-none lg:grid-cols-2">
					<div class="flex flex-col gap-16">
						<h2 class="flex text-lg font-semibold text-zinc-900 dark:text-zinc-100">
							Recent Notes
						</h2>
						{#each data.notes as note (note.slug)}
							<NoteEntry
								title={note.meta.title}
								description={note.meta.description}
								date={note.meta.date}
								slug={note.slug}
							/>
						{/each}
					</div>
					<div class="space-y-10 lg:pl-16 xl:pl-24">
						<Technologies />
						<!-- <Newsletter />
						<Experience /> -->
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
