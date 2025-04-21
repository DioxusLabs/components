use crate::use_controlled;
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
    }

    fn focus_next(&mut self) {
        if let Some(current_focus) = (self.current_focus)() {
            let new_focus = (current_focus + 1) % (self.item_count)();
            self.current_focus.set(Some(new_focus));
        }
    }

    fn focus_prev(&mut self) {
        if let Some(current_focus) = (self.current_focus)() {
            let new_focus = if current_focus == 0 {
                (self.item_count)().saturating_sub(1)
            } else {
                current_focus - 1
            };
            self.current_focus.set(Some(new_focus));
        }
    }

    fn focus_first(&mut self) {
        self.current_focus.set(Some(0));
    }

    fn focus_last(&mut self) {
        let last = (self.item_count)().saturating_sub(1);
        self.current_focus.set(Some(last));
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
    open: Option<Signal<bool>>,

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

    let mut ctx = use_context_provider(|| DropdownMenuContext {
        open: open.into(),
        set_open,
        disabled: props.disabled,
        item_count: Signal::new(0),
        recent_focus: Signal::new(0),
        current_focus: Signal::new(None),
    });

    rsx! {
        div {
            role: "menu",
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),

            onfocusout: move |_| ctx.set_focus(None),
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
    let ctx: DropdownMenuContext = use_context();

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
    let open = ctx.open;

    rsx! {
        div {
            role: "menu",
            "data-state": if open() { "open" } else { "closed" },
            hidden: !open(),

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

    // Cleanup when the component is unmounted
    use_drop(move || {
        ctx.item_count -= 1;
        if (ctx.current_focus)() == Some((props.index)()) {
            ctx.set_focus(None);
        }
    });

    let tab_index = use_memo(move || {
        if (ctx.current_focus)() == Some((props.index)()) {
            "0"
        } else {
            "-1"
        }
    });

    rsx! {
        div {
            role: "menuitem",
            tabindex: tab_index,
            "data-disabled": (ctx.disabled)() || (props.disabled)(),

            onclick: {
                let value = (props.value)().clone();
                move |_| {
                    if !(ctx.disabled)() && !(props.disabled)() {
                        props.on_select.call(value.clone());
                        ctx.set_open.call(false);
                    }
                }
            },

            onfocus: move |_| ctx.set_focus(Some((props.index)())),

            onkeydown: {
                let value = (props.value)().clone();
                move |event: Event<KeyboardData>| {
                    let mut prevent_default = true;
                    match event.key() {
                        Key::Enter => {
                            if !(ctx.disabled)() && !(props.disabled)() {
                                props.on_select.call(value.clone());
                                ctx.set_open.call(false);
                            }
                        }
                        Key::Escape => ctx.set_open.call(false),
                        Key::ArrowUp => ctx.focus_prev(),
                        Key::ArrowDown => ctx.focus_next(),
                        Key::Home => ctx.focus_first(),
                        Key::End => ctx.focus_last(),
                        _ => prevent_default = false,
                    }
                    if prevent_default {
                        event.prevent_default();
                    }
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}
