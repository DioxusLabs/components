use dioxus::{document::eval, prelude::*};
use primitives::{
    accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger},
    aspect_ratio::AspectRatio,
    checkbox::{Checkbox, CheckboxIndicator},
    collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger},
    separator::Separator,
};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }

        h1 { "Components Preview" }

        Collapsible {
            CollapsibleTrigger { "Form Example" }
            CollapsibleContent {
                FormExample {}
            }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        Collapsible {
            CollapsibleTrigger { "Aspect Ratio Example" }
            CollapsibleContent {
                AspectRatioExample {}
            }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }

        Collapsible {
            disabled: true,

            CollapsibleTrigger { "Accordion Example" }
            CollapsibleContent {
                AccordionExample {}
            }
        }

        Separator {
            class: "separator",
            style: "margin: 15px 0;",
            horizontal: true,
            decorative: true,
        }
    }
}

#[component]
fn FormExample() -> Element {
    rsx! {

        form {
            onsubmit: move |e| {
                println!("{:?}", e.values());
            },

            Checkbox {
                id: "tos-check",
                name: "tos-check",
                CheckboxIndicator { "+" }
            }
            label {
                for: "tos-check",
                "I agree to the terms presented."
            }
            br {}
            button {
                type: "submit",
                "Submit"
            }
        }
    }
}

#[component]
fn AspectRatioExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/aspect-ratio.css") }
        div {
            class: "aspect-ratio-container",
            AspectRatio {
                ratio: 4.0 / 3.0,
                img {
                    class: "aspect-ratio-image",
                    src: "https://upload.wikimedia.org/wikipedia/commons/thumb/e/ea/Van_Gogh_-_Starry_Night_-_Google_Art_Project.jpg/1280px-Van_Gogh_-_Starry_Night_-_Google_Art_Project.jpg",
                }
            }
        }
    }
}

#[component]
fn AccordionExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/accordion.css") }
        Accordion {
            class: "accordion",
            allow_multiple_open: false,
            horizontal: false,

            for i in 0..4 {
                AccordionItem {
                    class: "accordion-item",
                    index: i,

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
            }

            // AccordionItem {
            //     class: "accordion-item",
            //     default_open: true,

            //     AccordionTrigger {
            //         class: "accordion-trigger",
            //         "This statement is false",
            //     }
            //     AccordionContent {
            //         class: "accordion-content",
            //         div {
            //             class: "accordion-content-inner",
            //             p { "hi" }
            //         }
            //     }
            // }

            // AccordionItem {
            //     class: "accordion-item",

            //     AccordionTrigger {
            //         class: "accordion-trigger",
            //         "Does it work?",
            //     }
            //     AccordionContent {
            //         class: "accordion-content",
            //         div {
            //             class: "accordion-content-inner",
            //             p { "If you can see this, good news! It does!" }
            //         }
            //     }
            // }
        }
    }
}
