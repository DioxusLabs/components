//! Root combobox component.

use dioxus::prelude::*;

use super::super::context::{default_combobox_filter, ComboboxContext, RcPartialEqValue};
use crate::selectable::{use_selectable_root, SelectionMode};

/// Props for [`Combobox`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled value. If supplied, the combobox is controlled
    /// and the signal's `None` value means no option is selected.
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    /// The default uncontrolled value.
    #[props(default)]
    pub default_value: Option<T>,

    /// Callback fired when the value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Whether the combobox is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The controlled open state of the popup.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// The initial open state when uncontrolled.
    #[props(default)]
    pub default_open: ReadSignal<bool>,

    /// Callback fired when the popup open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

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
    values: Memo<Vec<RcPartialEqValue>>,
    set_value: Callback<Option<RcPartialEqValue>>,
    disabled: ReadSignal<bool>,
    roving_loop: ReadSignal<bool>,
    open: ReadSignal<Option<bool>>,
    default_open: ReadSignal<bool>,
    on_open_change: Callback<bool>,
    filter: Callback<(String, String), bool>,
) -> Memo<bool> {
    let selectable = use_selectable_root(
        values,
        set_value,
        SelectionMode::Single,
        disabled,
        roving_loop,
        open,
        default_open.cloned(),
        on_open_change,
    );
    let query = use_signal(String::new);
    let open = selectable.open;

    use_context_provider(|| ComboboxContext {
        selectable,
        query,
        filter,
    });

    open
}

/// A single-select autocomplete input with a filterable popup list.
#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element {
    let controlled_value = props.value;
    let on_change = props.on_value_change;
    let mut internal_value: Signal<Option<T>> = use_signal(|| props.default_value.clone());
    let value = use_memo(move || match controlled_value {
        Some(value) => value.cloned(),
        None => internal_value.cloned(),
    });

    let selected = use_memo(move || value().map(RcPartialEqValue::new).into_iter().collect());
    let set_value = use_callback(move |incoming: Option<RcPartialEqValue>| {
        let value = incoming.map(|value| {
            value
                .as_ref::<T>()
                .expect("combobox and option value types must match")
                .clone()
        });
        internal_value.set(value.clone());
        on_change.call(value);
    });

    let open = use_combobox_root(
        selected,
        set_value,
        props.disabled,
        props.roving_loop,
        props.open,
        props.default_open,
        props.on_open_change,
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
