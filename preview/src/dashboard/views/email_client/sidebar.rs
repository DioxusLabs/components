use dioxus::prelude::*;

use crate::components::avatar::{
    Avatar, AvatarFallback, AvatarImage, AvatarImageSize, AvatarShape,
};
use crate::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
    SidebarHeader, SidebarMenu, SidebarMenuBadge, SidebarMenuButton, SidebarMenuButtonSize,
    SidebarMenuItem, SidebarRail, SidebarVariant,
};
use crate::dashboard::common::{FolderId, IconKind, LucideIcon, AVATAR_PROFILE_OPTIONS, FOLDERS};

use super::state::{EmailClientState, EmailClientStateStoreExt, EmailClientStateStoreImplExt};

#[component]
pub(super) fn EmailSidebar(mut state: Store<EmailClientState>) -> Element {
    rsx! {
        Sidebar {
            variant: SidebarVariant::Sidebar,
            collapsible: SidebarCollapsible::Icon,

            SidebarHeader {
                SidebarMenu { SidebarMenuItem {
                    SidebarMenuButton {
                        size: SidebarMenuButtonSize::Lg,
                        tooltip: rsx! { "Mail · you@yourcompany.com" },
                        Avatar {
                            size: AvatarImageSize::Small,
                            shape: AvatarShape::Rounded,
                            AvatarImage {
                                src: "{AVATAR_PROFILE_OPTIONS[2].src}",
                                alt: "Mail",
                            }
                            AvatarFallback { "M" }
                        }
                        div { class: "dx-sidebar-info-block",
                            span { class: "dx-sidebar-info-title", "Mail" }
                            span { class: "dx-sidebar-info-subtitle", "you@yourcompany.com" }
                        }
                    }
                } }
            }

            SidebarContent {
                SidebarGroup {
                    SidebarMenu { SidebarMenuItem {
                        SidebarMenuButton {
                            class: "ec-compose",
                            tooltip: rsx! { "Compose (C)" },
                            as: move |attrs: Vec<Attribute>| rsx! {
                                button {
                                    r#type: "button",
                                    onclick: move |_| state.open_compose(),
                                    ..attrs,
                                    LucideIcon { kind: IconKind::Pen }
                                    span { "Compose" }
                                }
                            },
                        }
                    } }
                }

                SidebarGroup {
                    SidebarGroupLabel { "Folders" }
                    SidebarMenu {
                        for f in FOLDERS.iter() {
                            FolderItem {
                                key: "{f.id.as_str()}",
                                folder_id: f.id,
                                label: f.label,
                                icon: f.icon,
                                count: Some(state.folder_count(f.id)),
                                state,
                            }
                        }
                    }
                }
            }

            SidebarFooter {
                SidebarMenu { SidebarMenuItem {
                    SidebarMenuButton {
                        size: SidebarMenuButtonSize::Lg,
                        tooltip: rsx! { "You" },
                        Avatar {
                            size: AvatarImageSize::Small,
                            shape: AvatarShape::Rounded,
                            AvatarImage {
                                src: "{AVATAR_PROFILE_OPTIONS[0].src}",
                                alt: "You",
                            }
                            AvatarFallback { "Y" }
                        }
                        div { class: "dx-sidebar-info-block",
                            span { class: "dx-sidebar-info-title", "You" }
                            span { class: "dx-sidebar-info-subtitle", "you@yourcompany.com" }
                        }
                    }
                } }
            }

            SidebarRail {}
        }
    }
}

#[component]
fn FolderItem(
    folder_id: FolderId,
    label: &'static str,
    icon: IconKind,
    count: Option<u32>,
    mut state: Store<EmailClientState>,
) -> Element {
    let is_active = state.active_folder().cloned() == folder_id;

    rsx! {
        SidebarMenuItem {
            SidebarMenuButton {
                is_active,
                tooltip: rsx! { {label} },
                as: move |attrs: Vec<Attribute>| rsx! {
                    button {
                        r#type: "button",
                        onclick: move |_| {
                            state.set_active_folder(folder_id);
                        },
                        ..attrs,
                        LucideIcon { kind: icon }
                        span { {label} }
                    }
                },
            }
            if let Some(c) = count {
                SidebarMenuBadge { {format!("{}", c)} }
            }
        }
    }
}
