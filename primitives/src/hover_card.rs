use crate::{use_controlled, use_id_or, use_unique_id};
use dioxus_lib::prelude::*;

#[derive(Clone)]
struct HoverCardCtx {
    // State
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,

    // ARIA attributes
    content_id: Signal<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct HoverCardProps {
    /// Whether the hover card is open
    open: Option<Signal<bool>>,

    /// Default open state
    #[props(default)]
    default_open: bool,

    /// Callback when open state changes
    #[props(default)]
    on_open_change: Callback<bool>,

    /// Whether the hover card is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    // Generate a unique ID for the hover card content
    let content_id = use_unique_id();

    let _ctx = use_context_provider(|| HoverCardCtx {
        open,
        set_open,
        disabled: props.disabled,
        content_id,
    });

    rsx! {
        div {
            class: "hover-card",
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct HoverCardTriggerProps {
    /// Optional ID for the trigger element
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    let ctx: HoverCardCtx = use_context();

    // Generate a unique ID for the trigger
    let trigger_id = use_unique_id();

    // Use use_id_or to handle the ID
    let id = use_id_or(trigger_id, props.id);

    // Handle mouse events
    let handle_mouse_enter = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let handle_mouse_leave = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };

    rsx! {
        div {
            id,
            class: "hover-card-trigger",

            // Mouse events
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,

            // ARIA attributes
            aria_haspopup: "dialog",
            aria_expanded: (ctx.open)(),
            aria_controls: ctx.content_id.peek().clone(),

            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HoverCardSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl HoverCardSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            HoverCardSide::Top => "top",
            HoverCardSide::Right => "right",
            HoverCardSide::Bottom => "bottom",
            HoverCardSide::Left => "left",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HoverCardAlign {
    Start,
    Center,
    End,
}

impl HoverCardAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            HoverCardAlign::Start => "start",
            HoverCardAlign::Center => "center",
            HoverCardAlign::End => "end",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct HoverCardContentProps {
    /// Optional ID for the hover card content
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    /// Side of the trigger to place the hover card
    #[props(default = HoverCardSide::Top)]
    side: HoverCardSide,

    /// Alignment of the hover card relative to the trigger
    #[props(default = HoverCardAlign::Center)]
    align: HoverCardAlign,

    /// Whether to force the hover card to stay open when hovered
    #[props(default = true)]
    force_mount: bool,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    let ctx: HoverCardCtx = use_context();

    // Only render if the hover card is open or force_mount is true
    let is_open = (ctx.open)();
    if !is_open && !props.force_mount {
        return rsx!({});
    }

    // Use use_id_or to handle the ID
    let id = use_id_or(ctx.content_id, props.id);

    // Handle mouse events to keep the hover card open when hovered
    let handle_mouse_enter = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let handle_mouse_leave = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };

    rsx! {
        div {
            id: id,
            class: "hover-card-content",
            role: "dialog",
            "data-state": if is_open { "open" } else { "closed" },
            "data-side": props.side.as_str(),
            "data-align": props.align.as_str(),

            // Mouse events to keep the hover card open when hovered
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,

            ..props.attributes,
            {props.children}
        }
    }
}
