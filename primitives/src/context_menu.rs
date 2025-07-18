//! Defines the [`ContextMenu`] component and its subcomponents, which provide a context menu interface.

use crate::{
    focus::{use_focus_controlled_item, use_focus_provider, FocusState},
    use_animated_open, use_controlled, use_effect_cleanup, use_id_or, use_unique_id,
};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct ContextMenuCtx {
    // State
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,

    // Position of the context menu
    position: Signal<(i32, i32)>,

    // Focus state
    focus: FocusState,
}

/// The props for the [`ContextMenu`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuProps {
    /// Whether the context menu is disabled
    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    pub disabled: ReadOnlySignal<bool>,

    /// Whether the context menu is open
    pub open: ReadOnlySignal<Option<bool>>,

    /// Default open state
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    pub roving_loop: ReadOnlySignal<bool>,

    /// Additional attributes for the context menu element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The children of the context menu component.
    children: Element,
}

/// # ContextMenu
///
/// The [`ContextMenu`] component is a container that can be used to create a context menu. You can
/// use the [`ContextMenuTrigger`] to open the menu on a right-click, and the [`ContextMenuContent`] to define the menu item.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::context_menu::{
///     ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ContextMenu {
///             ContextMenuTrigger {
///                 "right click here"
///             }
///             ContextMenuContent {
///                 ContextMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Edit"
///                 }
///                 ContextMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
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
/// The [`ContextMenu`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the state of the context menu. Values are `open` or `closed`.
/// - `data-disabled`: Indicates if the context menu is disabled. values are `true` or `false`.
#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let position = use_signal(|| (0, 0));

    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| ContextMenuCtx {
        open,
        set_open,
        disabled: props.disabled,
        position,
        focus,
    });

    use_effect(move || {
        let focused = focus.any_focused();
        if *ctx.open.peek() != focused {
            (ctx.set_open)(focused);
        }
    });

    // If the context menu is open, prevent pointer and scroll events outside of it
    let pointer_events_disabled = |disabled| {
        if disabled {
            dioxus::document::eval(
                "document.body.style.pointerEvents = 'none'; document.documentElement.style.overflow = 'hidden';",
            );
        } else {
            dioxus::document::eval(
                "document.body.style.pointerEvents = 'auto'; document.documentElement.style.overflow = 'auto';",
            );
        }
    };
    use_effect(move || {
        pointer_events_disabled(ctx.open.cloned());
    });
    use_effect_cleanup(move || {
        // If the context menu was open, reset pointer events
        if ctx.open.cloned() {
            pointer_events_disabled(false);
        }
    });

    // Handle escape key to close the menu
    let handle_keydown = move |event: Event<KeyboardData>| {
        if open() && event.key() == Key::Escape {
            event.prevent_default();
            set_open.call(false);
            ctx.focus.blur();
        }
    };

    rsx! {
        div {
            tabindex: 0, // Make the menu container focusable
            onkeydown: handle_keydown,
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ContextMenuTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuTriggerProps {
    /// Additional attributes for the context menu trigger element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The children of the context menu trigger.
    children: Element,
}

/// # ContextMenuTrigger
///
/// The [`ContextMenuTrigger`] component is used to define the element that will trigger the context menu when right-clicked.
///
/// This must be used inside a [`ContextMenu`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::context_menu::{
///     ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ContextMenu {
///             ContextMenuTrigger {
///                 "right click here"
///             }
///             ContextMenuContent {
///                 ContextMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Edit"
///                 }
///                 ContextMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element {
    let mut ctx: ContextMenuCtx = use_context();

    let handle_context_menu = move |event: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.position.set((
                event.data().client_coordinates().x as i32,
                event.data().client_coordinates().y as i32,
            ));
            ctx.set_open.call(true);
            event.prevent_default();
        }
    };

    rsx! {
        div {
            oncontextmenu: handle_context_menu,
            role: "button",
            aria_haspopup: "menu",
            aria_expanded: (ctx.open)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ContextMenuContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuContentProps {
    /// The ID of the context menu content element.
    id: ReadOnlySignal<Option<String>>,

    /// Additional attributes for the context menu content element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The children of the context menu content.
    children: Element,
}

/// # ContextMenuContent
///
/// The [`ContextMenuContent`] component is used to define the content of the context menu. It is only rendered
/// when the context menu is open.
///
/// This must be used inside a [`ContextMenu`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::context_menu::{
///     ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ContextMenu {
///             ContextMenuTrigger {
///                 "right click here"
///             }
///             ContextMenuContent {
///                 ContextMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Edit"
///                 }
///                 ContextMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
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
/// The [`ContextMenuContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the state of the context menu. Values are `open` or `closed`.
#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    let mut ctx: ContextMenuCtx = use_context();
    let position = ctx.position;
    let (x, y) = position();

    let open = ctx.open;

    let onkeydown = move |event: Event<KeyboardData>| {
        match event.key() {
            Key::Escape => ctx.focus.blur(),
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

    let mut menu_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let focused = move || open() && !ctx.focus.any_focused();
    // If the menu is open, but no item is focused, focus the div itself to capture events
    use_effect(move || {
        let Some(menu) = menu_ref() else {
            return;
        };
        if focused() {
            spawn(async move {
                // Focus the menu itself to capture keyboard events
                _ = menu.set_focus(true).await;
            });
        }
    });

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);

    let render = use_animated_open(id, open);

    rsx! {
        if render() {
            div {
                id,
                role: "menu",
                aria_orientation: "vertical",
                position: "fixed",
                left: "{x}px",
                top: "{y}px",
                tabindex: if focused() { "0" } else { "-1" },
                pointer_events: open().then_some("auto"),
                "data-state": if open() { "open" } else { "closed" },
                onkeydown,
                onblur: move |_| {
                    if focused() {
                        ctx.focus.blur();
                    }
                },
                onmounted: move |evt| menu_ref.set(Some(evt.data())),
                ..props.attributes,

                {props.children}
            }
        }
    }
}

/// The props for the [`ContextMenuItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuItemProps {
    /// Whether the item is disabled
    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    pub disabled: ReadOnlySignal<bool>,

    /// The value of the menu item
    pub value: ReadOnlySignal<String>,

    /// The index of the item in the menu
    pub index: ReadOnlySignal<usize>,

    /// Callback when the item is selected
    #[props(default)]
    pub on_select: Callback<String>,

    /// Additional attributes for the context menu item element
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The children of the context menu item
    children: Element,
}

/// # ContextMenuItem
///
/// The [`ContextMenuItem`] component defines an individual item in the context menu. You must define an index that
/// controls the order items are focused when navigating the menu with the keyboard.
///
/// When an item is selected with either the pointer or the keyboard, the menu is closed and the `on_select` callback is called with the item's value.
///
/// This must be used inside a [`ContextMenuContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::context_menu::{
///     ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ContextMenu {
///             ContextMenuTrigger {
///                 "right click here"
///             }
///             ContextMenuContent {
///                 ContextMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Edit"
///                 }
///                 ContextMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
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
/// The [`ContextMenuItem`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the item is disabled. Possible values are `true` or `false`.
#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    let mut ctx: ContextMenuCtx = use_context();

    let disabled = use_memo(move || (props.disabled)() || (ctx.disabled)());
    let focused = move || ctx.focus.is_focused(props.index.cloned());

    // Handle settings focus
    let onmounted = use_focus_controlled_item(props.index);

    // Determine if this item is currently focused
    let tab_index = use_memo(move || if focused() { "0" } else { "-1" });

    let handle_click = {
        let value = (props.value)().clone();
        move |event: Event<PointerData>| {
            if !disabled() {
                props.on_select.call(value.clone());
                ctx.focus.blur();
                event.prevent_default();
                event.stop_propagation();
            }
        }
    };

    let handle_keydown = {
        let value = (props.value)().clone();
        move |event: Event<KeyboardData>| {
            // Check for Enter or Space key
            if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
                if !disabled() {
                    props.on_select.call(value.clone());
                    ctx.focus.blur();
                }
                event.prevent_default();
                event.stop_propagation();
            }
        }
    };

    rsx! {
        div {
            role: "menuitem",
            tabindex: tab_index,
            onpointerdown: handle_click,
            onkeydown: handle_keydown,
            onblur: move |_| {
                if focused() {
                    ctx.focus.blur();
                }
            },
            onmounted,
            aria_disabled: disabled(),
            "data-disabled": disabled(),
            ..props.attributes,

            {props.children}
        }
    }
}
