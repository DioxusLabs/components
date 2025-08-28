//! SelectOption and SelectItemIndicator component implementations.

use crate::{
    focus::use_focus_controlled_item,
    select::context::{RcPartialEqValue, SelectListContext},
    use_effect, use_effect_cleanup, use_id_or, use_unique_id,
};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use super::super::context::{OptionState, SelectContext, SelectOptionContext};

/// The props for the [`SelectOption`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectOptionProps<T: Clone + PartialEq + 'static> {
    /// The value of the option
    pub value: ReadOnlySignal<T>,

    /// The text value of the option used for typeahead search
    #[props(default)]
    pub text_value: ReadOnlySignal<Option<String>>,

    /// Whether the option is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Optional ID for the option
    #[props(default)]
    pub id: ReadOnlySignal<Option<String>>,

    /// The index of the option in the list. This is used to define the focus order for keyboard navigation.
    pub index: ReadOnlySignal<usize>,

    /// Optional label for the option (for accessibility)
    #[props(default)]
    pub aria_label: Option<String>,

    /// Optional description role for the option (for accessibility)
    #[props(default)]
    pub aria_roledescription: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// # SelectOption
///
/// An individual selectable option within a [`SelectList`](super::list::SelectList) component. Each option represents
/// a value that can be selected.
///
/// ## Value vs Text Value
///
/// - **`value`**: The programmatic value (e.g., `"apple"`, `"user_123"`) used internally
/// - **`text_value`**: The text value (e.g., `"Apple"`, `"John Doe"`) used for typeahead search and displayed in the [`SelectValue`](super::value::SelectValue)
///
/// This must be used inside a [`SelectList`](super::list::SelectList) component.
///
/// ## Example
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
///                         index: 0usize,
///                         value: "apple",
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
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
#[component]
pub fn SelectOption<T: PartialEq + Clone + 'static>(props: SelectOptionProps<T>) -> Element {
    // Generate a unique ID for this option for accessibility
    let option_id = use_unique_id();

    // Use use_id_or to handle the ID
    let id = use_id_or(option_id, props.id);

    let index = props.index;
    let value = props.value;
    let text_value = use_memo(move || match (props.text_value)() {
        Some(text) => text,
        None => {
            let value = value.read();
            let as_any: &dyn std::any::Any = &*value;
            as_any
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| as_any.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| {
                    tracing::warn!(
                        "SelectOption with non-string types requires text_value to be set"
                    );
                    String::new()
                })
        }
    });

    // Push this option to the context
    let mut ctx: SelectContext = use_context();
    use_effect(move || {
        let option_state = OptionState {
            tab_index: index(),
            value: RcPartialEqValue::new(value.cloned()),
            text_value: text_value.cloned(),
            id: id(),
        };

        // Add the option to the context's options
        ctx.options.write().push(option_state);
    });

    use_effect_cleanup(move || {
        ctx.options.write().retain(|opt| opt.id != *id.read());
    });

    let onmounted = use_focus_controlled_item(props.index);
    let focused = move || ctx.focus_state.is_focused(index());
    let disabled = ctx.disabled.cloned() || props.disabled.cloned();
    let selected = use_memo(move || {
        ctx.value.read().as_ref().and_then(|v| v.as_ref::<T>()) == Some(&props.value.read())
    });

    use_context_provider(|| SelectOptionContext {
        selected: selected.into(),
    });

    let render = use_context::<SelectListContext>().render;

    rsx! {
        if render() {
            div {
                role: "option",
                id,
                tabindex: if focused() { "0" } else { "-1" },
                onmounted,

                // ARIA attributes
                aria_selected: selected(),
                aria_disabled: disabled,
                aria_label: props.aria_label.clone(),
                aria_roledescription: props.aria_roledescription.clone(),

                onpointerdown: move |event| {
                    if !disabled && event.trigger_button() == Some(MouseButton::Primary) {
                        ctx.set_value.call(Some(RcPartialEqValue::new(props.value.cloned())));
                        ctx.open.set(false);
                    }
                },
                onblur: move |_| {
                    if focused() {
                        ctx.focus_state.blur();
                        ctx.open.set(false);
                    }
                },

                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`SelectItemIndicator`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectItemIndicatorProps {
    /// The children to render inside the indicator
    children: Element,
}

/// # SelectItemIndicator
///
/// The `SelectItemIndicator` component is used to render an indicator for a selected item within a [`SelectList`](super::list::SelectList). The
/// children will only be rendered if the option is selected.
///
/// This must be used inside a [`SelectOption`](SelectOption) component.
///
/// ## Example
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
///                         index: 0usize,
///                         value: "apple",
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
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
#[component]
pub fn SelectItemIndicator(props: SelectItemIndicatorProps) -> Element {
    let ctx: SelectOptionContext = use_context();
    if !(ctx.selected)() {
        return rsx! {};
    }
    rsx! {
        {props.children}
    }
}
