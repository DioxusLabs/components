use dioxus::document;
use dioxus_lib::prelude::*;

use crate::{use_controlled, use_id_or, use_unique_id, ContentAlign, ContentSide, FOCUS_TRAP_JS};

#[derive(Clone, Copy)]
struct PopoverCtx {
    #[allow(unused)]
    open: Memo<bool>,
    #[allow(unused)]
    set_open: Callback<bool>,

    // Whether the dialog is a modal and should capture focus.
    #[allow(unused)]
    is_modal: ReadOnlySignal<bool>,
    labelledby: Signal<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct PopoverRootProps {
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
pub fn PopoverRoot(props: PopoverRootProps) -> Element {
    let labelledby = use_unique_id();

    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    use_context_provider(|| PopoverCtx {
        open,
        set_open,
        is_modal: props.is_modal,
        labelledby,
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
            "data-state": if open() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    id: ReadOnlySignal<Option<String>>,

    #[props(default)]
    class: Option<String>,

    /// Side of the trigger to place the tooltip
    #[props(default = ContentSide::Bottom)]
    side: ContentSide,

    /// Alignment of the tooltip relative to the trigger
    #[props(default = ContentAlign::Center)]
    align: ContentAlign,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn PopoverContent(props: PopoverProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let open = ctx.open;
    let is_open = open();
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
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: ctx.labelledby,
            aria_hidden: (!is_open).then_some("true"),
            class: props.class.clone().unwrap_or_else(|| "alert-dialog".to_string()),
            "data-state": if is_open { "open" } else { "closed" },
            "data-side": props.side.as_str(),
            "data-align": props.align.as_str(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PopoverTitleProps {
    id: ReadOnlySignal<Option<String>>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[derive(Props, Clone, PartialEq)]
pub struct PopoverTriggerProps {
    /// Whether to use ARIA attributes
    #[props(default = true)]
    use_aria: bool,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let mut id = ctx.labelledby;
    let id_attribute = props
        .attributes
        .iter()
        .find(|attr| attr.name == "id")
        .cloned();
    use_effect(use_reactive!(|id_attribute| {
        if let Some(id_attribute) = id_attribute {
            match &id_attribute.value {
                dioxus_core::AttributeValue::Text(val) => id.set(val.to_string()),
                dioxus_core::AttributeValue::Float(val) => id.set(val.to_string()),
                dioxus_core::AttributeValue::Int(val) => id.set(val.to_string()),
                dioxus_core::AttributeValue::Bool(val) => id.set(val.to_string()),
                _ => {}
            }
        }
    }));

    rsx! {
        button {
            id,
            onclick: move |e| {
                // Prevent the click event from propagating to the overlay.
                e.stop_propagation();
                ctx.set_open.call(!(ctx.open)());
            },
            ..props.attributes,
            {props.children}
        }
    }
}
