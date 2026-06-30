import { chromium } from '@playwright/test';
import { spawn } from 'node:child_process';

const port = 4330;
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
  await page.waitForSelector('.runtime-dependency-edge path', { timeout: 10_000 });
  await page.waitForTimeout(500);
  const info = await page.evaluate(() => {
    const path = document.querySelector('.runtime-dependency-edge path');
    const ctm = path.getScreenCTM();
    const length = path.getTotalLength();
    const samples = [0.1, 0.25, 0.5, 0.75, 0.9].map((ratio) => {
      const p = path.getPointAtLength(length * ratio);
      const sp = new DOMPoint(p.x, p.y).matrixTransform(ctm);
      const el = document.elementFromPoint(sp.x, sp.y);
      const style = getComputedStyle(path);
      return {
        ratio,
        x: sp.x,
        y: sp.y,
        topElement: el && { tag: el.tagName, className: el.getAttribute('class'), text: el.textContent?.slice(0, 40) },
        pathMatchesTop: el === path,
        pathStroke: style.stroke,
        pathStrokeWidth: style.strokeWidth,
      };
    });
    return { samples };
  });
  console.log(JSON.stringify(info, null, 2));
  await browser.close();
} finally {
  server.kill();
  setTimeout(() => process.exit(0), 100);
}
