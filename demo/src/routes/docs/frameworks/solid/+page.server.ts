import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<
	string,
	{ code: string; lang: 'rust' | 'typescript' | 'toml' | 'shellscript' }
> = {
	setupToml: {
		lang: 'toml',
		code: `# metaxy.config.toml
[output]
solid = "src/lib/rpc.solid.ts"`
	},
	setupCli: {
		lang: 'shellscript',
		code: `metaxy generate --solid-output src/lib/rpc.solid.ts`
	},
	query: {
		lang: 'typescript',
		code: `import { createQuery } from './rpc.solid';
import { rpc } from './client';
import { createSignal } from 'solid-js';

const [page, setPage] = createSignal(1);

// Getter input — fine-grained reactivity
const users = createQuery(rpc, 'list_users', () => page(), {
  enabled: () => page() > 0,
  refetchInterval: 30000,
});

// Access via signal accessors
users.data()       // T | undefined
users.error()      // RpcError | undefined
users.isLoading()  // boolean
users.isSuccess()  // boolean (memo)
users.refetch()    // manual refetch`
	},
	mutation: {
		lang: 'typescript',
		code: `import { createMutation } from './rpc.solid';

const create = createMutation(rpc, 'create_user', {
  onSuccess: (data) => console.log('Created:', data),
});

// Trigger
create.mutate({ name: 'Alice' });

create.data()       // T | undefined
create.isLoading()  // boolean
create.reset()      // clear state`
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
