//! Main Combobox component.

use dioxus::prelude::*;

use super::super::context::{
    default_combobox_filter, match_score, ComboboxContext, OptionState, RcPartialEqValue,
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
/// `Combobox` is an autocomplete input with a popover list of filterable
/// options. Following WAI-ARIA 1.2's combobox pattern, a single
/// [`ComboboxInput`](super::input::ComboboxInput) is the trigger and the
/// search field — the keyboard-and-typeahead cousin of
/// [`Select`](crate::select::Select).
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
    let mut query = use_signal(String::new);
    let options: Signal<Vec<OptionState>> = use_signal(Vec::default);
    let list_id = use_signal(|| None);

    let value = use_memo(move || value().map(RcPartialEqValue::new));
    let set_value = use_callback(move |cursor_opt: Option<RcPartialEqValue>| {
        if let Some(value) = cursor_opt {
            let v = value
                .as_ref::<T>()
                .expect("combobox and option value types must match")
                .clone();
            set_value_internal.call(Some(v));
        } else {
            set_value_internal.call(None);
        }
    });

    let focus_state = use_focus_provider(props.roving_loop);

    use_effect(move || {
        if !open() {
            query.set(String::new());
        }
    });

    let filter = props.filter;
    let visible: Memo<Vec<(usize, Callback<(), Element>)>> = use_memo(move || {
        let options = options.read();
        let query_str = query.read().clone();
        let q_trim = query_str.trim().to_string();

        let mut v: Vec<(Option<u32>, usize, Callback<(), Element>)> = options
            .iter()
            .filter(|o| !o.disabled && filter.call((query_str.clone(), o.text_value.clone())))
            .map(|o| {
                let score = if q_trim.is_empty() {
                    None
                } else {
                    match_score(&q_trim, &o.text_value)
                };
                (score, o.tab_index, o.render)
            })
            .collect();

        v.sort_by(|(s1, t1, _), (s2, t2, _)| match (s1, s2) {
            (Some(a), Some(b)) => a.cmp(b).then_with(|| t1.cmp(t2)),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => t1.cmp(t2),
        });

        v.into_iter().map(|(_, t, r)| (t, r)).collect()
    });

    use_context_provider(|| ComboboxContext {
        open,
        query,
        value,
        set_value,
        options,
        list_id,
        focus_state,
        disabled: props.disabled,
        visible,
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
