# Neutrino UF App

[GitHub](https://github.com/deathbreakfast/neutrino-uf-app) ·
`cargo doc -p neutrino-app --features ssr --open`

## About

Neutrino UF App is the Unified Field **operations UI** for sealed secrets under
`/secrets`. Domain vault APIs, Valence schemas, and Gauge `Secrets*` permissions
live in the sibling [neutrino](https://github.com/unified-field-dev/neutrino)
crate; this repo mounts the Orbital pages and Higgs `#[server]` wrappers
operators use.

- **UI (`neutrino-app`)** — list / create / reveal / rotate / delete, ACL manage,
  `NeutrinoRoutes`, `uf_app!` registration at app id `secrets` / path `/secrets`

Hosts supply Valence + auth, enable `ssr` / hydrate to match the host, and mount
`NeutrinoRoutes`. Crate-root rustdoc owns the **Features** index, mount guide
(Gauge bootstrap + `NeutrinoPermission` grants), and Feature flags.

## Getting started

```toml
[dependencies]
# Tracks main; pin rev for reproducible production builds.
neutrino-app = { git = "https://github.com/deathbreakfast/neutrino-uf-app", package = "neutrino-app", branch = "main", default-features = false }
neutrino = { git = "https://github.com/unified-field-dev/neutrino", package = "neutrino", branch = "main", default-features = false }
```

Mount teaching (ordered steps, hydrate variant, permission grants) is in
`cargo doc -p neutrino-app --features ssr` under **Mount Neutrino routes**.

Domain-only smoke (bootstrap → role gate → rotate/reveal without the UI graph)
lives in neutrino's
[`vault-host`](https://github.com/unified-field-dev/neutrino/tree/main/examples/vault-host).

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-neutrino-uf-app
cargo check -p neutrino-app --features ssr
```

## Security

Admin mutations require a session and Gauge `Secrets*` permissions (Higgs
`#[uf_product_macros::server(permission = "…")]`). Owner / vault-context checks
and Valence policy notes for the domain live in
[neutrino `SECURITY.md`](https://github.com/unified-field-dev/neutrino/blob/main/SECURITY.md).
Report vulnerabilities privately — do not open a public issue for
security-sensitive reports.

## Verify

Matches [docs/VERIFICATION.md](docs/VERIFICATION.md) and CI on PR / main:

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-neutrino-uf-app
cargo fmt -p neutrino-app -p neutrino-uf-app-e2e -- --check
cargo clippy -p neutrino-app --features ssr --all-targets -- -D warnings
cargo test -p neutrino-app --features ssr
cargo check -p neutrino-app --features ssr
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p neutrino-app --features ssr --no-deps
cargo leptos end-to-end --project neutrino-uf-app-e2e
```

Crate-root rustdoc includes a **Module map** (`layout`, `pages`, `server`, `permissions`, `shell`).
Domain CI and teaching-host gates live in the
[neutrino](https://github.com/unified-field-dev/neutrino) repo.

## FAQ

**Is this a standalone server?** No. `neutrino-app` mounts under a host `<Routes>`
tree. Persistence and seal/reveal live in `neutrino`; hosts supply Valence and
session chrome.

**Do I need this crate for backend vault calls?** No. Call `neutrino` vault /
`SecretStore` APIs from the domain crate alone. Depend on `neutrino-app` when
operators need the `/secrets` UI.
