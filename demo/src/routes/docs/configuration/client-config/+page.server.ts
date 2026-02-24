import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	basicConfig: {
		lang: 'typescript',
		code: `import { createRpcClient } from './rpc-client';

const rpc = createRpcClient({
  baseUrl: '/api',
});`
	},
	fullConfig: {
		lang: 'typescript',
		code: `const rpc = createRpcClient({
  baseUrl: '/api',

  // Custom fetch (SSR, testing)
  fetch: customFetch,

  // Static or async headers
  headers: { Authorization: 'Bearer token' },
  // or: headers: async () => ({ Authorization: await getToken() }),

  // Lifecycle hooks
  onRequest: (ctx) => {
    console.log(\`→ \${ctx.procedure}\`);
  },
  onResponse: (ctx) => {
    console.log(\`← \${ctx.procedure} (\${ctx.duration}ms)\`);
  },
  onError: (ctx) => {
    if (ctx.willRetry) console.log(\`Retrying \${ctx.procedure}...\`);
  },

  // Retry policy
  retry: {
    attempts: 3,
    delay: (n) => Math.min(1000 * 2 ** n, 10000),
    retryOn: [408, 429, 500, 502, 503, 504],
  },

  // Timeout (ms)
  timeout: 10000,

  // Request deduplication
  dedupe: true,

  // Cancellation
  signal: controller.signal,

  // Custom serialization
  serialize: JSON.stringify,
  deserialize: JSON.parse,
});`
	},
	hooksTypes: {
		lang: 'typescript',
		code: `interface RequestContext {
  procedure: string;
  method: 'GET' | 'POST';
  url: string;
  headers: Record<string, string>; // mutable
  input: unknown;
}

interface ResponseContext {
  procedure: string;
  method: 'GET' | 'POST';
  url: string;
  response: Response;
  data: unknown;
  duration: number;
}

interface ErrorContext {
  procedure: string;
  method: 'GET' | 'POST';
  url: string;
  error: Error;
  attempt: number;
  willRetry: boolean;
}`
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
