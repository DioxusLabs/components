//! ComboboxGroup and ComboboxGroupLabel components.

use dioxus::prelude::*;

use super::super::context::{ComboboxContentContext, ComboboxContext, ComboboxGroupContext};
use crate::{use_effect, use_id_or, use_unique_id};

/// Props for [`ComboboxGroup`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxGroupProps {
    /// Whether the entire group is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Optional id for the group element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children — typically a [`ComboboxGroupLabel`] and [`ComboboxOption`]s.
    pub children: Element,
}

/// # ComboboxGroup
///
/// A semantic grouping of related options inside a [`ComboboxList`]. The
/// group is hidden when none of its options match the current query.
#[component]
pub fn ComboboxGroup(props: ComboboxGroupProps) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let disabled = ctx.disabled.cloned() || props.disabled.cloned();

    let labeled_by = use_signal(|| None);
    use_context_provider(|| ComboboxGroupContext { labeled_by });

    let render = use_context::<ComboboxContentContext>().render;

    rsx! {
        if render() {
            div {
                role: "group",
                aria_disabled: disabled,
                aria_labelledby: labeled_by,
                ..props.attributes,
                {props.children}
            }
        } else {
            {props.children}
        }
    }
}

/// Props for [`ComboboxGroupLabel`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxGroupLabelProps {
    /// Optional id for the label.
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children rendered inside the label.
    pub children: Element,
}

/// # ComboboxGroupLabel
///
/// Label for a [`ComboboxGroup`].
#[component]
pub fn ComboboxGroupLabel(props: ComboboxGroupLabelProps) -> Element {
    let mut ctx: ComboboxGroupContext = use_context();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    use_effect(move || {
        ctx.labeled_by.set(Some(id()));
    });

    let render = use_context::<ComboboxContentContext>().render;

    rsx! {
        if render() {
            div {
                id,
                ..props.attributes,
                {props.children}
            }
        }
    }
}
