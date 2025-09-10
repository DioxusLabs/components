use dioxus::prelude::*;
use dioxus_primitives::radio_group::{RadioGroup, RadioItem};

#[component]
pub fn Demo() -> Element {
    rsx! {
        RadioGroup {
            RadioItem {
                value: "option1".to_string(),
                index: 0usize,
                "Blue"
            }
            RadioItem {
                value: "option2".to_string(),
                index: 1usize,
                "Red"
            }
            RadioItem {
                value: "option3".to_string(),
                index: 2usize,
                disabled: true,
                "Green"
            }
        }
    }
}
