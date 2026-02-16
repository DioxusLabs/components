use dioxus::prelude::*;
use dioxus_primitives::drag_and_drop_list::{self, DragAndDropListItemProps, DragAndDropListProps};

pub fn example_items() -> Vec<Element> {
    let animals = ["Cat 🐱", "Cow 🐮", "Dog 🐶", "Fox 🦊", "Pig 🐷"];

    animals
        .iter()
        .map(|&text| {
            rsx! {
                {text}
            }
        })
        .collect()
}

#[component]
pub fn DragAndDropList(props: DragAndDropListProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        drag_and_drop_list::DragAndDropList {
            items: props.items,
            is_removable: props.is_removable,
            aria_label: props.aria_label,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DragAndDropListItem(props: DragAndDropListItemProps) -> Element {
    rsx! {
        drag_and_drop_list::DragAndDropListItem {
            index: props.index,
            is_removable: props.is_removable,
            attributes: props.attributes,
            {props.children}
        }
    }
}
