//! Secrets vault data table (loading / empty / error / rows).

use leptos::prelude::*;
use uf_product::components::{Card, EmptyState, SkeletonItemSize};
use uf_product::primitives::{
    Button, ButtonAppearance, ButtonSize, Flex, FlexGap, Menu, MenuItem, MenuTrigger, MessageBar,
    MessageBarIntent, SkeletonItem, Table, TableBody, TableCell, TableHeader, TableHeaderCell,
    TableRow,
};

use crate::server::VaultSecretRow;

#[component]
pub(super) fn SecretsTable(
    secrets: Resource<Result<Vec<VaultSecretRow>, ServerFnError>>,
    action_busy_id: RwSignal<Option<String>>,
    on_reveal: Callback<VaultSecretRow>,
    on_rotate: Callback<VaultSecretRow>,
    on_delete: Callback<VaultSecretRow>,
) -> impl IntoView {
    view! {
        <Flex vertical=true fill=true full_width=true gap=FlexGap::Size(0)>
        <Card>
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell>"Name"</TableHeaderCell>
                        <TableHeaderCell>"Scope"</TableHeaderCell>
                        <TableHeaderCell>"Kind"</TableHeaderCell>
                        <TableHeaderCell>"Version"</TableHeaderCell>
                        <TableHeaderCell>"Created"</TableHeaderCell>
                        <TableHeaderCell>"Actions"</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <Suspense fallback=move || view! {
                    <TableBody>
                        {(0..3).map(|_| view! {
                            <TableRow>
                                <TableCell attr:colspan="6">
                                    <SkeletonItem
                                        size=Signal::from(SkeletonItemSize::S32)
                                        width="100%".to_string()
                                    />
                                </TableCell>
                            </TableRow>
                        }).collect_view()}
                    </TableBody>
                }>
                    {move || match secrets.get() {
                        Some(Ok(rows)) => {
                            if rows.is_empty() {
                                return view! {
                                    <TableBody>
                                        <TableRow>
                                            <TableCell attr:colspan="6">
                                                <EmptyState message="No secrets in the vault yet." />
                                            </TableCell>
                                        </TableRow>
                                    </TableBody>
                                }.into_any();
                            }
                            view! {
                                <TableBody>
                                    {rows.into_iter().map(|row| {
                                        let row_name = row.name.clone();
                                        let row_name_ver = row.name.clone();
                                        let row_name_act = row.name.clone();
                                        let row_reveal = row.clone();
                                        let row_rotate = row.clone();
                                        let row_delete = row.clone();
                                        let is_busy = Signal::derive({
                                            let id = row.id.clone();
                                            move || action_busy_id.get().as_deref() == Some(id.as_str())
                                        });
                                        view! {
                                            <TableRow>
                                                <TableCell>
                                                    <div data-testid=format!("neutrino-secret-row-{}", row_name)>
                                                        {row.name.clone()}
                                                    </div>
                                                </TableCell>
                                                <TableCell>{row.scope_path.clone()}</TableCell>
                                                <TableCell>{row.kind.clone()}</TableCell>
                                                <TableCell>
                                                    <div data-testid=format!("neutrino-secret-version-{}", row_name_ver)>
                                                        {row.current_version}
                                                    </div>
                                                </TableCell>
                                                <TableCell>{row.created_at.clone()}</TableCell>
                                                <TableCell>
                                                    <div data-testid=format!("neutrino-secret-actions-{}", row_name_act)>
                                                    <Menu
                                                        on_select=move |key: &str| {
                                                            match key {
                                                                "reveal" => on_reveal.run(row_reveal.clone()),
                                                                "rotate" => on_rotate.run(row_rotate.clone()),
                                                                "delete" => on_delete.run(row_delete.clone()),
                                                                _ => {}
                                                            }
                                                        }
                                                    >
                                                        <MenuTrigger slot>
                                                            <Button
                                                                appearance=ButtonAppearance::Subtle
                                                                icon=icondata::AiMoreOutlined
                                                                size=ButtonSize::Small
                                                                disabled=is_busy
                                                                attr:aria-label="Open secret actions"
                                                            />
                                                        </MenuTrigger>
                                                        <MenuItem value="reveal">"Reveal"</MenuItem>
                                                        <MenuItem value="rotate">"Rotate"</MenuItem>
                                                        <MenuItem value="delete">"Delete"</MenuItem>
                                                    </Menu>
                                                    </div>
                                                </TableCell>
                                            </TableRow>
                                        }
                                    }).collect_view()}
                                </TableBody>
                            }.into_any()
                        }
                        Some(Err(err)) => {
                            view! {
                                <TableBody>
                                    <TableRow>
                                        <TableCell attr:colspan="6">
                                            <MessageBar intent=MessageBarIntent::Error>
                                                "Failed to load secrets: " {err.to_string()}
                                            </MessageBar>
                                        </TableCell>
                                    </TableRow>
                                </TableBody>
                            }.into_any()
                        }
                        None => view! {
                            <TableBody>
                                <TableRow>
                                    <TableCell attr:colspan="6"></TableCell>
                                </TableRow>
                            </TableBody>
                        }.into_any(),
                    }}
                </Suspense>
            </Table>
        </Card>
        </Flex>
    }
}
