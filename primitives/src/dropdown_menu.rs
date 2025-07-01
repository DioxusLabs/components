//! Defines the [`DropdownMenu`] component and its subcomponents.

use crate::{
    focus::{use_focus_controlled_item, use_focus_provider, FocusState},
    use_animated_open, use_controlled, use_id_or, use_unique_id,
};
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct DropdownMenuContext {
    // State
    open: ReadOnlySignal<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,

    // Focus state
    focus: FocusState,

    // Unique ID for the trigger button
    trigger_id: Signal<String>,
}

/// The props for the [`DropdownMenu`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
    /// Whether the dropdown menu is open. If not provided, the component will be uncontrolled and use `default_open`.
    open: ReadOnlySignal<Option<bool>>,

    /// Default open state if the component is not controlled.
    #[props(default)]
    default_open: bool,

    /// Callback when the open state changes. This is called when the dropdown menu is opened or closed.
    #[props(default)]
    on_open_change: Callback<bool>,

    /// Whether the dropdown menu is disabled. If true, the menu will not open and items will not be selectable.
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    roving_loop: ReadOnlySignal<bool>,

    /// Additional attributes to apply to the dropdown menu element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the dropdown menu, which should include a [`DropdownMenuTrigger`] and a [`DropdownMenuContent`].
    children: Element,
}

/// # DropdownMenu
///
/// The `DropdownMenu` component is a container for a [`DropdownMenuContent`] component activated by a [`DropdownMenuTrigger`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         DropdownMenu { default_open: false,
///             DropdownMenuTrigger { "Open Menu" }
///             DropdownMenuContent {
///                 DropdownMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DropdownMenu`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the dropdown menu. values are `open` or `closed`.
/// - `data-disabled`: Indicates if the dropdown menu is disabled. values are `true` or `false`.
#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let disabled = props.disabled;
    let trigger_id = use_unique_id();
    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| DropdownMenuContext {
        open: open.into(),
        set_open,
        disabled,
        focus,
        trigger_id,
    });

    use_effect(move || {
        let focused = focus.any_focused();
        if *ctx.open.peek() != focused {
            (ctx.set_open)(focused);
        }
    });

    // Handle escape key to close the menu
    let handle_keydown = move |event: Event<KeyboardData>| {
        if disabled() {
            return;
        }
        match event.key() {
            Key::Enter => {
                let new_open = !(ctx.open)();
                ctx.set_open.call(new_open);
            }
            Key::Escape => ctx.set_open.call(false),
            Key::ArrowDown => {
                ctx.focus.focus_next();
            }
            Key::ArrowUp => {
                if open() {
                    ctx.focus.focus_prev();
                }
            }
            Key::Home => ctx.focus.focus_first(),
            Key::End => ctx.focus.focus_last(),
            _ => return,
        }
        event.prevent_default();
    };

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            onkeydown: handle_keydown,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuTriggerProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

/// # DropdownMenuTrigger
///
/// The trigger button for the parent [`DropdownMenu`]. This button toggles the visibility of the [`DropdownMenuContent`].
///
/// This must be used inside a [`DropdownMenu`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         DropdownMenu { default_open: false,
///             DropdownMenuTrigger { "Open Menu" }
///             DropdownMenuContent {
///                 DropdownMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DropdownMenuTrigger`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the dropdown menu. values are `open` or `closed`.
/// - `data-disabled`: Indicates if the dropdown menu is disabled. values are `true` or `false`.
#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    let mut ctx: DropdownMenuContext = use_context();

    rsx! {
        button {
            id: "{ctx.trigger_id}",
            r#type: "button",
            "data-state": if (ctx.open)() { "open" } else { "closed" },
            "data-disabled": (ctx.disabled)(),
            disabled: (ctx.disabled)(),
            aria_expanded: ctx.open,
            aria_haspopup: "listbox",

            onclick: move |_| {
                let new_open = !(ctx.open)();
                ctx.set_open.call(new_open);
            },
            onblur: move |_| {
                if !ctx.focus.any_focused() {
                    ctx.focus.blur();
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuContentProps {
    id: ReadOnlySignal<Option<String>>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

/// # DropdownMenuTrigger
///
/// The contents of a [`DropdownMenu`]. The component will only be rendered when the parent [`DropdownMenu`] is open (as control by the [`DropdownMenuTrigger`]).
///
/// This must be used inside a [`DropdownMenu`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         DropdownMenu { default_open: false,
///             DropdownMenuTrigger { "Open Menu" }
///             DropdownMenuContent {
///                 DropdownMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DropdownMenuContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the dropdown menu. values are `open` or `closed`.
#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let ctx: DropdownMenuContext = use_context();

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);
    let render = use_animated_open(id, ctx.open);

    rsx! {
        if render() {
            div {
                id,
                role: "listbox",
                aria_labelledby: "{ctx.trigger_id}",
                "data-state": if (ctx.open)() { "open" } else { "closed" },
                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`DropdownMenuItem`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuItemProps {
    /// The value of the item, which will be passed to the `on_select` callback when clicked.
    pub value: ReadOnlySignal<String>,
    /// The index of the item within the [`DropdownMenuContent`]. This is used to order the items for keyboard navigation.
    pub index: ReadOnlySignal<usize>,

    /// Whether the item is disabled. If true, the item will not be clickable and will not respond to keyboard events.
    /// Defaults to false.
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// The callback function that will be called when the item is selected. The value of the item will be passed as an argument.
    #[props(default)]
    pub on_select: Callback<String>,

    /// Additional attributes to apply to the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the item, which will be rendered inside the item element.
    pub children: Element,
}

/// # DropdownMenuTrigger
///
/// An item within a [`DropdownMenuContent`]. This component represents an individual selectable item in the dropdown menu.
///
/// This must be used inside a [`DropdownMenu`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         DropdownMenu { default_open: false,
///             DropdownMenuTrigger { "Open Menu" }
///             DropdownMenuContent {
///                 DropdownMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DropdownMenuItem`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates whether the item is disabled. Values are `true` or `false`.
#[component]
pub fn DropdownMenuItem(props: DropdownMenuItemProps) -> Element {
    let mut ctx: DropdownMenuContext = use_context();

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let focused = move || ctx.focus.is_focused((props.index)());

    let onmounted = use_focus_controlled_item(props.index);

    rsx! {
        div {
            role: "option",
            "data-disabled": disabled(),
            tabindex: if focused() { "0" } else { "-1" },

            onclick: move |e: Event<MouseData>| {
                e.stop_propagation();
                if !disabled() {
                    props.on_select.call((props.value)());
                    ctx.set_open.call(false);
                }
            },

            onkeydown: move |event: Event<KeyboardData>| {
                if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
                    if !disabled() {
                        props.on_select.call((props.value)());
                        ctx.set_open.call(false);
                    }
                    event.prevent_default();
                    event.stop_propagation();
                }
            },

            onmounted,

            onblur: move |_| {
                if focused() {
                    ctx.focus.blur();
                }
            },


            ..props.attributes,
            {props.children}
        }
    }
}
