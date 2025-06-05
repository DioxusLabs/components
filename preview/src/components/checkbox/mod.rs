use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/checkbox/style.css"),
        }
        Checkbox { class: "checkbox", name: "tos-check",
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