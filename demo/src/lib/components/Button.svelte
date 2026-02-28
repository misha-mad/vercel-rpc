<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		onclick,
		disabled = false,
		variant = 'ts',
		children
	}: {
		onclick: () => void;
		disabled?: boolean;
		variant?: 'ts' | 'rust';
		children: Snippet;
	} = $props();
</script>

<button
	{onclick}
	{disabled}
	class="rounded-md px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
	class:bg-accent-ts={variant === 'ts'}
	class:bg-accent-rust={variant === 'rust'}
>
	{@render children()}
</button>
