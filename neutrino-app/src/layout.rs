//! Orbital shell layout for the Secrets app (`NeutrinoAppLayout`).
//!
//! Wraps routed pages with the Unified Field app bar, auth menu, and left nav.
//! Mount via [`crate::NeutrinoVerifiedGuardRouteView`] inside [`crate::NeutrinoRoutes`].

use lepton_shell::AppBarUserMenu;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use uf_integrations::{
    ShellAppBar, ShellAuthMenu, ShellLeftNav, UnifiedFieldAppBar, UnifiedFieldShellLayout,
};
use uf_product::components::{
    Navigation, NavigationBody, NavigationConfig, NavigationLink, NavigationMaterial,
};

use crate::AppMetadata;

/// Shell layout for the Secrets app: app bar + left nav wrapping the routed page [`Outlet`].
#[component]
pub fn NeutrinoAppLayout() -> impl IntoView {
    let app_name = AppMetadata::name().to_string();
    let selected_value = RwSignal::new(None::<String>);
    let open_categories = RwSignal::new(Vec::<String>::new());

    view! {
        <div data-testid="neutrino-app-root">
        <UnifiedFieldShellLayout>
            <ShellAppBar slot>
                <UnifiedFieldAppBar
                    app_name=app_name
                    app_id=AppMetadata::id()
                    homepage_url="/".to_string()
                >
                    <ShellAuthMenu slot:auth_menu>
                        <AppBarUserMenu />
                    </ShellAuthMenu>
                </UnifiedFieldAppBar>
            </ShellAppBar>
            <ShellLeftNav slot>
                <Navigation config=NavigationConfig::new().with_selected_value(selected_value).with_open_categories(open_categories)>
                    <NavigationMaterial slot />
                    <NavigationBody slot>
                        <NavigationLink path="/neutrino" value="/neutrino" icon=icondata::AiKeyOutlined exact=true test_id="nav-secrets">"Secrets"</NavigationLink>
                    </NavigationBody>
                </Navigation>
            </ShellLeftNav>
            <Outlet />
        </UnifiedFieldShellLayout>
        </div>
    }
}
