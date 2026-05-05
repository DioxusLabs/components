//! Main Combobox and ComboboxMulti components.

use dioxus::prelude::*;

use super::super::context::{
    default_combobox_filter, match_score, ComboboxContext, OptionState, RcPartialEqValue,
    VisibleOptionState,
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

/// Props for the [`ComboboxMulti`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxMultiProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled list of selected values.
    #[props(default)]
    pub values: ReadSignal<Option<Vec<T>>>,

    /// The default list of selected values when uncontrolled.
    #[props(default)]
    pub default_values: Vec<T>,

    /// Callback fired when the list of selected values changes.
    #[props(default)]
    pub on_values_change: Callback<Vec<T>>,

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

/// Sets up the shared signals, focus state, visible-options memo, and context
/// that both [`Combobox`] and [`ComboboxMulti`] need. Returns the `open` signal
/// so the root component can drive its `data-state` attribute.
fn use_combobox_root(
    values: Memo<Vec<RcPartialEqValue>>,
    set_value: Callback<Option<RcPartialEqValue>>,
    multi: bool,
    disabled: ReadSignal<bool>,
    roving_loop: ReadSignal<bool>,
    filter: Callback<(String, String), bool>,
) -> Signal<bool> {
    let open = use_signal(|| false);
    let mut query = use_signal(String::new);
    let options: Signal<Vec<OptionState>> = use_signal(Vec::default);
    let list_id = use_signal(|| None);
    let focus_state = use_focus_provider(roving_loop);

    use_effect(move || {
        if !open() {
            query.set(String::new());
        }
    });

    let visible: Memo<Vec<VisibleOptionState>> = use_memo(move || {
        let options = options.read();
        let query_str = query.read().clone();
        let q_trim = query_str.trim().to_string();

        let mut v: Vec<_> = options
            .iter()
            .filter(|o| filter.call((query_str.clone(), o.text_value.clone())))
            .map(|o| {
                let score = if q_trim.is_empty() {
                    None
                } else {
                    match_score(&q_trim, &o.text_value)
                };
                (score, o.tab_index, o.disabled, o.group_id.clone(), o.render)
            })
            .collect();

        v.sort_by(|(s1, t1, ..), (s2, t2, ..)| match (s1, s2) {
            (Some(a), Some(b)) => a.cmp(b).then_with(|| t1.cmp(t2)),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => t1.cmp(t2),
        });

        v.into_iter()
            .map(
                |(_, tab_index, disabled, group_id, render)| VisibleOptionState {
                    tab_index,
                    disabled,
                    group_id,
                    render,
                },
            )
            .collect()
    });

    use_effect(move || {
        let Some(focused) = focus_state.current_focus() else {
            return;
        };
        if !visible
            .read()
            .iter()
            .any(|option| option.tab_index == focused && !option.disabled)
        {
            let mut focus_state = focus_state;
            focus_state.set_focus(None);
        }
    });

    use_context_provider(|| ComboboxContext {
        open,
        query,
        values,
        set_value,
        multi,
        options,
        list_id,
        focus_state,
        disabled,
        visible,
    });

    open
}

/// # Combobox
///
/// `Combobox` is an autocomplete input with a popover list of filterable
/// options. Following WAI-ARIA 1.2's combobox pattern, a single
/// [`ComboboxInput`](super::input::ComboboxInput) is the trigger and the
/// search field — the keyboard-and-typeahead cousin of
/// [`Select`](crate::select::Select). For picking more than one item, see
/// [`ComboboxMulti`].
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

    let values = use_memo(move || value().map(RcPartialEqValue::new).into_iter().collect());
    let set_value = use_callback(move |incoming: Option<RcPartialEqValue>| {
        if let Some(value) = incoming {
            let v = value
                .as_ref::<T>()
                .expect("combobox and option value types must match")
                .clone();
            set_value_internal.call(Some(v));
        } else {
            set_value_internal.call(None);
        }
    });

    let open = use_combobox_root(
        values,
        set_value,
        false,
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

/// # ComboboxMulti
///
/// `ComboboxMulti` is the multi-select cousin of [`Combobox`]: clicking or
/// pressing Enter on an option toggles it in or out of the selection while the
/// popup stays open, so the user can keep filtering and picking. The popup
/// closes via Escape, blur, or Tab. The input shows the currently selected
/// options' text values comma-joined when closed, and the user's query when
/// open. The listbox advertises `aria-multiselectable="true"`.
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::combobox::{
///     ComboboxMulti, ComboboxContent, ComboboxEmpty, ComboboxInput,
///     ComboboxItemIndicator, ComboboxList, ComboboxOption,
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ComboboxMulti::<String> {
///             default_values: vec!["next".into()],
///             ComboboxInput { placeholder: "Pick frameworks..." }
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
pub fn ComboboxMulti<T: Clone + PartialEq + 'static>(props: ComboboxMultiProps<T>) -> Element {
    let (multi_values, set_multi_internal) =
        use_controlled(props.values, props.default_values, props.on_values_change);

    let values = use_memo(move || {
        multi_values()
            .into_iter()
            .map(RcPartialEqValue::new)
            .collect()
    });
    let set_value = use_callback(move |incoming: Option<RcPartialEqValue>| {
        let Some(value) = incoming else {
            return;
        };
        let value_t = value
            .as_ref::<T>()
            .expect("combobox and option value types must match")
            .clone();
        let mut current = multi_values();
        if let Some(pos) = current.iter().position(|v| v == &value_t) {
            current.remove(pos);
        } else {
            current.push(value_t);
        }
        set_multi_internal.call(current);
    });

    let open = use_combobox_root(
        values,
        set_value,
        true,
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
