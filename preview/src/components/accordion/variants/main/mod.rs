use super::super::component::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let triggers = ["What is Dioxus?", "How do I install it?", "Is it free?"];
    let answers = [
        "A Rust framework for building fullstack web, desktop, and mobile apps.",
        "Run cargo install dioxus-cli, then dx components add to scaffold.",
        "Yes — MIT and Apache-2.0 dual licensed.",
    ];
    rsx! {
        Accordion { allow_multiple_open: false, horizontal: false,
            for (i , trigger) in triggers.iter().enumerate() {
                AccordionItem { index: i,
                    AccordionTrigger { "{trigger}" }
                    AccordionContent {
                        div { padding_bottom: "0.75rem", font_size: "0.85rem",
                            "{answers[i]}"
                        }
                    }
                }
            }
        }
    }
}
