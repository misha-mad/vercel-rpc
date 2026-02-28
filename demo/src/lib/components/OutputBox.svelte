<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		status = 'neutral',
		mono = false,
		children
	}: {
		status?: 'neutral' | 'success' | 'error';
		mono?: boolean;
		children: Snippet;
	} = $props();
</script>

<div
	class="rounded-md bg-bg-code p-3 text-sm space-y-1 overflow-x-auto"
	class:font-mono={mono}
	class:text-text-muted={status === 'neutral'}
	class:text-green-400={status === 'success'}
	class:text-red-400={status === 'error'}
>
	{@render children()}
</div>
