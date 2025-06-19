use dioxus::prelude::*;

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/input/style.css"),
        }

        input {
            class: "input",
            placeholder: "Enter your name",
        }
    }
}
