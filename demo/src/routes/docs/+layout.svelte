<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { slide } from 'svelte/transition';

	let { children } = $props();

	const nav = [
		{ label: 'Getting Started', href: '/docs/getting-started' },
		{
			label: 'Procedures',
			children: [
				{ label: 'Queries', href: '/docs/procedures/queries' },
				{ label: 'Mutations', href: '/docs/procedures/mutations' },
				{ label: 'Streaming', href: '/docs/procedures/streaming', badge: 'soon' }
			]
		},
		{
			label: 'Configuration',
			children: [
				{ label: 'Config File', href: '/docs/configuration/config-file' },
				{ label: 'CLI', href: '/docs/configuration/cli' },
				{ label: 'Macro Attributes', href: '/docs/configuration/macro-attributes' },
				{ label: 'RpcClientConfig', href: '/docs/configuration/client-config' }
			]
		},
		{
			label: 'Type System',
			children: [
				{ label: 'Type Mappings', href: '/docs/type-system/type-mappings' },
				{ label: 'Serde Support', href: '/docs/type-system/serde' },
				{ label: 'Generics', href: '/docs/type-system/generics' },
				{ label: 'Branded Newtypes', href: '/docs/type-system/branded-newtypes' }
			]
		},
		{
			label: 'Frameworks',
			children: [
				{ label: 'Svelte 5', href: '/docs/frameworks/svelte' },
				{ label: 'React', href: '/docs/frameworks/react' },
				{ label: 'Vue 3', href: '/docs/frameworks/vue' },
				{ label: 'SolidJS', href: '/docs/frameworks/solid' }
			]
		},
		{ label: 'Error Handling', href: '/docs/error-handling' }
	] as const;

	type NavEntry = (typeof nav)[number];
	type NavGroup = Extract<NavEntry, { children: readonly unknown[] }>;

	function isGroup(entry: NavEntry): entry is NavGroup {
		return 'children' in entry;
	}

	function isActive(href: string): boolean {
		return page.url.pathname === href;
	}

	// Alternating color on each navigation click
	let clickCount = $state(0);
	const activeColor = $derived(clickCount % 2 === 0 ? 'rust' : 'ts');

	function hasActiveChild(group: NavGroup): boolean {
		return group.children.some((child) => isActive(child.href));
	}

	// Track manually toggled groups. Key = group label, value = open/closed override.
	let toggleOverrides: Record<string, boolean> = $state({});

	function isGroupOpen(group: NavGroup): boolean {
		if (group.label in toggleOverrides) return toggleOverrides[group.label];
		return hasActiveChild(group);
	}

	function toggleGroup(group: NavGroup) {
		const current = isGroupOpen(group);
		toggleOverrides[group.label] = !current;
	}

	let sidebarOpen = $state(false);

	$effect(() => {
		document.body.style.overflow = sidebarOpen ? 'hidden' : '';
		return () => {
			document.body.style.overflow = '';
		};
	});

	// Reset manual overrides on navigation (so auto-expand takes over)
	$effect(() => {
		void page.url.pathname;
		toggleOverrides = {};
	});
</script>

<div class="mx-auto flex max-w-7xl">
	<!-- Mobile toggle -->
	<button
		class="fixed bottom-4 left-4 z-60 flex h-10 w-10 items-center justify-center rounded-lg bg-accent-rust text-white shadow-lg lg:hidden"
		onclick={() => (sidebarOpen = !sidebarOpen)}
	>
		{sidebarOpen ? '✕' : '☰'}
	</button>

	<!-- Sidebar -->
	<aside
		class="fixed top-0 bottom-0 left-0 z-40 w-60 overflow-y-auto border-r border-border bg-bg-sidebar pt-18 pb-16 px-4 transition-transform lg:translate-x-0 {sidebarOpen
			? 'translate-x-0'
			: '-translate-x-full'}"
	>
		<nav class="flex flex-col gap-0.5">
			{#each nav as entry (entry.label)}
				{#if isGroup(entry)}
					<button
						class="flex w-full items-center justify-between rounded-md px-3 py-1.5 text-sm font-medium text-text-muted transition-colors hover:bg-bg-soft hover:text-text-primary"
						onclick={() => toggleGroup(entry)}
					>
						{entry.label}
						<span
							class="text-[10px] text-text-faint transition-transform {isGroupOpen(entry)
								? 'rotate-90'
								: ''}">▶</span
						>
					</button>
					{#if isGroupOpen(entry)}
						<div
							transition:slide={{ duration: 150 }}
							class="ml-2 flex flex-col gap-0.5 border-l border-border pl-2"
						>
							{#each entry.children as child (child.href)}
								<a
									href={resolve(child.href)}
									class="flex items-center gap-2 rounded-md px-3 py-1 text-sm transition-colors {isActive(
										child.href
									)
										? `bg-bg-soft text-text-primary border-l-2 -ml-0.5 pl-2.5 ${activeColor === 'rust' ? 'border-accent-rust' : 'border-accent-ts'}`
										: 'text-text-muted hover:bg-bg-soft hover:text-text-primary'}"
									onclick={() => {
										clickCount++;
										sidebarOpen = false;
									}}
								>
									{child.label}
									{#if 'badge' in child}
										<span class="rounded-full bg-accent-rust/20 text-accent-rust text-[10px] px-1.5"
											>{child.badge}</span
										>
									{/if}
								</a>
							{/each}
						</div>
					{/if}
				{:else}
					<a
						href={resolve(entry.href)}
						class="rounded-md px-3 py-1.5 text-sm transition-colors {isActive(entry.href)
							? `bg-bg-soft text-text-primary border-l-2 -ml-0.5 pl-2.5 font-medium ${activeColor === 'rust' ? 'border-accent-rust' : 'border-accent-ts'}`
							: 'text-text-muted hover:bg-bg-soft hover:text-text-primary'}"
						onclick={() => {
							clickCount++;
							sidebarOpen = false;
						}}
					>
						{entry.label}
					</a>
				{/if}
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
