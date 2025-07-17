use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};
#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/checkbox/variants/main/style.css"),
        }
        Checkbox {
            class: "checkbox",
            name: "tos-check",
            aria_label: "Demo Checkbox",
            CheckboxIndicator {
                class: "checkbox-indicator",
                svg {
                    class: "checkbox-check-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M5 13l4 4L19 7" }
                }
            }
        }
    }
}