use crate::{use_controlled, use_effect_cleanup};
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct DropdownMenuContext {
    // State
    open: ReadOnlySignal<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,

    // Keyboard nav data
    item_count: Signal<usize>,
    recent_focus: Signal<usize>,
    current_focus: Signal<Option<usize>>,
}

impl DropdownMenuContext {
    fn set_focus(&mut self, id: Option<usize>) {
        self.current_focus.set(id);
        if let Some(id) = id {
            self.recent_focus.set(id);
        }
        if (self.open)() != id.is_some() {
            (self.set_open)(id.is_some());
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
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
    open: ReadOnlySignal<Option<bool>>,

    #[props(default)]
    default_open: bool,

    #[props(default)]
    on_open_change: Callback<bool>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let disabled = props.disabled;
    let mut ctx = use_context_provider(|| DropdownMenuContext {
        open: open.into(),
        set_open,
        disabled,
        item_count: Signal::new(0),
        recent_focus: Signal::new(0),
        current_focus: Signal::new(None),
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

    rsx! {
        div {
            role: "menu",
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            onkeydown: handle_keydown,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuTriggerProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    let mut ctx: DropdownMenuContext = use_context();

    rsx! {
        button {
            r#type: "button",
            "data-state": if (ctx.open)() { "open" } else { "closed" },
            "data-disabled": (ctx.disabled)(),
            disabled: (ctx.disabled)(),
            aria_expanded: ctx.open,
            aria_haspopup: "menu",

            onclick: move |_| {
                let new_open = !(ctx.open)();
                ctx.set_open.call(new_open);
            },
            onblur: move |_| {
                if ctx.current_focus.read().is_none() {
                    ctx.set_focus(None);
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuContentProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let ctx: DropdownMenuContext = use_context();

    rsx! {
        div {
            role: "menu",
            "data-state": if (ctx.open)() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuItemProps {
    value: ReadOnlySignal<String>,
    index: ReadOnlySignal<usize>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    on_select: Callback<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn DropdownMenuItem(props: DropdownMenuItemProps) -> Element {
    let mut ctx: DropdownMenuContext = use_context();

    use_effect(move || {
        ctx.item_count += 1;
    });
    use_effect_cleanup(move || {
        ctx.item_count -= 1;
        if (ctx.current_focus)() == Some((props.index)()) {
            ctx.current_focus.set(None);
        }
    });

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let focused = move || (ctx.current_focus)() == Some((props.index)());

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

    rsx! {
        div {
            role: "menuitem",
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

            onmounted: move |node| {
                item_ref.set(Some(node.data()));
            },

            onblur: move |_| {
                if focused() {
                    ctx.set_focus(None);
                }
            },


            ..props.attributes,
            {props.children}
        }
    }
}
