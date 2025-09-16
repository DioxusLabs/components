use dioxus::prelude::*;
use dioxus_primitives::collapsible::{
    self, CollapsibleContentProps, CollapsibleProps, CollapsibleTriggerProps,
};

#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        collapsible::Collapsible {
            keep_mounted: props.keep_mounted,
            default_open: props.default_open,
            disabled: props.disabled,
            open: props.open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            class: "collapsible",
            {props.children}
        }
    }
}

#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
    rsx! {
        collapsible::CollapsibleTrigger { class: "collapsible-trigger", attributes: props.attributes,
            {props.children}
            svg {
                class: "collapsible-expand-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                // shifted up by 6 polyline { points: "6 9 12 15 18 9" }
                polyline { points: "6 15 12 21 18 15" }
                // shifted down by 6 polyline { points: "6 15 12 9 18 15" }
                polyline { points: "6 9 12 3 18 9" }
            }
        }
    }
}

#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    rsx! {
        collapsible::CollapsibleContent {
            class: "collapsible-content",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CollapsibleItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            border: "1px solid var(--primary-color-6)",
            border_radius: "0.5rem",
            padding: "1rem",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CollapsibleList(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "0.5rem",
            max_width: "20rem",
            color: "var(--secondary-color-3)",
            ..attributes,
            {children}
        }
    }
}
