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
    open: ReadOnlySignal<bool>,
    set_open: Callback<bool>,
    labelledby: String,
    describedby: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogRootProps {
    #[props(default)]
    default_open: bool,
    #[props(default)]
    open: ReadOnlySignal<Option<bool>>,
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
    let open = use_memo(move || (props.open)().unwrap_or_else(|| open_signal()));
    use_context_provider(|| AlertDialogCtx {
        open: open.into(),
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
    let on_keydown = use_callback(move |e: Event<KeyboardData>| {
        if e.key() == Key::Escape {
            ctx.set_open.call(false);
            e.prevent_default();
        }
    });

    let on_focusout = use_callback(move |_e: Event<FocusData>| {
        ctx.set_open.call(false);
    });
    let on_backdrop_click = use_callback(move |_| ctx.set_open.call(false));
    if !(ctx.open)() {
        return rsx! {};
    }
    rsx! {
        div {
            class: "alert-dialog-backdrop",
            style: "position: fixed; inset: 0; background: rgba(0,0,0,0.3); z-index: 1000;",
            onclick: on_backdrop_click,
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
                    "position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); z-index: 1001;"
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
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    rsx! {
        h2 { id: ctx.labelledby.clone(), class: "alert-dialog-title", ..props.attributes, {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogDescriptionProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    rsx! {
        p { id: ctx.describedby.clone(), class: "alert-dialog-description", ..props.attributes, {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionsProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionProps {
    #[props(default)]
    on_click: Option<EventHandler<MouseEvent>>,
    #[props(default = "button".to_string())]
    r#type: String,
    children: Element,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    let set_open = ctx.set_open;
    let user_on_click = props.on_click;
    let on_click = use_callback(move |evt: MouseEvent| {
        set_open.call(false);
        if let Some(cb) = &user_on_click {
            cb.call(evt.clone());
        }
    });
    rsx! {
        button {
            r#type: props.r#type.clone(),
            onclick: on_click,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogCancelProps {
    #[props(default)]
    on_click: Option<EventHandler<MouseEvent>>,
    #[props(default = "button".to_string())]
    r#type: String,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    let set_open = ctx.set_open;
    let user_on_click = props.on_click;
    let on_click = use_callback(move |evt: MouseEvent| {
        set_open.call(false);
        if let Some(cb) = &user_on_click {
            cb.call(evt.clone());
        }
    });
    rsx! {
        button {
            r#type: props.r#type.clone(),
            onclick: on_click,
            autofocus: true,
            ..props.attributes,
            {props.children}
        }
    }
}
