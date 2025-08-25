<form class="rounded-2xl border border-zinc-100 p-6 dark:border-zinc-700/40" action="/thank-you">
	<h2 class="flex text-sm font-semibold text-zinc-900 dark:text-zinc-100">
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke-width="1.5"
			stroke-linecap="round"
			stroke-linejoin="round"
			aria-hidden="true"
			class="h-6 w-6 flex-none"
			><path
				d="M2.75 7.75a3 3 0 0 1 3-3h12.5a3 3 0 0 1 3 3v8.5a3 3 0 0 1-3 3H5.75a3 3 0 0 1-3-3v-8.5Z"
				class="fill-zinc-100 stroke-zinc-400 dark:fill-zinc-100/10 dark:stroke-zinc-500"
			></path><path
				d="m4 6 6.024 5.479a2.915 2.915 0 0 0 3.952 0L20 6"
				class="stroke-zinc-400 dark:stroke-zinc-500"
			></path></svg
		><span class="ml-3">Request CV</span>
	</h2>
	<p class="mt-2 text-sm text-zinc-600 dark:text-zinc-400">
		Leave your email here and I will send you my CV.
	</p>
	<div class="mt-6 flex flex-col items-center gap-4">
		<div class="flex w-full max-w-sm">
			<span class="flex min-w-0 flex-auto p-px"
				><input
					type="email"
					placeholder="Email address"
					aria-label="Email address"
					required=""
					class="w-full appearance-none rounded-[calc(var(--radius-md)-1px)] bg-white px-3 py-[calc(--spacing(2)-1px)] shadow-md shadow-zinc-800/5 outline outline-zinc-900/10 placeholder:text-zinc-400 focus:ring-4 focus:ring-emerald-500/10 focus:outline-emerald-500 sm:text-sm dark:bg-zinc-700/[0.15] dark:text-zinc-200 dark:outline-zinc-700 dark:placeholder:text-zinc-500 dark:focus:ring-emerald-400/10 dark:focus:outline-emerald-400"
				/></span
			><button
				class="ml-4 inline-flex flex-none items-center justify-center gap-2 rounded-md bg-zinc-800 px-3 py-2 text-sm font-semibold text-zinc-100 outline-offset-2 transition hover:bg-zinc-700 active:bg-zinc-800 active:text-zinc-100/70 active:transition-none dark:bg-zinc-700 dark:hover:bg-zinc-600 dark:active:bg-zinc-700 dark:active:text-zinc-100/70"
				type="submit">Request</button
			>
		</div>
		<div class="flex items-center justify-between gap-4">
			<!-- Add a checkbox to accept send-->
			<input type="checkbox" class="ml-4" required />
			<label for="checkbox" class="text-xs text-zinc-600 dark:text-zinc-400"
				>I agree to send my email address via Telegram to <strong>Leo Borai</strong> and also to receive
				the CV via email.</label
			>
		</div>
	</div>
</form>
