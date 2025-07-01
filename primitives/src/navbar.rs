use crate::{
    focus::{
        use_focus_control, use_focus_controlled_item, use_focus_entry, use_focus_provider,
        FocusState,
    },
    use_animated_open, use_id_or, use_unique_id,
};
use dioxus_lib::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Copy)]
struct NavbarContext {
    // Currently open nav index
    open_nav: Signal<Option<usize>>,
    set_open_nav: Callback<Option<usize>>,
    disabled: ReadOnlySignal<bool>,

    // Focus state
    focus: FocusState,
}

/// The props for the [`Navbar`] component
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

    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| NavbarContext {
        open_nav,
        set_open_nav,
        disabled: props.disabled,
        focus,
    });
    use_effect(move || {
        let index = ctx.focus.current_focus();
        if ctx.open_nav.peek().is_some() {
            ctx.set_open_nav.call(index);
        }
    });

    let aria_label = props
        .attributes
        .iter()
        .find_map(|attr| (attr.name == "aria-label").then(|| attr.value.clone()));

    rsx! {
        div {
            role: "navigation",
            display: "content",
            aria_label,
            div {
                role: "menubar",
                "data-disabled": (props.disabled)(),
                tabindex: (!ctx.focus.any_focused()).then_some("0"),
                // If the menu receives focus, focus the most recently focused menu item
                onfocus: move |_| {
                    ctx.focus.set_focus(Some(ctx.focus.recent_focus()));
                },
                onkeydown: move |event: Event<KeyboardData>| {
                    match event.key() {
                        Key::Escape => ctx.set_open_nav.call(None),
                        Key::ArrowLeft => ctx.focus.focus_prev(),
                        Key::ArrowRight => ctx.focus.focus_next(),
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
}

#[derive(Clone, Copy)]
struct NavbarNavContext {
    index: ReadOnlySignal<usize>,
    focus: FocusState,
    is_open: Memo<bool>,
    disabled: ReadOnlySignal<bool>,
}

impl NavbarNavContext {
    fn focus_next(&mut self) {
        self.focus.focus_next();
    }

    fn focus_prev(&mut self) {
        self.focus.focus_prev();
    }
}

/// The props for the [`NavbarNav`] component
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
        disabled: props.disabled,
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

            onmouseenter: move |_| {
                if !disabled() {
                    let index = Some(nav_ctx.index.cloned());
                    if (ctx.open_nav)().is_some() {
                        ctx.focus.set_focus(index);
                    } else {
                        ctx.set_open_nav.call(index);
                    }
                }
            },
            onmouseleave: move |_| {
                if is_open() {
                    ctx.focus.set_focus(None);
                }
            },
            onkeydown: move |event: Event<KeyboardData>| {
                match event.key() {
                    Key::Enter if !disabled() => {
                        ctx.set_open_nav.call((!is_open()).then(&*props.index));
                    }
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
                    _ => return,
                }
                event.prevent_default();
            },

            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`NavbarTrigger`] component
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
    let is_focused = move || {
        ctx.focus.current_focus() == Some(nav_ctx.index.cloned()) && !nav_ctx.focus.any_focused()
    };
    let disabled = move || (ctx.disabled)() || (nav_ctx.disabled)();
    let is_open = nav_ctx.is_open;

    rsx! {
        button {
            onmounted,
            onpointerdown: move |_| {
                if !disabled() {
                    let new_open = if is_open() { None } else { Some(nav_ctx.index.cloned()) };
                    ctx.set_open_nav.call(new_open);
                }
            },
            onblur: move |_| {
                if is_focused() {
                    ctx.focus.set_focus(None);
                    ctx.set_open_nav.call(None);
                }
            },
            role: "menuitem",
            tabindex: if is_focused() { "0" } else { "-1" },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`NavbarContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct NavbarContentProps {
    id: ReadOnlySignal<Option<String>>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    let ctx: NavbarContext = use_context();
    let nav_ctx: NavbarNavContext = use_context();
    let index = nav_ctx.index.cloned();
    let open_direction = match (ctx.open_nav)() {
        Some(open_index) if open_index > index => "start",
        Some(open_index) if open_index < index => "end",
        Some(_) => "open",
        None => "closed",
    };

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);

    let render = use_animated_open(id, nav_ctx.is_open);

    rsx! {
        if render() {
            div {
                id,
                role: "menu",
                "data-state": if (nav_ctx.is_open)() { "open" } else { "closed" },
                "data-open-menu-direction": "{open_direction}",
                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`NavbarItem`] component
#[derive(Props, Clone, PartialEq)]
pub struct NavbarItemProps {
    index: ReadOnlySignal<usize>,

    value: String,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    on_select: Callback<String>,

    /// The class attribute for the `a` tag.
    pub class: Option<String>,

    /// A class to apply to the generate HTML anchor tag if the `target` route is active.
    pub active_class: Option<String>,

    /// The children to render within the generated HTML anchor tag.
    pub children: Element,

    /// When [`true`], the `target` route will be opened in a new tab.
    ///
    /// This does not change whether the [`Link`] is active or not.
    #[props(default)]
    pub new_tab: bool,

    /// The onclick event handler.
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// The onmounted event handler.
    /// Fired when the `<a>` element is mounted.
    pub onmounted: Option<EventHandler<MountedEvent>>,

    #[props(default)]
    /// Whether the default behavior should be executed if an `onclick` handler is provided.
    ///
    /// 1. When `onclick` is [`None`] (default if not specified), `onclick_only` has no effect.
    /// 2. If `onclick_only` is [`false`] (default if not specified), the provided `onclick` handler
    ///    will be executed after the links regular functionality.
    /// 3. If `onclick_only` is [`true`], only the provided `onclick` handler will be executed.
    pub onclick_only: bool,

    /// The rel attribute for the generated HTML anchor tag.
    ///
    /// For external `a`s, this defaults to `noopener noreferrer`.
    pub rel: Option<String>,

    /// The navigation target. Roughly equivalent to the href attribute of an HTML anchor tag.
    #[props(into)]
    pub to: NavigationTarget,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn NavbarItem(mut props: NavbarItemProps) -> Element {
    let mut ctx: NavbarContext = use_context();
    let mut nav_ctx: Option<NavbarNavContext> = try_use_context();

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let focused = move || {
        nav_ctx.map_or_else(
            || ctx.focus.is_focused(props.index.cloned()),
            |nav_ctx| nav_ctx.focus.is_focused(props.index.cloned()) && (nav_ctx.is_open)(),
        )
    };

    let mut onmounted = use_focus_controlled_item(props.index);

    props.attributes.push(onkeydown({
        let value = props.value.clone();
        let to = props.to.clone();
        move |event: Event<KeyboardData>| {
            if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
                if !disabled() {
                    props.on_select.call(value.clone());
                    ctx.set_open_nav.call(None);
                    let navigator = navigator();
                    navigator.push(to.clone());
                }
                event.prevent_default();
                event.stop_propagation();
            }
        }
    }));

    props.attributes.push(onpointerdown(move |_| {
        if let Some(mut nav_ctx) = nav_ctx {
            nav_ctx.focus.set_focus(Some(props.index.cloned()));
        }
    }));

    props.attributes.push(onblur(move |_| {
        if focused() {
            if let Some(nav_ctx) = &mut nav_ctx {
                nav_ctx.focus.blur();
            }
            ctx.focus.set_focus(None);
            ctx.set_open_nav.call(None);
        }
    }));

    let tabindex =
        if focused() || (nav_ctx.is_none() && ctx.focus.recent_focus() == props.index.cloned()) {
            "0"
        } else {
            "-1"
        };

    rsx! {
        Link {
            class: props.class,
            active_class: props.active_class,
            new_tab: props.new_tab,
            onclick_only: props.onclick_only,
            rel: props.rel,
            to: props.to,
            role: "menuitem",
            "data-disabled": disabled(),
            tabindex,

            onclick: {
                let value = props.value.clone();
                move |mouse_event| {
                    if !disabled() {
                        props.on_select.call(value.clone());
                        ctx.set_open_nav.call(None);
                    }
                    if let Some(onclick) = props.onclick {
                        onclick.call(mouse_event);
                    }
                }
            },

            onmounted: move |evt: MountedEvent| {
                onmounted(evt.clone());
                if let Some(onmounted) = &props.onmounted {
                    onmounted.call(evt);
                }
            },

            attributes: props.attributes,
            {props.children}
        }
    }
}
