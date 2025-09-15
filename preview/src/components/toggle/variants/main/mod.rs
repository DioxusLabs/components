use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { width: "2rem", height: "2rem",
            em { "B" }
        }
    }
}
