import { chromium } from "@playwright/test";
import { spawn } from "node:child_process";

const port = 4324;
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

async function waitForServer() {
  const deadline = Date.now() + 30_000;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(
        `${baseURL}/en/reference/runtime-dependencies/`,
      );
      if (response.ok) return;
    } catch {}
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error("Preview server did not become ready");
}

try {
  await waitForServer();
  const browser = await chromium.launch();
  const page = await browser.newPage({
    viewport: { width: 1280, height: 1000 },
  });
  await page.goto(`${baseURL}/en/reference/runtime-dependencies/`);
  await page.waitForSelector(".react-flow__edge path.react-flow__edge-path", {
    timeout: 10_000,
  });
  const info = await page.evaluate(() => {
    const flow = document.querySelector(".react-flow");
    const flowBox = flow?.getBoundingClientRect();
    const viewport = document.querySelector(".react-flow__viewport");
    const paths = Array.from(
      document.querySelectorAll(".react-flow__edge path.react-flow__edge-path"),
    ).slice(0, 8);
    return {
      flowBox: flowBox && {
        width: flowBox.width,
        height: flowBox.height,
        left: flowBox.left,
        top: flowBox.top,
      },
      transform: viewport?.style.transform,
      edges: paths.map((path) => {
        const style = getComputedStyle(path);
        const box = path.getBoundingClientRect();
        return {
          stroke: style.stroke,
          strokeWidth: style.strokeWidth,
          opacity: style.opacity,
          width: box.width,
          height: box.height,
          d: path.getAttribute("d"),
        };
      }),
    };
  });
  console.log(JSON.stringify(info, null, 2));
  await browser.close();
} finally {
  server.kill();
  setTimeout(() => process.exit(0), 100);
}
