import { chromium } from '@playwright/test';
import { spawn } from 'node:child_process';

const port = 4328;
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
  await page.waitForSelector('.react-flow__edge');
  const info = await page.evaluate(() => {
    const edge = document.querySelector('.react-flow__edge');
    const paths = Array.from(edge.querySelectorAll('path')).map((path) => ({
      className: path.getAttribute('class'),
      style: path.getAttribute('style'),
      stroke: getComputedStyle(path).stroke,
      strokeWidth: getComputedStyle(path).strokeWidth,
      fill: getComputedStyle(path).fill,
      outer: path.outerHTML.slice(0, 500),
    }));
    const edgeStyle = getComputedStyle(edge);
    const edges = document.querySelector('.react-flow__edges');
    const bg = document.querySelector('.react-flow__background');
    const nodes = document.querySelector('.react-flow__nodes');
    return {
      edgeClass: edge.getAttribute('class'),
      edgeStyle: {
        opacity: edgeStyle.opacity,
        visibility: edgeStyle.visibility,
        display: edgeStyle.display,
      },
      paths,
      edgesStyle: edges?.getAttribute('style'),
      bgIndex: bg ? getComputedStyle(bg).zIndex : null,
      edgesIndex: edges ? getComputedStyle(edges).zIndex : null,
      nodesIndex: nodes ? getComputedStyle(nodes).zIndex : null,
    };
  });
  console.log(JSON.stringify(info, null, 2));
  await browser.close();
} finally {
  server.kill();
  setTimeout(() => process.exit(0), 100);
}
