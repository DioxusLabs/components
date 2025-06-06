use dioxus::prelude::*;
use dioxus_primitives::select::{Select, SelectGroup, SelectOption};
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
            SelectGroup { label: "Fruits".to_string(),
                SelectOption { value: "apple".to_string(), "Apple" }
                SelectOption { value: "banana".to_string(), "Banana" }
                SelectOption { value: "orange".to_string(), "Orange" }
                SelectOption { value: "strawberry".to_string(), "Strawberry" }
                SelectOption { value: "watermelon".to_string(), "Watermelon" }
            }
            SelectGroup { label: "Other".to_string(),
                SelectOption { value: "other".to_string(), "Other" }
            }
        }
    }
}
