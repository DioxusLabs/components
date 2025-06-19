use crate::{
    focus::{FocusState, use_focus_controlled_item, use_focus_provider},
    use_controlled,
};
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct ContextMenuCtx {
    // State
    open: ReadOnlySignal<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,

    // Position of the context menu
    position: Signal<(i32, i32)>,

    // Focus state
    focus: FocusState,
}

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuProps {
    /// Whether the context menu is disabled
    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    disabled: ReadOnlySignal<bool>,

    /// Whether the context menu is open
    open: ReadOnlySignal<Option<bool>>,

    /// Default open state
    #[props(default)]
    default_open: bool,

    /// Callback when open state changes
    #[props(default)]
    on_open_change: Callback<bool>,

    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    roving_loop: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let position = use_signal(|| (0, 0));

    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| ContextMenuCtx {
        open: open.into(),
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
    use_effect(move || {
        if ctx.open.cloned() {
            dioxus::document::eval("document.body.style.pointerEvents = 'none'; document.documentElement.style.overflow = 'hidden';");
        } else {
            dioxus::document::eval("document.body.style.pointerEvents = 'auto'; document.documentElement.style.overflow = 'auto';");
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

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuTriggerProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

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

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuContentProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

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

    rsx! {
        div {
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

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuItemProps {
    /// Whether the item is disabled
    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    disabled: ReadOnlySignal<bool>,

    /// The value of the menu item
    value: ReadOnlySignal<String>,

    /// The index of the item in the menu
    index: ReadOnlySignal<usize>,

    /// Callback when the item is selected
    #[props(default)]
    on_select: Callback<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

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
        move |_| {
            if !disabled() {
                props.on_select.call(value.clone());
                ctx.focus.blur();
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
