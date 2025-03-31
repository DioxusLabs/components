use crate::{use_aria_or, use_controlled, use_unique_id};
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct CollapsibleCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadOnlySignal<bool>,
    aria_controls_id: Signal<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleProps {
    #[props(default)]
    default_open: bool,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    open: Option<Signal<bool>>,

    #[props(default)]
    on_open_change: Callback<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let aria_controls_id = use_unique_id();
    let _ctx = use_context_provider(|| CollapsibleCtx {
        open,
        set_open,
        disabled: props.disabled,
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

#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    let ctx: CollapsibleCtx = use_context();
    let id = use_aria_or(ctx.aria_controls_id, props.id);

    let open = ctx.open;
    let state = open_state(open());

    rsx! {
        div {
            id: id,
            "data-state": state,
            "data-disabled": ctx.disabled,
            ..props.attributes,

            if open() {
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
