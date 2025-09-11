use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/toggle_group/style.css"),
        }
        ToggleGroup { horizontal: true, allow_multiple_pressed: true,
            ToggleItem { index: 0usize, b { "B" } }
            ToggleItem { index: 1usize, i { "I" } }
            ToggleItem { index: 2usize, u { "U" } }
        }
    }
}
