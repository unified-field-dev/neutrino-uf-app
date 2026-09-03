//! Create-dialog field spotlight steps (`/secrets`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::{help_stack, SecretsTourDialogs};

fn open_create() {
    if let Some(dialogs) = use_context::<SecretsTourDialogs>() {
        dialogs.create_open.set(true);
    }
}

/// Create dialog: Name field.
#[help_spotlight_step(
    route = "/secrets",
    feature_highlight = "secrets-create-name",
    title = "Name field",
    spotlight = "secrets-create-name",
    position = "bottom",
    order = 100
)]
#[component]
pub fn SecretsCreateNameHelp() -> impl IntoView {
    Effect::new(move |_| {
        open_create();
    });
    help_stack(
        "help-step-secrets-create-name",
        "Type the label that will show in the table. Pick something people will recognize in ops reviews.",
        None,
        &[],
    )
}

/// Create dialog: Scope path.
#[help_spotlight_step(
    route = "/secrets",
    feature_highlight = "secrets-create-scope",
    title = "Scope path",
    spotlight = "secrets-create-scope",
    position = "bottom",
    order = 110
)]
#[component]
pub fn SecretsCreateScopeHelp() -> impl IntoView {
    Effect::new(move |_| {
        open_create();
    });
    help_stack(
        "help-step-secrets-create-scope",
        "Enter the grouping path for this secret. Use a consistent pattern so related credentials sit together in the list.",
        None,
        &[],
    )
}

/// Create dialog: Kind field.
#[help_spotlight_step(
    route = "/secrets",
    feature_highlight = "secrets-create-kind",
    title = "Kind field",
    spotlight = "secrets-create-kind",
    position = "bottom",
    order = 120
)]
#[component]
pub fn SecretsCreateKindHelp() -> impl IntoView {
    Effect::new(move |_| {
        open_create();
    });
    help_stack(
        "help-step-secrets-create-kind",
        "Enter what kind of credential this is (for example provider_api_token). Kind is metadata, not the password.",
        None,
        &[],
    )
}

/// Create dialog: Plaintext field.
#[help_spotlight_step(
    route = "/secrets",
    feature_highlight = "secrets-create-plaintext",
    title = "Plaintext",
    spotlight = "secrets-create-plaintext",
    position = "bottom",
    order = 130
)]
#[component]
pub fn SecretsCreatePlaintextHelp() -> impl IntoView {
    Effect::new(move |_| {
        open_create();
    });
    help_stack(
        "help-step-secrets-create-plaintext",
        "This is the actual secret value. The field hides it like a password. Do not paste it into tickets or screenshots.",
        None,
        &[],
    )
}

/// Create dialog: Cancel.
#[help_spotlight_step(
    route = "/secrets",
    feature_highlight = "secrets-create-cancel",
    title = "Cancel create",
    spotlight = "secrets-create-cancel",
    position = "top",
    order = 140
)]
#[component]
pub fn SecretsCreateCancelHelp() -> impl IntoView {
    Effect::new(move |_| {
        open_create();
    });
    help_stack(
        "help-step-secrets-create-cancel",
        "Cancel closes the form without writing a vault entry. Draft fields are discarded.",
        None,
        &[],
    )
}

/// Create dialog: Submit.
#[help_spotlight_step(
    route = "/secrets",
    feature_highlight = "secrets-create-submit",
    title = "Save new secret",
    spotlight = "secrets-create-submit",
    position = "top",
    order = 150
)]
#[component]
pub fn SecretsCreateSubmitHelp() -> impl IntoView {
    Effect::new(move |_| {
        open_create();
    });
    help_stack(
        "help-step-secrets-create-submit",
        "Create writes the new entry when your role allows writes. On success the row appears in the table.",
        None,
        &[],
    )
}
