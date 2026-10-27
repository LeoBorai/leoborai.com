<script lang="ts">
	export type Skill = { name: string; logo?: string; href?: string };

	let { label, skills }: { label: string; skills: Skill[] } = $props();
</script>

<div class="grid gap-2 sm:grid-cols-[8.5rem_minmax(0,1fr)] sm:gap-4">
	<dt class="text-sm text-neutral-500 dark:text-neutral-400">{label}</dt>
	<dd class="flex flex-wrap gap-x-5 gap-y-2.5">
		{#each skills as skill (skill.name)}
			<svelte:element
				this={skill.href ? 'a' : 'span'}
				href={skill.href}
				target={skill.href ? '_blank' : undefined}
				rel={skill.href ? 'noopener noreferrer' : undefined}
				class="group hover:text-accent inline-flex items-center gap-2 text-sm text-neutral-800 transition-colors dark:text-neutral-200"
			>
				{#if skill.logo}
					<span
						class="block size-4 bg-current"
						style="mask:url({skill.logo}) center/contain no-repeat;-webkit-mask:url({skill.logo}) center/contain no-repeat"
						aria-hidden="true"
					></span>
				{/if}
				{skill.name}
			</svelte:element>
		{/each}
	</dd>
</div>
