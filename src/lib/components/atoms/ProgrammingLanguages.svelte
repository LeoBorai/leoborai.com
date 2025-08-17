<script lang="ts">
	const TECHNOLOGIES = [
		{
			name: 'Rust',
			since: 2021,
			logo: '/img/techstack/rust.png',
			bgColor: '#fff',
			note: 'HTTP Servers, CLIs, Front-End Development with Leptos and contributions.',
			isFullStack: true
		},
		{
			name: 'Python',
			since: 2020,
			logo: '/img/techstack/python.png',
			bgColor: '#fff',
			note: 'Django and Scripting Experience. Also on HuggingFace courses.',
			isFullStack: true
		},
		{
			name: 'SQL',
			since: 2019,
			logo: '/img/techstack/sql.png',
			bgColor: '#eee',
			note: 'Used SQL on databases like PostgreSQL and SQLite. With libraries like SQLx, SeaORM, TypeORM and Knex.js.',
			isFullStack: true
		},
		{
			name: 'TypeScript',
			since: 2017,
			logo: '/img/techstack/typescript.png',
			bgColor: '#3178c6',
			note: 'HTTP Servers and Web Applications with React, Next.js, and SvelteKit.',
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
				d="M17.25 6.75 22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3-4.5 16.5"
				class="fill-zinc-100 stroke-zinc-400 dark:fill-zinc-100/10 dark:stroke-zinc-500"
			/>
		</svg>

		<span class="ml-3">Programming Languages</span>
	</h2>
	<ol class="mt-6 space-y-4">
		{#each TECHNOLOGIES as tech (tech.name)}
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
