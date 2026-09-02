//! Per-secret ACL polish entry point for Neutrino UX.
//!
//! Today, coarse access is enforced via app permissions; fine-grained per-secret ACL UI will layer
//! on the listing + detail flows. This page is a stable route for future deep-links.

use leptos::prelude::*;
use uf_product::components::{ContentContainer, EmptyState, SpacingSize, Title3};
use uf_product::primitives::Flex;

/// Stable placeholder route for future per-secret ACL editing.
#[component]
pub fn AclManagePage() -> impl IntoView {
    view! {
        <ContentContainer data_testid="neutrino-acl-placeholder-page">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Secret ACLs"</Title3>
                <EmptyState
                    message="Fine-grained per-secret ACL editing is not available yet."
                    description="Use the secrets list to rotate and audit; ACL matrix UI will land here."
                />
            </Flex>
        </ContentContainer>
    }
}
