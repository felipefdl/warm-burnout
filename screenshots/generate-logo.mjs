// Generates branding assets from HTML templates via Playwright.
// Usage: npx playwright install chromium && node generate-logo.mjs
import { chromium } from "playwright";
import { fileURLToPath } from "url";
import { dirname, join } from "path";

const __dirname = dirname(fileURLToPath(import.meta.url));

const ASSETS = [
  {
    html: "logo-vscode.html",
    output: join("..", "vscode", "warm-burnout-logo-vscode.png"),
    width: 128,
    height: 128,
    scale: 1,
  },
  {
    html: "og-image.html",
    output: join("..", "site", "og-image.png"),
    width: 1200,
    height: 630,
    scale: 2,
  },
];

async function main() {
  const browser = await chromium.launch();

  for (const { html, output, width, height, scale } of ASSETS) {
    const context = await browser.newContext({
      viewport: { width, height },
      deviceScaleFactor: scale,
    });

    const page = await context.newPage();
    const htmlPath = join(__dirname, html);
    await page.goto(`file://${htmlPath}`);
    await page.waitForFunction(() => document.fonts.ready);
    await page.waitForTimeout(500);

    const outputPath = join(__dirname, output);
    await page.screenshot({ path: outputPath, type: "png" });
    const actualW = width * scale;
    const actualH = height * scale;
    console.log(`  ${output} (${actualW}x${actualH}${scale > 1 ? " @" + scale + "x" : ""})`);

    await page.close();
    await context.close();
  }

  await browser.close();
  console.log("\nDone.");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
