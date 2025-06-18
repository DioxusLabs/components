use dioxus::prelude::*;
use dioxus_primitives::select::{Select, SelectGroup, SelectList, SelectOption, SelectTrigger};
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/select/style.css"),
        }
        Select {
            class: "select",
            placeholder: "Select a fruit...",
            SelectTrigger {
                class: "select-trigger",
                aria_label: "Select Trigger",
            }
            SelectList {
                class: "select-list",
                aria_label: "Select Demo",
                SelectGroup {
                    class: "select-group",
                    label: "Fruits".to_string(),
                    SelectOption {
                        index: 0usize,
                        class: "select-option",
                        value: "apple".to_string(),
                        "Apple"
                    }
                    SelectOption {
                        index: 1usize,
                        class: "select-option",
                        value: "banana".to_string(),
                        "Banana"
                    }
                    SelectOption {
                        index: 2usize,
                        class: "select-option",
                        value: "orange".to_string(),
                        "Orange"
                    }
                    SelectOption {
                        index: 3usize,
                        class: "select-option",
                        value: "strawberry".to_string(),
                        "Strawberry"
                    }
                    SelectOption {
                        index: 4usize,
                        class: "select-option",
                        value: "watermelon".to_string(),
                        "Watermelon"
                    }
                }
                SelectGroup {
                    class: "select-group",
                    label: "Other".to_string(),
                    SelectOption {
                        index: 5usize,
                        class: "select-option",
                        value: "other".to_string(),
                        "Other"
                    }
                }
            }
        }
    }
}
