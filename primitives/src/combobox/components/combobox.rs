//! Root combobox component.

use dioxus::prelude::*;

use super::super::context::{
    default_combobox_filter, ComboboxContext, OptionState, RcPartialEqValue,
};
use crate::focus::use_focus_provider;
use crate::use_controlled;

/// Props for [`Combobox`].
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

    /// Whether arrow-key navigation should wrap.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Custom filter callback. Receives `(query, option_text_value)`.
    #[props(default = Callback::new(|(q, t): (String, String)| default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

fn use_combobox_root(
    value: Memo<Option<RcPartialEqValue>>,
    set_value: Callback<Option<RcPartialEqValue>>,
    disabled: ReadSignal<bool>,
    roving_loop: ReadSignal<bool>,
    filter: Callback<(String, String), bool>,
) -> Signal<bool> {
    let open = use_signal(|| false);
    let query = use_signal(String::new);
    let options: Signal<Vec<OptionState>> = use_signal(Vec::default);
    let list_id = use_signal(|| None);
    let focus_state = use_focus_provider(roving_loop);

    use_context_provider(|| ComboboxContext {
        open,
        query,
        value,
        set_value,
        options,
        list_id,
        focus_state,
        disabled,
        filter,
    });

    open
}

/// A single-select autocomplete input with a filterable popup list.
#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element {
    let (value, set_value_internal) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let selected = use_memo(move || value().map(RcPartialEqValue::new));
    let set_value = use_callback(move |incoming: Option<RcPartialEqValue>| {
        let value = incoming.map(|value| {
            value
                .as_ref::<T>()
                .expect("combobox and option value types must match")
                .clone()
        });
        set_value_internal.call(value);
    });

    let open = use_combobox_root(
        selected,
        set_value,
        props.disabled,
        props.roving_loop,
        props.filter,
    );

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}
