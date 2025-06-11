use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger,
};
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/menubar/style.css"),
        }
        div { class: "menubar-example",
            Menubar { class: "menubar",
                MenubarMenu { class: "menubar-menu", index: 0usize,
                    MenubarTrigger { class: "menubar-trigger", "File" }
                    MenubarContent { class: "menubar-content",
                        MenubarItem {
                            index: 0usize,
                            class: "menubar-item",
                            value: "new".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "New"
                        }
                        MenubarItem {
                            index: 1usize,
                            class: "menubar-item",
                            value: "open".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Open"
                        }
                        MenubarItem {
                            index: 2usize,
                            class: "menubar-item",
                            value: "save".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Save"
                        }
                    }
                }
                MenubarMenu { class: "menubar-menu", index: 1usize,
                    MenubarTrigger { class: "menubar-trigger", "Edit" }
                    MenubarContent { class: "menubar-content",
                        MenubarItem {
                            index: 0usize,
                            class: "menubar-item",
                            value: "cut".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Cut"
                        }
                        MenubarItem {
                            index: 1usize,
                            class: "menubar-item",
                            value: "copy".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Copy"
                        }
                        MenubarItem {
                            index: 2usize,
                            class: "menubar-item",
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
