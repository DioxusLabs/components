use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/accordion/style.css"),
        }
        Accordion {
            class: "accordion",
            width: "15rem",
            allow_multiple_open: false,
            horizontal: false,
            for i in 0..4 {
                AccordionItem {
                    class: "accordion-item",
                    index: i,
                    on_change: move |open| {
                        tracing::info!("{open};");
                    },
                    on_trigger_click: move || {
                        tracing::info!("trigger");
                    },
                    AccordionTrigger { class: "accordion-trigger",
                        "the quick brown fox"
                        svg {
                            class: "accordion-expand-icon",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            polyline { points: "6 9 12 15 18 9" }
                        }
                    }
                    AccordionContent { class: "accordion-content", style: "--collapsible-content-width: 140px",
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
