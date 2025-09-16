use dioxus::prelude::*;
use dioxus_primitives::hover_card::{
    self, HoverCardContentProps, HoverCardProps, HoverCardTriggerProps,
};

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        hover_card::HoverCard {
            class: "hover-card",
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    rsx! {
        hover_card::HoverCardTrigger {
            class: "hover-card-trigger",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    rsx! {
        hover_card::HoverCardContent {
            class: "hover-card-content",
            side: props.side,
            align: props.align,
            id: props.id,
            force_mount: props.force_mount,
            attributes: props.attributes,
            {props.children}
        }
    }
}
