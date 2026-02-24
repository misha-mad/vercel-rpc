import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	renameAllRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub user_name: String,
    pub avatar_url: Option<String>,
    #[serde(skip)]
    pub internal_score: f64,
}`
	},
	renameAllTs: {
		lang: 'typescript',
		code: `interface UserProfile {
  userName: string;       // renamed to camelCase
  avatarUrl: string | null;
  // internal_score omitted (skip)
}`
	},
	renameFieldRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
pub struct Config {
    #[serde(rename = "api-key")]
    pub api_key: String,
    #[serde(rename = "base_url")]
    pub endpoint: String,
}`
	},
	renameFieldTs: {
		lang: 'typescript',
		code: `interface Config {
  "api-key": string;   // exact rename
  base_url: string;    // exact rename
}`
	},
	flattenRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
pub struct Timestamped {
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Serialize)]
pub struct User {
    pub name: String,
    #[serde(flatten)]
    pub timestamps: Timestamped,
}`
	},
	flattenTs: {
		lang: 'typescript',
		code: `interface Timestamped {
  created_at: number;
  updated_at: number;
}

type User = {
  name: string;
} & Timestamped;`
	},
	enumExternalRust: {
		lang: 'rust',
		code: `// Default: externally tagged
#[derive(Serialize)]
pub enum Shape {
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
}`
	},
	enumExternalTs: {
		lang: 'typescript',
		code: `type Shape =
  | { Circle: { radius: number } }
  | { Rect: { w: number; h: number } };`
	},
	enumInternalRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Event {
    Click { x: i32, y: i32 },
    Scroll { delta: f64 },
}`
	},
	enumInternalTs: {
		lang: 'typescript',
		code: `type Event =
  | { type: "Click"; x: number; y: number }
  | { type: "Scroll"; delta: number };`
	},
	enumAdjacentRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Message {
    Text(String),
    Data { payload: Vec<u8> },
}`
	},
	enumAdjacentTs: {
		lang: 'typescript',
		code: `type Message =
  | { t: "Text"; c: string }
  | { t: "Data"; c: { payload: number[] } };`
	},
	enumUntaggedRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    Str(String),
    Num(f64),
}`
	},
	enumUntaggedTs: {
		lang: 'typescript',
		code: `type StringOrNumber = string | number;`
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
