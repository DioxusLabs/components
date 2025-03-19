use dioxus::{document::eval, prelude::*};
use primitives::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

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
            collapsible: true,

            AccordionItem {
                class: "accordion-item",
                on_change: move |open| {
                    eval(&format!("console.log({open});"));
                },
                on_trigger_click: move || {
                    eval("console.log('trigger');");
                },

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
                default_open: true,

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
