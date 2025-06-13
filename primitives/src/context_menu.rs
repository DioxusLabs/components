use crate::{use_controlled, use_effect_cleanup};
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

impl ContextMenuCtx {
    fn set_focus(&mut self, index: Option<usize>) {
        self.current_focus.set(index);
        if let Some(index) = index {
            self.recent_focus.set(index);
        }
        if (self.open)() != index.is_some() {
            (self.set_open)(index.is_some());
        }
    }

    fn focus_next(&mut self) {
        let focus = match (self.current_focus)() {
            Some(current_focus) => (current_focus + 1) % self.item_count.cloned(),
            None => 0,
        };
        self.set_focus(Some(focus));
    }

    fn focus_prev(&mut self) {
        let item_count = self.item_count.cloned();
        let focus = match (self.current_focus)() {
            Some(current_focus) if current_focus > 0 => current_focus - 1,
            Some(_) | None => item_count.saturating_sub(1),
        };
        self.set_focus(Some(focus));
    }

    fn focus_first(&mut self) {
        self.set_focus(Some(0));
    }

    fn focus_last(&mut self) {
        let last = (self.item_count)().saturating_sub(1);
        self.set_focus(Some(last));
    }

    // Focus management helper - no actual focus restoration since we don't have NodeRef
    fn restore_trigger_focus(&mut self) {
        // In a real implementation with DOM access, we would focus the trigger element here
        // For now, we just reset the focus state
        self.set_focus(None);
    }
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

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let position = use_signal(|| (0, 0));

    let mut ctx = use_context_provider(|| ContextMenuCtx {
        open: open.into(),
        set_open,
        disabled: props.disabled,
        position,
        item_count: Signal::new(0),
        recent_focus: Signal::new(0),
        current_focus: Signal::new(None),
    });

    // Handle escape key to close the menu
    let handle_keydown = move |event: Event<KeyboardData>| {
        if open() && event.key() == Key::Escape {
            event.prevent_default();
            set_open.call(false);
            ctx.restore_trigger_focus();
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
            Key::Escape => ctx.restore_trigger_focus(),
            Key::ArrowDown => {
                ctx.focus_next();
            }
            Key::ArrowUp => {
                if open() {
                    ctx.focus_prev();
                }
            }
            Key::Home => ctx.focus_first(),
            Key::End => ctx.focus_last(),
            _ => return,
        }
        event.prevent_default();
    };

    let mut menu_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let focused = move || open() && ctx.current_focus.read().is_none();
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
            "data-state": if open() { "open" } else { "closed" },
            onkeydown,
            onblur: move |_| {
                if focused() {
                    ctx.restore_trigger_focus();
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
    let focused = move || (ctx.current_focus)() == Some((props.index)());

    // Register this item with the menu
    use_effect(move || {
        ctx.item_count += 1;
    });

    // Cleanup when the component is unmounted
    use_effect_cleanup(move || {
        ctx.item_count -= 1;
        if focused() {
            ctx.set_focus(None);
        }
    });

    let mut item_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        let Some(item) = item_ref() else {
            return;
        };
        if focused() {
            spawn(async move {
                _ = item.set_focus(true).await;
            });
        }
    });

    // Determine if this item is currently focused
    let tab_index = use_memo(move || if focused() { "0" } else { "-1" });

    let handle_click = {
        let value = (props.value)().clone();
        move |_| {
            if !disabled() {
                props.on_select.call(value.clone());
                ctx.restore_trigger_focus();
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
                    ctx.restore_trigger_focus();
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
                    ctx.set_focus(None);
                }
            },
            onmounted: move |evt| item_ref.set(Some(evt.data())),
            aria_disabled: disabled(),
            "data-disabled": disabled(),
            ..props.attributes,

            {props.children}
        }
    }
}
