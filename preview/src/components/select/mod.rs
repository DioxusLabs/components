use dioxus::prelude::*;
use dioxus_primitives::select::{Select, SelectGroup, SelectOption};
#[component]
pub(super) fn Demo() -> Element {
    let mut selected = use_signal(|| None::<String>);
    use_effect(move || {
        if let Some(value) = selected() {
            println!("Selected value: {value}");
        }
    });
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/select/style.css"),
        }
        div { class: "select-example",
            div { class: "select-container",
                label { class: "select-label", "Choose a fruit:" }
                Select {
                    class: "select",
                    value: selected,
                    on_value_change: move |value| selected.set(value),
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
            div { class: "selected-value",
                if let Some(value) = selected() {
                    "Selected: {value}"
                } else {
                    "No selection"
                }
            }
        }
    }
}
