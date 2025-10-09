use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Checkbox { name: "tos-check", aria_label: "Demo Checkbox" }
    }
}
