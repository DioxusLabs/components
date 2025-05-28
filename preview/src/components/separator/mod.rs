use dioxus::prelude::*;
use dioxus_primitives::separator::Separator;

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/components/separator/style.css") }
        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }
    }
}
