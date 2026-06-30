import { expect, test } from "@playwright/test";

test("runtime dependency graph renders visible edges", async ({
  page,
}, testInfo) => {
  const consoleErrors = [];
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  await page.goto("/en/reference/runtime-dependencies/");

  const graph = page.locator(".runtime-deps");
  await expect(graph).toBeVisible();

  const nodes = page.locator(".react-flow__node");
  await expect(nodes.first()).toBeVisible();
  expect(await nodes.count()).toBeGreaterThan(5);
  await page.waitForTimeout(500);

  const edgePaths = page.locator(
    ".react-flow__edge path.react-flow__edge-path",
  );
  const edgeCount = await edgePaths.count();

  if (edgeCount === 0) {
    await graph.screenshot({
      path: testInfo.outputPath("runtime-deps-no-edges.png"),
    });
    console.log("browser console errors", consoleErrors);
  }

  expect(edgeCount).toBeGreaterThan(10);

  const invalidPaths = await edgePaths.evaluateAll((paths) =>
    paths
      .map((path) => {
        const style = getComputedStyle(path);
        const box = path.getBoundingClientRect();

        return {
          d: path.getAttribute("d"),
          stroke: style.stroke,
          strokeWidth: style.strokeWidth,
          width: box.width,
          height: box.height,
        };
      })
      .filter(
        ({ d, stroke, strokeWidth, width, height }) =>
          !d ||
          d.includes("NaN") ||
          stroke === "none" ||
          strokeWidth === "0px" ||
          (width === 0 && height === 0),
      ),
  );

  const visibleEdgeCount = await page
    .locator(".react-flow")
    .evaluate((flowElement) => {
      const flowBox = flowElement.getBoundingClientRect();
      const paths = Array.from(
        flowElement.querySelectorAll(
          ".react-flow__edge path.react-flow__edge-path",
        ),
      );

      return paths.filter((path) => {
        const style = getComputedStyle(path);
        if (style.stroke === "none" || style.strokeWidth === "0px")
          return false;

        const ctm = path.getScreenCTM();
        if (!ctm) return false;

        const length = path.getTotalLength();
        const samples = 16;
        for (let index = 0; index <= samples; index += 1) {
          const point = path.getPointAtLength((length * index) / samples);
          const screenPoint = new DOMPoint(point.x, point.y).matrixTransform(
            ctm,
          );
          if (
            screenPoint.x >= flowBox.left &&
            screenPoint.x <= flowBox.right &&
            screenPoint.y >= flowBox.top &&
            screenPoint.y <= flowBox.bottom
          ) {
            return true;
          }
        }

        return false;
      }).length;
    });

  if (invalidPaths.length > 0 || visibleEdgeCount < 6) {
    await graph.screenshot({
      path: testInfo.outputPath("runtime-deps-invalid-edges.png"),
    });
  }

  expect(invalidPaths).toEqual([]);
  expect(visibleEdgeCount).toBeGreaterThanOrEqual(6);
});
