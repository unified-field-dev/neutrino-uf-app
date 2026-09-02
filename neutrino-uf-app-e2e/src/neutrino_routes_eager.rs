//! Eager `/secrets` routes — no Lazy (hydrate_body panics with nested Lazy).

use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route},
    path,
};
use neutrino_app::{AclManagePage, NeutrinoVerifiedGuardRouteView, SecretsListPage};

/// Same paths as production [`neutrino_app::NeutrinoRoutes`], without `Lazy`.
#[component(transparent)]
pub fn NeutrinoRoutesEager() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("secrets") view=NeutrinoVerifiedGuardRouteView>
            <Route path=path!("") view=SecretsListPage />
            <Route path=path!("acl") view=AclManagePage />
        </ParentRoute>
    }
    .into_inner()
}
