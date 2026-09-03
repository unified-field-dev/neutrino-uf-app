//! Create / reveal / rotate / delete dialogs for the secrets list page.

use leptos::ev::MouseEvent;
use leptos::prelude::*;
use uf_product::components::{Caption1, SpacingSize, Subtitle2};
use uf_product::primitives::{
    Button, ButtonAppearance, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface,
    DialogTitle, Field, Flex, Input, InputAppearance, InputType, MessageBar, MessageBarIntent,
    Spinner,
};

use crate::server::VaultSecretRow;

#[component]
#[allow(clippy::too_many_lines)] // Orbital Dialog field markup
pub(super) fn CreateSecretDialog(
    open: RwSignal<bool>,
    new_name: RwSignal<String>,
    new_scope: RwSignal<String>,
    new_kind: RwSignal<String>,
    new_plaintext: RwSignal<String>,
    create_error: RwSignal<Option<String>>,
    create_submitting: RwSignal<bool>,
    on_cancel: Callback<MouseEvent>,
    on_submit: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <Dialog open=open>
            <DialogSurface>
                <div data-testid="neutrino-create-secret-dialog">
                <DialogBody>
                    <DialogTitle>"Create secret"</DialogTitle>
                    <DialogContent>
                        <Flex vertical=true gap=SpacingSize::Size120.flex_gap()>
                            <Field label="Name">
                                <div id="secrets-create-name" data-testid="neutrino-create-name">
                                    <Input bind=new_name appearance=InputAppearance::with_placeholder("my-api-token") />
                                </div>
                            </Field>
                            <Field label="Scope path">
                                <div id="secrets-create-scope" data-testid="neutrino-create-scope">
                                    <Input bind=new_scope appearance=InputAppearance::with_placeholder("/gluon/provider_account/...") />
                                </div>
                            </Field>
                            <Field label="Kind">
                                <div id="secrets-create-kind" data-testid="neutrino-create-kind">
                                    <Input bind=new_kind appearance=InputAppearance::with_placeholder("provider_api_token") />
                                </div>
                            </Field>
                            <Field label="Plaintext">
                                <div id="secrets-create-plaintext" data-testid="neutrino-create-plaintext">
                                <Input
                                    bind=new_plaintext
                                    appearance=InputAppearance {
                                        input_type: Signal::from(InputType::Password),
                                        placeholder: "secret value".into(),
                                        ..Default::default()
                                    }
                                />
                                </div>
                            </Field>
                            {move || create_error.get().map(|msg| view! {
                                <div data-testid="neutrino-create-error">
                                    <MessageBar intent=MessageBarIntent::Error>{msg}</MessageBar>
                                </div>
                            })}
                        </Flex>
                    </DialogContent>
                    <DialogActions>
                        <div id="secrets-create-cancel" data-testid="neutrino-create-cancel">
                            <Button appearance=ButtonAppearance::Secondary on_click=on_cancel>
                                "Cancel"
                            </Button>
                        </div>
                        <div id="secrets-create-submit" data-testid="neutrino-create-submit">
                        <Button
                            appearance=ButtonAppearance::Primary
                            on_click=on_submit
                            disabled=Signal::derive(move || create_submitting.get())
                        >
                            {move || if create_submitting.get() { "Creating..." } else { "Create" }}
                        </Button>
                        </div>
                    </DialogActions>
                </DialogBody>
                </div>
            </DialogSurface>
        </Dialog>
    }
}

#[component]
pub(super) fn RevealSecretDialog(
    open: RwSignal<bool>,
    reveal_target: RwSignal<Option<VaultSecretRow>>,
    reveal_b64: RwSignal<String>,
    reveal_error: RwSignal<Option<String>>,
    reveal_loading: RwSignal<bool>,
    on_close: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <Dialog open=open>
            <DialogSurface>
                <div data-testid="neutrino-reveal-secret-dialog">
                <DialogBody>
                    <DialogTitle>"Reveal secret"</DialogTitle>
                    <DialogContent>
                        <Flex vertical=true gap=SpacingSize::Size120.flex_gap()>
                            {move || reveal_target.get().map(|t| view! {
                                <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                                    <Subtitle2>{t.name.clone()}</Subtitle2>
                                    <Caption1>{t.scope_path.clone()}</Caption1>
                                </Flex>
                            })}
                            {move || reveal_loading.get().then(|| view! {
                                <Spinner />
                            })}
                            {move || reveal_error.get().map(|msg| view! {
                                <div data-testid="neutrino-reveal-error">
                                    <MessageBar intent=MessageBarIntent::Error>{msg}</MessageBar>
                                </div>
                            })}
                            // Spotlight cutout uses HTML id (always mounted). data-testid
                            // only when plaintext is present so denied-reveal e2e can assert absence.
                            <div id="secrets-reveal-plaintext">
                                <Show
                                    when=move || !reveal_b64.get().is_empty()
                                    fallback=move || {
                                        view! {
                                            <Show when=move || !reveal_loading.get()>
                                                <Caption1>"The revealed value appears here after a successful reveal."</Caption1>
                                            </Show>
                                        }
                                    }
                                >
                                    <Field label="Plaintext (base64)">
                                        <div data-testid="neutrino-reveal-plaintext">
                                            <Input bind=reveal_b64 appearance=InputAppearance { readonly: Signal::from(true), ..Default::default() } />
                                        </div>
                                    </Field>
                                </Show>
                            </div>
                            <Caption1>"Copy once; closing clears this dialog."</Caption1>
                        </Flex>
                    </DialogContent>
                    <DialogActions>
                        <div id="secrets-reveal-close" data-testid="neutrino-reveal-close">
                            <Button appearance=ButtonAppearance::Primary on_click=on_close>
                                "Close"
                            </Button>
                        </div>
                    </DialogActions>
                </DialogBody>
                </div>
            </DialogSurface>
        </Dialog>
    }
}

#[component]
pub(super) fn RotateSecretDialog(
    open: RwSignal<bool>,
    rotate_target: RwSignal<Option<VaultSecretRow>>,
    rotate_plaintext: RwSignal<String>,
    rotate_error: RwSignal<Option<String>>,
    rotate_submitting: RwSignal<bool>,
    on_submit: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <Dialog open=open>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>"Rotate secret"</DialogTitle>
                    <DialogContent>
                        <Flex vertical=true gap=SpacingSize::Size120.flex_gap()>
                            {move || rotate_target.get().map(|t| view! {
                                <Subtitle2>{t.name.clone()}</Subtitle2>
                            })}
                            <Field label="New plaintext">
                                <div id="secrets-rotate-plaintext" data-testid="neutrino-rotate-plaintext">
                                <Input
                                    bind=rotate_plaintext
                                    appearance=InputAppearance {
                                        input_type: Signal::from(InputType::Password),
                                        placeholder: "new secret value".into(),
                                        ..Default::default()
                                    }
                                />
                                </div>
                            </Field>
                            {move || rotate_error.get().map(|msg| view! {
                                <div data-testid="neutrino-rotate-error">
                                    <MessageBar intent=MessageBarIntent::Error>{msg}</MessageBar>
                                </div>
                            })}
                        </Flex>
                    </DialogContent>
                    <DialogActions>
                        <Button
                            appearance=ButtonAppearance::Secondary
                            on_click=Callback::new(move |_| {
                                open.set(false);
                                rotate_target.set(None);
                                rotate_plaintext.set(String::new());
                                rotate_error.set(None);
                            })
                        >
                            "Cancel"
                        </Button>
                        <div id="secrets-rotate-submit" data-testid="neutrino-rotate-submit">
                        <Button
                            appearance=ButtonAppearance::Primary
                            on_click=on_submit
                            disabled=Signal::derive(move || rotate_submitting.get())
                        >
                            {move || if rotate_submitting.get() { "Rotating..." } else { "Rotate" }}
                        </Button>
                        </div>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}

#[component]
pub(super) fn DeleteSecretDialog(
    open: RwSignal<bool>,
    delete_target: RwSignal<Option<VaultSecretRow>>,
    delete_error: RwSignal<Option<String>>,
    delete_submitting: RwSignal<bool>,
    on_confirm: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <Dialog open=open>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>"Delete secret"</DialogTitle>
                    <DialogContent>
                        {move || match delete_target.get() {
                            Some(t) => view! {
                                <p>
                                    "Permanently delete " <strong>{t.name}</strong>
                                    " and all versions?"
                                </p>
                            }.into_any(),
                            None => view! {
                                <p>"Confirm permanently deletes the secret and its versions. Cancel aborts."</p>
                            }.into_any(),
                        }}
                        {move || delete_error.get().map(|msg| view! {
                            <MessageBar intent=MessageBarIntent::Error>{msg}</MessageBar>
                        })}
                    </DialogContent>
                    <DialogActions>
                        <Button
                            appearance=ButtonAppearance::Secondary
                            on_click=Callback::new(move |_| {
                                open.set(false);
                                delete_target.set(None);
                            })
                        >
                            "Cancel"
                        </Button>
                        <div id="secrets-delete-confirm" data-testid="neutrino-delete-confirm">
                        <Button
                            appearance=ButtonAppearance::Primary
                            on_click=on_confirm
                            disabled=Signal::derive(move || delete_submitting.get())
                        >
                            {move || if delete_submitting.get() { "Deleting..." } else { "Delete" }}
                        </Button>
                        </div>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
