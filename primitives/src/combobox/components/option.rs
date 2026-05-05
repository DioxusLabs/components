//! ComboboxOption and ComboboxItemIndicator components.

use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use super::super::context::{
    ComboboxContext, ComboboxOptionContext, OptionState, RcPartialEqValue,
};
use crate::{use_effect, use_effect_cleanup, use_id_or, use_unique_id};

/// Props for [`ComboboxOption`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxOptionProps<T: Clone + PartialEq + 'static> {
    /// The value carried by this option.
    pub value: ReadSignal<T>,

    /// Display/searchable text. Required for non-string types.
    #[props(default)]
    pub text_value: ReadSignal<Option<String>>,

    /// Whether the option is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Optional id for the option element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Registration order — used for stable keyboard navigation order.
    pub index: ReadSignal<usize>,

    /// Optional aria-label.
    #[props(default)]
    pub aria_label: Option<String>,

    /// Optional aria-roledescription.
    #[props(default)]
    pub aria_roledescription: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children rendered inside the option.
    pub children: Element,
}

/// # ComboboxOption
///
/// An individual selectable option inside a [`ComboboxList`]. Registers itself
/// with the parent combobox; the actual `<div role="option">` is emitted by
/// [`ComboboxList`](super::list::ComboboxList) in relevance-ranked order so
/// DOM order matches what sighted users see.
///
/// ## Value vs text_value
///
/// - `value` — the programmatic value passed to `on_value_change`.
/// - `text_value` — the user-visible string used for filtering and the trigger
///   value display. Required for non-string `T`.
#[component]
pub fn ComboboxOption<T: PartialEq + Clone + 'static>(props: ComboboxOptionProps<T>) -> Element {
    let option_id = use_unique_id();
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
                        "ComboboxOption with non-string types requires text_value to be set"
                    );
                    String::new()
                })
        }
    });

    let mut ctx: ComboboxContext = use_context();
    let disabled_signal = props.disabled;

    let focused = move || ctx.focus_state.is_focused(index());
    let disabled = move || ctx.disabled.cloned() || props.disabled.cloned();
    let selected = use_memo(move || {
        ctx.value.read().as_ref().and_then(|v| v.as_ref::<T>()) == Some(&*props.value.read())
    });

    let mut did_drag = use_signal(|| false);

    let aria_label = props.aria_label.clone();
    let aria_roledescription = props.aria_roledescription.clone();
    let attrs = props.attributes;
    let children_element = props.children;
    let value_signal = props.value;

    // Stable Callback handle; `use_callback` swaps the inner closure on every
    // render so it always captures the freshest props. `ComboboxList` calls
    // this in relevance-ranked order to render the option in the DOM.
    let render = use_callback(move |_: ()| -> Element {
        let attrs = attrs.clone();
        let children = children_element.clone();
        let aria_label = aria_label.clone();
        let aria_roledescription = aria_roledescription.clone();

        rsx! {
            ProvideOptionContext { selected,
                div {
                    role: "option",
                    id,
                    tabindex: "-1",

                    aria_selected: selected(),
                    aria_disabled: disabled(),
                    aria_label,
                    aria_roledescription,

                    "data-highlighted": focused(),
                    "data-disabled": disabled(),
                    "data-selected": selected(),

                    onmouseenter: move |_| {
                        if !disabled() {
                            ctx.focus_state.set_focus(Some(index()));
                        }
                    },
                    onpointerdown: move |event: PointerEvent| {
                        if !disabled()
                            && &event.pointer_type() == "mouse"
                            && event.trigger_button() == Some(MouseButton::Primary)
                        {
                            ctx.set_value.call(Some(RcPartialEqValue::new(value_signal.cloned())));
                            ctx.open.set(false);
                            ctx.query.set(String::new());
                            event.prevent_default();
                        }
                    },
                    ontouchstart: move |_| {
                        did_drag.set(false);
                    },
                    ontouchend: move |_| {
                        if !disabled() && !did_drag() {
                            ctx.set_value.call(Some(RcPartialEqValue::new(value_signal.cloned())));
                            ctx.open.set(false);
                            ctx.query.set(String::new());
                        }
                    },
                    ontouchmove: move |_| {
                        did_drag.set(true);
                    },

                    ..attrs,
                    {children}
                }
            }
        }
    });

    use_effect(move || {
        let option_state = OptionState {
            tab_index: index(),
            value: RcPartialEqValue::new(value.cloned()),
            text_value: text_value.cloned(),
            id: id(),
            disabled: disabled_signal.cloned(),
            render,
        };
        ctx.options.write().push(option_state);
    });

    use_effect_cleanup(move || {
        ctx.options.write().retain(|opt| opt.id != *id.read());
    });

    // No visible markup — the list emits us via `render` in ranked order.
    rsx! {}
}

/// Tiny wrapper that provides [`ComboboxOptionContext`] to its children so
/// [`ComboboxItemIndicator`] can read the option's selected state. Used
/// internally by the option's render callback. Takes `Memo<bool>` (concrete
/// type) to avoid the `SuperInto` ambiguity introduced by `dioxus-stores`.
#[component]
fn ProvideOptionContext(selected: Memo<bool>, children: Element) -> Element {
    use_context_provider(|| ComboboxOptionContext {
        selected: selected.into(),
    });
    rsx! { {children} }
}

/// Props for [`ComboboxItemIndicator`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxItemIndicatorProps {
    /// Children rendered only when the parent option is selected.
    pub children: Element,
}

/// # ComboboxItemIndicator
///
/// Visual indicator that's only rendered when the parent
/// [`ComboboxOption`] is the currently selected value.
#[component]
pub fn ComboboxItemIndicator(props: ComboboxItemIndicatorProps) -> Element {
    let ctx: ComboboxOptionContext = use_context();
    if !(ctx.selected)() {
        return rsx! {};
    }
    rsx! { {props.children} }
}
