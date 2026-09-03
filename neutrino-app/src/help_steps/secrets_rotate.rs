//! Rotate action and dialog spotlight steps (`/secrets`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::{help_stack, SecretsTourDialogs};

fn open_rotate_teaching() {
    if let Some(dialogs) = use_context::<SecretsTourDialogs>() {
        dialogs.rotate_open.set(true);
    }
}

/// Rotate row action button.
#[help_spotlight_step(
    route = "/secrets",
    feature_highlight = "secrets-rotate",
    title = "Rotate a value",
    spotlight = "secrets-action-rotate",
    position = "left",
    order = 190
)]
#[component]
pub fn SecretsRotateHelp() -> impl IntoView {
    help_stack(
        "help-step-secrets-rotate",
        "Rotate replaces the stored value. Use after a leak or a planned key swap. The version number goes up.",
        None,
        &[],
    )
}

/// Rotate dialog: new plaintext.
#[help_spotlight_step(
    route = "/secrets",
    feature_highlight = "secrets-rotate-plaintext",
    title = "New plaintext",
    spotlight = "secrets-rotate-plaintext",
    position = "bottom",
    order = 200
)]
#[component]
pub fn SecretsRotatePlaintextHelp() -> impl IntoView {
    Effect::new(move |_| {
        open_rotate_teaching();
    });
    help_stack(
        "help-step-secrets-rotate-plaintext",
        "Enter the replacement secret here. It is hidden like a password until you save the rotation.",
        None,
        &[],
    )
}

/// Rotate dialog: submit.
#[help_spotlight_step(
    route = "/secrets",
    feature_highlight = "secrets-rotate-submit",
    title = "Save rotation",
    spotlight = "secrets-rotate-submit",
    position = "top",
    order = 210
)]
#[component]
pub fn SecretsRotateSubmitHelp() -> impl IntoView {
    Effect::new(move |_| {
        open_rotate_teaching();
    });
    help_stack(
        "help-step-secrets-rotate-submit",
        "Rotate saves the new value when allowed. Cancel next to it closes without saving.",
        None,
        &[],
    )
}
