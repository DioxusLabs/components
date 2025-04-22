//! Content that can be collapsed.

use crate::{use_controlled, use_id_or, use_unique_id};
use dioxus_lib::prelude::*;

// TODO: more docs

#[derive(Clone, Copy)]
struct CollapsibleCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,
    keep_mounted: ReadOnlySignal<bool>,
    aria_controls_id: Signal<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleProps {
    /// Keep [`CollapsibleContent`] mounted in the DOM when the collapsible is closed.
    ///
    /// This does not apply any special ARIA or other attributes.
    #[props(default)]
    keep_mounted: ReadOnlySignal<bool>,

    /// The default `open` state.
    ///
    /// This will be overridden if the component is controlled.
    #[props(default)]
    default_open: bool,

    /// The disabled state of the collapsible.
    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    /// The controlled `open` state of the collapsible.
    ///
    /// If this is provided, you must use `on_open_change`.
    open: Option<Signal<bool>>,

    /// A callback for when the open state changes.
    ///
    /// The provided argument is a bool of whether the collapsible is open or closed.
    #[props(default)]
    on_open_change: Callback<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

/// The provider for a collapsible piece of content.
#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let aria_controls_id = use_unique_id();
    let _ctx = use_context_provider(|| CollapsibleCtx {
        open,
        set_open,
        disabled: props.disabled,
        keep_mounted: props.keep_mounted,
        aria_controls_id,
    });

    let state = open_state(open());

    rsx! {
        div {
            "data-state": state,
            "data-disabled": props.disabled,
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleContentProps {
    id: ReadOnlySignal<Option<String>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

/// A section of content that can be collapsed.
#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    let ctx: CollapsibleCtx = use_context();
    let id = use_id_or(ctx.aria_controls_id, props.id);

    let open = ctx.open;
    let state = open_state(open());

    rsx! {
        div {
            id: id,
            "data-state": state,
            "data-disabled": ctx.disabled,
            ..props.attributes,

            if open() || (ctx.keep_mounted)() {
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleTriggerProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

/// The trigger of a collapsible piece of content.
#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
    let ctx: CollapsibleCtx = use_context();

    let open = ctx.open;
    let state = open_state(open());

    rsx! {

        button {
            type: "button",
            "data-state": state,
            "data-disabled": ctx.disabled,
            disabled: ctx.disabled,

            aria_controls: ctx.aria_controls_id,
            aria_expanded: open,

            onclick: move |_| {
                let new_open = !open();
                ctx.set_open.call(new_open);
            },

            ..props.attributes,
            {props.children}
        }
    }
}

fn open_state(open: bool) -> &'static str {
    match open {
        true => "open",
        false => "closed",
    }
}
