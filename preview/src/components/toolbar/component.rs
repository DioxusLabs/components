use dioxus::prelude::*;
use dioxus_primitives::toolbar::{self, ToolbarButtonProps, ToolbarProps, ToolbarSeparatorProps};

#[component]
pub fn Toolbar(props: ToolbarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        toolbar::Toolbar {
            class: "toolbar",
            aria_label: props.aria_label,
            disabled: props.disabled,
            horizontal: props.horizontal,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    rsx! {
        toolbar::ToolbarButton {
            class: "toolbar-button",
            index: props.index,
            disabled: props.disabled,
            on_click: props.on_click,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ToolbarSeparator(props: ToolbarSeparatorProps) -> Element {
    rsx! {
        toolbar::ToolbarSeparator {
            class: "toolbar-separator",
            decorative: props.decorative,
            horizontal: props.horizontal,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn ToolbarGroup(
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "toolbar-group", ..attributes, {children} }
    }
}
