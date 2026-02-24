import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
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
