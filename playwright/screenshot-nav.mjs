import { chromium } from "playwright";

const URL = "http://127.0.0.1:56476/component/?name=combobox&dark_mode=false";

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 720, height: 700 } });
await page.goto(URL, { timeout: 120_000, waitUntil: "networkidle" });
await page.waitForSelector(".dx-combobox-input", { timeout: 60_000 });
await page.waitForTimeout(500);

const trigger = page.locator(".dx-combobox-input").first();
await trigger.click();
await page.waitForSelector(".dx-combobox-content[data-state='open']", { timeout: 5000 });
await page.waitForTimeout(300);

const input = page.locator(".dx-combobox-input");
await input.fill("s");
await page.waitForTimeout(250);

// Read DOM order so we know expected sequence.
const expected = await page.$$eval(".dx-combobox-option", (els) =>
  els.map((el) => el.textContent.trim())
);
console.log("DOM order:", expected);

// Walk arrow-down from initial focus, capture which option is highlighted each step.
const traversed = [];
for (let i = 0; i < expected.length + 2; i++) {
  const focused = await page.$$eval(
    ".dx-combobox-option[data-highlighted='true']",
    (els) => els.map((el) => el.textContent.trim())
  );
  traversed.push(focused[0] ?? "(none)");
  await page.keyboard.press("ArrowDown");
  await page.waitForTimeout(80);
}
console.log("ArrowDown sequence:", traversed);

// Reset and walk up from end
await input.fill("s");
await page.waitForTimeout(200);
await page.keyboard.press("End");
await page.waitForTimeout(80);
const upTraversed = [];
for (let i = 0; i < expected.length + 2; i++) {
  const focused = await page.$$eval(
    ".dx-combobox-option[data-highlighted='true']",
    (els) => els.map((el) => el.textContent.trim())
  );
  upTraversed.push(focused[0] ?? "(none)");
  await page.keyboard.press("ArrowUp");
  await page.waitForTimeout(80);
}
console.log("ArrowUp sequence:", upTraversed);

await browser.close();
