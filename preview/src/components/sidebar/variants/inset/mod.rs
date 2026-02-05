use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
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
    Team {
        name: "Acme Inc",
        plan: "Enterprise",
    },
    Team {
        name: "Acme Corp.",
        plan: "Startup",
    },
    Team {
        name: "Evil Corp.",
        plan: "Free",
    },
];

const NAV_MAIN: &[NavMainItem] = &[
    NavMainItem {
        title: "Playground",
        url: "#",
        is_active: true,
        items: &[
            SubItem {
                title: "History",
                url: "#",
            },
            SubItem {
                title: "Starred",
                url: "#",
            },
            SubItem {
                title: "Settings",
                url: "#",
            },
        ],
    },
    NavMainItem {
        title: "Models",
        url: "#",
        is_active: false,
        items: &[
            SubItem {
                title: "Genesis",
                url: "#",
            },
            SubItem {
                title: "Explorer",
                url: "#",
            },
            SubItem {
                title: "Quantum",
                url: "#",
            },
        ],
    },
    NavMainItem {
        title: "Documentation",
        url: "#",
        is_active: false,
        items: &[
            SubItem {
                title: "Introduction",
                url: "#",
            },
            SubItem {
                title: "Get Started",
                url: "#",
            },
            SubItem {
                title: "Tutorials",
                url: "#",
            },
            SubItem {
                title: "Changelog",
                url: "#",
            },
        ],
    },
    NavMainItem {
        title: "Settings",
        url: "#",
        is_active: false,
        items: &[
            SubItem {
                title: "General",
                url: "#",
            },
            SubItem {
                title: "Team",
                url: "#",
            },
            SubItem {
                title: "Billing",
                url: "#",
            },
            SubItem {
                title: "Limits",
                url: "#",
            },
        ],
    },
];

const PROJECTS: &[Project] = &[
    Project {
        name: "Design Engineering",
        url: "#",
    },
    Project {
        name: "Sales & Marketing",
        url: "#",
    },
    Project {
        name: "Travel",
        url: "#",
    },
];

#[component]
pub fn Demo() -> Element {
    let side = use_signal(|| SidebarSide::Left);
    let collapsible = use_signal(|| SidebarCollapsible::Offcanvas);

    rsx! {
        SidebarProvider {
            Sidebar {
                variant: SidebarVariant::Inset,
                collapsible: collapsible(),
                side: side(),
                SidebarHeader {
                    TeamSwitcher { teams: TEAMS }
                }
                SidebarContent {
                    NavMain { items: NAV_MAIN }
                    NavProjects { projects: PROJECTS }
                }
                SidebarFooter { NavUser {} }
                SidebarRail {}
            }
            SidebarInset {
                header { style: "display:flex; align-items:center; justify-content:space-between; height:3.5rem; flex-shrink:0; padding:0 1rem; border-bottom:1px solid var(--sidebar-border); background:var(--primary-color-1);",
                    div { style: "display: flex; align-items: center; gap: 0.75rem;",
                        SidebarTrigger {}
                        Separator { height: "1rem", horizontal: false }
                        span { "Sidebar Setting" }
                    }
                }
                div { style: "display:flex; flex:1; flex-direction:column; gap:1.5rem; padding:1.5rem; min-height:0; overflow-y:auto; overflow-x:hidden;",
                    DemoSettingControls { side, collapsible }
                    Skeleton { style: "height: 10rem; width: 100%; flex-shrink:0;" }
                    Skeleton { style: "height: 20rem; width: 100%; flex-shrink:0;" }
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
                                div { style: "display:flex; flex-shrink:0; align-items:center; justify-content:center; width:2rem; height:2rem; aspect-ratio:1; border-radius:0.5rem; background:var(--sidebar-accent); color:var(--sidebar-accent-foreground);",
                                    Icon {}
                                }
                                div { class: "sidebar-info-block",
                                    span { class: "sidebar-info-title", {teams[active_team()].name} }
                                    span { class: "sidebar-info-subtitle", {teams[active_team()].plan} }
                                }
                                ChevronIcon {}
                            }
                        },
                    }
                    DropdownMenuContent {
                        div { style: "padding:0.5rem; font-size:0.75rem; opacity:0.7;",
                            "Teams"
                        }
                        for (idx , team) in teams.iter().enumerate() {
                            DropdownMenuItem {
                                index: idx,
                                value: idx,
                                on_select: move |v: usize| active_team.set(v),
                                Icon {}
                                {team.name}
                                span { style: "margin-left:auto; font-size:0.75rem; opacity:0.7;",
                                    "⌘{idx + 1}"
                                }
                            }
                        }
                        Separator { decorative: true }
                        DropdownMenuItem {
                            index: teams.len(),
                            value: 999usize,
                            on_select: move |_: usize| {},
                            Icon {}
                            div { style: "opacity:0.7; font-weight:500;", "Add team" }
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
        SidebarGroup { class: "sidebar-hide-on-collapse",
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
                                        span { class: "sr-only", "More" }
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
                    SidebarMenuButton { style: "opacity:0.7; font-weight:500;",
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
                                Avatar { size: AvatarImageSize::Small, style: "border-radius:0.5rem;",
                                    AvatarImage {
                                        src: asset!("/assets/dioxus-logo.png", ImageAssetOptions::new().with_avif()),
                                        alt: "dioxus avatar",
                                    }
                                    AvatarFallback { "DX" }
                                }
                                div { class: "sidebar-info-block",
                                    span { class: "sidebar-info-title", "Dioxus" }
                                    span { class: "sidebar-info-subtitle", "m@example.com" }
                                }
                                ChevronIcon {}
                            }
                        },
                    }
                    DropdownMenuContent {
                        div { style: "display:flex; align-items:center; gap:0.5rem; padding:0.375rem 0.25rem; text-align:left; font-size:0.875rem;",
                            Avatar {
                                size: AvatarImageSize::Small,
                                style: "border-radius:0.5rem;",
                                AvatarImage {
                                    src: asset!("/assets/dioxus-logo.png", ImageAssetOptions::new().with_avif()),
                                    alt: "dioxus avatar",
                                }
                                AvatarFallback { "DX" }
                            }
                            div { class: "sidebar-info-block",
                                span { class: "sidebar-info-title", "Dioxus" }
                                span { class: "sidebar-info-subtitle", "m@example.com" }
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
        div { style: "display: flex; flex-direction: column; gap: 0.75rem; padding: 0.75rem; border: 1px solid var(--sidebar-border); border-radius: 0.75rem; background: var(--primary-color-2);",
            div { style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; flex-wrap: wrap;",
                span { style: "font-size: 0.75rem; font-weight: 600; color: var(--secondary-color-4);",
                    "Side"
                }
                div { style: "display: inline-flex; gap: 0.5rem;",
                    Button {
                        variant: if side() == SidebarSide::Left { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| side.set(SidebarSide::Left),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Left"
                    }
                    Button {
                        variant: if side() == SidebarSide::Right { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| side.set(SidebarSide::Right),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Right"
                    }
                }
            }
            div { style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; flex-wrap: wrap;",
                span { style: "font-size: 0.75rem; font-weight: 600; color: var(--secondary-color-4);",
                    "Collapse"
                }
                div { style: "display: inline-flex; gap: 0.5rem; flex-wrap: wrap;",
                    Button {
                        variant: if collapsible() == SidebarCollapsible::Offcanvas { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::Offcanvas),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Offcanvas"
                    }
                    Button {
                        variant: if collapsible() == SidebarCollapsible::Icon { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::Icon),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Icon"
                    }
                    Button {
                        variant: if collapsible() == SidebarCollapsible::None { ButtonVariant::Primary } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::None),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "None"
                    }
                }
            }
        }
    }
}

#[component]
fn Icon(#[props(default = "sidebar-icon")] class: &'static str) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "10" }
        }
    }
}

#[component]
fn ChevronIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "sidebar-icon sidebar-chevron",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "m9 18 6-6-6-6" }
        }
    }
}
