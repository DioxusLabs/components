import { chromium } from "playwright";

const URL_BASE = "http://127.0.0.1:56476/component/?name=combobox";
const OUT_DIR = "/tmp/cb-shots";
import { mkdirSync } from "node:fs";
mkdirSync(OUT_DIR, { recursive: true });

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 720, height: 540 }, deviceScaleFactor: 2 });

async function shot(name) {
  await page.screenshot({ path: `${OUT_DIR}/${name}.png` });
  console.log(`saved ${name}.png`);
}

// LIGHT MODE
await page.goto(`${URL_BASE}&dark_mode=false`, { timeout: 120_000, waitUntil: "networkidle" });
await page.waitForSelector(".dx-combobox-trigger", { timeout: 60_000 });
await page.waitForTimeout(500);

const trigger = page.locator(".dx-combobox-trigger").first();
await trigger.scrollIntoViewIfNeeded();
const box = await trigger.boundingBox();

// 1. light, closed
await page.screenshot({
  path: `${OUT_DIR}/light-closed.png`,
  clip: {
    x: Math.max(0, box.x - 24),
    y: Math.max(0, box.y - 24),
    width: 248,
    height: 100,
  },
});
console.log("saved light-closed.png");

// 2. light, open
await trigger.click();
await page.waitForSelector(".dx-combobox-content[data-state='open']", { timeout: 5000 });
await page.waitForTimeout(300);
await page.screenshot({
  path: `${OUT_DIR}/light-open.png`,
  clip: {
    x: Math.max(0, box.x - 24),
    y: Math.max(0, box.y - 24),
    width: 248,
    height: 380,
  },
});
console.log("saved light-open.png");

// 3. light, open + first option highlighted via arrow
await page.keyboard.press("ArrowDown");
await page.waitForTimeout(200);
await page.screenshot({
  path: `${OUT_DIR}/light-open-highlighted.png`,
  clip: {
    x: Math.max(0, box.x - 24),
    y: Math.max(0, box.y - 24),
    width: 248,
    height: 380,
  },
});
console.log("saved light-open-highlighted.png");

// DARK MODE
await page.goto(`${URL_BASE}&dark_mode=true`, { timeout: 120_000, waitUntil: "networkidle" });
await page.waitForSelector(".dx-combobox-trigger", { timeout: 60_000 });
await page.waitForTimeout(500);
const trigger2 = page.locator(".dx-combobox-trigger").first();
await trigger2.scrollIntoViewIfNeeded();
const box2 = await trigger2.boundingBox();

await page.screenshot({
  path: `${OUT_DIR}/dark-closed.png`,
  clip: {
    x: Math.max(0, box2.x - 24),
    y: Math.max(0, box2.y - 24),
    width: 248,
    height: 100,
  },
});
console.log("saved dark-closed.png");

await trigger2.click();
await page.waitForSelector(".dx-combobox-content[data-state='open']", { timeout: 5000 });
await page.waitForTimeout(300);
await page.screenshot({
  path: `${OUT_DIR}/dark-open.png`,
  clip: {
    x: Math.max(0, box2.x - 24),
    y: Math.max(0, box2.y - 24),
    width: 248,
    height: 380,
  },
});
console.log("saved dark-open.png");

await page.keyboard.press("ArrowDown");
await page.waitForTimeout(200);
await page.screenshot({
  path: `${OUT_DIR}/dark-open-highlighted.png`,
  clip: {
    x: Math.max(0, box2.x - 24),
    y: Math.max(0, box2.y - 24),
    width: 248,
    height: 380,
  },
});
console.log("saved dark-open-highlighted.png");

await browser.close();
