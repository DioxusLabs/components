use dioxus_lib::prelude::*;

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
    let set_open_menu = Callback::new(move |idx| open_menu.set(idx));

    let mut ctx = use_context_provider(|| MenubarContext {
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

            onfocusout: move |_| ctx.set_focus(None),
            ..props.attributes,

            {props.children}
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

    use_effect(move || {
        ctx.menu_count += 1;
    });

    use_drop(move || {
        ctx.menu_count -= 1;
        if (ctx.current_focus)() == Some(props.index) {
            ctx.set_focus(None);
        }
    });

    let is_open = use_memo(move || (ctx.open_menu)() == Some(props.index));
    let tab_index = use_memo(move || {
        if (ctx.current_focus)() == Some(props.index) {
            "0"
        } else {
            "-1"
        }
    });

    rsx! {
        div {
            role: "menu",
            tabindex: tab_index,
            "data-state": if is_open() { "open" } else { "closed" },
            "data-disabled": (ctx.disabled)() || (props.disabled)(),

            onclick: move |_| {
                if !(ctx.disabled)() && !(props.disabled)() {
                    let new_open = if is_open() { None } else { Some(props.index) };
                    ctx.set_open_menu.call(new_open);
                }
            },

            onfocus: move |_| ctx.set_focus(Some(props.index)),

            onkeydown: move |event: Event<KeyboardData>| {
                let mut prevent_default = true;
                match event.key() {
                    Key::Enter => {
                        if !(ctx.disabled)() && !(props.disabled)() {
                            let new_open = if is_open() { None } else { Some(props.index) };
                            ctx.set_open_menu.call(new_open);
                        }
                    }
                    Key::Escape => ctx.set_open_menu.call(None),
                    Key::ArrowLeft => ctx.focus_prev(),
                    Key::ArrowRight => ctx.focus_next(),
                    Key::Home => ctx.focus_first(),
                    Key::End => ctx.focus_last(),
                    _ => prevent_default = false,
                }
                if prevent_default {
                    event.prevent_default();
                }
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
    rsx! {
        button { ..props.attributes,{props.children} }
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
    rsx! {
        div { role: "menu", ..props.attributes, {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenubarItemProps {
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
    let ctx: MenubarContext = use_context();

    rsx! {
        div {
            role: "menuitem",
            "data-disabled": (ctx.disabled)() || (props.disabled)(),

            onclick: {
                let value = props.value.clone();
                move |_| {
                    if !(ctx.disabled)() && !(props.disabled)() {
                        props.on_select.call(value.clone());
                        ctx.set_open_menu.call(None);
                    }
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}
