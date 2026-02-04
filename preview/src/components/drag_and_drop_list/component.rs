use dioxus::prelude::*;
use dioxus_primitives::drag_and_drop_list::{self, DragAndDropListItemProps, DragAndDropListProps};

use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(EnumIter, Display)]
enum Animals {
    Cat,
    Cow,
    Dog,
    Fox,
    Pig,
}

impl Animals {
    const fn emoji(&self) -> &'static str {
        match self {
            Animals::Cat => "🐱",
            Animals::Cow => "🐮",
            Animals::Dog => "🐶",
            Animals::Fox => "🦊",
            Animals::Pig => "🐷",
        }
    }
}

pub fn default_items() -> Vec<Element> {
    Animals::iter()
        .map(|a| {
            rsx! {
                {format!("{} {a}", a.emoji())}
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
