<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">{@render children()}</code>
