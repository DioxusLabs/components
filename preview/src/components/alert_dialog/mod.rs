use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::*;

#[component]
pub(super) fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut confirmed = use_signal(|| false);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/alert_dialog/style.css"),
        }
        div {
            class: "alert-dialog-example",
            style: "padding: 20px; max-width: 420px; margin: 0 auto; background: var(--dim-background-color); border-radius: 8px; border: 1px solid var(--dim-border-color); box-shadow: 0 2px 8px rgba(0,0,0,0.08);",
            button {
                class: "alert-dialog-trigger",
                style: "margin-bottom: 1.5rem;",
                onclick: move |_| open.set(true),
                "Show Alert Dialog (Primitive)"
            }
            AlertDialogRoot { open: Some(open), on_open_change: move |v| open.set(v),
                AlertDialogContent { class: "alert-dialog",
                    AlertDialogTitle { "Delete item" }
                    AlertDialogDescription { "Are you sure you want to delete this item? This action cannot be undone." }
                    AlertDialogActions {
                        AlertDialogCancel { class: "alert-dialog-cancel", "Cancel" }
                        AlertDialogAction {
                            class: "alert-dialog-action",
                            on_click: move |_| confirmed.set(true),
                            "Delete"
                        }
                    }
                }
            }
            if confirmed() {
                p { style: "color: var(--error-text-color); margin-top: 16px; font-weight: 600;",
                    "Item deleted!"
                }
            }
        }
    }
}
