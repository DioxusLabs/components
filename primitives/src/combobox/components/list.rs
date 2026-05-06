//! ComboboxList component.

use dioxus::prelude::*;

use super::super::context::ComboboxContext;
use crate::listbox::{use_listbox_id, ListboxContext};

/// Props for [`ComboboxList`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxListProps {
    /// Optional id for the list element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children, typically [`ComboboxOption`](super::option::ComboboxOption)s
    /// and an optional [`ComboboxEmpty`](super::empty::ComboboxEmpty).
    pub children: Element,
}

/// Listbox that contains the visible options.
#[component]
pub fn ComboboxList(props: ComboboxListProps) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let render = use_context::<ListboxContext>().render;

    let id = use_listbox_id(props.id, ctx.selectable.list_id);

    rsx! {
        if render() {
            div {
                id,
                role: "listbox",
                ..props.attributes,
                {props.children}
            }
        } else {
            {props.children}
        }
    }
}
