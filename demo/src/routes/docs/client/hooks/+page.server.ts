import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'typescript' | 'rust' }> = {
	onRequest: {
		lang: 'typescript',
		code: `const rpc = createRpcClient({
  baseUrl: '/api',
  onRequest: (ctx) => {
    console.log(\`→ \${ctx.method} \${ctx.procedure}\`);
    // Mutate headers before the request is sent
    ctx.headers['X-Request-Id'] = crypto.randomUUID();
  },
});`
	},
	requestCtx: {
		lang: 'typescript',
		code: `interface RequestContext {
  procedure: string;               // e.g. "get_user"
  method: 'GET' | 'POST';         // GET for queries, POST for mutations
  url: string;                     // full request URL
  headers: Record<string, string>; // mutable — add/modify headers here
  input: unknown;                  // the serialized input
}`
	},
	onResponse: {
		lang: 'typescript',
		code: `const rpc = createRpcClient({
  baseUrl: '/api',
  onResponse: (ctx) => {
    console.log(\`← \${ctx.procedure} \${ctx.response.status} (\${ctx.duration}ms)\`);
  },
});`
	},
	responseCtx: {
		lang: 'typescript',
		code: `interface ResponseContext {
  procedure: string;
  method: 'GET' | 'POST';
  url: string;
  response: Response;   // the raw fetch Response
  data: unknown;        // parsed response body
  duration: number;     // request duration in ms
}`
	},
	onError: {
		lang: 'typescript',
		code: `const rpc = createRpcClient({
  baseUrl: '/api',
  onError: (ctx) => {
    console.error(\`✗ \${ctx.procedure} attempt \${ctx.attempt}\`, ctx.error);
    if (ctx.willRetry) {
      console.log('Retrying...');
    } else {
      reportToSentry(ctx.error);
    }
  },
});`
	},
	errorCtx: {
		lang: 'typescript',
		code: `interface ErrorContext {
  procedure: string;
  method: 'GET' | 'POST';
  url: string;
  error: unknown;       // the thrown error
  attempt: number;      // current attempt (1 = first try)
  willRetry: boolean;   // true if retry policy will retry
}`
	},
	allHooks: {
		lang: 'typescript',
		code: `const rpc = createRpcClient({
  baseUrl: '/api',
  onRequest:  (ctx) => { /* before fetch */ },
  onResponse: (ctx) => { /* after success */ },
  onError:    (ctx) => { /* on failure */ },
});

// Execution order:
// 1. onRequest  — mutate headers, log
// 2. fetch      — actual HTTP call
// 3. onResponse — if success
//    onError    — if failure (may repeat if retry is configured)`
	},
	retryDemoRust: {
		lang: 'rust',
		code: `static CALL_COUNT: AtomicU32 = AtomicU32::new(0);

/// Returns 500 for the first \`fail_count\` calls, then 200.
#[rpc_query]
async fn retry_demo(input: RetryDemoInput) -> Result<RetryDemoResponse, String> {
    if input.reset { CALL_COUNT.store(0, Ordering::Relaxed); }
    let call_number = CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if call_number <= input.fail_count {
        return Err(format!("Simulated failure (call {})", call_number));
    }
    Ok(RetryDemoResponse { call_number, total_calls: CALL_COUNT.load(Ordering::Relaxed), message: format!("Success on call {}", call_number) })
}`
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
