use super::super::component::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/button/variants/main/style.css"),
        }
        button {
            class: "button",
            type: "button",
            "data-style": "outline",
            style: "margin-bottom: 1.5rem;",
            onclick: move |_| open.set(true),
            "Show Dialog"
        }
        DialogRoot {
            open: open(),
            on_open_change: move |v| open.set(v),
            DialogContent {
                button {
                    class: "dialog-close",
                    type: "button",
                    aria_label: "Close",
                    tabindex: if open() { "0" } else { "-1" },
                    onclick: move |_| open.set(false),
                    "×"
                }
                DialogTitle {
                    "Item information"
                }
                DialogDescription {
                    "Here is some additional information about the item."
                }
            }
        }
    }
}
