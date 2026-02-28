<script lang="ts">
	import CodeBlock from './CodeBlock.svelte';

	let { html, label = 'Rust' }: { html: string; label?: string } = $props();

	let open = $state(false);
</script>

<button
	class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
	onclick={() => (open = !open)}
>
	{open ? '▾ Hide' : '▸ Show'}
	{label}
</button>
{#if open}
	<div class="mt-3">
		<CodeBlock {html} />
	</div>
{/if}
