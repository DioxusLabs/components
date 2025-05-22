// AlertDialog primitive for Dioxus, Radix-style composable API
// Usage:
// rsx! {
//     let open = use_signal(|| false);
//     button { onclick: move |_| open.set(true), "Show Alert Dialog" }
//     AlertDialogRoot { open: Some(open), on_open_change: move |v| open.set(v),
//         AlertDialogContent {
//             AlertDialogTitle { "Title" }
//             AlertDialogDescription { "Description" }
//             AlertDialogActions {
//                 AlertDialogCancel { "Cancel" }
//                 AlertDialogAction { "Confirm" }
//             }
//         }
//     }
// }
//
// You can pass on_click to AlertDialogAction/Cancel for custom logic.

use crate::use_unique_id;
use dioxus_lib::prelude::*;

#[derive(Clone)]
struct AlertDialogCtx {
    open: Signal<bool>,
    set_open: Callback<bool>,
    labelledby: String,
    describedby: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogRootProps {
    #[props(default)]
    default_open: bool,
    #[props(default)]
    open: Option<Signal<bool>>,
    #[props(default)]
    on_open_change: Callback<bool>,
    children: Element,
}

#[component]
pub fn AlertDialogRoot(props: AlertDialogRootProps) -> Element {
    let labelledby = use_unique_id().to_string();
    let describedby = use_unique_id().to_string();
    let mut open_signal = use_signal(|| props.default_open);
    let set_open = Callback::new({
        let user_on_open_change = props.on_open_change;
        move |v: bool| {
            open_signal.set(v);
            user_on_open_change.call(v);
        }
    });
    use_context_provider(|| AlertDialogCtx {
        open: props.open.unwrap_or(open_signal),
        set_open,
        labelledby,
        describedby,
    });
    rsx! {
        {props.children}
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogContentProps {
    #[props(default)]
    style: Option<String>,
    #[props(default)]
    class: Option<String>,
    children: Element,
}

#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    // TODO: Implement focus trap so Tab/Shift+Tab cycles focus within the dialog.
    // This is important for accessibility. Currently, focus can escape the dialog and close the dialog.
    // See: https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/#keyboard-interaction
    let ctx: AlertDialogCtx = use_context();
    let open = ctx.open;
    let on_keydown = {
        let set_open = ctx.set_open;
        move |e: Event<KeyboardData>| {
            if e.key() == Key::Escape {
                set_open.call(false);
                e.prevent_default();
            }
        }
    };
    let on_focusout = move |_e: Event<FocusData>| {
        ctx.set_open.call(false);
    };
    if !open() {
        return rsx! {};
    }
    rsx! {
        div {
            class: "alert-dialog-backdrop",
            style: "position: fixed; inset: 0; background: rgba(0,0,0,0.3); z-index: 1000;",
            onclick: move |_| ctx.set_open.call(false),
        }
        div {
            role: "alertdialog",
            aria_modal: "true",
            aria_labelledby: ctx.labelledby.clone(),
            aria_describedby: ctx.describedby.clone(),
            tabindex: "0",
            class: props.class.clone().unwrap_or_else(|| "alert-dialog".to_string()),
            style: props
                .style
                .clone()
                .unwrap_or_else(|| {
                    "position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); z-index: 1001; background: white; outline: none;"
                        .to_string()
                }),
            autofocus: true,
            onfocusout: on_focusout,
            onkeydown: on_keydown,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogTitleProps {
    children: Element,
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    rsx! {
        h2 { id: ctx.labelledby.clone(), class: "alert-dialog-title", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogDescriptionProps {
    children: Element,
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    rsx! {
        p { id: ctx.describedby.clone(), class: "alert-dialog-description", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionsProps {
    children: Element,
}

#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    rsx! {
        div { class: "alert-dialog-actions", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionProps {
    #[props(default)]
    on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    style: Option<String>,
    #[props(default = "button".to_string())]
    r#type: String,
    children: Element,
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    let set_open = ctx.set_open;
    let user_on_click = props.on_click;
    let on_click = EventHandler::new(move |evt: MouseEvent| {
        set_open.call(false);
        if let Some(cb) = &user_on_click {
            cb.call(evt.clone());
        }
    });
    rsx! {
        button {
            r#type: props.r#type.clone(),
            class: props.class.clone().unwrap_or_else(|| "alert-dialog-action".to_string()),
            style: props.style.clone().unwrap_or_default(),
            onclick: on_click,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogCancelProps {
    #[props(default)]
    on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    style: Option<String>,
    #[props(default = "button".to_string())]
    r#type: String,
    children: Element,
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    let set_open = ctx.set_open;
    let user_on_click = props.on_click;
    let on_click = EventHandler::new(move |evt: MouseEvent| {
        set_open.call(false);
        if let Some(cb) = &user_on_click {
            cb.call(evt.clone());
        }
    });
    rsx! {
        button {
            r#type: props.r#type.clone(),
            class: props.class.clone().unwrap_or_else(|| "alert-dialog-cancel".to_string()),
            style: props.style.clone().unwrap_or_default(),
            onclick: on_click,
            autofocus: true,
            {props.children}
        }
    }
}
