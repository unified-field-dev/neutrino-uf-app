import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-vault-auth", () => {
  test("pw-vault-auth-anonymous-sad", async ({ page }) => {
    await seedAuth(page, "anonymous");
    await page.goto("/secrets", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByTestId("neutrino-secrets-list-page")).toHaveCount(0);
  });

  test("pw-vault-auth-unverified-sad", async ({ page }) => {
    await seedAuth(page, "unverified");
    await page.goto("/secrets", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("neutrino-secrets-list-page")).toHaveCount(0);
    await expect(
      page.getByTestId("email-verification-required-empty-state"),
    ).toBeAttached({ timeout: 30_000 });
  });
});
