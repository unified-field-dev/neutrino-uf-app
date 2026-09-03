//! Help spotlight tour steps for Neutrino Secrets routes.
//!
//! Inventory is registered via [`uf_help_macros::help_spotlight_step`]. Call
//! [`ensure_help_steps_linked`] from the host or [`crate::NeutrinoRoutes`] so
//! `inventory` submissions survive linking.

mod acl;
mod secrets_create;
mod secrets_delete;
mod secrets_list;
mod secrets_reveal;
mod secrets_rotate;

use leptos::prelude::*;
use uf_product::components::{Caption1, SpacingSize};
use uf_product::primitives::{Body1, Flex};

/// Signals Help steps use to open vault dialogs without calling mutate/reveal APIs.
#[derive(Clone, Copy)]
pub struct SecretsTourDialogs {
    /// Create-secret dialog open flag.
    pub create_open: RwSignal<bool>,
    /// Reveal dialog open flag (teaching chrome; may be empty).
    pub reveal_open: RwSignal<bool>,
    /// Rotate dialog open flag.
    pub rotate_open: RwSignal<bool>,
    /// Delete dialog open flag.
    pub delete_open: RwSignal<bool>,
}

/// Shared step body: lead paragraph, optional detail, optional legend lines.
pub(crate) fn help_stack(
    testid: &'static str,
    lead: &'static str,
    detail: Option<&'static str>,
    legend: &'static [&'static str],
) -> impl IntoView {
    view! {
        <div data-testid=testid>
            <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                <Body1>{lead}</Body1>
                {detail.map(|d| view! { <Caption1>{d}</Caption1> })}
                {legend
                    .iter()
                    .copied()
                    .map(|line| view! { <Caption1>{line}</Caption1> })
                    .collect_view()}
            </Flex>
        </div>
    }
}

/// Force-link Neutrino Help spotlight inventory into the host binary.
///
/// Empty body; `#[help_spotlight_step]` submissions in child modules are retained
/// when this crate is linked and this function is called from routes or the host.
pub const fn ensure_help_steps_linked() {}
