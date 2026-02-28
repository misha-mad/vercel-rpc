import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<
	string,
	{ code: string; lang: 'rust' | 'typescript' | 'toml' | 'shellscript' }
> = {
	setupToml: {
		lang: 'toml',
		code: `# metaxy.config.toml
[output]
react = "src/lib/rpc.react.ts"`
	},
	setupCli: {
		lang: 'shellscript',
		code: `metaxy generate --react-output src/lib/rpc.react.ts`
	},
	query: {
		lang: 'typescript',
		code: `import { useQuery } from './rpc.react';
import { rpc } from './client';

function UserList() {
  const [page, setPage] = useState(1);

  // Refetches when page changes
  const users = useQuery(rpc, 'list_users', page, {
    enabled: page > 0,
    refetchInterval: 30000,
    onSuccess: (data) => console.log(data),
  });

  // Void input
  const time = useQuery(rpc, 'time');

  return (
    <>
      {users.isLoading && <p>Loading...</p>}
      {users.data?.map(u => <div key={u.id}>{u.name}</div>)}
      <button onClick={() => users.refetch()}>Refresh</button>
    </>
  );
}`
	},
	mutation: {
		lang: 'typescript',
		code: `import { useMutation } from './rpc.react';

function CreateForm() {
  const create = useMutation(rpc, 'create_user', {
    onSuccess: (data) => console.log('Created:', data),
    onError: (err) => console.error(err),
  });

  // Await the result
  const handleSubmit = async () => {
    try {
      const user = await create.mutateAsync({ name: 'Alice' });
      console.log('Created:', user);
    } catch (err) {
      // err is RpcError
    }
  };

  return (
    <>
      <button onClick={handleSubmit} disabled={create.isLoading}>
        {create.isLoading ? 'Creating...' : 'Create'}
      </button>
      {create.isError && <p>Error: {create.error?.message}</p>}
      {create.isSuccess && <p>Created: {create.data?.name}</p>}
    </>
  );
}`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
