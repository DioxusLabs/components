use dioxus::prelude::*;
use dioxus_primitives::scroll_area::{ScrollArea, ScrollDirection};
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/scroll_area/style.css"),
        }
        div { class: "scroll-area-demo",
            div { class: "scroll-demo-section",
                h3 { "Vertical Scroll" }
                ScrollArea {
                    class: "demo-scroll-area",
                    direction: ScrollDirection::Vertical,
                    div { class: "scroll-content",
                        for i in 1..=20 {
                            p { "Scrollable content item {i}" }
                        }
                    }
                }
            }
            div { class: "scroll-demo-section",
                h3 { "Horizontal Scroll" }
                ScrollArea {
                    class: "demo-scroll-area",
                    direction: ScrollDirection::Horizontal,
                    div { class: "scroll-content-horizontal",
                        for i in 1..=20 {
                            span { "Column {i} " }
                        }
                    }
                }
            }
            div { class: "scroll-demo-section",
                h3 { "Both Directions" }
                ScrollArea {
                    class: "demo-scroll-area",
                    direction: ScrollDirection::Both,
                    div { class: "scroll-content-both",
                        for i in 1..=20 {
                            div {
                                for j in 1..=20 {
                                    span { "Cell {i},{j} " }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
