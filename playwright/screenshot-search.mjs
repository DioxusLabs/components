import { chromium } from "playwright";
import { mkdirSync } from "node:fs";

const URL = "http://127.0.0.1:56476/component/?name=combobox&dark_mode=false";
const OUT_DIR = "/tmp/cb-shots";
mkdirSync(OUT_DIR, { recursive: true });

const browser = await chromium.launch();
const page = await browser.newPage({
  viewport: { width: 720, height: 600 },
  deviceScaleFactor: 2,
});

await page.goto(URL, { timeout: 120_000, waitUntil: "networkidle" });
await page.waitForSelector(".dx-combobox-trigger", { timeout: 60_000 });
await page.waitForTimeout(500);

const trigger = page.locator(".dx-combobox-trigger").first();
await trigger.scrollIntoViewIfNeeded();
const box = await trigger.boundingBox();
const clip = {
  x: Math.max(0, box.x - 24),
  y: Math.max(0, box.y - 24),
  width: 248,
  height: 380,
};

await trigger.click();
await page.waitForSelector(".dx-combobox-content[data-state='open']", { timeout: 5000 });
await page.waitForTimeout(300);

// 1. baseline open
await page.screenshot({ path: `${OUT_DIR}/search-empty.png`, clip });
console.log("saved search-empty.png");

// 2. type "svk" — should match only SvelteKit
const input = page.locator(".dx-combobox-input");
await input.fill("svk");
await page.waitForTimeout(250);
await page.screenshot({ path: `${OUT_DIR}/search-svk.png`, clip });
console.log("saved search-svk.png");

// 3. clear, type "nxt" — should match Next.js (and maybe Nuxt.js)
await input.fill("");
await page.waitForTimeout(150);
await input.fill("nxt");
await page.waitForTimeout(250);
await page.screenshot({ path: `${OUT_DIR}/search-nxt.png`, clip });
console.log("saved search-nxt.png");

// 4. type "zzz" — empty state
await input.fill("");
await page.waitForTimeout(150);
await input.fill("zzz");
await page.waitForTimeout(250);
await page.screenshot({ path: `${OUT_DIR}/search-empty-state.png`, clip });
console.log("saved search-empty-state.png");

await browser.close();
