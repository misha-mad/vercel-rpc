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
vue = "src/lib/rpc.vue.ts"`
	},
	setupCli: {
		lang: 'shellscript',
		code: `metaxy generate --vue-output src/lib/rpc.vue.ts`
	},
	query: {
		lang: 'typescript',
		code: `import { useQuery } from './rpc.vue';
import { rpc } from './client';
import { ref } from 'vue';

const page = ref(1);

// Getter input — refetches when page.value changes
const users = useQuery(rpc, 'list_users', () => page.value, {
  enabled: () => page.value > 0,
  refetchInterval: 30000,
});

// Access reactive refs
users.data.value       // T | undefined
users.error.value      // RpcError | undefined
users.isLoading.value  // boolean
users.isSuccess.value  // ComputedRef<boolean>
users.refetch()        // manual refetch`
	},
	mutation: {
		lang: 'typescript',
		code: `import { useMutation } from './rpc.vue';

const create = useMutation(rpc, 'create_user', {
  onSuccess: (data) => console.log('Created:', data),
  onError: (err) => console.error(err),
});

// Fire-and-forget
create.mutate({ name: 'Alice' });

// Await the result
const user = await create.mutateAsync({ name: 'Alice' });

// Access reactive state
create.data.value       // T | undefined
create.error.value      // RpcError | undefined
create.isLoading.value  // boolean
create.isSuccess.value  // boolean (ComputedRef)
create.isError.value    // boolean (ComputedRef)
create.reset()          // clear state`
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
