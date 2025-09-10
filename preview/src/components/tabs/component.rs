use dioxus::prelude::*;
use dioxus_primitives::tabs::{self, TabContentProps, TabListProps, TabTriggerProps, TabsProps};

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/tabs/variants/main/style.css"),
        }
        tabs::Tabs {
            class: "tabs",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TabList(props: TabListProps) -> Element {
    rsx! {
        tabs::TabList {
            class: "tabs-list",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TabTrigger(props: TabTriggerProps) -> Element {
    rsx! {
        tabs::TabTrigger {
            class: "tabs-trigger",
            id: props.id,
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TabContent(props: TabContentProps) -> Element {
    rsx! {
        tabs::TabContent {
            class: "tabs-content",
            value: props.value,
            id: props.id,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}
