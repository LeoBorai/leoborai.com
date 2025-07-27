<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const Component = $derived(data.content);
</script>

<div class="mt-9 sm:px-8">
	<div class="mx-auto w-full max-w-7xl lg:px-8">
		<div class="relative px-4 sm:px-8 lg:px-12">
			<div class="mx-auto max-w-2xl lg:max-w-5xl">
				<div class="xl:relative">
					<div class="mx-auto max-w-2xl">
						<a
							aria-label="Go back to articles"
							class="group mb-8 flex h-10 w-10 items-center justify-center rounded-full bg-white shadow-md ring-1 shadow-zinc-800/5 ring-zinc-900/5 transition lg:absolute lg:-left-5 lg:-mt-2 lg:mb-0 xl:-top-1.5 xl:left-0 xl:mt-0 dark:border dark:border-zinc-700/50 dark:bg-zinc-800 dark:ring-0 dark:ring-white/10 dark:hover:border-zinc-700 dark:hover:ring-white/20"
							href="/notes"
						>
							<svg
								viewBox="0 0 16 16"
								fill="none"
								aria-hidden="true"
								class="h-4 w-4 stroke-zinc-500 transition group-hover:stroke-zinc-700 dark:stroke-zinc-500 dark:group-hover:stroke-zinc-400"
							>
								<path
									d="M7.25 11.25 3.75 8m0 0 3.5-3.25M3.75 8h8.5"
									stroke-width="1.5"
									stroke-linecap="round"
									stroke-linejoin="round"
								></path>
							</svg>
						</a>
						<article>
							<header class="flex flex-col">
								<h1
									class="mt-6 text-4xl font-bold tracking-tight text-zinc-800 sm:text-5xl dark:text-zinc-100"
								>
									{data.metadata.title}
								</h1>
								<time
									datetime={data.metadata.date}
									class="order-first flex items-center text-base text-zinc-400 dark:text-zinc-500"
								>
									<span class="h-4 w-0.5 rounded-full bg-zinc-200 dark:bg-zinc-500"></span>
									<span class="ml-3">{new Date(data.metadata.date).toDateString()}</span>
								</time>
							</header>
							<div class="prose dark:prose-invert mt-8" data-mdx-content="true">
								<Component />
							</div>
						</article>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
