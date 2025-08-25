<script lang="ts">
	const FRAMEWORKS = [
		{
			name: 'Svelte/Kit',
			since: 2021,
			logo: '/img/techstack/sveltekit.png',
			bgColor: '#FFFFFF',
			note: 'Web Applications with SveletKit running on Cloudflare Workers',
			isFullStack: true
		},
		{
			name: 'NextJS',
			since: 2019,
			logo: '/img/techstack/next.png',
			bgColor: '#FFFFFF',
			note: 'Full-Stack Web Applications with NextJS (ReactJS SSR/SSG Framework)',
			isFullStack: true
		},
		{
			name: 'EmberJS',
			since: 2018,
			logo: '/img/techstack/emberjs.png',
			bgColor: '#FFFFFF',
			note: 'Develop Web Applications with EmbjerJS using Ember Data and Ember CLI',
			isFullStack: true
		},
		{
			name: 'ReactJS',
			since: 2017,
			logo: '/img/techstack/reactjs.webp',
			bgColor: '#222222',
			note: 'Worked on building Web Applications with React as Single Page App and also Full-Stack with NextJS',
			isFullStack: true
		}
	];
</script>

<div class="rounded-2xl border border-zinc-100 p-6 dark:border-zinc-700/40">
	<h2 class="flex text-sm font-semibold text-zinc-900 dark:text-zinc-100">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="1.5"
			stroke="currentColor"
			class="h-6 w-6 flex-none"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="M11.42 15.17 17.25 21A2.652 2.652 0 0 0 21 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 1 1-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 0 0 4.486-6.336l-3.276 3.277a3.004 3.004 0 0 1-2.25-2.25l3.276-3.276a4.5 4.5 0 0 0-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085m-1.745 1.437L5.909 7.5H4.5L2.25 3.75l1.5-1.5L7.5 4.5v1.409l4.26 4.26m-1.745 1.437 1.745-1.437m6.615 8.206L15.75 15.75M4.867 19.125h.008v.008h-.008v-.008Z"
				class="fill-zinc-100 stroke-zinc-400 dark:fill-zinc-100/10 dark:stroke-zinc-500"
			/>
		</svg>

		<span class="ml-3">Frameworks</span>
	</h2>
	<ol class="mt-6 space-y-4">
		{#each FRAMEWORKS as tech (tech.name)}
			<li class="flex gap-4">
				<div
					class="relative mt-1 flex h-10 w-10 flex-none items-center justify-center rounded-full shadow-md ring-1 shadow-zinc-800/5 ring-zinc-900/5 dark:border dark:border-zinc-700/50 dark:bg-zinc-800 dark:ring-0"
					style="background-color:{tech.bgColor}"
				>
					<img
						alt=""
						loading="lazy"
						width="32"
						height="32"
						decoding="async"
						data-nimg="1"
						class="h-7 w-7"
						style="background-color:{tech.bgColor}"
						src={tech.logo}
					/>
				</div>
				<dl class="flex flex-auto flex-wrap space-y-2 gap-x-2">
					<dt class="sr-only">{tech.name}</dt>
					<dd class="w-full flex-none text-sm font-medium text-zinc-900 dark:text-zinc-100">
						{tech.name}
					</dd>
					<dt class="sr-only">Description</dt>
					<dd class="text-xs text-zinc-500 dark:text-zinc-400">{tech.note}</dd>
					<dt class="sr-only">Date</dt>
					<dd
						class="ml-auto text-xs text-zinc-400 dark:text-zinc-500"
						aria-label="2019 until Present"
					>
						<time datetime={tech.since.toString()}>{tech.since}</time>
						<span aria-hidden="true">—</span>
						<span>{new Date().getFullYear() - tech.since} yoe.</span>
					</dd>
				</dl>
			</li>
		{/each}
	</ol>
</div>
