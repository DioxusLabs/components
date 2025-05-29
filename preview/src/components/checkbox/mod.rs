use dioxus::prelude::*;
use dioxus_primitives::checkbox::{Checkbox, CheckboxIndicator};

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/components/checkbox/style.css") }
        Checkbox { id: "tos-check", name: "tos-check",
            CheckboxIndicator { "✓" }
        }
    }
}
