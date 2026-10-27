<script lang="ts">
	import { onMount } from 'svelte';

	import Container from '#lib/components/atoms/Container.svelte';
	import NoteRow from '#lib/components/atoms/NoteRow.svelte';
	import Section from '#lib/components/atoms/Section.svelte';
	import SkillGroup, { type Skill } from '#lib/components/atoms/SkillGroup.svelte';
	import { SITE, SOCIAL_LINKS } from '#lib/config/site.ts';

	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const TIME_ZONE = 'Europe/Berlin';
	const clockFormat = new Intl.DateTimeFormat('en-GB', {
		timeZone: TIME_ZONE,
		hour: '2-digit',
		minute: '2-digit',
		second: '2-digit',
		hourCycle: 'h23',
		timeZoneName: 'short'
	});

	// empty on the server so hydration matches; filled once mounted
	let clock = $state('');

	onMount(() => {
		const tick = () => {
			const parts = clockFormat.formatToParts(new Date());
			const pick = (type: string) => parts.find((p) => p.type === type)?.value ?? '';
			clock = `${pick('hour')}:${pick('minute')}:${pick('second')} ${pick('timeZoneName')}`;
		};
		tick();
		const id = setInterval(tick, 1000);
		return () => clearInterval(id);
	});

	const SKILL_GROUPS: { label: string; skills: Skill[] }[] = [
		{
			label: 'Languages',
			skills: [
				{ name: 'Rust', logo: '/img/techstack/rust.svg', href: 'https://www.rust-lang.org' },
				{
					name: 'TypeScript',
					logo: '/img/techstack/typescript.svg',
					href: 'https://www.typescriptlang.org'
				},
				{ name: 'Python', logo: '/img/techstack/python.svg', href: 'https://www.python.org' },
				{ name: 'SQL', logo: '/img/techstack/postgresql.svg' }
			]
		},
		{
			label: 'Frameworks',
			skills: [
				{ name: 'SvelteKit', logo: '/img/techstack/svelte.svg', href: 'https://svelte.dev' },
				{ name: 'Next.js', logo: '/img/techstack/nextdotjs.svg', href: 'https://nextjs.org' },
				{ name: 'Ember.js', logo: '/img/techstack/emberdotjs.svg', href: 'https://emberjs.com' }
			]
		},
		{
			label: 'Infrastructure',
			skills: [
				{ name: 'Docker', logo: '/img/techstack/docker.svg', href: 'https://www.docker.com' },
				{
					name: 'Cloudflare',
					logo: '/img/techstack/cloudflare.svg',
					href: 'https://www.cloudflare.com'
				},
				{ name: 'Fly.io', logo: '/img/techstack/flydotio.svg', href: 'https://fly.io' },
				{ name: 'GCP', logo: '/img/techstack/googlecloud.svg', href: 'https://cloud.google.com' }
			]
		},
		{
			label: 'Workflow',
			skills: [
				{ name: 'Git', logo: '/img/techstack/git.svg', href: 'https://git-scm.com' },
				{
					name: 'Claude Code',
					logo: '/img/techstack/claude.svg',
					href: 'https://www.anthropic.com/claude-code'
				}
			]
		}
	];
</script>

<Container class="py-10">
	<div class="flex flex-col gap-8 md:flex-row md:items-center md:justify-between md:gap-16">
		<div class="min-w-0 flex-1">
			<h1 class="mb-2 text-3xl font-bold sm:text-4xl">{SITE.name}</h1>
			<p class="text-accent mb-2 text-xl sm:text-2xl">{SITE.role}</p>
			<p class="mb-6 h-4 font-mono text-xs text-neutral-500 tabular-nums" aria-label="Local time">
				Europe · {clock}
			</p>
			<ul class="flex flex-wrap gap-2">
				{#each SOCIAL_LINKS as link (link.label)}
					<li>
						<a
							href={link.href}
							target="_blank"
							rel="noopener noreferrer"
							aria-label={link.label}
							class="hover:text-accent hover:border-accent/50 inline-flex border border-neutral-200 p-2 text-neutral-500 transition-all dark:border-neutral-800"
						>
							<svg viewBox={link.viewBox} fill="currentColor" class="size-5" aria-hidden="true"
								><path
									d={link.path}
									fill-rule={link.evenodd ? 'evenodd' : 'nonzero'}
								/>{#each link.layers ?? [] as layer (layer)}<path d={layer} />{/each}</svg
							>
						</a>
					</li>
				{/each}
			</ul>
		</div>
		<div class="min-w-0 flex-1 md:max-w-md">
			<h2 class="mb-3 font-mono text-[11px] tracking-[0.2em] text-neutral-500 uppercase">About</h2>
			<p class="leading-relaxed text-neutral-600 dark:text-neutral-400">
				Doing Software Development since 2017. Experience in Web Development, Systems Programming
				and now learning AI Engineering.
			</p>
		</div>
	</div>
</Container>

<Container>
	<div class="grid gap-x-12 lg:grid-cols-2">
		<Section id="notes" label="Notes" href="/notes" hrefLabel="View all notes">
			<div class="divide-y divide-neutral-200 dark:divide-neutral-800">
				{#each data.notes as note (note.slug)}
					<NoteRow title={note.meta.title} date={note.meta.date} slug={note.slug} />
				{/each}
			</div>
		</Section>

		<Section id="skills" label="Stack & Skills">
			<dl class="space-y-4">
				{#each SKILL_GROUPS as group (group.label)}
					<SkillGroup label={group.label} skills={group.skills} />
				{/each}
			</dl>
		</Section>
	</div>
</Container>
