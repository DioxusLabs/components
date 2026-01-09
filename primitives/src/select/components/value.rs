//! SelectValue component implementation.

use dioxus::prelude::*;

use super::super::context::SelectContext;

/// The props for the [`SelectValue`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectValueProps {
    /// Additional attributes for the value element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # SelectValue
///
/// The trigger button for the [`Select`](super::select::Select) component which controls if the [`SelectList`](super::list::SelectList) is rendered.
///
/// This must be used inside a [`Select`](super::select::Select) component.
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select::<String> {
///             placeholder: "Select a fruit...",
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///                 SelectValue {}
///             }
///             SelectList {
///                 aria_label: "Select Demo",
///                 SelectGroup {
///                     SelectGroupLabel { "Fruits" }
///                     SelectOption::<String> {
///                         tab_index: 0usize,
///                         value: "apple",
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         tab_index: 1usize,
///                         value: "banana",
///                         "Banana"
///                         SelectItemIndicator { "✔️" }
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
///
/// ## Styling
///
/// The [`SelectValue`] component defines a span with a `data-placeholder` attribute if a placeholder is set.
#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    let ctx = use_context::<SelectContext>();

    let selected_text_value = use_memo(move || {
        let value = ctx.value.read();
        value.as_ref().and_then(|v| {
            ctx.options
                .read()
                .values()
                .find(|state| state.value == *v)
                .map(|state| state.text_value.clone())
        })
    });

    let display_value = selected_text_value().unwrap_or_else(|| ctx.placeholder.cloned());

    rsx! {
        // Add placeholder option if needed
        span { "data-placeholder": ctx.value.read().is_none(), ..props.attributes, {display_value} }
    }
}
