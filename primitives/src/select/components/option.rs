//! SelectOption and SelectItemIndicator component implementations.

use crate::{
    focus::use_focus_controlled_item, use_effect, use_effect_cleanup, use_id_or, use_unique_id,
};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use super::super::context::{OptionState, SelectContext, SelectOptionContext, SelectValue};

/// The props for the [`SelectOption`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectOptionProps<T: Clone + PartialEq + 'static> {
    /// The value of the option
    pub value: ReadOnlySignal<SelectValue<T>>,

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
/// - **`text_value`**: The user-facing text (e.g., `"Apple"`, `"John Doe"`) shown in the UI
///
/// This must be used inside a [`SelectList`](super::list::SelectList) component.
#[component]
pub fn SelectOption<T: PartialEq + Clone + 'static>(props: SelectOptionProps<T>) -> Element {
    // Generate a unique ID for this option for accessibility
    let option_id = use_unique_id();

    // Use use_id_or to handle the ID
    let id = use_id_or(option_id, props.id);

    let index = props.index;
    let value = props.value;
    let text_value = use_memo(move || (props.value)().text_value);

    // Push this option to the context
    let mut ctx: SelectContext<T> = use_context();
    use_effect(move || {
        let option_state = OptionState {
            tab_index: index(),
            value: value.cloned(),
            text_value: text_value.read().to_string(),
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
    let selected = use_memo(move || ctx.cursor.read().clone() == Some(props.value.read().clone()));

    use_context_provider(|| SelectOptionContext {
        selected: selected.into(),
    });

    rsx! {
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
                    ctx.set_value.call(Some(props.value.read().clone()));
                    ctx.open.set(false);
                }
            },
            // Note: We intentionally don't handle blur events on individual options.
            // The blur handler on the list container (SelectList) manages closing the dropdown.
            // Having blur handlers on options causes issues with keyboard navigation where
            // moving between options would incorrectly close the dropdown.

            ..props.attributes,
            {props.children}
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
