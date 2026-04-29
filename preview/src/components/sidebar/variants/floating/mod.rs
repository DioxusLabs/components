use crate::components::avatar::{
    Avatar, AvatarFallback, AvatarImage, AvatarImageSize, AvatarShape,
};
use crate::components::button::{Button, ButtonVariant};
use crate::components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::separator::Separator;
use crate::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
    SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuAction, SidebarMenuBadge,
    SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem, SidebarMenuSub,
    SidebarMenuSubButton, SidebarMenuSubItem, SidebarProvider, SidebarRail, SidebarSide,
    SidebarTrigger, SidebarVariant,
};
use crate::components::skeleton::Skeleton;
use dioxus::prelude::*;
use dioxus_primitives::icon;

#[derive(Clone, PartialEq)]
struct Team {
    name: &'static str,
    plan: &'static str,
}

#[derive(Clone, PartialEq)]
struct NavMainItem {
    title: &'static str,
    url: &'static str,
    is_active: bool,
    items: &'static [SubItem],
}

#[derive(Clone, PartialEq)]
struct SubItem {
    title: &'static str,
    url: &'static str,
}

#[derive(Clone, PartialEq)]
struct Project {
    name: &'static str,
    url: &'static str,
}

const TEAMS: &[Team] = &[
    Team { name: "Acme Inc", plan: "Enterprise" },
    Team { name: "Acme Corp.", plan: "Startup" },
    Team { name: "Evil Corp.", plan: "Free" },
];

const NAV_MAIN: &[NavMainItem] = &[
    NavMainItem {
        title: "Playground",
        url: "#",
        is_active: true,
        items: &[
            SubItem { title: "History", url: "#" },
            SubItem { title: "Starred", url: "#" },
            SubItem { title: "Settings", url: "#" },
        ],
    },
    NavMainItem {
        title: "Models",
        url: "#",
        is_active: false,
        items: &[
            SubItem { title: "Genesis", url: "#" },
            SubItem { title: "Explorer", url: "#" },
            SubItem { title: "Quantum", url: "#" },
        ],
    },
    NavMainItem {
        title: "Documentation",
        url: "#",
        is_active: false,
        items: &[
            SubItem { title: "Introduction", url: "#" },
            SubItem { title: "Get Started", url: "#" },
            SubItem { title: "Tutorials", url: "#" },
            SubItem { title: "Changelog", url: "#" },
        ],
    },
    NavMainItem {
        title: "Settings",
        url: "#",
        is_active: false,
        items: &[
            SubItem { title: "General", url: "#" },
            SubItem { title: "Team", url: "#" },
            SubItem { title: "Billing", url: "#" },
            SubItem { title: "Limits", url: "#" },
        ],
    },
];

const PROJECTS: &[Project] = &[
    Project { name: "Design Engineering", url: "#" },
    Project { name: "Sales & Marketing", url: "#" },
    Project { name: "Travel", url: "#" },
];

#[component]
pub fn Demo() -> Element {
    let side = use_signal(|| SidebarSide::Left);
    let collapsible = use_signal(|| SidebarCollapsible::Offcanvas);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../demo.css") }
        SidebarProvider {
            Sidebar {
                variant: SidebarVariant::Floating,
                collapsible: collapsible(),
                side: side(),
                SidebarHeader { TeamSwitcher { teams: TEAMS } }
                SidebarContent {
                    NavMain { items: NAV_MAIN }
                    NavProjects { projects: PROJECTS }
                }
                SidebarFooter { NavUser {} }
                SidebarRail {}
            }
            SidebarInset {
                header { class: "dx-sidebar-demo-header",
                    div { class: "dx-sidebar-demo-header-inner",
                        SidebarTrigger {}
                        Separator { height: "1rem", horizontal: false }
                        span { "Sidebar Setting" }
                    }
                }
                div { class: "dx-sidebar-demo-content",
                    DemoSettingControls { side, collapsible }
                    Skeleton { style: "height: 10rem; width: 100%; flex-shrink: 0;" }
                    Skeleton { style: "height: 20rem; width: 100%; flex-shrink: 0;" }
                }
            }
        }
    }
}

#[component]
fn TeamSwitcher(teams: &'static [Team]) -> Element {
    let mut active_team = use_signal(|| 0usize);

    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                DropdownMenu {
                    DropdownMenuTrigger {
                        as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuButton { size: SidebarMenuButtonSize::Lg, attributes,
                                div { class: "dx-sidebar-team-icon", Icon {} }
                                div { class: "dx-sidebar-info-block",
                                    span { class: "dx-sidebar-info-title", {teams[active_team()].name} }
                                    span { class: "dx-sidebar-info-subtitle", {teams[active_team()].plan} }
                                }
                                ChevronIcon {}
                            }
                        },
                    }
                    DropdownMenuContent {
                        div { class: "dx-sidebar-dropdown-label", "Teams" }
                        for (idx , team) in teams.iter().enumerate() {
                            DropdownMenuItem {
                                index: idx,
                                value: idx,
                                on_select: move |v: usize| active_team.set(v),
                                Icon {}
                                {team.name}
                                span { class: "dx-sidebar-dropdown-shortcut", "⌘{idx + 1}" }
                            }
                        }
                        Separator { decorative: true }
                        DropdownMenuItem {
                            index: teams.len(),
                            value: 999usize,
                            on_select: move |_: usize| {},
                            Icon {}
                            span { class: "dx-sidebar-muted", "Add team" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavMain(items: &'static [NavMainItem]) -> Element {
    rsx! {
        SidebarGroup {
            SidebarGroupLabel { "Platform" }
            SidebarMenu {
                for item in items.iter() {
                    Collapsible {
                        default_open: item.is_active,
                        as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuItem { key: "{item.title}", attributes,
                                CollapsibleTrigger {
                                    as: move |attributes: Vec<Attribute>| rsx! {
                                        SidebarMenuButton {
                                            tooltip: rsx! {
                                                {item.title}
                                            },
                                            attributes,
                                            Icon {}
                                            span { {item.title} }
                                            ChevronIcon {}
                                        }
                                    },
                                }
                                CollapsibleContent {
                                    SidebarMenuSub {
                                        for sub_item in item.items {
                                            SidebarMenuSubItem { key: "{sub_item.title}",
                                                SidebarMenuSubButton {
                                                    as: move |attributes: Vec<Attribute>| rsx! {
                                                        a { href: sub_item.url, ..attributes,
                                                            span { {sub_item.title} }
                                                        }
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn NavProjects(projects: &'static [Project]) -> Element {
    rsx! {
        SidebarGroup { class: "dx-sidebar-hide-on-collapse",
            SidebarGroupLabel { "Projects" }
            SidebarMenu {
                for project in projects.iter() {
                    SidebarMenuItem { key: "{project.name}",
                        SidebarMenuButton {
                            as: move |attributes: Vec<Attribute>| rsx! {
                                a { href: project.url, ..attributes,
                                    Icon {}
                                    span { {project.name} }
                                }
                            },
                        }
                        DropdownMenu {
                            DropdownMenuTrigger {
                                as: move |attributes: Vec<Attribute>| rsx! {
                                    SidebarMenuAction { show_on_hover: true, attributes,
                                        Icon {}
                                        span { class: "dx-sr-only", "More" }
                                    }
                                },
                            }
                            DropdownMenuContent {
                                DropdownMenuItem {
                                    index: 0usize,
                                    value: "view".to_string(),
                                    on_select: move |_: String| {},
                                    Icon {}
                                    span { "View Project" }
                                }
                                DropdownMenuItem {
                                    index: 1usize,
                                    value: "share".to_string(),
                                    on_select: move |_: String| {},
                                    Icon {}
                                    span { "Share Project" }
                                }
                                Separator { decorative: true }
                                DropdownMenuItem {
                                    index: 2usize,
                                    value: "delete".to_string(),
                                    on_select: move |_: String| {},
                                    Icon {}
                                    span { "Delete Project" }
                                }
                            }
                        }
                    }
                }
                SidebarMenuItem {
                    SidebarMenuButton { class: "dx-sidebar-muted",
                        Icon {}
                        span { "More" }
                    }
                    SidebarMenuBadge { "+99" }
                }
            }
        }
    }
}

#[component]
fn NavUser() -> Element {
    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                DropdownMenu {
                    DropdownMenuTrigger {
                        as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuButton { size: SidebarMenuButtonSize::Lg, attributes,
                                Avatar {
                                    size: AvatarImageSize::Small,
                                    shape: AvatarShape::Rounded,
                                    AvatarImage {
                                        src: asset!("/assets/dioxus-logo.png", ImageAssetOptions::new().with_avif()),
                                        alt: "dioxus avatar",
                                    }
                                    AvatarFallback { "DX" }
                                }
                                div { class: "dx-sidebar-info-block",
                                    span { class: "dx-sidebar-info-title", "Dioxus" }
                                    span { class: "dx-sidebar-info-subtitle", "m@example.com" }
                                }
                                ChevronIcon {}
                            }
                        },
                    }
                    DropdownMenuContent {
                        div { class: "dx-sidebar-user-card",
                            Avatar {
                                size: AvatarImageSize::Small,
                                shape: AvatarShape::Rounded,
                                AvatarImage {
                                    src: asset!("/assets/dioxus-logo.png", ImageAssetOptions::new().with_avif()),
                                    alt: "dioxus avatar",
                                }
                                AvatarFallback { "DX" }
                            }
                            div { class: "dx-sidebar-info-block",
                                span { class: "dx-sidebar-info-title", "Dioxus" }
                                span { class: "dx-sidebar-info-subtitle", "m@example.com" }
                            }
                        }
                        Separator { decorative: true }
                        DropdownMenuItem {
                            index: 0usize,
                            value: "upgrade".to_string(),
                            on_select: move |_: String| {},
                            Icon {}
                            "Upgrade to Pro"
                        }
                        Separator { decorative: true }
                        DropdownMenuItem {
                            index: 1usize,
                            value: "account".to_string(),
                            on_select: move |_: String| {},
                            Icon {}
                            "Account"
                        }
                        DropdownMenuItem {
                            index: 2usize,
                            value: "billing".to_string(),
                            on_select: move |_: String| {},
                            Icon {}
                            "Billing"
                        }
                        DropdownMenuItem {
                            index: 3usize,
                            value: "notifications".to_string(),
                            on_select: move |_: String| {},
                            Icon {}
                            "Notifications"
                        }
                        Separator { decorative: true }
                        DropdownMenuItem {
                            index: 4usize,
                            value: "logout".to_string(),
                            on_select: move |_: String| {},
                            Icon {}
                            "Log out"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DemoSettingControls(
    side: Signal<SidebarSide>,
    collapsible: Signal<SidebarCollapsible>,
) -> Element {
    rsx! {
        div { class: "dx-sidebar-controls",
            div { class: "dx-sidebar-controls-row",
                span { class: "dx-sidebar-controls-label", "Side" }
                div { class: "dx-sidebar-controls-actions",
                    Button {
                        variant: if side() == SidebarSide::Left { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| side.set(SidebarSide::Left),
                        class: "dx-sidebar-controls-button",
                        "Left"
                    }
                    Button {
                        variant: if side() == SidebarSide::Right { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| side.set(SidebarSide::Right),
                        class: "dx-sidebar-controls-button",
                        "Right"
                    }
                }
            }
            div { class: "dx-sidebar-controls-row",
                span { class: "dx-sidebar-controls-label", "Collapse" }
                div { class: "dx-sidebar-controls-actions",
                    Button {
                        variant: if collapsible() == SidebarCollapsible::Offcanvas { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::Offcanvas),
                        class: "dx-sidebar-controls-button",
                        "Offcanvas"
                    }
                    Button {
                        variant: if collapsible() == SidebarCollapsible::Icon { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::Icon),
                        class: "dx-sidebar-controls-button",
                        "Icon"
                    }
                    Button {
                        variant: if collapsible() == SidebarCollapsible::None { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::None),
                        class: "dx-sidebar-controls-button",
                        "None"
                    }
                }
            }
        }
    }
}

#[component]
fn Icon(#[props(default = "dx-sidebar-icon")] class: &'static str) -> Element {
    rsx! {
        icon::Icon {
            class,
            width: "24px",
            height: "24px",
            circle { cx: "12", cy: "12", r: "10" }
        }
    }
}

#[component]
fn ChevronIcon() -> Element {
    rsx! {
        icon::Icon {
            class: "dx-sidebar-icon dx-sidebar-chevron",
            width: "24px",
            height: "24px",
            path { d: "m9 18 6-6-6-6" }
        }
    }
}
