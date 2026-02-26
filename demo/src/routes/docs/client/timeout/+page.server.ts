import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'typescript' }> = {
	clientTimeout: {
		lang: 'typescript',
		code: `// Global timeout for all requests (ms)
const rpc = createRpcClient({
  baseUrl: '/api',
  timeout: 10000, // 10 seconds
});`
	},
	perCallTimeout: {
		lang: 'typescript',
		code: `// Override timeout for a single call
const report = await rpc.query('heavy_report', input, {
  timeout: 30000, // 30 seconds for this call only
});`
	},
	abortSignal: {
		lang: 'typescript',
		code: `// Client-level signal — aborts all in-flight requests
const controller = new AbortController();

const rpc = createRpcClient({
  baseUrl: '/api',
  signal: controller.signal,
});

// Cancel everything
controller.abort();`
	},
	perCallSignal: {
		lang: 'typescript',
		code: `// Per-call signal — cancel a single request
const controller = new AbortController();

const result = rpc.query('slow_query', input, {
  signal: controller.signal,
});

// Cancel just this request
controller.abort();`
	},
	errorHandling: {
		lang: 'typescript',
		code: `try {
  await rpc.query('slow_query', input, { timeout: 5000 });
} catch (e) {
  if (e instanceof DOMException && e.name === 'AbortError') {
    // Timeout or manual abort — NOT an RpcError
    console.log('Request timed out or was cancelled');
  }
}`
	},
	combinedSignals: {
		lang: 'typescript',
		code: `// Client signal + per-call signal are combined.
// The request aborts when either signal fires.
const clientCtrl = new AbortController();
const callCtrl = new AbortController();

const rpc = createRpcClient({
  baseUrl: '/api',
  signal: clientCtrl.signal,
});

await rpc.query('data', input, {
  signal: callCtrl.signal, // combined with clientCtrl.signal
});`
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
