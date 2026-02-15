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
			res.end(
				JSON.stringify({
					result: { type: 'response', data: `Hello, ${name} from Rust on Vercel!` }
				})
			);
		});

		middlewares.use('/api/time', (_req, res) => {
			res.setHeader('Content-Type', 'application/json');
			res.end(
				JSON.stringify({
					result: {
						type: 'response',
						data: {
							timestamp: Math.floor(Date.now() / 1000),
							message: 'Current server time in seconds since epoch'
						}
					}
				})
			);
		});

		middlewares.use('/api/echo', (req, res) => {
			let body = '';
			req.on('data', (chunk: Buffer) => {
				body += chunk.toString();
			});
			req.on('end', () => {
				const input = JSON.parse(body || '{}');
				const transformed = input.uppercase ? input.message.toUpperCase() : input.message;
				res.setHeader('Content-Type', 'application/json');
				res.end(
					JSON.stringify({
						result: {
							type: 'response',
							data: { original: input.message, transformed, length: transformed.length }
						}
					})
				);
			});
		});

		middlewares.use('/api/math', (req, res) => {
			const url = new URL(req.url ?? '/', 'http://localhost');
			const raw = url.searchParams.get('input');
			const input = raw ? JSON.parse(raw) : { a: 0, b: 0, op: 'Add' };
			const ops: Record<string, [string, (a: number, b: number) => number | null]> = {
				Add: ['+', (a, b) => a + b],
				Subtract: ['-', (a, b) => a - b],
				Multiply: ['×', (a, b) => a * b],
				Divide: ['÷', (a, b) => (b === 0 ? null : a / b)]
			};
			const [symbol, fn] = ops[input.op] ?? ['+', (a: number, b: number) => a + b];
			const result = fn(input.a, input.b);
			res.setHeader('Content-Type', 'application/json');
			if (result === null) {
				res.statusCode = 400;
				res.end(JSON.stringify({ error: { type: 'error', message: 'Division by zero' } }));
			} else {
				res.end(
					JSON.stringify({
						result: {
							type: 'response',
							data: { result, expression: `${input.a} ${symbol} ${input.b} = ${result}` }
						}
					})
				);
			}
		});

		middlewares.use('/api/stats', (req, res) => {
			const url = new URL(req.url ?? '/', 'http://localhost');
			const raw = url.searchParams.get('input');
			const numbers: number[] = raw ? JSON.parse(raw) : [];
			res.setHeader('Content-Type', 'application/json');
			if (numbers.length === 0) {
				res.statusCode = 400;
				res.end(
					JSON.stringify({
						error: { type: 'error', message: 'Cannot compute stats for empty list' }
					})
				);
				return;
			}
			const sum = numbers.reduce((a, b) => a + b, 0);
			const frequencies: Record<string, number> = {};
			numbers.forEach((n) => {
				frequencies[String(n)] = (frequencies[String(n)] || 0) + 1;
			});
			res.end(
				JSON.stringify({
					result: {
						type: 'response',
						data: {
							count: numbers.length,
							sum,
							mean: sum / numbers.length,
							min: Math.min(...numbers),
							max: Math.max(...numbers),
							frequencies
						}
					}
				})
			);
		});

		middlewares.use('/api/status', (_req, res) => {
			res.setHeader('Content-Type', 'application/json');
			res.end(
				JSON.stringify({
					result: {
						type: 'response',
						data: {
							name: 'vercel-rpc-demo',
							status: 'Healthy',
							uptime_secs: Math.floor(Date.now() / 1000),
							version: '0.1.0'
						}
					}
				})
			);
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
