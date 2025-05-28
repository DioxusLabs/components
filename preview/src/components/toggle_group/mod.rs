use dioxus::prelude::*;
use dioxus_primitives::toggle_group::{ToggleGroup, ToggleItem};

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/components/toggle_group/style.css") }
        ToggleGroup { class: "toggle-group", horizontal: true,
            ToggleItem { class: "toggle-item", index: 0usize, "Align Left" }
            ToggleItem { class: "toggle-item", index: 1usize, "Align Middle" }
            ToggleItem { class: "toggle-item", index: 2usize, "Align Right" }
        }
    }
}
