import { expect, test } from "@playwright/test";

test("react flow smoke renders a visible edge", async ({ page }) => {
  await page.goto("/react-flow-smoke/");

  await expect(page.locator(".react-flow")).toBeVisible();
  await expect(page.locator(".react-flow__node")).toHaveCount(2);

  const edgePath = page
    .locator(".react-flow__edge path.react-flow__edge-path")
    .first();
  await expect(edgePath).toHaveCount(1);

  const edgeInfo = await edgePath.evaluate((path) => {
    const style = getComputedStyle(path);
    const box = path.getBoundingClientRect();
    return {
      d: path.getAttribute("d"),
      stroke: style.stroke,
      strokeWidth: style.strokeWidth,
      width: box.width,
      height: box.height,
    };
  });

  expect(edgeInfo.d).toBeTruthy();
  expect(edgeInfo.stroke).not.toBe("none");
  expect(edgeInfo.strokeWidth).not.toBe("0px");
  expect(edgeInfo.width).toBeGreaterThan(20);
});
