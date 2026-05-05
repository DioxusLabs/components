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
///
/// `ComboboxOption` no longer renders its own DOM. Instead, options register
/// themselves with the parent combobox and this list emits each visible root
/// option's `<div role="option">` in relevance-ranked order. Grouped options
/// are emitted by their [`ComboboxGroup`](super::group::ComboboxGroup) so they
/// remain inside their `role="group"` containers.
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
        // Mount children so options can still register themselves with the
        // context (the trigger uses that to look up the selected label).
        return rsx! { {props.children} };
    }

    let visible_renders = ctx.root_visible_renders();

    rsx! {
        div {
            id,
            role: "listbox",
            tabindex: "-1",
            aria_multiselectable: ctx.multi,
            ..props.attributes,
            {props.children}
            for render in visible_renders {
                {render.call(())}
            }
        }
    }
}
