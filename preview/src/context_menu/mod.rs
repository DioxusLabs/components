use dioxus::prelude::*;
use dioxus_primitives::context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger};


#[component]
pub(super) fn ContextMenuExample() -> Element {
    let mut selected_value = use_signal(String::new);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/context_menu/style.css") }
        div { class: "context-menu-example",
            ContextMenu {
                ContextMenuTrigger { class: "context-menu-trigger", "Right click here to open context menu" }

                ContextMenuContent { class: "context-menu-content",
                    ContextMenuItem {
                        class: "context-menu-item",
                        value: "edit".to_string(),
                        index: 0usize,
                        on_select: move |value| {
                            selected_value.set(value);
                        },
                        "Edit"
                    }

                    ContextMenuItem {
                        class: "context-menu-item",
                        value: "duplicate".to_string(),
                        index: 1usize,
                        on_select: move |value| {
                            selected_value.set(value);
                        },
                        "Duplicate"
                    }

                    ContextMenuItem {
                        class: "context-menu-item",
                        value: "delete".to_string(),
                        index: 2usize,
                        on_select: move |value| {
                            selected_value.set(value);
                        },
                        "Delete"
                    }
                }
            }

            div { class: "selected-value",
                if selected_value().is_empty() {
                    "No action selected"
                } else {
                    "Selected action: {selected_value()}"
                }
            }
        }
    }
}
