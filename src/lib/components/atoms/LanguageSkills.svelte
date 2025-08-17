<script lang="ts">
	const LANGUAGES = [
		{
			name: 'Spanish',
			level: 'Native',
			flag: '/img/languages/spain.png'
		},
		{
			name: 'English',
			level: 'Fluent',
			flag: '/img/languages/usa.png'
		},
		{
			name: 'Hungarian',
			level: 'Beginner',
			flag: '/img/languages/hungary.png'
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
				d="m10.5 21 5.25-11.25L21 21m-9-3h7.5M3 5.621a48.474 48.474 0 0 1 6-.371m0 0c1.12 0 2.233.038 3.334.114M9 5.25V3m3.334 2.364C11.176 10.658 7.69 15.08 3 17.502m9.334-12.138c.896.061 1.785.147 2.666.257m-4.589 8.495a18.023 18.023 0 0 1-3.827-5.802"
				class="fill-zinc-100 stroke-zinc-400 dark:fill-zinc-100/10 dark:stroke-zinc-500"
			/>
		</svg>

		<span class="ml-3">Language Skills</span>
	</h2>
	<ol class="mt-6 space-y-4">
		{#each LANGUAGES as lang (lang.name)}
			<li class="flex gap-4">
				<div
					class="relative mt-1 flex h-10 w-10 flex-none items-center justify-center rounded-full shadow-md ring-1 shadow-zinc-800/5 ring-zinc-900/5 dark:border dark:border-zinc-700/50 dark:bg-zinc-800 dark:ring-0"
				>
					<img
						alt=""
						loading="lazy"
						width="32"
						height="32"
						decoding="async"
						data-nimg="1"
						class="h-7 w-7"
						src={lang.flag}
					/>
				</div>
				<dl class="flex flex-auto flex-wrap space-y-2 gap-x-2">
					<dt class="sr-only">{lang.name}</dt>
					<dd class="w-full flex-none text-sm font-medium text-zinc-900 dark:text-zinc-100">
						{lang.name}
					</dd>
					<dt class="sr-only">Level</dt>
					<dd class="text-xs text-zinc-500 dark:text-zinc-400">{lang.level}</dd>
				</dl>
			</li>
		{/each}
	</ol>
</div>
