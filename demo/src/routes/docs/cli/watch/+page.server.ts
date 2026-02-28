import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'shellscript' }> = {
	basic: {
		lang: 'shellscript',
		code: `# Watch for .rs changes and regenerate
metaxy watch`
	},
	examples: {
		lang: 'shellscript',
		code: `# Custom debounce and clear screen
metaxy watch --debounce-ms 500 --clear-screen

# Watch with all generate flags
metaxy watch \\
  --dir src/handlers \\
  --svelte-output src/lib/rpc.svelte.ts \\
  --preserve-docs \\
  --fields camelCase

# Watch with custom output paths
metaxy watch -o src/generated/types.ts -c src/generated/client.ts`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
