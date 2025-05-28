use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger};



#[component]
pub(super) fn DropdownMenuExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./src/dropdown_menu/style.css") }
        DropdownMenu { class: "dropdown-menu", default_open: false,

            DropdownMenuTrigger { class: "dropdown-menu-trigger", "Open Menu" }

            DropdownMenuContent { class: "dropdown-menu-content",

                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "item1".to_string(),
                    index: 0usize,
                    on_select: move |value| {
                        tracing::info!("Selected: {}", value);
                    },
                    "Item 1"
                }

                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "item2".to_string(),
                    index: 1usize,
                    on_select: move |value| {
                        tracing::info!("Selected: {}", value);
                    },
                    "Item 2"
                }

                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "item3".to_string(),
                    index: 2usize,
                    on_select: move |value| {
                        tracing::info!("Selected: {}", value);
                    },
                    "Item 3"
                }
            }
        }
    }
}