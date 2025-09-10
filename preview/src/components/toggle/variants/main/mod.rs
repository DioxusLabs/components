use dioxus::prelude::*;
use dioxus_primitives::toggle::Toggle;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { width: "2rem", height: "2rem",
            em { "B" }
        }
    }
}
