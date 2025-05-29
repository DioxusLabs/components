use dioxus::prelude::*;
use dioxus_primitives::separator::Separator;

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/components/separator/style.css") }
        "One thing"
        Separator {
            class: "separator",
            style: "margin: 15px 0; width: 50%;",
            horizontal: true,
            decorative: true,
        }
        "Another thing"
    }
}
