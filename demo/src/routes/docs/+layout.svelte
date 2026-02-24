<script lang="ts">
	let { children } = $props();

	const sections = [
		{ id: 'getting-started', label: 'Getting Started' },
		{ id: 'queries', label: 'Queries' },
		{ id: 'mutations', label: 'Mutations' },
		{ id: 'type-mappings', label: 'Type Mappings' },
		{ id: 'error-handling', label: 'Error Handling' },
		{ id: 'streaming', label: 'Streaming' }
	];

	let sidebarOpen = $state(false);

	$effect(() => {
		document.body.style.overflow = sidebarOpen ? 'hidden' : '';
		return () => { document.body.style.overflow = ''; };
	});
</script>

<div class="mx-auto flex max-w-7xl">
	<!-- Mobile toggle -->
	<button
		class="fixed bottom-4 right-4 z-50 flex h-10 w-10 items-center justify-center rounded-lg bg-accent-rust text-white shadow-lg lg:hidden"
		onclick={() => (sidebarOpen = !sidebarOpen)}
	>
		{sidebarOpen ? '✕' : '☰'}
	</button>

	<!-- Sidebar -->
	<aside
		class="fixed top-14 left-0 z-40 h-[calc(100vh-3.5rem)] w-60 border-r border-border bg-bg-sidebar p-4 transition-transform lg:translate-x-0 {sidebarOpen ? 'translate-x-0' : '-translate-x-full'}"
	>
		<nav class="flex flex-col gap-1">
			{#each sections as section}
				<a
					href="#{section.id}"
					class="rounded-md px-3 py-1.5 text-sm text-text-muted transition-colors hover:bg-bg-soft hover:text-text-primary"
					onclick={() => (sidebarOpen = false)}
				>
					{section.label}
				</a>
			{/each}
		</nav>
	</aside>

	<!-- Backdrop for mobile -->
	{#if sidebarOpen}
		<button
			class="fixed inset-0 z-30 bg-black/50 lg:hidden"
			onclick={() => (sidebarOpen = false)}
			aria-label="Close sidebar"
		></button>
	{/if}

	<!-- Content -->
	<div class="min-w-0 flex-1 px-4 py-8 sm:px-8 lg:ml-60 lg:pl-8">
		{@render children()}
	</div>
</div>
