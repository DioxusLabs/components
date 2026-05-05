import { chromium } from "playwright";
import { mkdirSync } from "node:fs";

const URL = "http://127.0.0.1:56476/component/?name=combobox&dark_mode=false";
const OUT_DIR = "/tmp/cb-shots";
mkdirSync(OUT_DIR, { recursive: true });

const browser = await chromium.launch();
const page = await browser.newPage({
  viewport: { width: 720, height: 700 },
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
  height: 480,
};

await trigger.click();
await page.waitForSelector(".dx-combobox-content[data-state='open']", { timeout: 5000 });
await page.waitForTimeout(300);

const input = page.locator(".dx-combobox-input");

for (const q of ["s", "n", "i", "a"]) {
  await input.fill(q);
  await page.waitForTimeout(250);
  await page.screenshot({ path: `${OUT_DIR}/rank-${q}.png`, clip });
  console.log(`saved rank-${q}.png`);

  // Read DOM order directly (no need to consult CSS `order:` anymore — the
  // list emits options in relevance order, so DOM IS the visual order).
  const labels = await page.$$eval(".dx-combobox-option", (els) =>
    els.map((el) => el.textContent.trim())
  );
  console.log(`  query="${q}":`, labels);
}

await browser.close();
