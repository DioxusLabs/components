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

use crate::{use_animated_open, use_id_or, use_unique_id, FOCUS_TRAP_JS};
use dioxus::document;
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
    id: ReadOnlySignal<Option<String>>,
    #[props(default)]
    default_open: bool,
    #[props(default)]
    open: ReadOnlySignal<Option<bool>>,
    #[props(default)]
    on_open_change: Callback<bool>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn AlertDialogRoot(props: AlertDialogRootProps) -> Element {
    let labelledby = use_unique_id().to_string();
    let describedby = use_unique_id().to_string();
    let mut open_signal = use_signal(|| props.default_open);
    let set_open = use_callback(move |v: bool| {
        open_signal.set(v);
        props.on_open_change.call(v);
    });
    let open = use_memo(move || (props.open)().unwrap_or_else(&*open_signal));
    use_context_provider(|| AlertDialogCtx {
        open: open.into(),
        set_open,
        labelledby,
        describedby,
    });

    // Add a escape key listener to the document when the dialog is open. We can't
    // just add this to the dialog itself because it might not be focused if the user
    // is highlighting text or interacting with another element.
    use_effect(move || {
        let mut escape = document::eval(
            "document.addEventListener('keydown', (event) => {
                if (event.key === 'Escape') {
                    event.preventDefault();
                    dioxus.send(true);
                }
            });",
        );
        spawn(async move {
            while let Ok(true) = escape.recv().await {
                set_open.call(false);
            }
        });
    });

    let id = use_unique_id();
    let id = use_id_or(id, props.id);
    let render_element = use_animated_open(id, open);

    rsx! {
        if render_element() {
            div {
                id,
                class: "alert-dialog-overlay",
                "data-state": if open() { "open" } else { "closed" },
                ..props.attributes,
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogContentProps {
    id: ReadOnlySignal<Option<String>>,

    #[props(default)]
    class: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    let ctx: AlertDialogCtx = use_context();

    let open = ctx.open;

    let gen_id = use_unique_id();
    let id = use_id_or(gen_id, props.id);
    use_effect(move || {
        document::eval(&format!(
            r#"let dialog = document.getElementById("{id}");
            let is_open = {open};

            if (is_open) {{
                dialog.trap = window.createFocusTrap(dialog);
            }}
            if (!is_open && dialog.trap) {{
                dialog.trap.remove();
                dialog.trap = null;
            }}"#
        ));
    });

    rsx! {
        document::Script {
            src: FOCUS_TRAP_JS,
            defer: true,
        }
        div {
            id,
            role: "alertdialog",
            aria_modal: "true",
            aria_labelledby: ctx.labelledby.clone(),
            aria_describedby: ctx.describedby.clone(),
            class: props.class.clone().unwrap_or_else(|| "alert-dialog".to_string()),
            ..props.attributes,
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
    let open = ctx.open;
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
            tabindex: if open() { "0" } else { "-1" },
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
    let open = ctx.open;
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
            tabindex: if open() { "0" } else { "-1" },
            onclick: on_click,
            ..props.attributes,
            {props.children}
        }
    }
}
