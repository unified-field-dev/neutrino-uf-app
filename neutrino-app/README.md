# neutrino-app

Orbital secrets admin UI (`NeutrinoRoutes` at `/secrets`).

Workspace root: [neutrino-uf-app](https://github.com/unified-field-dev/neutrino-uf-app).
Domain vault crate: [neutrino](https://github.com/unified-field-dev/neutrino).

Includes Help spotlight tours (`help_steps`); call `ensure_help_steps_linked` and enable
`offering-help` on the product shell. See crate rustdoc **Help spotlight tours**.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-neutrino-uf-app
cargo doc -p neutrino-app --features ssr --open
```
