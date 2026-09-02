//! Gate: neutrino-app and the e2e lab host are members of this workspace.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn neutrino_product_workspace_members_happy_path() {
    let root_path = workspace_root().join("Cargo.toml");
    let Ok(root) = fs::read_to_string(&root_path) else {
        panic!(
            "failed to read workspace Cargo.toml at {}",
            root_path.display()
        );
    };
    for member in ["neutrino-app", "neutrino-uf-app-e2e"] {
        assert!(
            root.contains(&format!("\"{member}\"")),
            "workspace must list {member}"
        );
        assert!(
            workspace_root().join(member).join("Cargo.toml").is_file(),
            "missing crate dir {member}"
        );
    }
}
