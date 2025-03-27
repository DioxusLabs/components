use crate::{use_aria_or, use_unique_id};
use dioxus_lib::prelude::*;

#[derive(Clone, Copy)]
struct CollapsibleCtx {
    internal_open: Signal<bool>,
    open: Option<Signal<bool>>,
    on_open_changed: Callback<bool>,
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
    on_open_changed: Callback<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    let internal_open = use_signal(|| props.open.map(|x| x()).unwrap_or(props.default_open));
    let aria_controls_id = use_unique_id();
    let _ctx = use_context_provider(|| CollapsibleCtx {
        internal_open,
        open: props.open,
        on_open_changed: props.on_open_changed,
        disabled: props.disabled,
        aria_controls_id,
    });

    let open = use_memo(move || props.open.unwrap_or(internal_open)());
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

    let open = use_memo(move || ctx.open.unwrap_or(ctx.internal_open)());
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
    let mut ctx: CollapsibleCtx = use_context();

    let open = use_memo(move || ctx.open.unwrap_or(ctx.internal_open)());
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
                let new_open = !(ctx.internal_open)();
                ctx.internal_open.set(new_open);
                ctx.on_open_changed.call(new_open);
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
