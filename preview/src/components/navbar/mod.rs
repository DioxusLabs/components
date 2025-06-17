use dioxus::prelude::*;
use dioxus_primitives::navbar::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger};
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/navbar/style.css"),
        }
        div { class: "navbar-example",
            Navbar { class: "navbar",
                NavbarNav { class: "navbar-nav", index: 0usize,
                    NavbarTrigger { class: "navbar-trigger",
                        "File"
                        svg {
                            class: "navbar-expand-icon",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            polyline { points: "6 9 12 15 18 9" }
                        }
                    }
                    NavbarContent { class: "navbar-content",
                        NavbarItem {
                            index: 0usize,
                            class: "navbar-item",
                            value: "new".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "New"
                        }
                        NavbarItem {
                            index: 1usize,
                            class: "navbar-item",
                            value: "open".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Open"
                        }
                        NavbarItem {
                            index: 2usize,
                            class: "navbar-item",
                            value: "save".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Save"
                        }
                    }
                }
                NavbarNav { class: "navbar-nav", index: 1usize,
                    NavbarTrigger { class: "navbar-trigger",
                        "Edit"
                        svg {
                            class: "navbar-expand-icon",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            polyline { points: "6 9 12 15 18 9" }
                        }
                    }
                    NavbarContent { class: "navbar-content",
                        NavbarItem {
                            index: 0usize,
                            class: "navbar-item",
                            value: "cut".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Cut"
                        }
                        NavbarItem {
                            index: 1usize,
                            class: "navbar-item",
                            value: "copy".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Copy"
                        }
                        NavbarItem {
                            index: 2usize,
                            class: "navbar-item",
                            value: "paste".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Paste"
                        }
                    }
                }
            }
        }
    }
}
