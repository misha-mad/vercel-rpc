<script lang="ts">
	let { html, large = false }: { html: string; large?: boolean } = $props();
</script>

<div class="code-block" class:code-block-lg={large}>
	{@html html}
</div>
