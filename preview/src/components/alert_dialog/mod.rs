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
        button {
            class: "alert-dialog-trigger",
            style: "margin-bottom: 1.5rem;",
            onclick: move |_| open.set(true),
            "Show Alert Dialog"
        }
        AlertDialogRoot { open: Some(open), on_open_change: move |v| open.set(v),
            AlertDialogContent { class: "alert-dialog",
                AlertDialogTitle { "Delete item" }
                AlertDialogDescription { "Are you sure you want to delete this item? This action cannot be undone." }
                AlertDialogActions {
                    class: "alert-dialog-actions",
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
