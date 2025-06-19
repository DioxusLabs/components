use crate::{
    focus::{FocusState, use_focus_controlled_item, use_focus_provider},
    use_controlled, use_unique_id,
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

#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
    open: ReadOnlySignal<Option<bool>>,

    #[props(default)]
    default_open: bool,

    #[props(default)]
    on_open_change: Callback<bool>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    roving_loop: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

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
            role: "listbox",
            aria_labelledby: "{ctx.trigger_id}",
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
