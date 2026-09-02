import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-vault-acl", () => {
  test("pw-vault-acl-placeholder-happy", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/secrets/acl", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("neutrino-acl-placeholder-page")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByText("Secret ACLs", { exact: true })).toBeVisible();
    await expect(page.getByText(/ACL matrix UI will land here/i)).toBeVisible();
  });
});
