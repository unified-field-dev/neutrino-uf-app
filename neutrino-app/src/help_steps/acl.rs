//! Spotlight steps for the ACL placeholder (`/secrets/acl`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Centered ACL product intro.
#[help_spotlight_step(
    route = "/secrets/acl",
    feature_highlight = "secrets-acl-intro",
    title = "Sharing, coming soon",
    order = 10
)]
#[component]
pub fn SecretsAclIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-secrets-acl-intro",
        "An access control list (ACL) will let you share one secret with specific people or groups without opening the whole vault.",
        Some("This page is reserved for that editor. Until it ships, you see a placeholder, and you manage credentials on the Secrets list."),
        &[],
    )
}

/// ACL page title.
#[help_spotlight_step(
    route = "/secrets/acl",
    feature_highlight = "secrets-acl-title",
    title = "Secret ACLs",
    spotlight = "secrets-acl-title",
    position = "bottom",
    order = 20
)]
#[component]
pub fn SecretsAclTitleHelp() -> impl IntoView {
    help_stack(
        "help-step-secrets-acl-title",
        "When the matrix UI lands, this heading will still mark the sharing workspace for a single secret.",
        None,
        &[],
    )
}

/// Empty-state placeholder.
#[help_spotlight_step(
    route = "/secrets/acl",
    feature_highlight = "secrets-acl-empty",
    title = "What you see today",
    spotlight = "secrets-acl-empty",
    position = "bottom",
    order = 30
)]
#[component]
pub fn SecretsAclEmptyHelp() -> impl IntoView {
    help_stack(
        "help-step-secrets-acl-empty",
        "The empty state means fine-grained ACL editing is not available yet. It is a placeholder, and it does not mean your vault has no secrets.",
        Some("Today, access is coarse: roles get permissions such as read metadata, reveal, write, or rotate. Those checks run on the server when you press a button."),
        &[],
    )
}

/// Nav back to Secrets from ACL page.
#[help_spotlight_step(
    route = "/secrets/acl",
    feature_highlight = "secrets-acl-nav-secrets",
    title = "Back to the vault",
    spotlight = "secrets-nav-secrets",
    position = "right",
    order = 40
)]
#[component]
pub fn SecretsAclNavSecretsHelp() -> impl IntoView {
    help_stack(
        "help-step-secrets-acl-nav-secrets",
        "Open Secrets for create, reveal, rotate, and delete. Help, then Replay, restarts this page's tour.",
        None,
        &[],
    )
}
