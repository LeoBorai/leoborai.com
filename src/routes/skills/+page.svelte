<script lang="ts">
	import Hero from '$lib/components/atoms/Hero.svelte';

	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<div class="mt-9 sm:px-8">
	<div class="mx-auto w-full max-w-7xl lg:px-8">
		<div class="relative px-4 sm:px-8 lg:px-12">
			<div class="mx-auto max-w-2xl lg:max-w-5xl">
				<Hero
					title="Skills"
					description="AI Agent Skills I've built to extend how coding agents work."
				/>
				<div class="mt-16 sm:mt-20">
					<div class="md:border-l md:border-zinc-100 md:pl-6 md:dark:border-zinc-700/40">
						<div class="flex max-w-3xl flex-col space-y-16">
							{#each data.skills as skill (skill.slug)}
								<article class="flex flex-col items-start">
									<div class="group relative flex flex-col items-start">
										<h2
											class="text-base font-semibold tracking-tight text-zinc-800 dark:text-zinc-100"
										>
											<div
												class="absolute -inset-x-4 -inset-y-6 z-0 scale-95 bg-zinc-50 opacity-0 transition group-hover:scale-100 group-hover:opacity-100 sm:-inset-x-6 sm:rounded-2xl dark:bg-zinc-800/50"
											></div>
											<a href="/skills/{skill.slug}"
												><span
													class="absolute -inset-x-4 -inset-y-6 z-20 sm:-inset-x-6 sm:rounded-2xl"
												></span><span class="relative z-10">{skill.meta.name}</span></a
											>
										</h2>
										<p class="relative z-10 mt-2 text-sm text-zinc-600 dark:text-zinc-400">
											{skill.meta.description}
										</p>
										<div
											aria-hidden="true"
											class="relative z-10 mt-4 flex items-center text-sm font-medium text-emerald-500"
										>
											Read skill<svg
												viewBox="0 0 16 16"
												fill="none"
												aria-hidden="true"
												class="ml-1 h-4 w-4 stroke-current"
												><path
													d="M6.75 5.75 9.25 8l-2.5 2.25"
													stroke-width="1.5"
													stroke-linecap="round"
													stroke-linejoin="round"
												></path></svg
											>
										</div>
									</div>
								</article>
							{/each}
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
