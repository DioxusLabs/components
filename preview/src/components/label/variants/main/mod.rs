use crate::components::input::component::Input;

use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "0.875rem",
            min_width: "16rem",
            div { display: "flex", flex_direction: "column", gap: "0.4rem",
                Label { html_for: "label-name", "Full name" }
                Input { id: "label-name", placeholder: "Ada Lovelace" }
            }
            div { display: "flex", flex_direction: "column", gap: "0.4rem",
                Label { html_for: "label-email", "Email" }
                Input { id: "label-email", placeholder: "ada@example.com" }
            }
        }
    }
}
