import { createRpcClient } from './rpc-client';

export const rpc = createRpcClient({ baseUrl: '/api' });
