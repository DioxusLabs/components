//! Combobox popup component.

use dioxus::prelude::*;

use super::super::context::{ComboboxContentContext, ComboboxContext};
use crate::{use_animated_open, use_id_or, use_unique_id};

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
    let open = ctx.open;

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    let render = use_animated_open(id, open);
    let render = use_memo(render);

    use_context_provider(|| ComboboxContentContext {
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
