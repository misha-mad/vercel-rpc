import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import type { Plugin, Connect } from 'vite';

// Local dev/preview mock for Rust API lambdas.
// In production, Vercel routes /api/* to compiled Rust binaries.
function apiMock(): Plugin {
	function attachHandlers(middlewares: Connect.Server) {
		middlewares.use('/api/hello', (req, res) => {
			const url = new URL(req.url ?? '/', 'http://localhost');
			const raw = url.searchParams.get('input');
			const name = raw ? JSON.parse(raw) : 'World';
			res.setHeader('Content-Type', 'application/json');
			res.end(JSON.stringify({
				result: { type: 'response', data: `Hello, ${name} from Rust on Vercel!` }
			}));
		});
		middlewares.use('/api/time', (_req, res) => {
			res.setHeader('Content-Type', 'application/json');
			res.end(JSON.stringify({
				result: {
					type: 'response',
					data: {
						timestamp: Math.floor(Date.now() / 1000),
						message: 'Current server time in seconds since epoch',
					}
				}
			}));
		});
	}

	return {
		name: 'api-mock',
		configureServer(server) {
			attachHandlers(server.middlewares);
		},
		configurePreviewServer(server) {
			attachHandlers(server.middlewares);
		}
	};
}

export default defineConfig({
	plugins: [apiMock(), sveltekit()]
});
