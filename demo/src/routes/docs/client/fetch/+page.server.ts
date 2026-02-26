import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'typescript' }> = {
	defaultFetch: {
		lang: 'typescript',
		code: `// By default the client uses the global fetch.
const rpc = createRpcClient({
  baseUrl: '/api',
});`
	},
	ssrFetch: {
		lang: 'typescript',
		code: `// SvelteKit — use the platform fetch for SSR (cookies, relative URLs)
export const load: PageServerLoad = async ({ fetch }) => {
  const rpc = createRpcClient({
    baseUrl: '/api',
    fetch,
  });

  const users = await rpc.query('list_users');
  return { users };
};`
	},
	testFetch: {
		lang: 'typescript',
		code: `// Testing — mock fetch for unit tests
import { vi } from 'vitest';

const mockFetch = vi.fn().mockResolvedValue(
  new Response(JSON.stringify({ id: 1, name: 'Alice' }), {
    status: 200,
    headers: { 'Content-Type': 'application/json' },
  })
);

const rpc = createRpcClient({
  baseUrl: '/api',
  fetch: mockFetch,
});

const user = await rpc.query('get_user', { id: 1 });
expect(mockFetch).toHaveBeenCalledOnce();`
	}
};

export const load: PageServerLoad = async () => {
	const entries = Object.entries(codeBlocks);
	const results = await Promise.all(entries.map(([, { code, lang }]) => highlightCode(code, lang)));
	const highlighted: Record<string, string> = {};
	entries.forEach(([key], i) => {
		highlighted[key] = results[i];
	});
	return { highlighted };
};
