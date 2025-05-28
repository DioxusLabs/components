use dioxus::prelude::*;
use dioxus_primitives::radio_group::{RadioGroup, RadioItem};

#[component]
pub(super) fn Demo() -> Element {
    let mut value = use_signal(|| String::from("option1"));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/components/radio_group/style.css") }
        RadioGroup {
            class: "radio-group",
            value,
            on_value_change: move |new_value| {
                value.set(new_value);
            },

            RadioItem {
                class: "radio-item",
                value: "option1".to_string(),
                index: 0usize,
                "Option 1"
            }
            RadioItem {
                class: "radio-item",
                value: "option2".to_string(),
                index: 1usize,
                "Option 2"
            }
            RadioItem {
                class: "radio-item",
                value: "option3".to_string(),
                index: 2usize,
                "Option 3"
            }
        }

        div { style: "margin-top: 1rem;", "Selected value: {value()}" }
    }
}
