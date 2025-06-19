use dioxus::prelude::*;
use dioxus_primitives::toggle::Toggle;

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/toggle/style.css"),
        }

        Toggle { class: "toggle", width: "2rem", height: "2rem",
            em { "B" }
        }
    }
}
