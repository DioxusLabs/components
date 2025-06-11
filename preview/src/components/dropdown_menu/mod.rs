use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/dropdown_menu/style.css"),
        }
        DropdownMenu { class: "dropdown-menu", default_open: false,
            DropdownMenuTrigger { class: "dropdown-menu-trigger", "Open Menu" }
            DropdownMenuContent { class: "dropdown-menu-content",
                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "edit".to_string(),
                    index: 0usize,
                    on_select: move |value| {
                        tracing::info!("Selected: {}", value);
                    },
                    "Edit"
                }
                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "undo".to_string(),
                    index: 1usize,
                    disabled: true,
                    on_select: move |value| {
                        tracing::info!("Selected: {}", value);
                    },
                    "Undo"
                }
                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "duplicate".to_string(),
                    index: 2usize,
                    on_select: move |value| {
                        tracing::info!("Selected: {}", value);
                    },
                    "Duplicate"
                }
                DropdownMenuItem {
                    class: "dropdown-menu-item",
                    value: "delete".to_string(),
                    index: 3usize,
                    on_select: move |value| {
                        tracing::info!("Selected: {}", value);
                    },
                    "Delete"
                }
            }
        }
    }
}
