import { test, expect } from "@playwright/test";

test("all views render and capture produces a report without browser errors", async ({
  page,
}) => {
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto("/");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Movement accuracy, live",
  );
  await expect(page.getByText("SIMULATED FEED · DEMO")).toBeVisible();
  await page.getByRole("button", { name: /Dashboard/ }).click();
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Where your form is going",
  );
  await page.getByRole("button", { name: /Move library/ }).click();
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Every move, graded",
  );
  await expect(
    page.getByRole("button", { name: /REFERENCE CLIP/ }),
  ).toHaveCount(8);
  await page.getByRole("button", { name: /Session report/ }).click();
  await expect(
    page.getByText(
      "No reps captured yet — start a capture session to generate a breakdown.",
    ),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Start capture", exact: true })
    .click();
  await expect(
    page.getByText("DEMO RUNNING · 9 SIMULATED JOINTS"),
  ).toBeVisible();
  await expect(page.getByText("1/12", { exact: true })).toBeVisible({
    timeout: 8000,
  });
  await page.getByRole("button", { name: "End session", exact: true }).click();
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Session 041 breakdown",
  );
  await expect(page.getByText(/1 reps captured/)).toBeVisible();
  await page
    .getByRole("button", { name: "Resume capture", exact: true })
    .click();
  await expect(page.getByText("2/12", { exact: true })).toBeVisible({
    timeout: 8000,
  });
  await page.getByRole("button", { name: "End session", exact: true }).click();
  await page.reload();
  await expect(
    page.getByRole("button", { name: "Start capture", exact: true }),
  ).toBeVisible();
  expect(errors).toEqual([]);
});

for (const width of [320, 768, 1024, 1440]) {
  test(`views fit a ${width}px viewport`, async ({ page }) => {
    await page.setViewportSize({ width, height: 1000 });
    await page.goto("/");
    for (const view of [
      "Live session",
      "Dashboard",
      "Move library",
      "Session report",
    ]) {
      await page.getByRole("button", { name: new RegExp(view) }).click();
      expect(
        await page.evaluate(
          () => document.documentElement.scrollWidth <= innerWidth,
        ),
      ).toBe(true);
    }
    await page.getByRole("button", { name: /Live session/ }).click();
    await page.screenshot({
      path: `test-results/formline-${width}.png`,
      fullPage: true,
    });
  });
}

test("navigation and capture work with keyboard", async ({ page }) => {
  await page.goto("/");
  await page.keyboard.press("Tab");
  await expect(
    page.getByRole("button", { name: /Live session/ }),
  ).toBeFocused();
  await page.keyboard.press("Tab");
  await page.keyboard.press("Enter");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText(
    "Where your form is going",
  );
  await page
    .getByRole("button", { name: "Start capture", exact: true })
    .focus();
  await page.keyboard.press("Enter");
  await expect(
    page.getByRole("button", { name: "End session", exact: true }),
  ).toBeVisible();
});
