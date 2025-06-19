use dioxus::document;
use dioxus_lib::prelude::*;

use crate::{use_controlled, use_id_or, use_unique_id, FOCUS_TRAP_JS};

#[derive(Clone, Copy)]
struct DialogCtx {
    #[allow(unused)]
    open: Memo<bool>,
    #[allow(unused)]
    set_open: Callback<bool>,

    // Whether the dialog is a modal and should capture focus.
    #[allow(unused)]
    is_modal: ReadOnlySignal<bool>,
    dialog_labelledby: Signal<String>,
    dialog_describedby: Signal<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogRootProps {
    id: ReadOnlySignal<Option<String>>,

    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    is_modal: ReadOnlySignal<bool>,

    open: ReadOnlySignal<Option<bool>>,

    #[props(default)]
    default_open: bool,

    #[props(default)]
    on_open_change: Callback<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn DialogRoot(props: DialogRootProps) -> Element {
    let dialog_labelledby = use_unique_id();
    let dialog_describedby = use_unique_id();

    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    use_context_provider(|| DialogCtx {
        open,
        set_open,
        is_modal: props.is_modal,
        dialog_labelledby,
        dialog_describedby,
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

    rsx! {
        div {
            class: "dialog-overlay",
            aria_hidden: (!open()).then_some("true"),
            onclick: move |_| {
                set_open.call(false);
            },
            "data-state": if open() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    id: ReadOnlySignal<Option<String>>,

    #[props(default)]
    class: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn DialogContent(props: DialogProps) -> Element {
    let ctx: DialogCtx = use_context();
    let open = ctx.open;
    let is_modal = ctx.is_modal;

    let gen_id = use_unique_id();
    let id = use_id_or(gen_id, props.id);
    use_effect(move || {
        let is_modal = is_modal();
        if !is_modal {
            // If the dialog is not modal, we don't need to trap focus.
            return;
        }

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
            aria_labelledby: ctx.dialog_labelledby,
            aria_describedby: ctx.dialog_describedby,
            class: props.class.clone().unwrap_or_else(|| "alert-dialog".to_string()),
            onclick: move |e| {
                // Prevent the click event from propagating to the overlay.
                e.stop_propagation();
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    id: ReadOnlySignal<Option<String>>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let ctx: DialogCtx = use_context();
    let id = use_id_or(ctx.dialog_labelledby, props.id);

    rsx! {
        h2 {
            id: id,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogDescriptionProps {
    id: ReadOnlySignal<Option<String>>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let ctx: DialogCtx = use_context();
    let id = use_id_or(ctx.dialog_describedby, props.id);

    rsx! {
        p {
            id: id,
            ..props.attributes,
            {props.children}
        }
    }
}
