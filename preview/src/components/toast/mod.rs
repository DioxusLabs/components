use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, ToastProvider, use_toast};
use std::time::Duration;

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        ToastProvider { ToastButton {} }
    }
}

#[component]
fn ToastButton() -> Element {
    let toast_api = use_toast();

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/toast/style.css"),
        }
        // Additional styles just for the trigger button
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/alert_dialog/style.css"),
        }
        button {
            class: "alert-dialog-trigger",
            onclick: move |_| {
                toast_api
                    .info(
                        "Custom Toast".to_string(),
                        Some(ToastOptions {
                            description: Some(
                                "Some info you need".to_string(),
                            ),
                            duration: Some(Duration::from_secs(60)),
                            permanent: false,
                        }),
                    );
            },
            "Info (60s)"
        }
        style {
            ""
        }
    }
}
