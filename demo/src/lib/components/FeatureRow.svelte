<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		reverse = false,
		code,
		children
	}: {
		title: string;
		reverse?: boolean;
		code: Snippet;
		children: Snippet;
	} = $props();
</script>

{#if reverse}
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">{title}</h3>
		<p class="text-sm text-text-muted leading-relaxed">{@render children()}</p>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		{@render code()}
	</div>
{:else}
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		{@render code()}
	</div>
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">{title}</h3>
		<p class="text-sm text-text-muted leading-relaxed">{@render children()}</p>
	</div>
{/if}
