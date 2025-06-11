use dioxus_lib::prelude::*;

use crate::use_effect_cleanup;

#[derive(Clone, Copy)]
struct MenubarContext {
    // Currently open menu index
    open_menu: Signal<Option<usize>>,
    set_open_menu: Callback<Option<usize>>,
    disabled: ReadOnlySignal<bool>,

    // Keyboard nav data
    menu_count: Signal<usize>,
    recent_focus: Signal<usize>,
    current_focus: Signal<Option<usize>>,
}

impl MenubarContext {
    fn set_focus(&mut self, index: Option<usize>) {
        if let Some(idx) = index {
            self.recent_focus.set(idx);
        }
        self.current_focus.set(index);
        if self.open_menu.read().is_some() {
            self.set_open_menu.call(index);
        }
    }

    fn focus_next(&mut self) {
        let next = (*self.recent_focus.read() + 1) % *self.menu_count.read();
        self.set_focus(Some(next));
    }

    fn focus_prev(&mut self) {
        let prev = if *self.recent_focus.read() == 0 {
            *self.menu_count.read() - 1
        } else {
            *self.recent_focus.read() - 1
        };
        self.set_focus(Some(prev));
    }

    fn focus_first(&mut self) {
        self.set_focus(Some(0));
    }

    fn focus_last(&mut self) {
        let last_index = *self.menu_count.read() - 1;
        self.set_focus(Some(last_index));
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenubarProps {
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    let mut open_menu = use_signal(|| None);
    let set_open_menu = use_callback(move |idx| open_menu.set(idx));

    use_context_provider(|| MenubarContext {
        open_menu,
        set_open_menu,
        disabled: props.disabled,
        menu_count: Signal::new(0),
        recent_focus: Signal::new(0),
        current_focus: Signal::new(None),
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
    index: usize,
    current_focus: Signal<Option<usize>>,
    item_count: Signal<usize>,
    is_open: Memo<bool>,
}

impl MenubarMenuContext {
    fn focus_next(&mut self) {
        let mut current_focus = self.current_focus.write();
        match &mut *current_focus {
            Some(current_focus) => *current_focus = (*current_focus + 1) % self.item_count.cloned(),
            None => *current_focus = Some(0),
        }
    }

    fn focus_prev(&mut self) {
        let mut current_focus = self.current_focus.write();
        let item_count = self.item_count.cloned();
        match &mut *current_focus {
            Some(current_focus) if *current_focus > 0 => *current_focus -= 1,
            Some(_) | None => *current_focus = Some(item_count - 1),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenubarMenuProps {
    index: usize,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    let mut ctx: MenubarContext = use_context();
    let is_open = use_memo(move || (ctx.open_menu)() == Some(props.index));
    let mut menu_ctx = use_context_provider(|| MenubarMenuContext {
        index: props.index,
        current_focus: Signal::new(None),
        item_count: Signal::new(0),
        is_open,
    });

    use_effect(move || {
        if !is_open() {
            menu_ctx.current_focus.set(None);
        }
    });

    use_effect(move || {
        ctx.menu_count += 1;
    });

    use_effect_cleanup(move || {
        ctx.menu_count -= 1;
        if (ctx.current_focus)() == Some(props.index) {
            ctx.set_focus(None);
        }
    });

    let disabled = move || (ctx.disabled)() || (props.disabled)();

    rsx! {
        div {
            role: "menu",
            "data-state": if is_open() { "open" } else { "closed" },
            "data-disabled": (ctx.disabled)() || (props.disabled)(),

            onclick: move |_| {
                if !disabled() {
                    let new_open = if is_open() { None } else { Some(props.index) };
                    ctx.set_open_menu.call(new_open);
                }
            },

            onpointerenter: move |_| {
                if !disabled() && (ctx.open_menu)().is_some() {
                    ctx.set_focus(Some(props.index));
                }
            },

            onkeydown: move |event: Event<KeyboardData>| {
                match event.key() {
                    Key::Enter if !disabled() => {
                        ctx.set_open_menu.call((!is_open()).then_some(props.index));
                    }
                    Key::Escape => ctx.set_open_menu.call(None),
                    Key::ArrowLeft => ctx.focus_prev(),
                    Key::ArrowRight => ctx.focus_next(),
                    Key::ArrowDown if !disabled() => {
                        if is_open() {
                            menu_ctx.focus_next();
                        } else {
                            ctx.set_open_menu.call(Some(props.index));
                        }
                    },
                    Key::ArrowUp if !disabled() => {
                        if is_open() {
                            menu_ctx.focus_prev();
                        }
                    },
                    Key::Home => ctx.focus_first(),
                    Key::End => ctx.focus_last(),
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
    let mut button_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        let Some(button) = button_ref() else {
            return;
        };
        if (ctx.current_focus)() == Some(menu_ctx.index) {
            spawn(async move {
                _ = button.set_focus(true).await;
            });
        }
    });

    rsx! {
        button {
            onmounted: move |node| {
                button_ref.set(Some(node.data()));
            },
            onfocus: move |_| ctx.set_focus(Some(menu_ctx.index)),
            onblur: move |_| {
                if (ctx.current_focus)() == Some(menu_ctx.index) && menu_ctx.current_focus.read().is_none() {
                    ctx.set_focus(None);
                }
            },
            role: "menuitem",
            tabindex: if (ctx.recent_focus)() == menu_ctx.index { "0" } else { "-1" },
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
    index: usize,

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

    use_effect(move || {
        menu_ctx.item_count += 1;
    });
    use_effect_cleanup(move || {
        menu_ctx.item_count -= 1;
        if (menu_ctx.current_focus)() == Some(props.index) {
            menu_ctx.current_focus.set(None);
        }
    });

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let focused = move || (menu_ctx.current_focus)() == Some(props.index) && (menu_ctx.is_open)();

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
                    if event.key() == Key::Enter {
                        if !disabled() {
                            props.on_select.call(value.clone());
                            ctx.set_open_menu.call(None);
                        }
                        event.prevent_default();
                        event.stop_propagation();
                    }
                }
            },

            onmounted: move |node| {
                item_ref.set(Some(node.data()));
            },

            onblur: move |_| {
                if focused() {
                    menu_ctx.current_focus.set(None);
                    ctx.set_focus(None);
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}
