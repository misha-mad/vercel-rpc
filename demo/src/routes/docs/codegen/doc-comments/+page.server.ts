import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<
	string,
	{ code: string; lang: 'rust' | 'typescript' | 'toml' | 'shellscript' }
> = {
	configToml: {
		lang: 'toml',
		code: `# metaxy.config.toml
[codegen]
preserve_docs = true`
	},
	configCli: {
		lang: 'shellscript',
		code: `metaxy generate --preserve-docs`
	},
	exampleRust: {
		lang: 'rust',
		code: `/// Returns the current server time.
#[rpc_query]
async fn time() -> TimeResponse { /* ... */ }

/// A timestamp with a human-readable message.
#[derive(Serialize)]
struct TimeResponse {
    /// Unix timestamp in seconds.
    timestamp: u64,
    /// Formatted message for display.
    message: String,
}

/// Supported user roles.
#[derive(Serialize)]
enum Role {
    /// Read-only access.
    Viewer,
    /// Can create and edit content.
    Editor,
    /// Full access including user management.
    Admin,
}`
	},
	exampleTs: {
		lang: 'typescript',
		code: `/** A timestamp with a human-readable message. */
export interface TimeResponse {
  /** Unix timestamp in seconds. */
  timestamp: number;
  /** Formatted message for display. */
  message: string;
}

/**
 * Supported user roles.
 * - \`"Viewer"\` — Read-only access.
 * - \`"Editor"\` — Can create and edit content.
 * - \`"Admin"\` — Full access including user management.
 */
export type Role = "Viewer" | "Editor" | "Admin";

export type Procedures = {
  queries: {
    /** Returns the current server time. */
    time: { input: void; output: TimeResponse };
  };
};`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
