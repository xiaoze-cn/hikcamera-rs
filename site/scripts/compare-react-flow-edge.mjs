import { chromium } from "@playwright/test";
import { spawn } from "node:child_process";

const port = 4329;
const baseURL = `http://localhost:${port}`;
const server = spawn(
  "pnpm",
  ["exec", "astro", "preview", "--host", "localhost", "--port", String(port)],
  {
    cwd: new URL("..", import.meta.url),
    env: {
      ...process.env,
      TEMP: "D:/tmp/hikcamera-site",
      TMP: "D:/tmp/hikcamera-site",
      CI: "false",
    },
    stdio: ["ignore", "pipe", "pipe"],
  },
);

async function waitFor(path) {
  const deadline = Date.now() + 30_000;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(`${baseURL}${path}`);
      if (response.ok) return;
    } catch {}
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error("Preview server did not become ready");
}

async function inspect(page, path) {
  await page.goto(`${baseURL}${path}`);
  await page.waitForSelector(".react-flow__edge path.react-flow__edge-path", {
    timeout: 10_000,
    state: "attached",
  });
  return page.evaluate(() => {
    const path = document.querySelector(
      ".react-flow__edge path.react-flow__edge-path",
    );
    const edge = path?.closest(".react-flow__edge");
    const edgeSvg = path?.closest("svg");
    const edgePane = document.querySelector(".react-flow__edges");
    const viewport = document.querySelector(".react-flow__viewport");
    const renderer = document.querySelector(".react-flow__renderer");
    const flow = document.querySelector(".react-flow");
    const describe = (el) => {
      if (!el) return null;
      const style = getComputedStyle(el);
      const box = el.getBoundingClientRect();
      return {
        tag: el.tagName,
        className: el.getAttribute("class"),
        styleAttr: el.getAttribute("style"),
        box: {
          left: box.left,
          top: box.top,
          width: box.width,
          height: box.height,
        },
        display: style.display,
        position: style.position,
        overflow: style.overflow,
        overflowX: style.overflowX,
        overflowY: style.overflowY,
        zIndex: style.zIndex,
        opacity: style.opacity,
        transform: style.transform,
      };
    };
    const pathStyle = path && getComputedStyle(path);
    return {
      flow: describe(flow),
      renderer: describe(renderer),
      viewport: describe(viewport),
      edgePane: describe(edgePane),
      edge,
      edgeInfo: describe(edge),
      edgeSvg: describe(edgeSvg),
      path: path && {
        styleAttr: path.getAttribute("style"),
        className: path.getAttribute("class"),
        box: (() => {
          const b = path.getBoundingClientRect();
          return { left: b.left, top: b.top, width: b.width, height: b.height };
        })(),
        stroke: pathStyle.stroke,
        strokeWidth: pathStyle.strokeWidth,
        visibility: pathStyle.visibility,
        display: pathStyle.display,
      },
    };
  });
}

try {
  await waitFor("/react-flow-smoke/");
  const browser = await chromium.launch();
  const page = await browser.newPage({
    viewport: { width: 1280, height: 1000 },
  });
  const smoke = await inspect(page, "/react-flow-smoke/");
  const runtime = await inspect(page, "/en/reference/runtime-dependencies/");
  console.log(JSON.stringify({ smoke, runtime }, null, 2));
  await browser.close();
} finally {
  server.kill();
  setTimeout(() => process.exit(0), 100);
}
