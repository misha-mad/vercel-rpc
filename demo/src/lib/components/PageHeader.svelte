<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		mono = false,
		children
	}: {
		title: string;
		mono?: boolean;
		children?: Snippet;
	} = $props();
</script>

<h1 class="text-3xl font-bold" class:font-mono={mono}>{title}</h1>
{#if children}
	<p class="text-text-muted leading-relaxed">{@render children()}</p>
{/if}
