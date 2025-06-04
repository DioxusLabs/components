use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger,
};
#[component]
pub(super) fn Demo() -> Element {
    let mut selected_value = use_signal(String::new);

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
                            class: "menubar-item",
                            value: "new".to_string(),
                            on_select: move |value| {
                                selected_value.set(value);
                            },
                            "New"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "open".to_string(),
                            on_select: move |value| {
                                selected_value.set(value);
                            },
                            "Open"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "save".to_string(),
                            on_select: move |value| {
                                selected_value.set(value);
                            },
                            "Save"
                        }
                    }
                }
                MenubarMenu { class: "menubar-menu", index: 1usize,
                    MenubarTrigger { class: "menubar-trigger", "Edit" }
                    MenubarContent { class: "menubar-content",
                        MenubarItem {
                            class: "menubar-item",
                            value: "cut".to_string(),
                            on_select: move |value| {
                                selected_value.set(value);
                            },
                            "Cut"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "copy".to_string(),
                            on_select: move |value| {
                                selected_value.set(value);
                            },
                            "Copy"
                        }
                        MenubarItem {
                            class: "menubar-item",
                            value: "paste".to_string(),
                            on_select: move |value| {
                                selected_value.set(value);
                            },
                            "Paste"
                        }
                    }
                }
            }
            div { class: "selected-value",
                if selected_value().is_empty() {
                    "No selection"
                } else {
                    "Selected: {selected_value()}"
                }
            }
        }
    }
}
