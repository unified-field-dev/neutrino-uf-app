import { test as base, expect, type Page } from "@playwright/test";

export type SeedAuthKind =
  | "anonymous"
  | "admin"
  | "operator"
  | "requestor"
  | "outsider"
  | "unverified";

export type SeedFixtures = {
  outsider_secret_id: string;
  outsider_secret_name: string;
  admin_secret_id: string;
  admin_secret_name: string;
};

export async function seedAuth(page: Page, auth: SeedAuthKind) {
  const res = await page.request.post("/api/test/seed-data", {
    data: { auth },
  });
  expect(res.ok()).toBeTruthy();
  return res.json() as Promise<{
    ok: boolean;
    auth: string;
    fixtures: SeedFixtures;
  }>;
}

/**
 * Wait for Orbital boot overlay to finish and hydrate to mark the document ready.
 */
export async function waitForHydrated(page: Page, timeoutMs = 240_000) {
  await expect
    .poll(
      async () =>
        page.evaluate(() => {
          const html = document.documentElement;
          if (html.getAttribute("data-orbital-boot-state") === "error") {
            return "error";
          }
          if (html.getAttribute("data-orbital-hydrated") === "true") {
            return "ready";
          }
          return "loading";
        }),
      { timeout: timeoutMs },
    )
    .not.toBe("error");
  await expect
    .poll(
      async () =>
        page.evaluate(
          () => document.documentElement.getAttribute("data-orbital-hydrated") === "true",
        ),
      { timeout: timeoutMs },
    )
    .toBe(true);
  await expect(page.getByTestId("orbital-boot-overlay")).toHaveCount(0, {
    timeout: 60_000,
  });
  await expect(page.getByTestId("e2e-auth-bootstrap")).toBeAttached({
    timeout: 30_000,
  });
}

/** Higgs / server-fn deny surfaces as an Orbital error MessageBar (dialog or page). */
export async function expectMutationDenied(page: Page) {
  await expect(
    page
      .locator(
        [
          ".orbital-message-bar--error",
          "[data-testid='neutrino-create-error']",
          "[data-testid='neutrino-reveal-error']",
          "[data-testid='neutrino-rotate-error']",
          "[data-testid='neutrino-delete-error']",
        ].join(", "),
      )
      .first(),
  ).toBeVisible({ timeout: 60_000 });
}

/** Open the row actions Menu (Orbital trigger button). */
export async function openSecretActions(page: Page, name: string) {
  await page
    .getByTestId(`neutrino-secret-actions-${name}`)
    .getByRole("button", { name: /Open secret actions/i })
    .click();
}

export const test = base;
export { expect };
