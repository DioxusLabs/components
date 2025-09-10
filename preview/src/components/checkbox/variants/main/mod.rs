use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/checkbox/variants/main/style.css"),
        }
        Checkbox {
            name: "tos-check",
            aria_label: "Demo Checkbox",
        }
    }
}
