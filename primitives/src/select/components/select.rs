//! Main Select component implementation.

use crate::{
    use_controlled, use_effect,
};
use dioxus::prelude::*;
use std::fmt::Display;

use super::super::context::{
    SelectContext, SelectCursor,
};
use crate::focus::use_focus_provider;

/// Props for the main Select component
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled value of the select
    #[props(default)]
    pub value: ReadOnlySignal<Option<Option<T>>>,

    /// The default value of the select
    #[props(default)]
    pub default_value: Option<T>,

    /// Callback when the value changes
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Callback when the display text changes
    #[props(default)]
    pub on_display_change: Callback<Option<String>>,

    /// Whether the select is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Whether the select is required
    #[props(default)]
    pub required: ReadOnlySignal<bool>,

    /// Name of the select for form submission
    #[props(default)]
    pub name: ReadOnlySignal<String>,

    /// Optional placeholder text
    #[props(default = ReadOnlySignal::new(Signal::new(String::from("Select an option"))))]
    pub placeholder: ReadOnlySignal<String>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    pub roving_loop: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// # Select
///
/// The `Select` component is a searchable dropdown that allows users to choose from a list of options with keyboard navigation and typeahead search functionality.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select::<String> {
///             placeholder: "Select a fruit...",
///             on_display_change: |_| {},
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///             }
///             SelectList {
///                 aria_label: "Select Demo",
///                 SelectGroup {
///                     SelectGroupLabel { "Fruits" }
///                     SelectOption::<String> {
///                         index: 0usize,
///                         value: "apple".to_string(),
///                         display: "Apple".to_string(), // Capitalized display text
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
///                         value: "banana".to_string(),
///                         display: "Banana".to_string(), // Capitalized display text
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
/// ## Styling
///
/// The [`Select`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the select. Values are `open` or `closed`.
#[component]
pub fn Select<T: Clone + PartialEq + Display + Default + 'static>(
    props: SelectProps<T>,
) -> Element {
    let (value, set_value_internal) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let open = use_signal(|| false);
    let mut typeahead_buffer = use_signal(String::new);
    let options = use_signal(Vec::default);
    let adaptive_keyboard = use_signal(super::super::text_search::AdaptiveKeyboard::new);
    let list_id = use_signal(|| None);
    let mut current_display = use_signal(|| None);

    let cursor = use_memo(move || {
        if let Some(val) = value() {
            SelectCursor {
                value: val.clone(),
                display: current_display
                    .read()
                    .clone()
                    .unwrap_or_else(|| format!("{}", val)),
            }
        } else {
            SelectCursor {
                value: T::default(),
                display: props.placeholder.cloned(),
            }
        }
    });

    let set_value = use_callback(move |cursor_opt: Option<SelectCursor<T>>| {
        if let Some(cursor) = cursor_opt {
            set_value_internal.call(Some(cursor.value.clone()));
            current_display.set(Some(cursor.display.clone()));
            props.on_display_change.call(Some(cursor.display.clone()));
        } else {
            set_value_internal.call(None);
            current_display.set(None);
            props.on_display_change.call(None);
        }
    });

    let focus_state = use_focus_provider(props.roving_loop);

    // Clear the typeahead buffer when the select is closed
    use_effect(move || {
        if !open() {
            typeahead_buffer.take();
        }
    });

    use_context_provider(|| SelectContext {
        typeahead_buffer,
        open,
        cursor,
        set_value,
        options,
        adaptive_keyboard,
        list_id,
        focus_state,
        disabled: props.disabled,
        placeholder: props.placeholder,
    });

    rsx! {
        div {
            // Data attributes
            "data-state": if open() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}