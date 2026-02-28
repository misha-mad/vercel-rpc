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

	test('GET /api/status returns service status', async ({ request }) => {
		const res = await request.get('/api/status');
		expect(res.ok()).toBe(true);
		const json = await res.json();
		expect(json.result.data.name).toBe('metaxy-demo');
		expect(json.result.data.status).toBe('Healthy');
	});

	test('GET /api/math computes addition', async ({ request }) => {
		const input = encodeURIComponent(JSON.stringify({ a: 5, b: 3, op: 'Add' }));
		const res = await request.get(`/api/math?input=${input}`);
		expect(res.ok()).toBe(true);
		const json = await res.json();
		expect(json.result.data.result).toBe(8);
	});

	test('GET /api/stats computes statistics', async ({ request }) => {
		const input = encodeURIComponent(JSON.stringify([1, 2, 3, 4, 5]));
		const res = await request.get(`/api/stats?input=${input}`);
		expect(res.ok()).toBe(true);
		const json = await res.json();
		expect(json.result.data.count).toBe(5);
		expect(json.result.data.mean).toBe(3);
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

	test('GET /api/profile returns user profile with serde attributes', async ({ request }) => {
		const input = encodeURIComponent(JSON.stringify(1));
		const res = await request.get(`/api/profile?input=${input}`);
		expect(res.ok()).toBe(true);
		const json = await res.json();
		const data = json.result.data;
		// camelCase fields from #[serde(rename_all = "camelCase")]
		expect(data.userId).toBe(1);
		expect(data.displayName).toBe('Alice');
		expect(data.emailAddress).toBe('alice@example.com');
		// enum with #[serde(rename_all = "snake_case")]
		expect(data.role).toBe('admin');
		// enum with #[serde(rename_all = "kebab-case")]
		expect(data.lastEvent).toBe('sign-in');
		// #[serde(rename = "profile_url")] overrides camelCase
		expect(data.profile_url).toBe('https://example.com/alice');
		// #[serde(skip)] — internal_score must not be present
		expect(data).not.toHaveProperty('internalScore');
		expect(data).not.toHaveProperty('internal_score');
		// #[serde(default)] + Option<String>
		expect(data.avatarUrl).toBe('https://example.com/alice/avatar.png');
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
