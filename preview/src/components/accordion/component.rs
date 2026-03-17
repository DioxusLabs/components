use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    self, AccordionContentProps, AccordionItemProps, AccordionProps, AccordionTriggerProps,
};
use dioxus_primitives::icon;

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        accordion::Accordion {
            class: "accordion",
            width: "15rem",
            id: props.id,
            allow_multiple_open: props.allow_multiple_open,
            disabled: props.disabled,
            collapsible: props.collapsible,
            horizontal: props.horizontal,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    rsx! {
        accordion::AccordionItem {
            class: "accordion-item",
            disabled: props.disabled,
            default_open: props.default_open,
            on_change: props.on_change,
            on_trigger_click: props.on_trigger_click,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    rsx! {
        accordion::AccordionTrigger {
            class: "accordion-trigger",
            id: props.id,
            attributes: props.attributes,
            {props.children}
            icon::Icon {
                class: "accordion-expand-icon",
                width: 20,
                height: 20,
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    rsx! {
        accordion::AccordionContent {
            class: "accordion-content",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
