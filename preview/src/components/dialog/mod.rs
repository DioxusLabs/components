use dioxus::prelude::*;
use dioxus_primitives::{dialog::{Dialog, DialogDescription, DialogTitle}};

#[component]
pub(super) fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/dialog/style.css"),
        }
        button {
            class: "dialog-trigger",
            style: "margin-bottom: 1.5rem;",
            onclick: move |_| open.set(true),
            "Show Dialog"
        }
        Dialog {
            class: "dialog",
            open: open(),
            on_open_change: move |v| open.set(v),
            button {
                class: "dialog-close",
                aria_label: "Close",
                onclick: move |_| open.set(false),
                "×"
            }
            DialogTitle {
                class: "dialog-title",
                "Item information"
            }
            DialogDescription {
                class: "dialog-description",
                "Here is some additional information about the item."
            }
        }
    }
}
