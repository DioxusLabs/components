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
/// themselves with the parent combobox and this list emits each visible
/// option's `<div role="option">` in relevance-ranked order. That way the DOM
/// order matches what sighted users see, so screen-reader exploration mode
/// (NVDA browse, JAWS list, VoiceOver rotor) walks the same sequence.
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

    // Resolve each visible option's render callback. Cloning Callbacks is
    // cheap (they wrap a CopyValue).
    let visible_renders: Vec<Callback<(), Element>> = {
        let visible = ctx.visible_indices();
        let options = ctx.options.read();
        visible
            .into_iter()
            .filter_map(|tab_index| {
                options
                    .iter()
                    .find(|opt| opt.tab_index == tab_index)
                    .map(|opt| opt.render)
            })
            .collect()
    };

    rsx! {
        div {
            id,
            role: "listbox",
            tabindex: "-1",
            ..props.attributes,
            // ComboboxEmpty (and any other non-option children, e.g. group
            // labels) live in the children tree and self-gate visibility.
            // ComboboxOption itself emits no markup, so the children tree
            // contributes only ancillary content.
            {props.children}
            for render in visible_renders {
                {render.call(())}
            }
        }
    }
}
