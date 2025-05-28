use dioxus::prelude::*;
use dioxus_primitives::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};


#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/components/accordion/style.css") }
        Accordion {
            class: "accordion",
            allow_multiple_open: false,
            horizontal: false,

            for i in 0..4 {
                AccordionItem {
                    class: "accordion-item",
                    index: i,

                    on_change: move |open| {
                        tracing::info!("console.log({open});");
                    },
                    on_trigger_click: move || {
                        tracing::info!("trigger");
                    },

                    AccordionTrigger { class: "accordion-trigger", "the quick brown fox" }
                    AccordionContent { class: "accordion-content",
                        div { class: "accordion-content-inner",
                            p { "lorem ipsum lorem ipsum" }
                        }
                    }
                }
            }
        }
    }
}
