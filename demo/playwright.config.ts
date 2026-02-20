import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
	testDir: 'tests/e2e',
	timeout: 30_000,
	retries: 1,
	use: {
		baseURL: process.env.BASE_URL || 'https://vercel-rpc.vercel.app',
		trace: 'on-first-retry'
	},
	projects: [
		{
			name: 'chromium',
			use: { ...devices['Desktop Chrome'] }
		}
	]
});
