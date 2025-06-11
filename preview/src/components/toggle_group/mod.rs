use dioxus::prelude::*;
use dioxus_primitives::toggle_group::{ToggleGroup, ToggleItem};
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/toggle_group/style.css"),
        }
        ToggleGroup { class: "toggle-group", horizontal: true, allow_multiple_pressed: true,
            ToggleItem { class: "toggle-item", index: 0usize, em { "B" } }
            ToggleItem { class: "toggle-item", index: 1usize, i { "I" } }
            ToggleItem { class: "toggle-item", index: 2usize, u { "U" } }
        }
    }
}
