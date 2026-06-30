import { chromium } from '@playwright/test';
import { spawn } from 'node:child_process';

const port = 4331;
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
  await page.waitForSelector('.react-flow__edge path.react-flow__edge-path', { timeout: 10_000 });
  const info = await page.evaluate(() => {
    const edge = document.querySelector('.react-flow__edge');
    const svg = edge?.closest('svg');
    const path = edge?.querySelector('path.react-flow__edge-path');
    const edgeRect = edge?.getBoundingClientRect();
    const svgRect = svg?.getBoundingClientRect();
    const pathRect = path?.getBoundingClientRect();
    return {
      edgeOuter: edge?.outerHTML.slice(0, 1200),
      svgOuter: svg?.outerHTML.slice(0, 1200),
      svgAttrs: svg && {
        width: svg.getAttribute('width'),
        height: svg.getAttribute('height'),
        viewBox: svg.getAttribute('viewBox'),
        style: svg.getAttribute('style'),
        className: svg.getAttribute('class'),
      },
      edgeRect: edgeRect && { left: edgeRect.left, top: edgeRect.top, width: edgeRect.width, height: edgeRect.height },
      svgRect: svgRect && { left: svgRect.left, top: svgRect.top, width: svgRect.width, height: svgRect.height },
      pathRect: pathRect && { left: pathRect.left, top: pathRect.top, width: pathRect.width, height: pathRect.height },
    };
  });
  console.log(JSON.stringify(info, null, 2));
  await browser.close();
} finally {
  server.kill();
  setTimeout(() => process.exit(0), 100);
}
