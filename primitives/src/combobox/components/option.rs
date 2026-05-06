//! Combobox option components.

use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use super::super::context::{ComboboxContext, RcPartialEqValue};
use crate::{
    focus::use_focus_entry_disabled,
    listbox::{use_listbox_option, ListboxContext, ListboxOptionContext},
};

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

    /// Registration order used for keyboard navigation.
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

/// A selectable option inside a [`ComboboxList`](super::list::ComboboxList).
#[component]
pub fn ComboboxOption<T: PartialEq + Clone + 'static>(props: ComboboxOptionProps<T>) -> Element {
    let index = props.index;
    let value = props.value;

    let mut ctx: ComboboxContext = use_context();
    let disabled = move || ctx.disabled.cloned() || props.disabled.cloned();
    let visible = move || ctx.is_visible(index());
    let selected = use_memo(move || ctx.is_selected(&RcPartialEqValue::new(props.value.cloned())));
    let id = use_listbox_option(
        props.id,
        index,
        value,
        props.text_value,
        ctx.options,
        disabled,
        "ComboboxOption",
    );

    use_focus_entry_disabled(ctx.focus_state, props.index, move || {
        disabled() || !visible()
    });

    let render = use_context::<ListboxContext>().render;
    let focused = move || ctx.focus_state.is_focused(index());
    let mut down_pos: Signal<Option<(f64, f64)>> = use_signal(|| None);

    use_context_provider(|| ListboxOptionContext {
        selected: selected.into(),
    });

    rsx! {
        if render() && visible() {
            div {
                role: "option",
                id,
                tabindex: "-1",

                aria_selected: selected(),
                aria_disabled: disabled(),
                aria_label: props.aria_label.clone(),
                aria_roledescription: props.aria_roledescription.clone(),

                "data-highlighted": focused(),
                "data-disabled": disabled(),
                "data-selected": selected(),

                onmouseenter: move |_| {
                    if !disabled() {
                        ctx.focus_state.set_focus(Some(index()));
                    }
                },
                onpointerdown: move |event| {
                    if disabled() || event.trigger_button() != Some(MouseButton::Primary) {
                        return;
                    }
                    // Keep focus on the combobox input while deferring selection until
                    // pointerup, so touch scroll gestures can be cancelled.
                    event.prevent_default();
                    let p = event.client_coordinates();
                    down_pos.set(Some((p.x, p.y)));
                },
                onpointerup: move |event| {
                    if disabled() || event.trigger_button() != Some(MouseButton::Primary) {
                        return;
                    }
                    let Some((x0, y0)) = down_pos.take() else {
                        return;
                    };
                    if event.pointer_type() == "touch" {
                        let p = event.client_coordinates();
                        let dx = p.x - x0;
                        let dy = p.y - y0;
                        if dx * dx + dy * dy > 25.0 {
                            return;
                        }
                    }
                    ctx.select_value(RcPartialEqValue::new(value.cloned()));
                },
                onpointercancel: move |_| {
                    down_pos.set(None);
                },

                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// Props for [`ComboboxItemIndicator`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxItemIndicatorProps {
    /// Children rendered only when the parent option is selected.
    pub children: Element,
}

/// Renders its children when the parent option is selected.
#[component]
pub fn ComboboxItemIndicator(props: ComboboxItemIndicatorProps) -> Element {
    let ctx: ListboxOptionContext = use_context();
    if !(ctx.selected)() {
        return rsx! {};
    }
    rsx! { {props.children} }
}
