use dioxus::prelude::*;
use dioxus_primitives::label::Label;

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/label/variants/main/style.css"),
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/input/variants/main/style.css"),
        }

        div {
            display: "flex",
            flex_direction: "column",
            gap: ".5rem",
            Label {
                class: "label",
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
