<script lang="ts">
	const TOOLS = [
		{
			name: 'Claude',
			logo: '/img/techstack/claude.png',
			bgColor: '#FFFFFF',
			note: 'Claude Code CLI for code generation on a "human-in-the-loop" basis.'
		},
		{
			name: 'GCP',
			logo: '/img/techstack/gcp.png',
			bgColor: '#fff',
			note: 'Hosting SQL Databases and Cloud Run Applications.'
		},
		{
			name: 'Fly.io',
			logo: '/img/techstack/flyio.png',
			bgColor: '#fff',
			note: 'Hosting applications and databases, including PostgreSQL intances and Rust Apps.'
		},
		{
			name: 'Cloudflare',
			logo: '/img/techstack/cloudflare.png',
			bgColor: '#fff',
			note: 'Setup Cloudflare Pages and Workers instances using "workers-rs" and Frontend frameworks.'
		},
		{
			name: 'Docker',
			logo: '/img/techstack/docker.png',
			bgColor: '#fff',
			note: 'Containerization for development and deployment, including Docker Compose.'
		},
		{
			name: 'Git',
			logo: '/img/techstack/git.png',
			bgColor: '#FFFFFF',
			note: 'Preferred VCS, used on both personal and professional projects.'
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
				d="M15.59 14.37a6 6 0 0 1-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 0 0 6.16-12.12A14.98 14.98 0 0 0 9.631 8.41m5.96 5.96a14.926 14.926 0 0 1-5.841 2.58m-.119-8.54a6 6 0 0 0-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 0 0-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 0 1-2.448-2.448 14.9 14.9 0 0 1 .06-.312m-2.24 2.39a4.493 4.493 0 0 0-1.757 4.306 4.493 4.493 0 0 0 4.306-1.758M16.5 9a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z"
				class="fill-zinc-100 stroke-zinc-400 dark:fill-zinc-100/10 dark:stroke-zinc-500"
			/>
		</svg>

		<span class="ml-3">Technologies</span>
	</h2>
	<ol class="mt-6 space-y-4">
		{#each TOOLS as tech (tech.name)}
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
				</dl>
			</li>
		{/each}
	</ol>
</div>
