use dioxus::prelude::*;
use dioxus_primitives::popover::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut confirmed = use_signal(|| false);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/popover/variants/main/style.css"),
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/button/variants/main/style.css"),
        }
        PopoverRoot {
            open: open(),
            on_open_change: move |v| open.set(v),
            class: "popover",
            PopoverTrigger {
                class: "popover-trigger",
                "Show Popover"
            }
            PopoverContent { class: "popover-content",
                gap: "0.25rem",
                h3 {
                    padding_top: "0.25rem",
                    padding_bottom: "0.25rem",
                    width: "100%",
                    text_align: "center",
                    margin: 0,
                    "Delete Item?"
                }
                button {
                    class: "button",
                    "data-style": "outline",
                    onclick: move |_| {
                        open.set(false);
                        confirmed.set(true);
                    },
                    "Confirm"
                }
                button {
                    class: "button",
                    "data-style": "outline",
                    onclick: move |_| {
                        open.set(false);
                    },
                    "Cancel"
                }
            }
        }
        if confirmed() {
            p { style: "color: var(--contrast-error-color); margin-top: 16px; font-weight: 600;",
                "Item deleted!"
            }
        }
    }
}
