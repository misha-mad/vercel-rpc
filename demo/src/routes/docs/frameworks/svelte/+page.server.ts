import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	setup: {
		lang: 'typescript',
		code: `# rpc.config.toml
[output]
svelte = "src/lib/rpc.svelte.ts"

# Or CLI: rpc generate --svelte-output src/lib/rpc.svelte.ts`
	},
	query: {
		lang: 'typescript',
		code: `import { createQuery } from './rpc.svelte';
import { rpc } from './client';

// Void input
const time = createQuery(rpc, 'time');

// Reactive input — refetches when name changes
let name = $state('World');
const hello = createQuery(rpc, 'hello', () => name);

// With options
const users = createQuery(rpc, 'list_users', () => page, {
  enabled: () => page > 0,
  refetchInterval: 30000,
  placeholderData: [],
  onSuccess: (data) => console.log(data),
});

// Access state
hello.data       // T | undefined
hello.error      // RpcError | undefined
hello.isLoading  // boolean
hello.isError    // boolean
hello.isSuccess  // boolean
hello.refetch()  // manual refetch`
	},
	mutation: {
		lang: 'typescript',
		code: `import { createMutation } from './rpc.svelte';

const echo = createMutation(rpc, 'echo', {
  onSuccess: (data) => console.log('Done:', data),
  onError: (err) => console.error(err),
});

// Trigger explicitly
echo.mutate({ message: 'Hello', uppercase: true });

echo.data       // T | undefined
echo.isLoading  // boolean
echo.reset()    // clear state`
	}
};

export const load: PageServerLoad = async () => {
	const entries = Object.entries(codeBlocks);
	const results = await Promise.all(
		entries.map(([, { code, lang }]) => highlightCode(code, lang))
	);
	const highlighted: Record<string, string> = {};
	entries.forEach(([key], i) => {
		highlighted[key] = results[i];
	});
	return { highlighted };
};
