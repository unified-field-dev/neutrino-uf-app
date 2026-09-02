//! Top-level route pages for the Secrets app: vault list and ACL placeholder.

mod acl_manage;
mod secrets_list;
pub use acl_manage::AclManagePage;
pub use secrets_list::SecretsListPage;
