import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	exampleRust: {
		lang: 'rust',
		code: `#[rpc_query]
async fn get_user(id: u32) -> User {
    db::find_user(id).await
}`
	},
	exampleTs: {
		lang: 'typescript',
		code: `// Direct call
const user = await rpc.query('get_user', 42);

// With options
const user = await rpc.query('get_user', 42, {
  timeout: 5000,
});`
	},
	exampleSvelte: {
		lang: 'typescript',
		code: `// Reactive wrapper — auto-refetches when input changes
let userId = $state(42);
const user = createQuery(rpc, 'get_user', () => userId);

// user.data       — User | undefined
// user.isLoading  — boolean
// user.isError    — boolean
// user.refetch()  — manual refetch`
	},
	callOptionsType: {
		lang: 'typescript',
		code: `interface CallOptions {
  headers?: Record<string, string>;  // merged with client headers
  timeout?: number;                   // override client timeout (ms)
  signal?: AbortSignal;              // combined with client signal
  dedupe?: boolean;                  // per-call dedup override
}`
	},
	callOptionsUsage: {
		lang: 'typescript',
		code: `// Void-input query with options
await rpc.query('time', { timeout: 5000 });

// Query with input + options
await rpc.query('get_user', { id: 1 }, {
  timeout: 5000,
  headers: { 'X-Request-Id': crypto.randomUUID() },
});`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
