//! SelectOption and SelectItemIndicator component implementations.

use crate::{
    focus::use_focus_controlled_item_disabled,
    listbox::{use_listbox_option, ListboxContext, ListboxOptionContext},
    select::context::RcPartialEqValue,
};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use super::super::context::SelectContext;

/// The props for the [`SelectOption`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectOptionProps<T: Clone + PartialEq + 'static> {
    /// The value of the option
    pub value: ReadSignal<T>,

    /// The text value of the option used for typeahead search
    #[props(default)]
    pub text_value: ReadSignal<Option<String>>,

    /// Whether the option is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Optional ID for the option
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// The index of the option in the list. This is used to define the focus order for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Optional label for the option (for accessibility)
    #[props(default)]
    pub aria_label: Option<String>,

    /// Optional description role for the option (for accessibility)
    #[props(default)]
    pub aria_roledescription: Option<String>,

    /// Additional attributes for the option element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the option
    pub children: Element,
}

/// # SelectOption
///
/// An individual selectable option within a [`SelectList`](super::list::SelectList) component. Each option represents
/// a value that can be selected.
///
/// ## Value vs Text Value
///
/// - **`value`**: The programmatic value (e.g., `"apple"`, `"user_123"`) used internally
/// - **`text_value`**: The text value (e.g., `"Apple"`, `"John Doe"`) used for typeahead search and displayed in the [`SelectValue`](super::value::SelectValue)
///
/// This must be used inside a [`SelectList`](super::list::SelectList) component.
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
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///                 SelectValue { placeholder: "Select a fruit..." }
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
#[component]
pub fn SelectOption<T: PartialEq + Clone + 'static>(props: SelectOptionProps<T>) -> Element {
    let index = props.index;
    let value = props.value;

    let mut ctx: SelectContext = use_context();
    let disabled = {
        let select_disabled = ctx.disabled;
        let option_disabled = props.disabled;
        move || select_disabled.cloned() || option_disabled.cloned()
    };
    let id = use_listbox_option(
        props.id,
        index,
        value,
        props.text_value,
        ctx.options,
        disabled,
        "SelectOption",
    );

    let onmounted = use_focus_controlled_item_disabled(props.index, disabled);
    let focused = move || ctx.focus_state.is_focused(index());
    let selected = use_memo(move || {
        let value = props.value.read();
        ctx.values
            .read()
            .iter()
            .any(|v| v.as_ref::<T>() == Some(&*value))
    });
    let mut down_pos: Signal<Option<(f64, f64)>> = use_signal(|| None);

    use_context_provider(|| ListboxOptionContext {
        selected: selected.into(),
    });

    let render = use_context::<ListboxContext>().render;

    rsx! {
        if render() {
            div {
                role: "option",
                id,
                tabindex: if focused() { "0" } else { "-1" },
                onmounted,

                aria_selected: selected(),
                aria_disabled: disabled(),
                aria_label: props.aria_label.clone(),
                aria_roledescription: props.aria_roledescription.clone(),
                "data-disabled": disabled(),

                onpointerdown: move |event| {
                    if disabled() || event.trigger_button() != Some(MouseButton::Primary) {
                        return;
                    }
                    // Suppress the synthesized focus shift and click event so the listbox
                    // keeps DOM focus (its onblur would otherwise close us mid-tap). We
                    // commit the selection ourselves on pointerup.
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
                    // Drag-cancel only matters for touch; mouse clicks shouldn't be
                    // suppressed by tiny cursor drift between down and up. ~5px
                    // threshold tolerates small touch wobble.
                    if event.pointer_type() == "touch" {
                        let p = event.client_coordinates();
                        let dx = p.x - x0;
                        let dy = p.y - y0;
                        if dx * dx + dy * dy > 25.0 {
                            return;
                        }
                    }
                    ctx.set_value.call(Some(RcPartialEqValue::new(props.value.cloned())));
                    if !ctx.multi {
                        ctx.open.set(false);
                    }
                },
                onpointercancel: move |_| {
                    down_pos.set(None);
                },
                onblur: move |_| {
                    if focused() {
                        ctx.focus_state.blur();
                        ctx.open.set(false);
                    }
                },

                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`SelectItemIndicator`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectItemIndicatorProps {
    /// The children to render inside the indicator
    pub children: Element,
}

/// # SelectItemIndicator
///
/// The `SelectItemIndicator` component is used to render an indicator for a selected item within a [`SelectList`](super::list::SelectList). The
/// children will only be rendered if the option is selected.
///
/// This must be used inside a [`SelectOption`](SelectOption) component.
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
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///                 SelectValue { placeholder: "Select a fruit..." }
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
#[component]
pub fn SelectItemIndicator(props: SelectItemIndicatorProps) -> Element {
    let ctx: ListboxOptionContext = use_context();
    if !(ctx.selected)() {
        return rsx! {};
    }
    rsx! {
        {props.children}
    }
}
