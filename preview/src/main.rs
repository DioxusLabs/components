use primitives::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        h1 { "Components Preview" }
        AccordionExample {}
    }
}

#[component]
fn AccordionExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/accordion.css") }
        Accordion {
            class: "accordion",
            allow_multiple_open: false,
            default_item: "gullible",

            AccordionItem {
                class: "accordion-item",
                disabled: true,
                AccordionTrigger {
                    class: "accordion-trigger",
                    "the quick brown fox",
                }
                AccordionContent {
                    class: "accordion-content",
                    div {
                        class: "accordion-content-inner",
                        p { "lorem ipsum lorem ipsum" }
                    }
                }
            }

            AccordionItem {
                class: "accordion-item",
                name: "gullible",

                AccordionTrigger {
                    class: "accordion-trigger",
                    "This statement is false",
                }
                AccordionContent {
                    class: "accordion-content",
                    div {
                        class: "accordion-content-inner",
                        p { "hi" }
                    }
                }
            }

            AccordionItem {
                class: "accordion-item",
                AccordionTrigger {
                    class: "accordion-trigger",
                    "Does it work?",
                }
                AccordionContent {
                    class: "accordion-content",
                    div {
                        class: "accordion-content-inner",
                        p { "If you can see this, good news! It does!" }
                    }
                }
            }
        }
    }
}
