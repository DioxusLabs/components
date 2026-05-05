//! ComboboxEmpty — placeholder shown when no options match the query.

use dioxus::prelude::*;

use super::super::context::{ComboboxContentContext, ComboboxContext};

/// Props for [`ComboboxEmpty`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxEmptyProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children rendered when no options match.
    pub children: Element,
}

/// # ComboboxEmpty
///
/// Renders its children only when the current query produces no visible
/// options. Place it inside a [`ComboboxList`](super::list::ComboboxList).
#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let render = use_context::<ComboboxContentContext>().render;

    let any_visible = use_memo(move || ctx.has_visible_options());

    if !render() || any_visible() {
        return rsx! {};
    }

    rsx! {
        div {
            role: "presentation",
            ..props.attributes,
            {props.children}
        }
    }
}
