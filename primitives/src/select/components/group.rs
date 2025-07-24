//! SelectGroup and SelectGroupLabel component implementations.

use crate::{use_effect, use_unique_id, use_id_or};
use dioxus::prelude::*;

use super::super::context::{SelectContext, SelectGroupContext};

/// The props for the [`SelectGroup`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupProps {
    /// Whether the group is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Optional ID for the group
    #[props(default)]
    pub id: ReadOnlySignal<Option<String>>,

    /// Additional attributes for the group
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children to render inside the group
    children: Element,
}

/// # SelectGroup
///
/// The `SelectGroup` component is used to group related options within a [`SelectList`]. It provides a way to organize options into logical sections.
///
/// This must be used inside a [`SelectList`] component.
#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    let ctx: SelectContext = use_context();
    let disabled = ctx.disabled.cloned() || props.disabled.cloned();

    let labeled_by = use_signal(|| None);

    use_context_provider(|| SelectGroupContext { labeled_by });

    rsx! {
        div {
            role: "group",

            // ARIA attributes
            aria_disabled: disabled,
            aria_labelledby: labeled_by,

            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`SelectGroupLabel`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupLabelProps {
    /// Optional ID for the label
    pub id: ReadOnlySignal<Option<String>>,

    /// Additional attributes for the label
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children to render inside the label
    children: Element,
}

/// # SelectGroupLabel
///
/// The `SelectGroupLabel` component is used to render a label for a group of options within a [`SelectList`].
///
/// This must be used inside a [`SelectGroup`] component.
#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let mut ctx: SelectGroupContext = use_context();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    use_effect(move || {
        ctx.labeled_by.set(Some(id()));
    });

    rsx! {
        div {
            // Set the ID for the label
            id,
            ..props.attributes,
            {props.children}
        }
    }
}