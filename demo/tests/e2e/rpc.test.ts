import { test, expect } from '@playwright/test';

test.describe('RPC page — full e2e cycle', () => {
	test.beforeEach(async ({ page }) => {
		await page.goto('/examples');
	});

	test('page loads with correct heading', async ({ page }) => {
		await expect(page.locator('h1')).toContainText('vercel-rpc');
	});

	test('displays server time on load', async ({ page }) => {
		const timeText = page.locator('strong').first();
		await expect(timeText).not.toHaveText('loading...', { timeout: 10_000 });
	});

	test('hello query returns greeting', async ({ page }) => {
		const section = page.locator('section').filter({ hasText: 'Hello — Simple Query' });
		const input = section.locator('input[type="text"]');
		const button = section.locator('button').filter({ hasText: 'Say Hello' });

		await input.fill('Playwright');
		await button.click();

		const result = section.locator('.result');
		await expect(result).toContainText('Hello, Playwright', { timeout: 10_000 });
	});

	test('math query calculates result', async ({ page }) => {
		const section = page.locator('section').filter({ hasText: 'Math — Enum Input' });
		const button = section.locator('button').filter({ hasText: 'Calculate' });
		await button.click();

		const result = section.locator('.result.success');
		await expect(result).toBeVisible({ timeout: 10_000 });
	});

	test('echo mutation works', async ({ page }) => {
		const section = page.locator('section').filter({ hasText: 'Echo — Mutation' });
		const button = section.locator('button').filter({ hasText: 'Send' });
		await button.click();

		const result = section.locator('.result.success');
		await expect(result).toBeVisible({ timeout: 10_000 });
		await expect(result).toContainText('Original:');
	});

	test('stats query computes statistics', async ({ page }) => {
		const section = page.locator('section').filter({ hasText: 'Stats — Vec' });
		const button = section.locator('button').filter({ hasText: 'Compute' });
		await button.click();

		const result = section.locator('.result.success');
		await expect(result).toBeVisible({ timeout: 10_000 });
		await expect(result).toContainText('Count:');
		await expect(result).toContainText('Mean:');
	});

	test('status query shows service info', async ({ page }) => {
		const section = page.locator('section').filter({ hasText: 'Status — Enum in Struct' });
		const result = section.locator('.result.success');
		await expect(result).toBeVisible({ timeout: 10_000 });
		await expect(result).toContainText('vercel-rpc-demo');
		await expect(result).toContainText('Healthy');
	});

	test('profile query returns user profile with serde attributes', async ({ page }) => {
		const section = page.locator('section').filter({ hasText: 'Profile — Serde Attributes' });
		const button = section.locator('button').filter({ hasText: 'Fetch Profile' });
		await button.click();

		const result = section.locator('.result.success');
		await expect(result).toBeVisible({ timeout: 10_000 });
		await expect(result).toContainText('Alice');
		await expect(result).toContainText('admin');
		await expect(result).toContainText('sign-in');
		await expect(result).toContainText('profile_url');
	});

	test('types query returns type showcase', async ({ page }) => {
		const section = page.locator('section').filter({ hasText: 'Types — Expanded Type Mappings' });
		const button = section.locator('button').filter({ hasText: 'Fetch Types' });
		await button.click();

		const result = section.locator('.result.success');
		await expect(result).toBeVisible({ timeout: 10_000 });
		await expect(result).toContainText('sorted_ids');
		await expect(result).toContainText('boxed_label');
		await expect(result).toContainText('cow_message');
	});

	test('secret endpoint rejects without token', async ({ page }) => {
		const section = page.locator('section').filter({ hasText: 'Secret — Protected Endpoint' });
		const button = section.locator('button').filter({ hasText: 'Call without token' });
		await button.click();

		const result = section.locator('.result.error');
		await expect(result).toBeVisible({ timeout: 10_000 });
		await expect(result).toContainText('Unauthorized');
	});

	test('secret endpoint succeeds with token', async ({ page }) => {
		const section = page.locator('section').filter({ hasText: 'Secret — Protected Endpoint' });
		const button = section.locator('button').filter({ hasText: 'Call with token' });
		await button.click();

		const result = section.locator('.result.success');
		await expect(result).toBeVisible({ timeout: 10_000 });
		await expect(result).toContainText('the cake is a lie');
	});

	test('navigation link to RPC exists in header', async ({ page }) => {
		const examplesLink = page.locator('nav a[href="/examples"]');
		await expect(examplesLink).toBeVisible();
		await expect(examplesLink).toHaveText('Examples');
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
		expect(json.result.data.name).toBe('vercel-rpc-demo');
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
		const input = encodeURIComponent(JSON.stringify('e2e'));
		const res = await request.get(`/api/types?input=${input}`);
		expect(res.ok()).toBe(true);
		const json = await res.json();
		const data = json.result.data;
		// HashSet<String> → string[]
		expect(Array.isArray(data.tags)).toBe(true);
		expect(data.tags).toContain('rust');
		expect(data.tags).toContain('typescript');
		expect(data.tags).toContain('rpc');
		// BTreeSet<i32> → number[] (sorted)
		expect(data.sorted_ids).toEqual([1, 2, 3]);
		// Box<String> → string
		expect(data.boxed_label).toBe('Category: e2e');
		// Cow<str> → string
		expect(data.cow_message).toBe('Hello from Cow — serialized as a plain string!');
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
