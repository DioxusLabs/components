use dioxus::prelude::*;
use dioxus_primitives::badge::{self, BadgeProps};

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        badge::Badge {
            count: props.count,
            overflow_count: props.overflow_count,
            dot: props.dot,
            show_zero: props.show_zero,
            color: props.color,
            attributes: props.attributes,
            {props.children}
        }
    }
}
