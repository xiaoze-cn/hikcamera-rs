import { chromium } from "@playwright/test";
import { spawn } from "node:child_process";
import { mkdir } from "node:fs/promises";
import { fileURLToPath } from "node:url";

const port = 4327;
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
  await mkdir(new URL("../test-results/", import.meta.url), {
    recursive: true,
  });
  const browser = await chromium.launch();
  const page = await browser.newPage({
    viewport: { width: 1280, height: 1000 },
  });
  await page.goto(`${baseURL}/en/reference/runtime-dependencies/`);
  await page.waitForSelector(".react-flow__edge path.react-flow__edge-path", {
    timeout: 10_000,
  });
  await page.waitForTimeout(500);
  const screenshotPath = fileURLToPath(
    new URL("../test-results/runtime-flow-only.png", import.meta.url),
  );
  await page.locator(".react-flow").screenshot({ path: screenshotPath });
  const info = await page.evaluate(() => {
    const flowBox = document
      .querySelector(".react-flow")
      ?.getBoundingClientRect();
    const firstPath = document.querySelector(
      ".react-flow__edge path.react-flow__edge-path",
    );
    const pathBox = firstPath?.getBoundingClientRect();
    const style = firstPath ? getComputedStyle(firstPath) : null;
    return {
      flowBox: flowBox && {
        left: flowBox.left,
        top: flowBox.top,
        width: flowBox.width,
        height: flowBox.height,
      },
      pathBox: pathBox && {
        left: pathBox.left,
        top: pathBox.top,
        width: pathBox.width,
        height: pathBox.height,
      },
      stroke: style?.stroke,
      strokeWidth: style?.strokeWidth,
    };
  });
  console.log(JSON.stringify(info, null, 2));
  await browser.close();
} finally {
  server.kill();
  setTimeout(() => process.exit(0), 100);
}
