# neutrino-uf-app-e2e

Leptos host that mounts Neutrino secrets pages for Playwright. Lab-only:
insecure session cookies, `POST /api/test/seed-data`, harness auth (no lepton sign-in).

## Run

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-neutrino-uf-app
export CARGO_PROFILE_DEV_DEBUG=0
# from neutrino-uf-app workspace root
cd neutrino-uf-app-e2e/end2end && npm ci && npx playwright install chromium && cd ../..
cargo leptos end-to-end --project neutrino-uf-app-e2e
```

Host listens on `127.0.0.1:3160`. Do not Ctrl-C; the run exits when Playwright finishes.

The lab host mounts the same page components as `NeutrinoRoutes`, without `Lazy`
(wasm-split Lazy under `ParentRoute` panics on hydrate in the current Leptos pin).
Production hosts keep `NeutrinoRoutes` + `--split` for code-splitting.

## Seed

`POST /api/test/seed-data` with JSON
`{ "auth": "admin" | "operator" | "requestor" | "outsider" | "unverified" | "anonymous" }`.

## Scenario catalog (implemented)

Auth: `pw-vault-auth-anonymous-sad`, `pw-vault-auth-unverified-sad`

Vault CRUD: `pw-vault-crud-happy` (create → list → reveal → rotate → delete)

Validation: `pw-vault-create-blank-sad`, `pw-vault-rotate-blank-sad`

Authz (outsider read-only): `pw-vault-write-denied-sad`, `pw-vault-reveal-denied-sad`, `pw-vault-rotate-denied-sad`, `pw-vault-delete-denied-sad`

Partial role (requestor: read+write+rotate, no reveal): `pw-vault-requestor-mutate-happy`, `pw-vault-requestor-reveal-denied-sad`

ACL placeholder route: `pw-vault-acl-placeholder-happy`

### Deferred

| ID | Why deferred |
|----|--------------|
| ACL grant editing UI | Placeholder page only — route smoke covers `/secrets/acl`; grant matrix not shipped |
| Lepton MFA / OAuth / confirm | Owned by `lepton-auth-ui-e2e` / `lepton-uf-app-e2e` |
| Chronon / Boson / Photon IsolatedLab | Broader host platform composition, not Neutrino UI |
