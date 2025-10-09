use dioxus::prelude::*;
use dioxus_primitives::dialog::{
    self, DialogContentProps, DialogDescriptionProps, DialogRootProps, DialogTitleProps,
};

#[component]
pub fn DialogRoot(props: DialogRootProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        dialog::DialogRoot {
            class: "dialog-backdrop",
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
    rsx! {
        dialog::DialogContent { class: "dialog", id: props.id, attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    rsx! {
        dialog::DialogTitle {
            class: "dialog-title",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    rsx! {
        dialog::DialogDescription {
            class: "dialog-description",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
