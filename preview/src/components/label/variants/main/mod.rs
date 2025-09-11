use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/input/style.css"),
        }

        div {
            display: "flex",
            flex_direction: "column",
            gap: ".5rem",
            Label {
                html_for: "name",
                "Name"
            }

            input {
                class: "input",
                id: "name",
                placeholder: "Enter your name",
            }
        }

    }
}
