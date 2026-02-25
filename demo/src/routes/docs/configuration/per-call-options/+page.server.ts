import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	callOptionsType: {
		lang: 'typescript',
		code: `interface CallOptions {
  headers?: Record<string, string>;  // merged with client headers
  timeout?: number;                   // override client timeout (ms)
  signal?: AbortSignal;              // combined with client signal
  dedupe?: boolean;                  // per-call dedup override
}`
	},
	usage: {
		lang: 'typescript',
		code: `// Void-input query with options
await rpc.query('time', { timeout: 5000 });

// Query with input + options
await rpc.query('get_user', { id: 1 }, {
  timeout: 5000,
  headers: { 'X-Request-Id': crypto.randomUUID() },
});

// Mutation with input + options
await rpc.mutate('create_order', orderInput, {
  signal: abortController.signal,
  dedupe: false,
});`
	},
	resolution: {
		lang: 'typescript',
		code: `// Resolution chain (highest wins):
// callOptions?.timeout
//   ?? PROCEDURE_TIMEOUTS[procedure]  (from macro timeout attr)
//   ?? config.timeout
//   ?? undefined (no timeout)`
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
