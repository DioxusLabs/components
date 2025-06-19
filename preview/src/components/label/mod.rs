use dioxus::prelude::*;
use dioxus_primitives::label::Label;

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/label/style.css"),
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/input/style.css"),
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
