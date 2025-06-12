use dioxus::prelude::*;
use dioxus_primitives::scroll_area::{ScrollArea, ScrollDirection};
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/scroll_area/style.css"),
        }
        ScrollArea {
            width: "10em",
            height: "10em",
            border: "1px solid var(--dim-border-color)",
            border_radius: "0.5em",
            padding: "0 1em 1em 1em",
            direction: ScrollDirection::Vertical,
            tabindex: "0",
            div { class: "scroll-content",
                for i in 1..=20 {
                    p {
                        "Scrollable content item {i}"
                    }
                }
            }
        }
    }
}
