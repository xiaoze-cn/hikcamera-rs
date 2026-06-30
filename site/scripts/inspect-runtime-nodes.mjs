import { chromium } from '@playwright/test';
import { spawn } from 'node:child_process';

const port = 4326;
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
      const response = await fetch(`${baseURL}/en/reference/runtime-dependencies/`);
      if (response.ok) return;
    } catch {}
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error('Preview server did not become ready');
}

try {
  await waitForServer();
  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 1280, height: 1000 } });
  await page.goto(`${baseURL}/en/reference/runtime-dependencies/`);
  const info = await page.evaluate(() => {
    const flow = document.querySelector('.react-flow');
    const flowBox = flow?.getBoundingClientRect();
    const viewport = document.querySelector('.react-flow__viewport');
    const transform = viewport?.style.transform;
    const nodes = Array.from(document.querySelectorAll('.react-flow__node')).map((node) => {
      const box = node.getBoundingClientRect();
      const label = node.textContent?.replace(/\s+/g, ' ').trim();
      return { label, left: box.left, top: box.top, width: box.width, height: box.height };
    });
    return { flowBox, transform, nodes };
  });
  console.log(JSON.stringify(info, null, 2));
  await browser.close();
} finally {
  server.kill();
  setTimeout(() => process.exit(0), 100);
}
