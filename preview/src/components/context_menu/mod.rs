use dioxus::prelude::*;
use dioxus_primitives::context_menu::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
};
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/context_menu/style.css"),
        }
        ContextMenu {
            ContextMenuTrigger {
                padding: "20px",
                background: "var(--primary-color)",
                border: "1px dashed var(--primary-color-6)",
                border_radius: ".5rem",
                cursor: "context-menu",
                user_select: "none",
                text_align: "center",
                "right click here"
            }
            ContextMenuContent { class: "context-menu-content",
                ContextMenuItem {
                    class: "context-menu-item",
                    value: "edit".to_string(),
                    index: 0usize,
                    on_select: move |value| {
                        tracing::info!("Selected item: {}", value);
                    },
                    "Edit"
                }
                ContextMenuItem {
                    class: "context-menu-item",
                    value: "undo".to_string(),
                    index: 1usize,
                    disabled: true,
                    on_select: move |value| {
                        tracing::info!("Selected item: {}", value);
                    },
                    "Undo"
                }
                ContextMenuItem {
                    class: "context-menu-item",
                    value: "duplicate".to_string(),
                    index: 2usize,
                    on_select: move |value| {
                        tracing::info!("Selected item: {}", value);
                    },
                    "Duplicate"
                }
                ContextMenuItem {
                    class: "context-menu-item",
                    value: "delete".to_string(),
                    index: 3usize,
                    on_select: move |value| {
                        tracing::info!("Selected item: {}", value);
                    },
                    "Delete"
                }
            }
        }
    }
}
