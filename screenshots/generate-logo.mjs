// Generates the VS Code extension logo from logo-vscode.html
// Usage: npx playwright install chromium && node generate-logo.mjs
import { chromium } from "playwright";
import { fileURLToPath } from "url";
import { dirname, join } from "path";

const __dirname = dirname(fileURLToPath(import.meta.url));

const SIZE = 128;

async function main() {
  const browser = await chromium.launch();
  const context = await browser.newContext({
    viewport: { width: SIZE, height: SIZE },
    deviceScaleFactor: 1,
  });

  const page = await context.newPage();
  const htmlPath = join(__dirname, "logo-vscode.html");
  await page.goto(`file://${htmlPath}`);
  await page.waitForFunction(() => document.fonts.ready);
  await page.waitForTimeout(300);

  const outputPath = join(__dirname, "..", "vscode", "warm-burnout-logo-vscode.png");
  await page.screenshot({ path: outputPath, type: "png" });
  console.log(`  warm-burnout-logo-vscode.png (${SIZE}x${SIZE})`);

  await page.close();
  await browser.close();
  console.log("\nDone.");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
