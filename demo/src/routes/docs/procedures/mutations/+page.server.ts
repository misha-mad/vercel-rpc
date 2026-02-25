import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	exampleRust: {
		lang: 'rust',
		code: `#[rpc_mutation]
async fn create_order(input: OrderInput) -> Order {
    db::insert_order(input).await
}`
	},
	exampleTs: {
		lang: 'typescript',
		code: `// Direct call
const order = await rpc.mutate('create_order', orderInput);

// With options
const order = await rpc.mutate('create_order', orderInput, {
  timeout: 10000,
});`
	},
	exampleSvelte: {
		lang: 'typescript',
		code: `// Reactive wrapper — triggered explicitly via .mutate()
const order = createMutation(rpc, 'create_order');

// Trigger:
order.mutate(orderInput);

// order.data       — Order | undefined
// order.isLoading  — boolean
// order.isError    — boolean
// order.error      — RpcError | undefined`
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
	echoRust: {
		lang: 'rust',
		code: `#[derive(Deserialize)]
pub struct EchoInput {
    pub message: String,
    pub uppercase: bool,
}

#[derive(Serialize)]
pub struct EchoOutput {
    pub message: String,
    pub length: usize,
}

#[rpc_mutation]
async fn echo(input: EchoInput) -> EchoOutput {
    let msg = if input.uppercase {
        input.message.to_uppercase()
    } else {
        input.message
    };
    EchoOutput { message: msg.clone(), length: msg.len() }
}`
	},
	echoTs: {
		lang: 'typescript',
		code: `const echo = createMutation(rpc, "echo");

// Trigger explicitly:
echo.mutate({ message: "Hello", uppercase: true });

echo.data       // EchoOutput | undefined
echo.isLoading  // boolean (loading)
echo.isError    // boolean
echo.error      // RpcError | undefined`
	},
	callOptionsMutation: {
		lang: 'typescript',
		code: `// Mutation with input + options
await rpc.mutate('create_order', orderInput, {
  signal: abortController.signal,
  dedupe: false,
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
