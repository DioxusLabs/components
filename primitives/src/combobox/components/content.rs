//! ComboboxContent component (the popup container).

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

/// # ComboboxContent
///
/// The popup container that wraps the [`ComboboxList`](super::list::ComboboxList).
/// Renders only when the combobox is open and animates open/closed via the
/// `data-state` attribute. `ComboboxInput` lives outside this — it's the
/// persistent trigger that owns keyboard focus.
///
/// Must be used inside a [`Combobox`](super::combobox::Combobox).
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
                    // Keep focus on the search input during clicks inside the popup
                    // so option clicks register before the input blurs.
                    event.prevent_default();
                },
                ..props.attributes,
                {props.children}
            }
        } else {
            // Still render children when closed so options/groups can register
            // themselves with the context, but they won't render markup.
            {props.children}
        }
    }
}
