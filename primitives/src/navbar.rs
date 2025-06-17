use dioxus_lib::prelude::*;

use crate::focus::{
    FocusState, use_focus_control, use_focus_controlled_item, use_focus_entry, use_focus_provider,
};

#[derive(Clone, Copy, PartialEq)]
enum MovementDirection {
    End,
    Start,
}

#[derive(Clone, Copy)]
struct NavbarContext {
    // Currently open nav index
    open_nav: Signal<Option<usize>>,
    set_open_nav: Callback<Option<usize>>,
    disabled: ReadOnlySignal<bool>,
    last_movement: Signal<MovementDirection>,

    // Focus state
    focus: FocusState,
}

#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    roving_loop: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let mut open_nav = use_signal(|| None);
    let set_open_nav = use_callback(move |idx| open_nav.set(idx));
    let mut last_movement = use_signal(|| MovementDirection::End);

    let focus = use_focus_provider(props.roving_loop);
    let ctx = use_context_provider(|| NavbarContext {
        open_nav,
        set_open_nav,
        disabled: props.disabled,
        focus,
        last_movement,
    });
    use_effect(move || {
        let index = ctx.focus.current_focus();
        if ctx.open_nav.peek().is_some() {
            ctx.set_open_nav.call(index);
        }
    });

    // Keep track of the current and last open nav index to determine movement direction
    let mut last_open_nav = use_signal(|| None);
    use_effect(move || {
        let current_open_nav = ctx.open_nav.cloned();
        {
            let last_open_nav = *last_open_nav.peek();
            match (last_open_nav, current_open_nav) {
                (Some(last), Some(current)) if last < current => {
                    last_movement.set(MovementDirection::End)
                }
                (Some(last), Some(current)) if last > current => {
                    last_movement.set(MovementDirection::Start)
                }
                (Some(_), None) => last_movement.set(MovementDirection::End),
                (None, Some(_)) => last_movement.set(MovementDirection::Start),
                _ => {}
            }
        }
        last_open_nav.set(current_open_nav);
    });

    let aria_label = props.attributes.iter().find_map(|attr| {
        (attr.name == "aria-label")
            .then(|| attr.value.clone())
    });

    rsx! {
        div {
            role: "navigation",
            display: "content",
            aria_label,
            div {
                role: "menubar",
                "data-disabled": (props.disabled)(),

                ..props.attributes,

                {props.children}
            }
        }
    }
}

#[derive(Clone, Copy)]
struct NavbarNavContext {
    index: ReadOnlySignal<usize>,
    focus: FocusState,
    is_open: Memo<bool>,
}

impl NavbarNavContext {
    fn focus_next(&mut self) {
        self.focus.focus_next();
    }

    fn focus_prev(&mut self) {
        self.focus.focus_prev();
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NavbarNavProps {
    index: ReadOnlySignal<usize>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    let mut ctx: NavbarContext = use_context();
    let is_open = use_memo(move || (ctx.open_nav)() == Some(props.index.cloned()));
    let focus = use_focus_provider(ctx.focus.roving_loop);
    let mut nav_ctx = use_context_provider(|| NavbarNavContext {
        index: props.index,
        focus,
        is_open,
    });

    use_effect(move || {
        if !is_open() {
            nav_ctx.focus.blur();
        }
    });

    use_focus_entry(ctx.focus, nav_ctx.index);

    let disabled = move || (ctx.disabled)() || (props.disabled)();

    rsx! {
        div {
            role: "menu",
            "data-state": if is_open() { "open" } else { "closed" },
            "data-disabled": (ctx.disabled)() || (props.disabled)(),

            onclick: move |_| {
                if !disabled() {
                    let new_open = if is_open() { None } else { Some(props.index.cloned()) };
                    ctx.set_open_nav.call(new_open);
                }
            },

            onpointerenter: move |_| {
                if !disabled() {
                    let index = Some(props.index.cloned());
                    if (ctx.open_nav)().is_some() {
                        ctx.focus.set_focus(index);
                    } else {
                        ctx.set_open_nav.call(index);
                    }
                }
            },
            onpointerleave: move |_| {
                if is_open() {
                    ctx.focus.set_focus(None);
                }
            },

            onkeydown: move |event: Event<KeyboardData>| {
                match event.key() {
                    Key::Enter if !disabled() => {
                        ctx.set_open_nav.call((!is_open()).then(&*props.index));
                    }
                    Key::Escape => ctx.set_open_nav.call(None),
                    Key::ArrowLeft => ctx.focus.focus_prev(),
                    Key::ArrowRight => ctx.focus.focus_next(),
                    Key::ArrowDown if !disabled() => {
                        if is_open() {
                            nav_ctx.focus_next();
                        } else {
                            ctx.set_open_nav.call(Some(props.index.cloned()));
                        }
                    },
                    Key::ArrowUp if !disabled() => {
                        if is_open() {
                            nav_ctx.focus_prev();
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
pub struct NavbarTriggerProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element {
    let mut ctx: NavbarContext = use_context();
    let nav_ctx: NavbarNavContext = use_context();
    let onmounted = use_focus_control(ctx.focus, nav_ctx.index);

    rsx! {
        button {
            onmounted,
            onfocus: move |_| ctx.focus.set_focus(Some(nav_ctx.index.cloned())),
            onblur: move |_| {
                if ctx.focus.current_focus() == Some(nav_ctx.index.cloned()) && !nav_ctx.focus.any_focused() {
                    ctx.focus.set_focus(None);
                }
            },
            role: "menuitem",
            tabindex: if ctx.focus.recent_focus() == nav_ctx.index.cloned() { "0" } else { "-1" },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NavbarContentProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    let ctx: NavbarContext = use_context();
    let nav_ctx: NavbarNavContext = use_context();
    let open = nav_ctx.is_open.cloned();
    let last_movement = ctx.last_movement.cloned();
    let direction = match last_movement {
        MovementDirection::End => "end",
        MovementDirection::Start => "start",
    };
    let movement = if open { "in" } else { "out" };

    rsx! {
        div {
            role: "menubar",
            "data-state": if (nav_ctx.is_open)() { "open" } else { "closed" },
            "data-movement": "{direction}-{movement}",
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NavbarItemProps {
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
pub fn NavbarItem(props: NavbarItemProps) -> Element {
    let mut ctx: NavbarContext = use_context();
    let mut nav_ctx: NavbarNavContext = use_context();

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let focused = move || nav_ctx.focus.is_focused(props.index.cloned()) && (nav_ctx.is_open)();

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
                        ctx.set_open_nav.call(None);
                    }
                }
            },

            onkeydown: {
                let value = props.value.clone();
                move |event: Event<KeyboardData>| {
                    if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
                        if !disabled() {
                            props.on_select.call(value.clone());
                            ctx.set_open_nav.call(None);
                        }
                        event.prevent_default();
                        event.stop_propagation();
                    }
                }
            },

            onmounted,

            onblur: move |_| {
                if focused() {
                    nav_ctx.focus.blur();
                    ctx.focus.set_focus(None);
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}
