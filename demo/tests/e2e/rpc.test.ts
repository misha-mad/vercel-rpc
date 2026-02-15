import { test, expect } from "@playwright/test";

test.describe("RPC page — full e2e cycle", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/rpc");
  });

  test("page loads with correct heading", async ({ page }) => {
    await expect(page.locator("h1")).toHaveText("RPC + SvelteKit + Rust");
  });

  test("displays server time on load", async ({ page }) => {
    const timeText = page.locator("strong").first();
    // Wait for the time to load (not "loading...")
    await expect(timeText).not.toHaveText("loading...", { timeout: 10_000 });
  });

  test("hello query returns greeting", async ({ page }) => {
    const input = page.locator('input[type="text"]');
    const button = page.locator("button");

    await input.fill("Playwright");
    await button.click();

    const result = page.locator(".result");
    await expect(result).toContainText("Hello, Playwright", { timeout: 10_000 });
  });

  test("hello query works with empty input", async ({ page }) => {
    const input = page.locator('input[type="text"]');
    const button = page.locator("button");

    await input.clear();
    await input.fill("");
    await button.click();

    const result = page.locator(".result");
    await expect(result).toBeVisible({ timeout: 10_000 });
  });

  test("navigation link to RPC exists in header", async ({ page }) => {
    const rpcLink = page.locator('nav a[href="/rpc"]');
    await expect(rpcLink).toBeVisible();
    await expect(rpcLink).toHaveText("RPC");
  });
});

test.describe("API endpoints — direct HTTP", () => {
  test("GET /api/hello returns greeting", async ({ request }) => {
    const input = encodeURIComponent(JSON.stringify("E2E"));
    const res = await request.get(`/api/hello?input=${input}`);
    expect(res.ok()).toBe(true);

    const json = await res.json();
    expect(json.result.data).toContain("Hello, E2E");
  });

  test("GET /api/time returns timestamp", async ({ request }) => {
    const res = await request.get("/api/time");
    expect(res.ok()).toBe(true);

    const json = await res.json();
    const data = json.result.data;
    expect(data).toHaveProperty("timestamp");
    expect(data).toHaveProperty("message");
    expect(typeof data.timestamp).toBe("number");
    expect(data.timestamp).toBeGreaterThan(1_000_000_000);
  });

  test("GET /api/hello without input returns default", async ({ request }) => {
    const res = await request.get("/api/hello");
    expect(res.ok()).toBe(true);

    const json = await res.json();
    expect(json.result.data).toContain("Hello,");
  });
});
