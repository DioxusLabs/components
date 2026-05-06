//! Combobox popup component.

use dioxus::prelude::*;

use super::super::context::ComboboxContext;
use crate::{
    listbox::{use_listbox_render, ListboxContext},
    use_id_or, use_unique_id,
};

/// Props for [`ComboboxContent`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxContentProps {
    /// Optional id for the content wrapper.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the popup.
    pub children: Element,
}

/// Popup container for the combobox list.
#[component]
pub fn ComboboxContent(props: ComboboxContentProps) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let open = ctx.selectable.open;

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    let render = use_listbox_render(id, open);

    use_context_provider(|| ListboxContext {
        render: render.into(),
    });

    rsx! {
        if render() {
            div {
                id,
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
