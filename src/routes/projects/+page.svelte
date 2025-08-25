<script lang="ts">
	import Hero from '$lib/components/atoms/Hero.svelte';

	const PROJECTS = [
		{
			name: 'Rust HTTP Server',
			url: new URL('http://http-server-rs.github.io/'),
			description: 'Building a Safe, Fast, Simple and Configurable HTTP Server Solution in Rust.',
			image: '/img/projects/http-server-rs.png',
			bgColor: '#fff'
		},
		{
			name: 'Fluvio',
			url: new URL('https://fluvio.io/'),
			description: 'Event stream processing solution similar to Kafka, built in Rust.',
			image: '/img/projects/fluvio.png',
			bgColor: 'transparent'
		},
		{
			name: 'Local IP Address Crate',
			url: new URL('https://github.com/LeoBorai/local-ip-address'),
			description:
				'Rust crate to get the local IP address of the machine using system libaries via FFI on multiple platforms.',
			image: '/img/techstack/rust.png',
			bgColor: '#fff'
		},
		{
			name: 'Network-Interface Crate',
			url: new URL('https://github.com/LeoBorai/network-interface'),
			description:
				'Rust crate to get network interface information such as IP addresses, MAC addresses, and more.',
			image: '/img/techstack/rust.png',
			bgColor: '#fff'
		},
		{
			name: 'Svelte Forms',
			url: new URL('https://github.com/whizzes/svelte-forms'),
			description: 'Formik inspired form library for Svelte and SvelteKit.',
			image: '/img/techstack/sveltekit.png',
			bgColor: '#fff'
		},
		{
			name: 'Pxid',
			url: new URL('https://github.com/whizzes/pxid'),
			description:
				'Stripe IDs inspired extension of XID (Globally Unique Identifiers) which supports prefixes and is stored in 16 bytes',
			image: '/img/techstack/rust.png',
			bgColor: '#fff'
		},
		{
			name: 'ImageKit Client for Rust',
			url: new URL('https://github.com/whizzes/imagekit'),
			description: 'Rust API Client for ImageKit.io a file storage and image processing service',
			image: '/img/techstack/rust.png',
			bgColor: '#fff'
		}
	];
</script>

<div class="mt-9 sm:px-8">
	<div class="mx-auto w-full max-w-7xl lg:px-8">
		<div class="relative px-4 sm:px-8 lg:px-12">
			<div class="mx-auto max-w-2xl lg:max-w-5xl">
				<Hero
					title="Projects & Contributions"
					description="Software I've built or contributed to."
				/>
				<div class="mt-16 sm:mt-20">
					<div class="md:pl-6 md:dark:border-zinc-700/40">
						<div class="flex flex-col space-y-16">
							<ul
								role="list"
								class="grid grid-cols-1 gap-x-12 gap-y-16 sm:grid-cols-2 lg:grid-cols-3"
							>
								{#each PROJECTS as project (project.name)}
									<li class="group relative flex flex-col items-start">
										<div
											class="relative z-10 flex h-12 w-12 items-center justify-center rounded-full bg-white shadow-md ring-1 shadow-zinc-800/5 ring-zinc-900/5 dark:border dark:border-zinc-700/50 dark:bg-zinc-800 dark:ring-0"
											style="background-color:{project.bgColor}"
										>
											<img
												alt=""
												loading="lazy"
												width="32"
												height="32"
												decoding="async"
												data-nimg="1"
												class="h-8 w-8"
												src={project.image}
												style="color: transparent;"
											/>
										</div>
										<h2 class="mt-6 text-base font-semibold text-zinc-800 dark:text-zinc-100">
											<div
												class="absolute -inset-x-4 -inset-y-6 z-0 scale-95 bg-zinc-50 opacity-0 transition group-hover:scale-100 group-hover:opacity-100 sm:-inset-x-6 sm:rounded-2xl dark:bg-zinc-800/50"
											></div>
											<a href={project.url.toString()} target="_blank" rel="noopener noreferrer"
												><span
													class="absolute -inset-x-4 -inset-y-6 z-20 sm:-inset-x-6 sm:rounded-2xl"
												></span><span class="relative z-10">{project.name}</span></a
											>
										</h2>
										<p class="relative z-10 mt-2 text-sm text-zinc-600 dark:text-zinc-400">
											{project.description}
										</p>
										<p
											class="relative z-10 mt-6 flex text-sm font-medium text-zinc-400 transition group-hover:text-teal-500 dark:text-zinc-200"
										>
											<svg viewBox="0 0 24 24" aria-hidden="true" class="h-6 w-6 flex-none"
												><path
													d="M15.712 11.823a.75.75 0 1 0 1.06 1.06l-1.06-1.06Zm-4.95 1.768a.75.75 0 0 0 1.06-1.06l-1.06 1.06Zm-2.475-1.414a.75.75 0 1 0-1.06-1.06l1.06 1.06Zm4.95-1.768a.75.75 0 1 0-1.06 1.06l1.06-1.06Zm3.359.53-.884.884 1.06 1.06.885-.883-1.061-1.06Zm-4.95-2.12 1.414-1.415L12 6.344l-1.415 1.413 1.061 1.061Zm0 3.535a2.5 2.5 0 0 1 0-3.536l-1.06-1.06a4 4 0 0 0 0 5.656l1.06-1.06Zm4.95-4.95a2.5 2.5 0 0 1 0 3.535L17.656 12a4 4 0 0 0 0-5.657l-1.06 1.06Zm1.06-1.06a4 4 0 0 0-5.656 0l1.06 1.06a2.5 2.5 0 0 1 3.536 0l1.06-1.06Zm-7.07 7.07.176.177 1.06-1.06-.176-.177-1.06 1.06Zm-3.183-.353.884-.884-1.06-1.06-.884.883 1.06 1.06Zm4.95 2.121-1.414 1.414 1.06 1.06 1.415-1.413-1.06-1.061Zm0-3.536a2.5 2.5 0 0 1 0 3.536l1.06 1.06a4 4 0 0 0 0-5.656l-1.06 1.06Zm-4.95 4.95a2.5 2.5 0 0 1 0-3.535L6.344 12a4 4 0 0 0 0 5.656l1.06-1.06Zm-1.06 1.06a4 4 0 0 0 5.657 0l-1.061-1.06a2.5 2.5 0 0 1-3.535 0l-1.061 1.06Zm7.07-7.07-.176-.177-1.06 1.06.176.178 1.06-1.061Z"
													fill="currentColor"
												></path></svg
											><span class="ml-2">{project.url.hostname}</span>
										</p>
									</li>
								{/each}
							</ul>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
