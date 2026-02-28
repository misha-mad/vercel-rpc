import { test, expect } from '@playwright/test';

test.describe('Landing page', () => {
	test.beforeEach(async ({ page }) => {
		await page.goto('/');
	});

	test('page loads with correct heading', async ({ page }) => {
		await expect(page.locator('h1')).toContainText('metaxy');
	});

	test('navigation links are present in header', async ({ page }) => {
		const docsLink = page.locator('nav a[href="/docs"]');
		await expect(docsLink).toBeVisible();
		await expect(docsLink).toHaveText('Docs');
	});
});

test.describe('API endpoints — direct HTTP', () => {
	test('GET /api/hello returns greeting', async ({ request }) => {
		const input = encodeURIComponent(JSON.stringify('E2E'));
		const res = await request.get(`/api/hello?input=${input}`);
		expect(res.ok()).toBe(true);
		const json = await res.json();
		expect(json.result.data).toContain('Hello, E2E');
	});

	test('GET /api/time returns timestamp', async ({ request }) => {
		const res = await request.get('/api/time');
		expect(res.ok()).toBe(true);
		const json = await res.json();
		const data = json.result.data;
		expect(data).toHaveProperty('timestamp');
		expect(typeof data.timestamp).toBe('number');
	});

	test('GET /api/math computes addition', async ({ request }) => {
		const input = encodeURIComponent(JSON.stringify({ a: 5, b: 3, op: 'Add' }));
		const res = await request.get(`/api/math?input=${input}`);
		expect(res.ok()).toBe(true);
		const json = await res.json();
		expect(json.result.data.result).toBe(8);
	});

	test('POST /api/echo transforms message', async ({ request }) => {
		const res = await request.post('/api/echo', {
			data: { message: 'hello', uppercase: true }
		});
		expect(res.ok()).toBe(true);
		const json = await res.json();
		expect(json.result.data.transformed).toBe('HELLO');
		expect(json.result.data.length).toBe(5);
	});

	test('GET /api/types returns type showcase', async ({ request }) => {
		const res = await request.get('/api/types');
		expect(res.ok()).toBe(true);
		const json = await res.json();
		const data = json.result.data;
		// HashSet<String> → string[]
		expect(Array.isArray(data.hash_set)).toBe(true);
		expect(data.hash_set).toContain('rust');
		expect(data.hash_set).toContain('typescript');
		expect(data.hash_set).toContain('rpc');
		// BTreeSet<i32> → number[] (sorted)
		expect(data.btree_set).toEqual([1, 2, 3]);
		// Box<String> → string
		expect(data.boxed).toBe('boxed value');
		// Cow<str> → string
		expect(data.cow).toBe('borrowed cow');
	});

	test('GET /api/secret without token returns error', async ({ request }) => {
		const res = await request.get('/api/secret');
		expect(res.ok()).toBe(false);
		const json = await res.json();
		expect(json.error.message).toContain('Unauthorized');
	});

	test('GET /api/secret with valid token returns secret', async ({ request }) => {
		const res = await request.get('/api/secret', {
			headers: { Authorization: 'Bearer secret-token-123' }
		});
		expect(res.ok()).toBe(true);
		const json = await res.json();
		expect(json.result.data).toBe('Top secret: the cake is a lie.');
	});
});
