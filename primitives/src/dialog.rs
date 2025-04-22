use dioxus_lib::{document::eval, prelude::*};

use crate::{use_controlled, use_id_or, use_unique_id};

#[derive(Clone, Copy)]
struct DialogCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,

    // Whether the dialog is a modal and should capture focus.
    is_modal: ReadOnlySignal<bool>,
    dialog_labelledby: Signal<String>,
    dialog_describedby: Signal<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    id: ReadOnlySignal<Option<String>>,

    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    is_modal: ReadOnlySignal<bool>,

    open: Option<Signal<bool>>,

    #[props(default)]
    default_open: bool,

    #[props(default)]
    on_open_change: Callback<bool>,

    children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let dialog_labelledby = use_unique_id();
    let dialog_describedby = use_unique_id();

    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let ctx = use_context_provider(|| DialogCtx {
        open,
        set_open,
        is_modal: props.is_modal,
        dialog_labelledby,
        dialog_describedby,
    });

    let gen_id = use_unique_id();
    let id = use_id_or(gen_id, props.id);
    use_effect(move || {
        let is_open = open();
        let is_modal = (props.is_modal)();

        let js = eval(
            r#"
            let id = await dioxus.recv();
            let is_open = await dioxus.recv();
            let is_modal = await dioxus.recv();

            let dialog = document.getElementById(id);

            if (is_open) {
                switch (is_modal) {
                    case true:
                        dialog.showModal();
                        break;
                    case false:
                        dialog.show();
                        break;
                }
            } else {
                dialog.close(); 
            }
            "#,
        );

        let _ = js.send(id());
        let _ = js.send(is_open);
        let _ = js.send(is_modal);
    });

    rsx! {
        dialog {
            id: id,
            aria_modal: props.is_modal,
            aria_labelledby: ctx.dialog_labelledby,
            aria_describedby: ctx.dialog_describedby,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    id: ReadOnlySignal<Option<String>>,
    children: Element,
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let ctx: DialogCtx = use_context();
    let id = use_id_or(ctx.dialog_labelledby, props.id);

    rsx! {
        h2 {
            id: id,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogDescriptionProps {
    id: ReadOnlySignal<Option<String>>,
    children: Element,
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let ctx: DialogCtx = use_context();
    let id = use_id_or(ctx.dialog_describedby, props.id);

    rsx! {
        p {
            id: id,
            {props.children}
        }
    }
}
