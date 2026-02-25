import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' | 'shellscript' }> = {
	// 1. End-to-end type safety — side-by-side Rust struct → TS interface
	typeSafetyRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub active: bool,
}`
	},
	typeSafetyTs: {
		lang: 'typescript',
		code: `// Auto-generated
// zero manual sync
interface User {
  id: number;
  name: string;
  active: boolean;
}`
	},

	// 2. Auto-generated client
	autoClient: {
		lang: 'typescript',
		code: `const greeting = await rpc.query('hello', 'World');
// greeting: string — fully typed, with autocomplete`
	},

	// 3. Watch mode
	watchMode: {
		lang: 'shellscript',
		code: `my-app/
├── api/
│   ├── hello.rs      # ← edit Rust
│   ├── users.rs      # ← add new file
│   └── math.rs
├── src/lib/
│   ├── rpc-types.ts  # ← auto-regenerated
│   └── rpc-client.ts # ← auto-regenerated
└── metaxy.config.toml

$ metaxy watch
  ▸ Watching api/ for changes...
  ✓ Generated rpc-types.ts (3 queries, 1 mutation)
  ▸ api/users.rs changed
  ✓ Re-generated (4 queries, 1 mutation)`
	},

	// 4. Macro-driven
	macroDriven: {
		lang: 'rust',
		code: `#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}`
	},

	// 5. Serde support
	serde: {
		lang: 'rust',
		code: `#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub display_name: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
}`
	},

	// 6. Init & state injection
	initState: {
		lang: 'rust',
		code: `#[rpc_query(init = "setup")]
async fn get_users(pool: &PgPool) -> Vec<User> {
    sqlx::query_as("SELECT * FROM users")
        .fetch_all(pool).await.unwrap()
}`
	},

	// 7. Edge caching
	edgeCache: {
		lang: 'rust',
		code: `#[rpc_query(cache = "1h")]
async fn get_config() -> AppConfig {
    // Response cached at Vercel's edge for 1 hour
    load_config().await
}`
	},

	// 8. Framework wrappers
	frameworks: {
		lang: 'typescript',
		code: `// Svelte 5
const user = createQuery(rpc, 'get_user', () => id);

// React
const user = useQuery(rpc, 'get_user', id);

// Vue 3
const user = useRpcQuery(rpc, 'get_user', id);

// SolidJS
const user = createRpcQuery(rpc, 'get_user', () => id);`
	},

	// 10. Vercel-native
	vercelNative: {
		lang: 'shellscript',
		code: `my-app/
├── api/
│   ├── hello.rs     → /api/hello
│   ├── users.rs     → /api/users
│   ├── math.rs      → /api/math
│   └── echo.rs      → /api/echo
├── src/              # frontend (any framework)
├── metaxy.config.toml
├── Cargo.toml
└── vercel.json       # zero config needed`
	},

	// 9. Rich client
	richClient: {
		lang: 'typescript',
		code: `const rpc = createRpcClient({
  baseUrl: '/api',
  retry: 3,
  timeout: 5000,
  headers: () => ({ Authorization: getToken() }),
  onError: (e) => logError(e),
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
