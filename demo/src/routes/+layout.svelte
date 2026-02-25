<script lang="ts">
	import Header from './Header.svelte';
	import '../app.css';

	let { children } = $props();
</script>

<div class="flex min-h-screen flex-col bg-bg-primary text-text-primary font-body">
	<Header />
	<div class="max-w-7xl w-full mx-auto border-x border-border flex-1 flex flex-col">
		<main class="flex-1">
			{@render children()}
		</main>
	</div>
	<footer
		class="relative z-50 bg-bg-primary text-center py-4 text-text-muted text-sm border-t border-border"
	>
		<p>
			visit <a
				href="https://github.com/misha-mad/metaxy"
				class="text-accent-ts hover:underline font-bold">GitHub</a
			> to learn more about metaxy
		</p>
	</footer>
</div>
