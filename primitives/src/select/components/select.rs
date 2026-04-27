//! Main Select and SelectMulti component implementations.

use core::panic;
use std::time::Duration;

use crate::{select::context::RcPartialEqValue, use_controlled, use_effect};
use dioxus::prelude::*;
use dioxus_core::Task;

use super::super::context::SelectContext;
use crate::focus::use_focus_provider;

/// Props for the [`Select`] (single-select) component
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled value of the select. The select is in controlled mode whenever
    /// the inner signal yields `Some(_)`. For a controllable "no selection" state,
    /// use `Select::<Option<MyType>>` and set the value to `Some(None)`.
    #[props(default)]
    pub value: ReadSignal<Option<T>>,

    /// The initial value of the select when uncontrolled. `None` means no initial
    /// selection — the placeholder is shown until the user picks an option.
    #[props(default)]
    pub default_value: Option<T>,

    /// Callback fired when the user selects a value.
    #[props(default)]
    pub on_value_change: Callback<T>,

    /// Whether the select is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Name of the select for form submission
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Optional placeholder text
    #[props(default = ReadSignal::new(Signal::new(String::from("Select an option"))))]
    pub placeholder: ReadSignal<String>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Timeout in milliseconds before clearing typeahead buffer
    #[props(default = ReadSignal::new(Signal::new(Duration::from_millis(1000))))]
    pub typeahead_timeout: ReadSignal<Duration>,

    /// Additional attributes for the select element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the Select component
    pub children: Element,
}

/// Props for the [`SelectMulti`] (multi-select) component
#[derive(Props, Clone, PartialEq)]
pub struct SelectMultiProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled list of selected values.
    #[props(default)]
    pub values: ReadSignal<Option<Vec<T>>>,

    /// The default list of selected values.
    #[props(default)]
    pub default_values: Vec<T>,

    /// Callback when the list of selected values changes.
    #[props(default)]
    pub on_values_change: Callback<Vec<T>>,

    /// Whether the select is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Name of the select for form submission
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Optional placeholder text
    #[props(default = ReadSignal::new(Signal::new(String::from("Select an option"))))]
    pub placeholder: ReadSignal<String>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Timeout in milliseconds before clearing typeahead buffer
    #[props(default = ReadSignal::new(Signal::new(Duration::from_millis(1000))))]
    pub typeahead_timeout: ReadSignal<Duration>,

    /// Additional attributes for the select element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the SelectMulti component
    pub children: Element,
}

/// Sets up the shared signals, focus, and context that both [`Select`] and
/// [`SelectMulti`] need. Returns the `open` signal for the root `<div>`.
fn use_select_root(
    values: Memo<Vec<RcPartialEqValue>>,
    set_value: Callback<Option<RcPartialEqValue>>,
    multi: bool,
    disabled: ReadSignal<bool>,
    placeholder: ReadSignal<String>,
    roving_loop: ReadSignal<bool>,
    typeahead_timeout: ReadSignal<Duration>,
) -> Signal<bool> {
    let open = use_signal(|| false);
    let mut typeahead_buffer = use_signal(String::new);
    let options = use_signal(Vec::default);
    let adaptive_keyboard = use_signal(super::super::text_search::AdaptiveKeyboard::new);
    let list_id = use_signal(|| None);
    let mut typeahead_clear_task: Signal<Option<Task>> = use_signal(|| None);
    let focus_state = use_focus_provider(roving_loop);

    // Clear the typeahead buffer when the select is closed
    use_effect(move || {
        if !open() {
            if let Some(task) = typeahead_clear_task.write().take() {
                task.cancel();
            }
            typeahead_buffer.take();
        }
    });
    let initial_focus = use_signal(|| None);

    use_context_provider(|| SelectContext {
        typeahead_buffer,
        open,
        values,
        set_value,
        multi,
        options,
        adaptive_keyboard,
        list_id,
        focus_state,
        disabled,
        placeholder,
        typeahead_clear_task,
        typeahead_timeout,
        initial_focus,
    });

    open
}

/// # Select
///
/// The `Select` component is a searchable single-select dropdown that allows users to choose
/// one option from a list with keyboard navigation and typeahead search functionality. For
/// selecting multiple values, see [`SelectMulti`].
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
///
/// ## Styling
///
/// The [`Select`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the select. Values are `open` or `closed`.
#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let prop_value = props.value;
    let on_change = props.on_value_change;
    let mut internal_value: Signal<Option<T>> =
        use_signal(|| prop_value.cloned().or_else(|| props.default_value.clone()));
    let single_value = use_memo(move || prop_value.cloned().or_else(|| internal_value.cloned()));

    let values = use_memo(move || match single_value() {
        Some(v) => vec![RcPartialEqValue::new(v)],
        None => vec![],
    });
    let set_value = use_callback(move |cursor_opt: Option<RcPartialEqValue>| {
        let Some(value) = cursor_opt else {
            return;
        };
        let value_t = value
            .as_ref::<T>()
            .unwrap_or_else(|| panic!("The values of select and all options must match types"))
            .clone();
        internal_value.set(Some(value_t.clone()));
        on_change.call(value_t);
    });

    let open = use_select_root(
        values,
        set_value,
        false,
        props.disabled,
        props.placeholder,
        props.roving_loop,
        props.typeahead_timeout,
    );

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}

/// # SelectMulti
///
/// The `SelectMulti` component is a searchable multi-select dropdown. Selecting an option
/// toggles it in or out of the selection and the dropdown stays open across selections; it
/// closes via Escape, the trigger, or tabbing out of the listbox. For single-selection use
/// [`Select`] instead.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     SelectMulti, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         SelectMulti::<String> {
///             placeholder: "Pick toppings...",
///             default_values: vec!["pepperoni".into()],
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "16rem",
///                 SelectValue {}
///             }
///             SelectList {
///                 aria_label: "Topping Picker",
///                 SelectGroup {
///                     SelectGroupLabel { "Toppings" }
///                     SelectOption::<String> {
///                         index: 0usize,
///                         value: "pepperoni",
///                         "Pepperoni"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
///                         value: "mushroom",
///                         "Mushroom"
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
/// The [`SelectMulti`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the select. Values are `open` or `closed`.
#[component]
pub fn SelectMulti<T: Clone + PartialEq + 'static>(props: SelectMultiProps<T>) -> Element {
    let (multi_values, set_multi_internal) =
        use_controlled(props.values, props.default_values, props.on_values_change);

    let values = use_memo(move || {
        multi_values()
            .into_iter()
            .map(RcPartialEqValue::new)
            .collect()
    });
    let set_value = use_callback(move |cursor_opt: Option<RcPartialEqValue>| {
        let Some(value) = cursor_opt else {
            return;
        };
        let value_t = value
            .as_ref::<T>()
            .unwrap_or_else(|| panic!("The values of select and all options must match types"))
            .clone();
        let mut current = multi_values();
        if let Some(pos) = current.iter().position(|v| v == &value_t) {
            current.remove(pos);
        } else {
            current.push(value_t);
        }
        set_multi_internal.call(current);
    });

    let open = use_select_root(
        values,
        set_value,
        true,
        props.disabled,
        props.placeholder,
        props.roving_loop,
        props.typeahead_timeout,
    );

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}
