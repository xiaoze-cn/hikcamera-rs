import { chromium } from '@playwright/test';
import { spawn } from 'node:child_process';
import { mkdir } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';

const port = 4325;
const baseURL = `http://localhost:${port}`;
const server = spawn('pnpm', ['exec', 'astro', 'preview', '--host', 'localhost', '--port', String(port)], {
  cwd: new URL('..', import.meta.url),
  env: { ...process.env, TEMP: 'D:/tmp/hikcamera-site', TMP: 'D:/tmp/hikcamera-site', CI: 'false' },
  stdio: ['ignore', 'pipe', 'pipe'],
});

async function waitForServer() {
  const deadline = Date.now() + 30_000;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(`${baseURL}/react-flow-smoke/`);
      if (response.ok) return;
    } catch {}
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error('Preview server did not become ready');
}

try {
  await waitForServer();
  await mkdir(new URL('../test-results/', import.meta.url), { recursive: true });
  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 900, height: 500 } });
  await page.goto(`${baseURL}/react-flow-smoke/`);
  const screenshotPath = fileURLToPath(new URL('../test-results/react-flow-smoke.png', import.meta.url));
  await page.locator('.react-flow').screenshot({ path: screenshotPath });
  const counts = await page.evaluate(() => ({
    nodes: document.querySelectorAll('.react-flow__node').length,
    edges: document.querySelectorAll('.react-flow__edge path.react-flow__edge-path').length,
  }));
  console.log(JSON.stringify(counts));
  await browser.close();
} finally {
  server.kill();
  setTimeout(() => process.exit(0), 100);
}
