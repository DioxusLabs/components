use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/input/variants/main/style.css"),
        }

        input {
            class: "input",
            placeholder: "Enter your name",
        }
    }
}
