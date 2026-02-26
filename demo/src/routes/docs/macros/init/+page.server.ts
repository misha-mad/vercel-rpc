import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' }> = {
	sideEffect: {
		lang: 'rust',
		code: `// Side-effects only — logger, dotenv, tracing
async fn setup_logger() {
    env_logger::init();
}

#[rpc_query(init = "setup_logger")]
async fn get_data() -> Data { ... }

#[rpc_mutation(init = "setup_logger")]
async fn create_item(input: ItemInput) -> Item { ... }`
	},
	sharedState: {
		lang: 'rust',
		code: `// Return shared state — DB pool, HTTP client, config
async fn setup_db() -> PgPool {
    PgPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap()
}

// The return type is injected as &T parameter
#[rpc_query(init = "setup_db")]
async fn get_users(pool: &PgPool) -> Vec<User> {
    sqlx::query_as("SELECT * FROM users")
        .fetch_all(pool).await.unwrap()
}

#[rpc_mutation(init = "setup_db")]
async fn create_user(pool: &PgPool, input: UserInput) -> User {
    sqlx::query_as("INSERT INTO users ...")
        .fetch_one(pool).await.unwrap()
}`
	},
	httpClient: {
		lang: 'rust',
		code: `// HTTP client for external API calls
async fn setup_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap()
}

#[rpc_query(init = "setup_client")]
async fn get_weather(client: &reqwest::Client, city: String) -> Weather {
    client.get(&format!("https://api.weather.com/{city}"))
        .send().await.unwrap()
        .json().await.unwrap()
}`
	},
	combined: {
		lang: 'rust',
		code: `// Combine with other attributes
#[rpc_query(init = "setup_db", cache = "5m", timeout = "10s")]
async fn get_stats(pool: &PgPool) -> Stats { ... }`
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
