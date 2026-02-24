<script lang="ts">
	let { html, large = false }: { html: string; large?: boolean } = $props();

	const lang = $derived(html.match(/data-lang="([^"]+)"/)?.[1] ?? '');
</script>

<div class="code-block relative" class:code-block-lg={large}>
	{#if lang}
		<span
			class="code-lang-badge absolute top-2 right-2 z-10 rounded px-1.5 py-0.5 text-[10px] font-medium leading-none select-none pointer-events-none"
			class:lang-rust={lang === 'rust'}
			class:lang-ts={lang === 'ts'}>{lang}</span
		>
	{/if}
	{@html html}
</div>
