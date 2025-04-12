use crate::use_controlled;
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct ContextMenuCtx {
    // State
    open: ReadOnlySignal<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,

    // Position of the context menu
    position: Signal<(i32, i32)>,

    // Keyboard nav data
    item_count: Signal<usize>,
    recent_focus: Signal<usize>,
    current_focus: Signal<Option<usize>>,
}

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuProps {
    /// Whether the context menu is disabled
    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    disabled: ReadOnlySignal<bool>,

    /// Whether the context menu is open
    open: Option<Signal<bool>>,

    /// Default open state
    #[props(default)]
    default_open: bool,

    /// Callback when open state changes
    #[props(default)]
    on_open_change: Callback<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let position = use_signal(|| (0, 0));

    let ctx = use_context_provider(|| ContextMenuCtx {
        open: open.into(),
        set_open,
        disabled: props.disabled,
        position,
        item_count: Signal::new(0),
        recent_focus: Signal::new(0),
        current_focus: Signal::new(None),
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
            event.prevent_default();
            ctx.position.set((
                event.data().client_coordinates().x as i32,
                event.data().client_coordinates().y as i32,
            ));
            ctx.set_open.call(true);
        }
    };

    rsx! {
        div { oncontextmenu: handle_context_menu, ..props.attributes, {props.children} }
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
    let ctx: ContextMenuCtx = use_context();
    let position = ctx.position;

    let style = use_memo(move || {
        let (x, y) = position();
        format!("position: fixed; left: {}px; top: {}px;", x, y)
    });

    // Close menu when clicking outside
    let handle_window_click = move |event: Event<MouseData>| {
        let coords = event.data().client_coordinates();
        let x = coords.x as i32;
        let y = coords.y as i32;
        let (menu_x, menu_y) = position();

        // Check if click is outside the menu
        if x < menu_x || y < menu_y {
            ctx.set_open.call(false);
        }
    };

    rsx! {
        div {
            style: "{style}",
            "data-state": if (ctx.open)() { "open" } else { "closed" },
            hidden: !(ctx.open)(),
            onclick: handle_window_click,
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuItemProps {
    /// The value of the menu item
    value: String,

    /// The index of the item in the menu
    index: usize,

    /// Callback when the item is selected
    #[props(default)]
    on_select: Callback<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    let ctx: ContextMenuCtx = use_context();

    let handle_click = move |_| {
        if !(ctx.disabled)() {
            props.on_select.call(props.value.clone());
            ctx.set_open.call(false);
        }
    };

    rsx! {
        div {
            role: "menuitem",
            tabindex: "0",
            onclick: handle_click,
            ..props.attributes,

            {props.children}
        }
    }
}
