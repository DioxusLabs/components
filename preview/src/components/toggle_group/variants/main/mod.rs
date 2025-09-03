use dioxus::prelude::*;
use dioxus_primitives::toggle_group::{ToggleGroup, ToggleItem};
#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/toggle_group/variants/main/style.css"),
        }
        ToggleGroup { class: "toggle-group", horizontal: true, allow_multiple_pressed: true,
            ToggleItem { class: "toggle-item", index: 0usize, b { "B" } }
            ToggleItem { class: "toggle-item", index: 1usize, i { "I" } }
            ToggleItem { class: "toggle-item", index: 2usize, u { "U" } }
        }
    }
}
