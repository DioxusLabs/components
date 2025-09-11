use super::super::component::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use dioxus::prelude::*;
#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/accordion/style.css"),
        }
        Accordion {
            allow_multiple_open: false,
            horizontal: false,
            for i in 0..4 {
                AccordionItem {
                    index: i,
                    AccordionTrigger {
                        "the quick brown fox"
                    }
                    AccordionContent {
                        div { padding_bottom: "1rem",
                            p {
                                padding: "0",
                                "lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum"
                            }
                        }
                    }
                }
            }
        }
    }
}
