use dioxus::prelude::*;
use dioxus_primitives::navbar::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger};
use crate::Route;

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/navbar/style.css"),
        }
        div { class: "navbar-example",
            Navbar { class: "navbar",
                aria_label: "Components",
                NavbarNav { class: "navbar-nav", index: 0usize,
                    NavbarTrigger { class: "navbar-trigger",
                        "Inputs"
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
                            value: "calendar".to_string(),
                            to: Route::ComponentDemo { component_name: "calendar".into() },
                            "Calendar"
                        }
                        NavbarItem {
                            index: 1usize,
                            class: "navbar-item",
                            value: "slider".to_string(),
                            to: Route::ComponentDemo { component_name: "slider".into() },
                            "Slider"
                        }
                        NavbarItem {
                            index: 2usize,
                            class: "navbar-item",
                            value: "checkbox".to_string(),
                            to: Route::ComponentDemo { component_name: "checkbox".into() },
                            "Checkbox"
                        }
                        NavbarItem {
                            index: 3usize,
                            class: "navbar-item",
                            value: "radio_group".to_string(),
                            to: Route::ComponentDemo { component_name: "radio_group".into() },
                            "Radio Group"
                        }
                    }
                }
                NavbarNav { class: "navbar-nav", index: 1usize,
                    NavbarTrigger { class: "navbar-trigger",
                        "Information"
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
                            value: "toast".to_string(),
                            to: Route::ComponentDemo { component_name: "toast".into() },
                            "Toast"
                        }
                        NavbarItem {
                            index: 1usize,
                            class: "navbar-item",
                            value: "tabs".to_string(),
                            to: Route::ComponentDemo { component_name: "tabs".into() },
                            "Tabs"
                        }
                        NavbarItem {
                            index: 2usize,
                            class: "navbar-item",
                            value: "dialog".to_string(),
                            to: Route::ComponentDemo { component_name: "dialog".into() },
                            "Dialog"
                        }
                        NavbarItem {
                            index: 3usize,
                            class: "navbar-item",
                            value: "alert_dialog".to_string(),
                            to: Route::ComponentDemo { component_name: "alert_dialog".into() },
                            "Alert Dialog"
                        }
                        NavbarItem {
                            index: 4usize,
                            class: "navbar-item",
                            value: "tooltip".to_string(),
                            to: Route::ComponentDemo { component_name: "tooltip".into() },
                            "Tooltip"
                        }
                    }
                }
                NavbarItem {
                    index: 2usize,
                    class: "navbar-item",
                    value: "home".to_string(),
                    to: Route::Home,
                    "Home"
                }
            }
        }
    }
}
