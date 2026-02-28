import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'shellscript' }> = {
	basic: {
		lang: 'shellscript',
		code: `# Generate types + client from ./api directory
metaxy generate`
	},
	inputExamples: {
		lang: 'shellscript',
		code: `# Custom source directory
metaxy generate --dir src/handlers

# Include only specific files
metaxy generate --include "api/**/*.rs"

# Exclude test files
metaxy generate --exclude "**/*_test.rs" --exclude "**/tests/**"`
	},
	outputExamples: {
		lang: 'shellscript',
		code: `# Custom output paths
metaxy generate -o src/generated/types.ts -c src/generated/client.ts

# Generate with Svelte wrappers
metaxy generate --svelte-output src/lib/rpc.svelte.ts

# Generate with multiple framework outputs
metaxy generate \\
  --svelte-output src/lib/rpc.svelte.ts \\
  --react-output src/lib/rpc.react.ts \\
  --vue-output src/lib/rpc.vue.ts \\
  --solid-output src/lib/rpc.solid.ts

# ESM import extension
metaxy generate --extension ".js"`
	},
	codegenExamples: {
		lang: 'shellscript',
		code: `# Enable doc comments and branded newtypes
metaxy generate --preserve-docs --branded-newtypes

# camelCase fields
metaxy generate --fields camelCase

# Type overrides
metaxy generate \\
  --type-override "chrono::DateTime=string" \\
  --type-override "uuid::Uuid=string"

# BigInt mapping
metaxy generate --bigint-type i64 --bigint-type u64

# All codegen options combined
metaxy generate \\
  --preserve-docs \\
  --branded-newtypes \\
  --fields camelCase \\
  --type-override "chrono::DateTime=string" \\
  --bigint-type i64 --bigint-type u64`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
