use dioxus_lib::prelude::*;

use crate::{use_controlled, use_id_or, use_unique_id};

#[derive(Clone, Copy)]
struct AlertDialogCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
    alert_labelledby: Signal<String>,
    alert_describedby: Signal<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogProps {
    id: ReadOnlySignal<Option<String>>,
    open: Option<Signal<bool>>,
    #[props(default)]
    default_open: bool,
    #[props(default)]
    on_open_change: Callback<bool>,
    children: Element,
}

#[component]
pub fn AlertDialog(props: AlertDialogProps) -> Element {
    let alert_labelledby = use_unique_id();
    let alert_describedby = use_unique_id();
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let ctx = use_context_provider(|| AlertDialogCtx {
        open,
        set_open,
        alert_labelledby,
        alert_describedby,
    });

    let gen_id = use_unique_id();
    let id = use_id_or(gen_id, props.id);

    // Keyboard handling: close on Escape
    let on_keydown = move |e: Event<KeyboardData>| {
        if e.key() == Key::Escape {
            set_open.call(false);
            e.prevent_default();
        }
    };

    // Focus out: close if focus leaves dialog
    let on_focusout = move |_e: Event<FocusData>| {
        set_open.call(false);
    };

    if !open() {
        return rsx! {};
    }
    rsx! {
        div {
            div {
                class: "alert-dialog-backdrop",
                style: "position: fixed; inset: 0; background: rgba(0,0,0,0.3); z-index: 1000;",
                onclick: move |_| set_open.call(false),
            }
            div {
                id,
                role: "alertdialog",
                aria_modal: "true",
                aria_labelledby: ctx.alert_labelledby.peek().clone(),
                aria_describedby: ctx.alert_describedby.peek().clone(),
                tabindex: "0",
                class: "alert-dialog",
                style: "position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); z-index: 1001; background: white; outline: none;",
                autofocus: true,
                onfocusout: on_focusout,
                onkeydown: on_keydown,
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogTitleProps {
    id: ReadOnlySignal<Option<String>>,
    children: Element,
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    let id = use_id_or(ctx.alert_labelledby, props.id);
    rsx! {
        h2 { id, {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogDescriptionProps {
    id: ReadOnlySignal<Option<String>>,
    children: Element,
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    let id = use_id_or(ctx.alert_describedby, props.id);
    rsx! {
        p { id, {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionProps {
    children: Element,
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    rsx! {
        button {
            r#type: "button",
            class: "alert-dialog-action",
            onclick: move |_| ctx.set_open.call(false),
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogCancelProps {
    children: Element,
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    rsx! {
        button {
            r#type: "button",
            class: "alert-dialog-cancel",
            onclick: move |_| ctx.set_open.call(false),
            {props.children}
        }
    }
}
