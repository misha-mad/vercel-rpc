<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		level = 'normal',
		children
	}: {
		level?: 'normal' | 'large';
		children: Snippet;
	} = $props();
</script>

{#if level === 'large'}
	<h2 class="text-2xl font-bold mt-12">{@render children()}</h2>
{:else}
	<h2 class="text-xl font-semibold mt-8">{@render children()}</h2>
{/if}
