use crate::{
    use_animated_open, use_controlled, use_id_or, use_unique_id, ContentAlign, ContentSide,
};
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct TooltipCtx {
    // State
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,

    // ARIA attributes
    tooltip_id: Signal<String>,
}

/// The props for the [`Tooltip`] component
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Whether the tooltip is open
    open: ReadOnlySignal<Option<bool>>,

    /// Default open state
    #[props(default)]
    default_open: bool,

    /// Callback when open state changes
    #[props(default)]
    on_open_change: Callback<bool>,

    /// Whether the tooltip is disabled
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let tooltip_id = use_unique_id();

    let _ctx = use_context_provider(|| TooltipCtx {
        open,
        set_open,
        disabled: props.disabled,
        tooltip_id,
    });

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`TooltipTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct TooltipTriggerProps {
    /// Optional ID for the trigger element
    #[props(default)]
    id: Option<String>,

    /// Whether to use ARIA attributes
    #[props(default = true)]
    use_aria: bool,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let ctx: TooltipCtx = use_context();

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

    // Handle focus events
    let handle_focus = move |_: Event<FocusData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let handle_blur = move |_: Event<FocusData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };

    // Handle keyboard events
    let handle_keydown = move |event: Event<KeyboardData>| {
        if event.key() == Key::Escape && (ctx.open)() {
            event.prevent_default();
            ctx.set_open.call(false);
        }
    };

    rsx! {
        div {
            id: props.id.clone(),
            tabindex: "0",
            // Mouse events
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,
            // Focus events
            onfocus: handle_focus,
            onblur: handle_blur,
            // Keyboard events
            onkeydown: handle_keydown,
            // ARIA attributes
            aria_describedby: if props.use_aria { ctx.tooltip_id.peek().clone() } else { String::new() },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`TooltipContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct TooltipContentProps {
    /// Optional ID for the tooltip content
    #[props(default)]
    id: ReadOnlySignal<Option<String>>,

    /// Side of the trigger to place the tooltip
    #[props(default = ContentSide::Top)]
    side: ContentSide,

    /// Alignment of the tooltip relative to the trigger
    #[props(default = ContentAlign::Center)]
    align: ContentAlign,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let mut ctx: TooltipCtx = use_context();

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);

    use_effect(move || {
        ctx.tooltip_id.set(id());
    });

    // Only render if the tooltip is open
    let render = use_animated_open(id, ctx.open);

    // Create the tooltip content
    rsx! {
        if render() {
            div {
                id,
                role: "tooltip",
                "data-state": if ctx.open.cloned() { "open" } else { "closed" },
                "data-side": props.side.as_str(),
                "data-align": props.align.as_str(),
                ..props.attributes,
                {props.children}
            }
        }
    }
}
