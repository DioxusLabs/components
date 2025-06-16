use dioxus_lib::prelude::*;

use crate::focus::{
    FocusState, use_focus_control, use_focus_controlled_item, use_focus_entry, use_focus_provider,
};

#[derive(Clone, Copy)]
struct MenubarContext {
    // Currently open menu index
    open_menu: Signal<Option<usize>>,
    set_open_menu: Callback<Option<usize>>,
    disabled: ReadOnlySignal<bool>,

    // Focus state
    focus: FocusState,
}

#[derive(Props, Clone, PartialEq)]
pub struct MenubarProps {
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    roving_loop: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    let mut open_menu = use_signal(|| None);
    let set_open_menu = use_callback(move |idx| open_menu.set(idx));

    let focus = use_focus_provider(props.roving_loop);
    let ctx = use_context_provider(|| MenubarContext {
        open_menu,
        set_open_menu,
        disabled: props.disabled,
        focus,
    });
    use_effect(move || {
        let index = ctx.focus.current_focus();
        if ctx.open_menu.read().is_some() {
            ctx.set_open_menu.call(index);
        }
    });

    rsx! {
        div {
            role: "menubar",
            "data-disabled": (props.disabled)(),

            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Clone, Copy)]
struct MenubarMenuContext {
    index: ReadOnlySignal<usize>,
    focus: FocusState,
    is_open: Memo<bool>,
}

impl MenubarMenuContext {
    fn focus_next(&mut self) {
        self.focus.focus_next();
    }

    fn focus_prev(&mut self) {
        self.focus.focus_prev();
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenubarMenuProps {
    index: ReadOnlySignal<usize>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    let mut ctx: MenubarContext = use_context();
    let is_open = use_memo(move || (ctx.open_menu)() == Some(props.index.cloned()));
    let focus = use_focus_provider(ctx.focus.roving_loop);
    let mut menu_ctx = use_context_provider(|| MenubarMenuContext {
        index: props.index,
        focus,
        is_open,
    });

    use_effect(move || {
        if !is_open() {
            menu_ctx.focus.blur();
        }
    });

    use_focus_entry(props.index);

    let disabled = move || (ctx.disabled)() || (props.disabled)();

    rsx! {
        div {
            role: "menu",
            "data-state": if is_open() { "open" } else { "closed" },
            "data-disabled": (ctx.disabled)() || (props.disabled)(),

            onclick: move |_| {
                if !disabled() {
                    let new_open = if is_open() { None } else { Some(props.index.cloned()) };
                    ctx.set_open_menu.call(new_open);
                }
            },

            onpointerenter: move |_| {
                if !disabled() && (ctx.open_menu)().is_some() {
                    ctx.focus.set_focus(Some(props.index.cloned()));
                }
            },

            onkeydown: move |event: Event<KeyboardData>| {
                match event.key() {
                    Key::Enter if !disabled() => {
                        ctx.set_open_menu.call((!is_open()).then(&*props.index));
                    }
                    Key::Escape => ctx.set_open_menu.call(None),
                    Key::ArrowLeft => ctx.focus.focus_prev(),
                    Key::ArrowRight => ctx.focus.focus_next(),
                    Key::ArrowDown if !disabled() => {
                        if is_open() {
                            menu_ctx.focus_next();
                        } else {
                            ctx.set_open_menu.call(Some(props.index.cloned()));
                        }
                    },
                    Key::ArrowUp if !disabled() => {
                        if is_open() {
                            menu_ctx.focus_prev();
                        }
                    },
                    Key::Home => ctx.focus.focus_first(),
                    Key::End => ctx.focus.focus_last(),
                    _ => return,
                }
                event.prevent_default();
            },

            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenubarTriggerProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let mut ctx: MenubarContext = use_context();
    let menu_ctx: MenubarMenuContext = use_context();
    let onmounted = use_focus_control(menu_ctx.index);

    rsx! {
        button {
            onmounted,
            onfocus: move |_| ctx.focus.set_focus(Some(menu_ctx.index.cloned())),
            onblur: move |_| {
                if ctx.focus.current_focus() == Some(menu_ctx.index.cloned()) && !menu_ctx.focus.any_focused() {
                    ctx.focus.set_focus(None);
                }
            },
            role: "menuitem",
            tabindex: if ctx.focus.recent_focus() == menu_ctx.index.cloned() { "0" } else { "-1" },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenubarContentProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    let menu_ctx: MenubarMenuContext = use_context();

    rsx! {
        div {
            role: "menu",
            "data-state": if (menu_ctx.is_open)() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenubarItemProps {
    index: ReadOnlySignal<usize>,

    value: String,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    on_select: Callback<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    let mut ctx: MenubarContext = use_context();
    let mut menu_ctx: MenubarMenuContext = use_context();

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let focused = move || menu_ctx.focus.is_focused(props.index.cloned()) && (menu_ctx.is_open)();

    let onmounted = use_focus_controlled_item(props.index);

    rsx! {
        div {
            role: "menuitem",
            "data-disabled": disabled(),
            tabindex: if focused() { "0" } else { "-1" },

            onclick: {
                let value = props.value.clone();
                move |_| {
                    if !disabled() {
                        props.on_select.call(value.clone());
                        ctx.set_open_menu.call(None);
                    }
                }
            },

            onkeydown: {
                let value = props.value.clone();
                move |event: Event<KeyboardData>| {
                    if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
                        if !disabled() {
                            props.on_select.call(value.clone());
                            ctx.set_open_menu.call(None);
                        }
                        event.prevent_default();
                        event.stop_propagation();
                    }
                }
            },

            onmounted,

            onblur: move |_| {
                if focused() {
                    menu_ctx.focus.blur();
                    ctx.focus.set_focus(None);
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}
