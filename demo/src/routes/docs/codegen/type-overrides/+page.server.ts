import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<
	string,
	{ code: string; lang: 'rust' | 'typescript' | 'toml' | 'shellscript' }
> = {
	configToml: {
		lang: 'toml',
		code: `# metaxy.config.toml
[codegen.type_overrides]
"chrono::DateTime" = "string"      # ISO 8601 strings
"chrono::NaiveDate" = "string"
"uuid::Uuid" = "string"
"serde_json::Value" = "unknown"
"rust_decimal::Decimal" = "string"
"url::Url" = "string"`
	},
	configCli: {
		lang: 'shellscript',
		code: `metaxy generate --type-override "chrono::DateTime=string" --type-override "uuid::Uuid=string"`
	},
	exampleRust: {
		lang: 'rust',
		code: `use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize)]
struct Event {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
}`
	},
	exampleTs: {
		lang: 'typescript',
		code: `// With type_overrides: chrono::DateTime = "string", uuid::Uuid = "string"
export interface Event {
  id: string;
  name: string;
  created_at: string;
}`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
