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
});
