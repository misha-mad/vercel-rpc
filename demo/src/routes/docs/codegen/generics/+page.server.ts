import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	genericsRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
}

#[rpc_query]
async fn list_users() -> Paginated<User> {
    // ...
}`
	},
	genericsTs: {
		lang: 'typescript',
		code: `interface Paginated<T> {
  items: T[];
  total: number;
  page: number;
}

interface User {
  id: number;
  name: string;
}

// list_users returns Paginated<User>`
	},
	multiRust: {
		lang: 'rust',
		code: `/// Multiple type parameters are preserved as-is.
#[derive(Serialize)]
pub struct ApiResponse<T, E> {
    pub data: Option<T>,
    pub error: Option<E>,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[rpc_query]
async fn get_profile(id: u64) -> ApiResponse<User, ValidationError> {
    // ...
}`
	},
	multiTs: {
		lang: 'typescript',
		code: `interface ApiResponse<T, E> {
  data: T | null;
  error: E | null;
  metadata: Record<string, string>;
}

interface ValidationError {
  field: string;
  message: string;
}

// get_profile returns ApiResponse<User, ValidationError>`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
