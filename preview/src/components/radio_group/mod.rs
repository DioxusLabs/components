use dioxus::prelude::*;
use dioxus_primitives::radio_group::{RadioGroup, RadioItem};
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/radio_group/style.css"),
        }
        RadioGroup {
            class: "radio-group",
            RadioItem {
                class: "radio-item",
                value: "option1".to_string(),
                index: 0usize,
                "Blue"
            }
            RadioItem {
                class: "radio-item",
                value: "option2".to_string(),
                index: 1usize,
                "Red"
            }
            RadioItem {
                class: "radio-item",
                value: "option3".to_string(),
                index: 2usize,
                disabled: true,
                "Green"
            }
        }
    }
}
