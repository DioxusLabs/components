use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};

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
        DialogRoot {
            class: "dialog-backdrop",
            open: open(),
            on_open_change: move |v| open.set(v),
            DialogContent {
                class: "dialog",
                button {
                    class: "dialog-close",
                    aria_label: "Close",
                    tabindex: if open() { "0" } else { "-1" },
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
}
