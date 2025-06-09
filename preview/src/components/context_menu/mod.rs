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
                background: "var(--background-color)",
                border: "1px dashed var(--dim-border-color)",
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
                    "Edit"
                }
                ContextMenuItem {
                    class: "context-menu-item",
                    value: "undo".to_string(),
                    index: 1usize,
                    disabled: true,
                    "Undo"
                }
                ContextMenuItem {
                    class: "context-menu-item",
                    value: "duplicate".to_string(),
                    index: 1usize,
                    "Duplicate"
                }
                ContextMenuItem {
                    class: "context-menu-item",
                    value: "delete".to_string(),
                    index: 2usize,
                    "Delete"
                }
            }
        }
    }
}
