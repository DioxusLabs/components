use dioxus::prelude::*;

use crate::use_aria_id;

#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    #[props(optional, default = "dxa-accordion".into())]
    class: String,

    label: String,
    expanded: Signal<bool>,

    children: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let aria_label_id = use_aria_id();
    let aria_controls_id = use_aria_id();

    rsx! {
        div {
            class: "{props.class}",
            h3 {
                button {
                    id: "{aria_label_id}",
                    aria_expanded: "false",
                    aria_controls: "{aria_controls_id}",
                    "{props.label}"
                }
            }
            div {
                id: "{aria_controls_id}",
                aria_labelledby: "{aria_label_id}",
                role: "region",

                {props.children}
            }
        }
    }
}
