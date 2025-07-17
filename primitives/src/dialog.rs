//! Defines the [`DialogRoot`] component and its sub-components.

use dioxus::document;
use dioxus::prelude::*;

use crate::{use_animated_open, use_controlled, use_id_or, use_unique_id, FOCUS_TRAP_JS};

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

/// The props for the [`DialogRoot`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogRootProps {
    /// The ID of the dialog root element.
    pub id: ReadOnlySignal<Option<String>>,

    /// Whether the dialog is modal. If true, it will trap focus within the dialog when open.
    #[props(default = ReadOnlySignal::new(Signal::new(true)))]
    pub is_modal: ReadOnlySignal<bool>,

    /// The controlled `open` state of the dialog.
    pub open: ReadOnlySignal<Option<bool>>,

    /// The default `open` state of the dialog if it is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// A callback that is called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional attributes to apply to the dialog root element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children of the dialog root component.
    children: Element,
}

/// # DialogRoot
///
/// The entry point for the dialog. It manages the open state of the dialog and provides context to its children. You
/// can use it to create a backdrop for the dialog if needed. The contents will only be rendered when the dialog is open.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Dialog"
///         }
///         DialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             DialogContent {
///                 button {
///                     aria_label: "Close",
///                     tabindex: if open() { "0" } else { "-1" },
///                     onclick: move |_| open.set(false),
///                     "×"
///                 }
///                 DialogTitle {
///                     "Item information"
///                 }
///                 DialogDescription {
///                     "Here is some additional information about the item."
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DialogRoot`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the dialog is open or closed. It can be either "open" or "closed".
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

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);

    let render = use_animated_open(id, open);

    rsx! {
        document::Script {
            src: FOCUS_TRAP_JS,
            defer: true
        }
        if render() {
            div {
                id,
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
}

/// The props for the [`DialogRoot`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// The ID of the dialog content element.
    pub id: ReadOnlySignal<Option<String>>,

    /// The class to apply to the dialog content element.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to apply to the dialog content element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The children of the dialog content.
    children: Element,
}

/// # DialogContent
///
/// The content of the dialog. Any interactive content in the dialog should be placed
/// inside this component. It will trap focus within the dialog while it is open
///
/// This must be used inside an [`DialogRoot`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Dialog"
///         }
///         DialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             DialogContent {
///                 button {
///                     aria_label: "Close",
///                     tabindex: if open() { "0" } else { "-1" },
///                     onclick: move |_| open.set(false),
///                     "×"
///                 }
///                 DialogTitle {
///                     "Item information"
///                 }
///                 DialogDescription {
///                     "Here is some additional information about the item."
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DialogRoot`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the dialog is open or closed. It can be either "open" or "closed".
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
        div {
            id,
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: ctx.dialog_labelledby,
            aria_describedby: ctx.dialog_describedby,
            class: props.class.clone().unwrap_or_else(|| "dialog".to_string()),
            onclick: move |e| {
                // Prevent the click event from propagating to the overlay.
                e.stop_propagation();
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DialogTitle`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    /// The ID of the dialog title element.
    pub id: ReadOnlySignal<Option<String>>,
    /// Additional attributes for the dialog title element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The children of the dialog title.
    children: Element,
}

/// # DialogTitle
///
/// The title of the dialog. This will be used to label the dialog for accessibility purposes.
///
/// This must be used inside an [`DialogRoot`] component and should be placed inside an [`DialogContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Dialog"
///         }
///         DialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             DialogContent {
///                 button {
///                     aria_label: "Close",
///                     tabindex: if open() { "0" } else { "-1" },
///                     onclick: move |_| open.set(false),
///                     "×"
///                 }
///                 DialogTitle {
///                     "Item information"
///                 }
///                 DialogDescription {
///                     "Here is some additional information about the item."
///                 }
///             }
///         }
///     }
/// }
/// ```
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

/// The props for the [`DialogDescription`] component
#[derive(Props, Clone, PartialEq)]
pub struct DialogDescriptionProps {
    /// The ID of the dialog description element.
    pub id: ReadOnlySignal<Option<String>>,
    /// Additional attributes for the dialog description element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The children of the dialog description.
    children: Element,
}

/// # DialogDescription
///
/// The description of the dialog. This will be used to describe the dialog for accessibility purposes.
///
/// This must be used inside an [`DialogRoot`] component and should be placed inside an [`DialogContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Dialog"
///         }
///         DialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             DialogContent {
///                 button {
///                     aria_label: "Close",
///                     tabindex: if open() { "0" } else { "-1" },
///                     onclick: move |_| open.set(false),
///                     "×"
///                 }
///                 DialogTitle {
///                     "Item information"
///                 }
///                 DialogDescription {
///                     "Here is some additional information about the item."
///                 }
///             }
///         }
///     }
/// }
/// ```
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
