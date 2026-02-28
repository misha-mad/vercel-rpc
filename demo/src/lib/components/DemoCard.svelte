<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		subtitle,
		children
	}: {
		title?: string;
		subtitle?: string;
		children: Snippet;
	} = $props();
</script>

<div class="rounded-lg border border-border bg-bg-soft p-6">
	{#if title}
		<h3 class="text-lg font-semibold mb-2">{title}</h3>
	{/if}
	{#if subtitle}
		<p class="text-text-muted text-sm mb-4">{subtitle}</p>
	{/if}
	{@render children()}
</div>
