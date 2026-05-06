//! Combobox option components.

use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use super::super::context::{
    ComboboxContentContext, ComboboxContext, ComboboxOptionContext, OptionState, RcPartialEqValue,
};
use crate::{
    focus::use_focus_entry_disabled,
    selection::{option_text_value, remove_option, sync_option},
    use_effect, use_effect_cleanup, use_id_or, use_unique_id,
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
    let option_id = use_unique_id();
    let id = use_id_or(option_id, props.id);

    let index = props.index;
    let value = props.value;
    let text_value =
        use_memo(move || option_text_value(&*value.read(), (props.text_value)(), "ComboboxOption"));

    let mut ctx: ComboboxContext = use_context();
    let disabled = move || ctx.disabled.cloned() || props.disabled.cloned();
    let visible = move || ctx.is_visible(index());
    let selected = use_memo(move || ctx.is_selected(&RcPartialEqValue::new(props.value.cloned())));

    use_effect(move || {
        let option_id = id();
        let option_state = OptionState {
            tab_index: index(),
            value: RcPartialEqValue::new(value.cloned()),
            text_value: text_value.cloned(),
            id: option_id.clone(),
            disabled: disabled(),
        };
        sync_option(ctx.options, option_state);
    });

    use_effect_cleanup(move || {
        remove_option(ctx.options, id.read().as_str());
    });

    use_focus_entry_disabled(ctx.focus_state, props.index, move || {
        disabled() || !visible()
    });

    let render = use_context::<ComboboxContentContext>().render;
    let focused = move || ctx.focus_state.is_focused(index());

    use_context_provider(|| ComboboxOptionContext {
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
                onpointerdown: move |event: PointerEvent| {
                    if !disabled() && event.trigger_button() == Some(MouseButton::Primary) {
                        ctx.select_value(RcPartialEqValue::new(value.cloned()));
                        event.prevent_default();
                    }
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
    let ctx: ComboboxOptionContext = use_context();
    if !(ctx.selected)() {
        return rsx! {};
    }
    rsx! { {props.children} }
}
