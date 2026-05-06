//! ComboboxList component.

use dioxus::prelude::*;

use super::super::context::ComboboxContext;
use crate::listbox::{use_listbox_id, use_listbox_render, ListboxContext};

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
    let open = ctx.selectable.open;
    let id = use_listbox_id(props.id, ctx.selectable.list_id);
    let render = use_listbox_render(id, open);

    use_context_provider(|| ListboxContext {
        render: render.into(),
    });

    rsx! {
        if render() {
            div {
                id,
                role: "listbox",
                "data-state": if open() { "open" } else { "closed" },
                onpointerdown: move |event| {
                    event.prevent_default();
                },
                ..props.attributes,
                {props.children}
            }
        } else {
            {props.children}
        }
    }
}
