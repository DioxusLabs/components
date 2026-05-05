//! ComboboxList component — the listbox container.

use dioxus::prelude::*;

use super::super::context::{ComboboxContentContext, ComboboxContext};
use crate::{use_effect, use_id_or, use_unique_id};

/// Props for [`ComboboxList`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxListProps {
    /// Optional id for the list element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children — typically [`ComboboxOption`]s, [`ComboboxGroup`]s, and an
    /// optional [`ComboboxEmpty`].
    pub children: Element,
}

/// # ComboboxList
///
/// The listbox that contains the visible options. Must be used inside
/// [`ComboboxContent`](super::content::ComboboxContent).
#[component]
pub fn ComboboxList(props: ComboboxListProps) -> Element {
    let mut ctx = use_context::<ComboboxContext>();
    let render = use_context::<ComboboxContentContext>().render;

    let id = use_unique_id();
    let id = use_id_or(id, props.id);
    use_effect(move || {
        ctx.list_id.set(Some(id()));
    });

    if !render() {
        return rsx! { {props.children} };
    }

    rsx! {
        div {
            id,
            role: "listbox",
            tabindex: "-1",
            ..props.attributes,
            {props.children}
        }
    }
}
