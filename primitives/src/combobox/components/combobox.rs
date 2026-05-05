//! Main Combobox component.

use core::panic;

use dioxus::prelude::*;

use super::super::context::{
    default_combobox_filter, ComboboxContext, RcPartialEqValue,
};
use crate::focus::use_focus_provider;
use crate::{use_controlled, use_effect};

/// Props for the [`Combobox`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled value.
    #[props(default)]
    pub value: ReadSignal<Option<Option<T>>>,

    /// The default uncontrolled value.
    #[props(default)]
    pub default_value: Option<T>,

    /// Callback fired when the value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Whether the combobox is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Form name.
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Whether arrow-key navigation should wrap.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Custom filter callback. Receives `(query, option_text_value)` and returns
    /// `true` if the option should be visible. Defaults to a case-insensitive
    /// substring match.
    #[props(default = Callback::new(|(q, t): (String, String)| default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

/// # Combobox
///
/// `Combobox` is a button + popover combination that lets the user pick a
/// value from a filterable list of options. It's the keyboard-and-typeahead
/// cousin of [`Select`](crate::select::Select).
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::combobox::{
///     Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput,
///     ComboboxItemIndicator, ComboboxList, ComboboxOption,
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Combobox::<String> {
///             ComboboxInput { placeholder: "Select a framework..." }
///             ComboboxContent {
///                 ComboboxList {
///                     ComboboxEmpty { "No framework found." }
///                     ComboboxOption::<String> { index: 0usize, value: "next",
///                         "Next.js"
///                         ComboboxItemIndicator { "✔" }
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
/// The root element exposes:
/// - `data-state`: `open` / `closed`
/// - `data-disabled`: `true` if disabled
#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element {
    let (value, set_value_internal) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let open = use_signal(|| false);
    let query = use_signal(String::new);
    let options = use_signal(Vec::default);
    let list_id = use_signal(|| None);

    let value = use_memo(move || value().map(RcPartialEqValue::new));
    let set_value = use_callback(move |cursor_opt: Option<RcPartialEqValue>| {
        if let Some(value) = cursor_opt {
            set_value_internal.call(Some(
                value
                    .as_ref::<T>()
                    .unwrap_or_else(|| {
                        panic!("The values of combobox and all options must match types")
                    })
                    .clone(),
            ));
        } else {
            set_value_internal.call(None);
        }
    });

    let focus_state = use_focus_provider(props.roving_loop);

    let mut query = query;
    use_effect(move || {
        if !open() {
            query.set(String::new());
        }
    });

    use_context_provider(|| ComboboxContext {
        open,
        query,
        value,
        set_value,
        options,
        filter: props.filter,
        list_id,
        focus_state,
        disabled: props.disabled,
    });

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}
