import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	// Getting Started — Quick example (3 blocks, large)
	gettingStartedRust: {
		lang: 'rust',
		code: `#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}`
	},
	gettingStartedTs: {
		lang: 'typescript',
		code: `import { createRpcClient } from './rpc-client';

const rpc = createRpcClient({ baseUrl: '/api' });
const greeting = await rpc.query('hello', 'World');
// greeting: string — "Hello, World from Rust on Vercel!"`
	},
	gettingStartedSvelte: {
		lang: 'typescript',
		code: `import { createQuery } from './rpc.svelte';

let name = $state('World');
const hello = createQuery(rpc, 'hello', () => name);
// hello.data reactively updates when 'name' changes`
	},

	// Hello section (2 blocks)
	helloRust: {
		lang: 'rust',
		code: `#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}`
	},
	helloTs: {
		lang: 'typescript',
		code: `const hello = createQuery(rpc, "hello", () => name);

hello.data       // string | undefined
hello.isLoading  // boolean
hello.isError    // boolean
hello.refetch()  // manual refetch`
	},

	// Time section (2 blocks)
	timeRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
pub struct TimeResponse {
    pub timestamp: u64,
    pub message: String,
}

#[rpc_query]
async fn time() -> TimeResponse {
    TimeResponse { timestamp: now, message: "..." }
}`
	},
	timeTs: {
		lang: 'typescript',
		code: `interface TimeResponse {
  timestamp: number;  // u64 → number
  message: string;    // String → string
}

const time = createQuery(rpc, "time");
// time.data?.timestamp, time.isLoading`
	},

	// Status section (2 blocks)
	statusRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
pub enum HealthStatus {
    Healthy, Degraded, Down,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub name: String,
    pub status: HealthStatus,
    pub uptime_secs: u64,
}`
	},
	statusTs: {
		lang: 'typescript',
		code: `type HealthStatus = "Healthy" | "Degraded" | "Down";

interface StatusResponse {
  name: string;
  status: HealthStatus;
  uptime_secs: number;
}`
	},

	// Echo section (2 blocks)
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

	// Secret section (2 blocks)
	secretRust: {
		lang: 'rust',
		code: `#[rpc_query]
async fn secret(req: Request) -> impl IntoResponse {
    let auth = req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    match auth {
        Some("Bearer secret-token-123") =>
            (StatusCode::OK, "Access granted!"),
        _ =>
            (StatusCode::UNAUTHORIZED, "Unauthorized"),
    }
}`
	},
	secretTs: {
		lang: 'typescript',
		code: `import { RpcError } from './rpc.svelte';

try {
  const result = await client.query('secret');
} catch (e) {
  if (e instanceof RpcError) {
    e.status   // HTTP status code
    e.message  // error message
    e.data     // parsed error body
  }
}`
	},

	// Streaming section (2 blocks)
	streamRust: {
		lang: 'rust',
		code: `#[rpc_stream]
async fn chat(input: ChatInput, tx: StreamSender) {
    for token in generate_tokens(&input.prompt) {
        tx.send(token).await;
    }
}`
	},
	streamTs: {
		lang: 'typescript',
		code: `const stream = rpc.stream('chat', {
  prompt: 'Explain Rust ownership'
});

for await (const token of stream) {
  console.log(token); // each chunk
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
