use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        form {
            onsubmit: move |e| {
                println!("{:?}", e.values());
            },

            Checkbox { id: "tos-check", name: "tos-check",
                CheckboxIndicator { "+" }
            }
            label { r#for: "tos-check", "I agree to the terms presented." }
            br {}
            button { r#type: "submit", "Submit" }
        }
    }
}

