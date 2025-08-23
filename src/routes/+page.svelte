<script lang="ts">
	import Frameworks from '$lib/components/atoms/Frameworks.svelte';
	import Hero from '$lib/components/atoms/Hero.svelte';
	import LanguageSkills from '$lib/components/atoms/LanguageSkills.svelte';
	import Miscellaneous from '$lib/components/atoms/Miscellaneous.svelte';
	import NoteEntry from '$lib/components/atoms/NoteEntry.svelte';
	import ProgrammingLanguages from '$lib/components/atoms/ProgrammingLanguages.svelte';
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
					<div class="mx-auto max-w-2xl py-8 lg:max-w-5xl lg:py-10">
						<div
							class="top-(--avatar-top,--spacing(3)) w-full"
							style="position:var(--header-inner-position)"
						>
							<div class="relative">
								<div
									class="absolute top-3 left-0 h-10 w-10 origin-left rounded-full bg-white/90 p-0.5 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm transition-opacity dark:bg-zinc-800/90 dark:ring-white/10"
									style="opacity:var(--avatar-border-opacity, 0);transform:var(--avatar-border-transform)"
								></div>
								<a
									aria-label="Home"
									class="pointer-events-auto block h-24 w-24 origin-left"
									style="transform:var(--avatar-image-transform)"
									href="/"
									><img
										alt=""
										fetchpriority="high"
										width="512"
										height="512"
										decoding="async"
										data-nimg="1"
										class="h-24 w-24 rounded-full bg-zinc-100 object-cover dark:bg-zinc-800"
										style="color:transparent"
										sizes="4rem"
										src="/img/whoami.jpg"
									/></a
								>
							</div>
						</div>
					</div>
					<Hero
						title="I'm Leo Borai a Software Developer"
						description="Doing Software Development since 2017. Experience in Web Development, Systems Programming and now learning AI Engineering."
					/>
					<div class="flex items-center justify-start gap-8">
						<SocialNetworks />
						<Miscellaneous />
					</div>
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
						<ProgrammingLanguages />
						<Frameworks />
						<Technologies />
						<LanguageSkills />
						<!-- <Newsletter />
						<Experience /> -->
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
