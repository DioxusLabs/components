//! Component definitions for the select primitive.

use crate::{
    focus::{use_focus_controlled_item, use_focus_provider},
    use_animated_open, use_controlled, use_effect_cleanup, use_id_or, use_unique_id,
};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use std::fmt::Display;

use super::context::{
    OptionState, SelectContext, SelectCursor, SelectGroupContext, SelectOptionContext,
};

/// Props for the main Select component
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled value of the select
    #[props(default)]
    pub value: ReadOnlySignal<Option<Option<T>>>,

    /// The default value of the select
    #[props(default)]
    pub default_value: Option<T>,

    /// Callback when the value changes
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Callback when the display text changes
    #[props(default)]
    pub on_display_change: Callback<Option<String>>,

    /// Whether the select is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Whether the select is required
    #[props(default)]
    pub required: ReadOnlySignal<bool>,

    /// Name of the select for form submission
    #[props(default)]
    pub name: ReadOnlySignal<String>,

    /// Optional placeholder text
    #[props(default = ReadOnlySignal::new(Signal::new(String::from("Select an option"))))]
    pub placeholder: ReadOnlySignal<String>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    pub roving_loop: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// # Select
///
/// The `Select` component is a searchable dropdown that allows users to choose from a list of options with keyboard navigation and typeahead search functionality.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select::<String> {
///             placeholder: "Select a fruit...",
///             on_display_change: |_| {},
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///             }
///             SelectList {
///                 aria_label: "Select Demo",
///                 SelectGroup {
///                     SelectGroupLabel { "Fruits" }
///                     SelectOption::<String> {
///                         index: 0usize,
///                         value: "apple".to_string(),
///                         display: "Apple".to_string(), // Capitalized display text
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
///                         value: "banana".to_string(),
///                         display: "Banana".to_string(), // Capitalized display text
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
pub fn Select<T: Clone + PartialEq + Display + Default + 'static>(
    props: SelectProps<T>,
) -> Element {
    let (value, set_value_internal) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let open = use_signal(|| false);
    let mut typeahead_buffer = use_signal(String::new);
    let options = use_signal(Vec::default);
    let adaptive_keyboard = use_signal(super::text_search::AdaptiveKeyboard::new);
    let list_id = use_signal(|| None);
    let mut current_display = use_signal(|| None);

    let cursor = use_memo(move || {
        if let Some(val) = value() {
            SelectCursor {
                value: val.clone(),
                display: current_display
                    .read()
                    .clone()
                    .unwrap_or_else(|| format!("{}", val)),
            }
        } else {
            SelectCursor {
                value: T::default(),
                display: props.placeholder.cloned(),
            }
        }
    });

    let set_value = use_callback(move |cursor_opt: Option<SelectCursor<T>>| {
        if let Some(cursor) = cursor_opt {
            set_value_internal.call(Some(cursor.value.clone()));
            current_display.set(Some(cursor.display.clone()));
            props.on_display_change.call(Some(cursor.display.clone()));
        } else {
            set_value_internal.call(None);
            current_display.set(None);
            props.on_display_change.call(None);
        }
    });

    let focus_state = use_focus_provider(props.roving_loop);

    // Clear the typeahead buffer when the select is closed
    use_effect(move || {
        if !open() {
            typeahead_buffer.take();
        }
    });

    use_context_provider(|| SelectContext {
        typeahead_buffer,
        open,
        cursor,
        set_value,
        options,
        adaptive_keyboard,
        list_id,
        focus_state,
        disabled: props.disabled,
        placeholder: props.placeholder,
    });

    rsx! {
        div {
            // Data attributes
            "data-state": if open() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`SelectTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectTriggerProps {
    /// Additional attributes for the trigger button
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children to render inside the trigger
    children: Element,
}

/// # SelectTrigger
///
/// The trigger button for the [`Select`] component which controls if the [`SelectList`] is rendered.
///
/// This must be used inside a [`Select`] component.
#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    let mut ctx: SelectContext = use_context();
    let mut open = ctx.open;

    rsx! {
        button {
            // Standard HTML attributes
            disabled: (ctx.disabled)(),

            onclick: move |_| {
                open.toggle();
            },
            onkeydown: move |event| {
                match event.key() {
                    Key::ArrowUp => {
                        open.set(true);
                        ctx.focus_state.focus_last();
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    Key::ArrowDown => {
                        open.set(true);
                        ctx.focus_state.focus_first();
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    _ => {}
                }
            },

            // ARIA attributes
            aria_haspopup: "listbox",
            aria_expanded: open(),
            aria_controls: ctx.list_id,

            // Pass through other attributes
            ..props.attributes,

            // Add placeholder option if needed
            span {
                "data-placeholder": ctx.cursor.read().display == ctx.placeholder.cloned(),
                {ctx.cursor.read().display.clone()}
            }

            // Render children (options)
            {props.children}
        }
    }
}

/// The props for the [`SelectList`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectListProps {
    /// The ID of the list for ARIA attributes
    #[props(default)]
    pub id: ReadOnlySignal<Option<String>>,

    /// Additional attributes for the list
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children to render inside the list
    children: Element,
}

/// # SelectList
///
/// The dropdown list container for the [`Select`] component that contains the
/// [`SelectOption`]s. The list will only be rendered when the select is open.
///
/// This must be used inside a [`Select`] component.
#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    let mut ctx: SelectContext = use_context();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);
    use_effect(move || {
        ctx.list_id.set(Some(id()));
    });

    let mut open = ctx.open;
    let mut listbox_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let focused = move || open() && !ctx.focus_state.any_focused();

    use_effect(move || {
        let Some(listbox_ref) = listbox_ref() else {
            return;
        };
        if focused() {
            spawn(async move {
                _ = listbox_ref.set_focus(true);
            });
        }
    });

    let onkeydown = move |event: KeyboardEvent| {
        let key = event.key();
        let code = event.code();

        // Learn from keyboard events for adaptive matching
        if let Key::Character(actual_char) = &key {
            if let Some(actual_char) = actual_char.chars().next() {
                ctx.learn_from_keyboard_event(&code.to_string(), actual_char);
            }
        }

        let mut arrow_key_navigation = |event: KeyboardEvent| {
            // Clear the typeahead buffer
            ctx.typeahead_buffer.take();
            event.prevent_default();
            event.stop_propagation();
        };

        match key {
            Key::Character(new_text) => {
                if new_text == " " {
                    ctx.select_current_item();
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }

                ctx.add_to_typeahead_buffer(&new_text);
            }
            Key::ArrowUp => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_prev();
            }
            Key::End => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_last();
            }
            Key::ArrowDown => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_next();
            }
            Key::Home => {
                arrow_key_navigation(event);
                ctx.focus_state.focus_first();
            }
            Key::Enter => {
                ctx.select_current_item();
                open.set(false);
                event.prevent_default();
                event.stop_propagation();
            }
            Key::Escape => {
                open.set(false);
                event.prevent_default();
                event.stop_propagation();
            }
            _ => {}
        }
    };

    let render = use_animated_open(id, open);

    rsx! {
        if render() {
            div {
                id,
                role: "listbox",
                tabindex: if focused() { "0" } else { "-1" },

                // Data attributes
                "data-state": if open() { "open" } else { "closed" },

                onmounted: move |evt| listbox_ref.set(Some(evt.data())),
                onkeydown,
                onblur: move |_| {
                    if focused() {
                        open.set(false);
                    }
                },

                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`SelectOption`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectOptionProps<T: Display + PartialEq + Clone + 'static = String> {
    /// The programmatic value of the option. This is passed to [`SelectProps::on_value_change`] callback
    /// when selected and used internally for comparison. Should be machine-readable (e.g., "apple", "user_123").
    pub value: ReadOnlySignal<T>,

    /// The human-readable display text shown to users. If not provided, the value will be formatted
    /// and used as display text. Use this for proper capitalization (e.g., "Apple" for value "apple").
    #[props(default)]
    pub display: ReadOnlySignal<Option<String>>,

    /// Whether the option is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Optional ID for the option
    #[props(default)]
    pub id: ReadOnlySignal<Option<String>>,

    /// The index of the option in the list. This is used to define the focus order for keyboard navigation.
    pub index: ReadOnlySignal<usize>,

    /// Optional label for the option (for accessibility)
    #[props(default)]
    pub aria_label: Option<String>,

    /// Optional description role for the option (for accessibility)
    #[props(default)]
    pub aria_roledescription: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// # SelectOption
///
/// An individual selectable option within a [`SelectList`] component. Each option represents
/// a value that can be selected.
///
/// ## Value vs Display
///
/// - **`value`**: The programmatic value (e.g., `"apple"`, `"user_123"`) used internally
/// - **`display`**: The user-facing text (e.g., `"Apple"`, `"John Doe"`) shown in the UI
///
/// If `display` is not provided, the `value` will be formatted and used as the display text.
///
/// This must be used inside a [`SelectList`] component.
#[component]
pub fn SelectOption<T: Display + PartialEq + Clone + 'static>(
    props: SelectOptionProps<T>,
) -> Element {
    // Generate a unique ID for this option for accessibility
    let option_id = use_unique_id();

    // Use use_id_or to handle the ID
    let id = use_id_or(option_id, props.id);

    let index = props.index;
    let value = props.value;
    let display = use_memo(move || {
        props
            .display
            .cloned()
            .unwrap_or_else(|| format!("{}", props.value.read()))
    });

    // Push this option to the context
    let mut ctx: SelectContext<T> = use_context();
    use_effect(move || {
        let option_state = OptionState {
            tab_index: index(),
            value: value.cloned(),
            display: display.read().to_string(),
            id: id(),
        };

        // Add the option to the context's options
        ctx.options.write().push(option_state);
    });

    use_effect_cleanup(move || {
        ctx.options.write().retain(|opt| opt.id != *id.read());
    });

    let onmounted = use_focus_controlled_item(props.index);
    let focused = move || ctx.focus_state.is_focused(index());
    let disabled = ctx.disabled.cloned() || props.disabled.cloned();
    let selected = use_memo(move || ctx.cursor.read().value == *props.value.read());

    use_context_provider(|| SelectOptionContext {
        selected: selected.into(),
    });

    rsx! {
        div {
            role: "option",
            id,
            tabindex: if focused() { "0" } else { "-1" },
            onmounted,

            // ARIA attributes
            aria_selected: selected(),
            aria_disabled: disabled,
            aria_label: props.aria_label.clone(),
            aria_roledescription: props.aria_roledescription.clone(),

            onpointerdown: move |event| {
                if !disabled && event.trigger_button() == Some(MouseButton::Primary) {
                    ctx.set_value.call(Some(SelectCursor {
                        value: props.value.read().clone(),
                        display: display.read().to_string(),
                    }));
                    ctx.open.set(false);
                }
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

/// The props for the [`SelectItemIndicator`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectItemIndicatorProps {
    /// The children to render inside the indicator
    children: Element,
}

/// # SelectItemIndicator
///
/// The `SelectItemIndicator` component is used to render an indicator for a selected item within a [`SelectList`]. The
/// children will only be rendered if the option is selected.
///
/// This must be used inside a [`SelectOption`] component.
#[component]
pub fn SelectItemIndicator(props: SelectItemIndicatorProps) -> Element {
    let ctx: SelectOptionContext = use_context();
    if !(ctx.selected)() {
        return rsx! {};
    }
    rsx! {
        {props.children}
    }
}

/// The props for the [`SelectGroup`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupProps {
    /// Whether the group is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Optional ID for the group
    #[props(default)]
    pub id: ReadOnlySignal<Option<String>>,

    /// Additional attributes for the group
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children to render inside the group
    children: Element,
}

/// # SelectGroup
///
/// The `SelectGroup` component is used to group related options within a [`SelectList`]. It provides a way to organize options into logical sections.
///
/// This must be used inside a [`SelectList`] component.
#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    let ctx: SelectContext = use_context();
    let disabled = ctx.disabled.cloned() || props.disabled.cloned();

    let labeled_by = use_signal(|| None);

    use_context_provider(|| SelectGroupContext { labeled_by });

    rsx! {
        div {
            role: "group",

            // ARIA attributes
            aria_disabled: disabled,
            aria_labelledby: labeled_by,

            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`SelectGroupLabel`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupLabelProps {
    /// Optional ID for the label
    pub id: ReadOnlySignal<Option<String>>,

    /// Additional attributes for the label
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children to render inside the label
    children: Element,
}

/// # SelectGroupLabel
///
/// The `SelectGroupLabel` component is used to render a label for a group of options within a [`SelectList`].
///
/// This must be used inside a [`SelectGroup`] component.
#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let mut ctx: SelectGroupContext = use_context();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    use_effect(move || {
        ctx.labeled_by.set(Some(id()));
    });

    rsx! {
        div {
            // Set the ID for the label
            id,
            ..props.attributes,
            {props.children}
        }
    }
}
