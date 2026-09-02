//! Secrets vault list page: create / reveal / rotate / delete dialogs + table.

mod dialogs;
mod table;

use dialogs::{CreateSecretDialog, DeleteSecretDialog, RevealSecretDialog, RotateSecretDialog};
use leptos::prelude::*;
use leptos::task::spawn_local_scoped;
use table::SecretsTable;
use uf_product::components::{Caption1, ContentContainer, SpacingSize, Title3};
use uf_product::primitives::{Button, ButtonAppearance, Flex, FlexAlign, FlexJustify};
use uf_product::services::permission_server_errors::{
    report_server_fn_error_with_bus, use_permission_toast_bus,
};

use crate::server::{
    create_vault_secret, delete_vault_secret, list_vault_secrets, reveal_vault_secret,
    rotate_vault_secret, VaultSecretRow,
};

#[component]
/// Lists Neutrino vault secrets with create / reveal / rotate / delete flows.
pub fn SecretsListPage() -> impl IntoView {
    let permission_toast_bus = use_permission_toast_bus();
    let refresh_trigger = RwSignal::new(0u32);
    let secrets = Resource::new(move || refresh_trigger.get(), |_| list_vault_secrets());

    let create_open = RwSignal::new(false);
    let new_name = RwSignal::new(String::new());
    let new_scope = RwSignal::new(String::new());
    let new_kind = RwSignal::new(String::new());
    let new_plaintext = RwSignal::new(String::new());
    let create_error = RwSignal::new(Option::<String>::None);
    let create_submitting = RwSignal::new(false);

    let reveal_open = RwSignal::new(false);
    let reveal_target = RwSignal::new(Option::<VaultSecretRow>::None);
    let reveal_b64 = RwSignal::new(String::new());
    let reveal_error = RwSignal::new(Option::<String>::None);
    let reveal_loading = RwSignal::new(false);

    let rotate_open = RwSignal::new(false);
    let rotate_target = RwSignal::new(Option::<VaultSecretRow>::None);
    let rotate_plaintext = RwSignal::new(String::new());
    let rotate_error = RwSignal::new(Option::<String>::None);
    let rotate_submitting = RwSignal::new(false);

    let delete_open = RwSignal::new(false);
    let delete_target = RwSignal::new(Option::<VaultSecretRow>::None);
    let delete_error = RwSignal::new(Option::<String>::None);
    let delete_submitting = RwSignal::new(false);

    let action_busy_id = RwSignal::new(Option::<String>::None);

    let on_cancel_create = move |_| {
        create_open.set(false);
        create_error.set(None);
        new_plaintext.set(String::new());
    };

    let on_submit_create = move |_| {
        let name = new_name.get_untracked();
        let scope_path = new_scope.get_untracked();
        let kind = new_kind.get_untracked();
        let plaintext = new_plaintext.get_untracked();
        create_error.set(None);
        create_submitting.set(true);
        spawn_local_scoped(async move {
            match create_vault_secret(name, scope_path, kind, plaintext).await {
                Ok(_) => {
                    create_submitting.set(false);
                    create_open.set(false);
                    new_plaintext.set(String::new());
                    refresh_trigger.update(|n| *n += 1);
                }
                Err(err) => {
                    create_submitting.set(false);
                    // Toast bus may be a no-op stub in this shell; always surface in-dialog.
                    let _ = report_server_fn_error_with_bus(permission_toast_bus, &err);
                    create_error.set(Some(err.to_string()));
                }
            }
        });
    };

    let on_close_reveal = move |_| {
        reveal_open.set(false);
        reveal_target.set(None);
        reveal_b64.update(|value| {
            value.clear();
        });
        reveal_error.set(None);
    };

    let on_submit_rotate = move |_| {
        let Some(target) = rotate_target.get_untracked() else {
            return;
        };
        let pt = rotate_plaintext.get_untracked();
        rotate_error.set(None);
        rotate_submitting.set(true);
        spawn_local_scoped(async move {
            match rotate_vault_secret(target.id.clone(), pt).await {
                Ok(_) => {
                    rotate_submitting.set(false);
                    rotate_open.set(false);
                    rotate_target.set(None);
                    rotate_plaintext.set(String::new());
                    refresh_trigger.update(|n| *n += 1);
                }
                Err(err) => {
                    rotate_submitting.set(false);
                    let _ = report_server_fn_error_with_bus(permission_toast_bus, &err);
                    rotate_error.set(Some(err.to_string()));
                }
            }
        });
    };

    let on_confirm_delete = move |_| {
        let Some(target) = delete_target.get_untracked() else {
            return;
        };
        delete_error.set(None);
        delete_submitting.set(true);
        spawn_local_scoped(async move {
            match delete_vault_secret(target.id.clone()).await {
                Ok(()) => {
                    delete_submitting.set(false);
                    delete_open.set(false);
                    delete_target.set(None);
                    refresh_trigger.update(|n| *n += 1);
                }
                Err(err) => {
                    delete_submitting.set(false);
                    let _ = report_server_fn_error_with_bus(permission_toast_bus, &err);
                    delete_error.set(Some(err.to_string()));
                }
            }
        });
    };

    let on_reveal_row = Callback::new(move |row: VaultSecretRow| {
        reveal_target.set(Some(row.clone()));
        reveal_b64.set(String::new());
        reveal_error.set(None);
        reveal_open.set(true);
        reveal_loading.set(true);
        spawn_local_scoped(async move {
            match reveal_vault_secret(row.id.clone()).await {
                Ok(mut p) => {
                    reveal_loading.set(false);
                    reveal_b64.set(std::mem::take(&mut p.plaintext_b64));
                }
                Err(err) => {
                    reveal_loading.set(false);
                    let _ = report_server_fn_error_with_bus(permission_toast_bus, &err);
                    reveal_error.set(Some(err.to_string()));
                }
            }
        });
    });

    let on_rotate_row = Callback::new(move |row: VaultSecretRow| {
        rotate_target.set(Some(row));
        rotate_plaintext.set(String::new());
        rotate_error.set(None);
        rotate_open.set(true);
    });

    let on_delete_row = Callback::new(move |row: VaultSecretRow| {
        delete_target.set(Some(row));
        delete_open.set(true);
    });

    view! {
        <div data-testid="neutrino-secrets-list-page">
        <ContentContainer>
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Flex justify=FlexJustify::SpaceBetween align=FlexAlign::Center>
                    <Title3>"Secret vault"</Title3>
                    <div data-testid="neutrino-create-secret-btn">
                        <Button
                            appearance=ButtonAppearance::Primary
                            on_click=Callback::new(move |_| {
                                create_error.set(None);
                                create_open.set(true);
                            })
                        >
                            "Create secret"
                        </Button>
                    </div>
                </Flex>

                <Caption1>
                    "Reveal and rotate are permission-gated server-side; buttons stay visible and return 403 if your role lacks access."
                </Caption1>

                <CreateSecretDialog
                    open=create_open
                    new_name=new_name
                    new_scope=new_scope
                    new_kind=new_kind
                    new_plaintext=new_plaintext
                    create_error=create_error
                    create_submitting=create_submitting
                    on_cancel=Callback::new(on_cancel_create)
                    on_submit=Callback::new(on_submit_create)
                />
                <RevealSecretDialog
                    open=reveal_open
                    reveal_target=reveal_target
                    reveal_b64=reveal_b64
                    reveal_error=reveal_error
                    reveal_loading=reveal_loading
                    on_close=Callback::new(on_close_reveal)
                />
                <RotateSecretDialog
                    open=rotate_open
                    rotate_target=rotate_target
                    rotate_plaintext=rotate_plaintext
                    rotate_error=rotate_error
                    rotate_submitting=rotate_submitting
                    on_submit=Callback::new(on_submit_rotate)
                />
                <DeleteSecretDialog
                    open=delete_open
                    delete_target=delete_target
                    delete_error=delete_error
                    delete_submitting=delete_submitting
                    on_confirm=Callback::new(on_confirm_delete)
                />

                <SecretsTable
                    secrets=secrets
                    action_busy_id=action_busy_id
                    on_reveal=on_reveal_row
                    on_rotate=on_rotate_row
                    on_delete=on_delete_row
                />
            </Flex>
        </ContentContainer>
        </div>
    }
}
