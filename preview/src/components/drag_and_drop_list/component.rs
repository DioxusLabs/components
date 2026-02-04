use dioxus::prelude::*;
use dioxus_primitives::drag_and_drop_list::{self, DragAndDropListProps, DragAndDropListItemProps};

#[component]
pub fn DragAndDropList(props: DragAndDropListProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        drag_and_drop_list::DragAndDropList { attributes: props.attributes, items: props.items, {props.children} }
    }
}

#[component]
pub fn DragAndDropListItem(props: DragAndDropListItemProps) -> Element {
    rsx! {
        drag_and_drop_list::DragAndDropListItem { index: props.index, attributes: props.attributes, {props.children} }
    }
}
