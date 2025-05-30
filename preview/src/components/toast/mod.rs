use std::time::Duration;
use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, ToastProvider, use_toast};

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
        div { class: "toast-example",
            h4 { "Timed Toasts (auto-dismiss)" }
            div { class: "toast-buttons",
                button {
                    onclick: move |_| {
                        toast_api
                            .success(
                                "Success".to_string(),
                                Some(ToastOptions {
                                    duration: Some(Duration::from_secs(3)),
                                    ..Default::default()
                                }),
                            );
                    },
                    "Success (3s)"
                }
                button {
                    onclick: move |_| {
                        toast_api
                            .error(
                                "Error".to_string(),
                                Some(ToastOptions {
                                    duration: Some(Duration::from_secs(5)),
                                    ..Default::default()
                                }),
                            );
                    },
                    "Error (5s)"
                }
                button {
                    onclick: move |_| {
                        toast_api
                            .warning(
                                "Warning".to_string(),
                                Some(ToastOptions {
                                    description: Some("This action might cause issues".to_string()),
                                    duration: Some(Duration::from_secs(3)),
                                    ..Default::default()
                                }),
                            );
                    },
                    "Warning (3s)"
                }
            }
            h4 { "Permanent Toasts (manual close)" }
            div { class: "toast-buttons",
                button {
                    onclick: move |_| {
                        toast_api
                            .success(
                                "Important".to_string(),
                                Some(ToastOptions {
                                    permanent: true,
                                    ..Default::default()
                                }),
                            );
                    },
                    "Permanent Success"
                }
                button {
                    onclick: move |_| {
                        toast_api
                            .error(
                                "Critical Error".to_string(),
                                Some(ToastOptions {
                                    permanent: true,
                                    ..Default::default()
                                }),
                            );
                    },
                    "Permanent Error"
                }
                button {
                    onclick: move |_| {
                        toast_api
                            .info(
                                "Custom Toast".to_string(),
                                Some(ToastOptions {
                                    description: Some(
                                        "This is a custom toast with specific settings".to_string(),
                                    ),
                                    duration: Some(Duration::from_secs(10)),
                                    permanent: false,
                                }),
                            );
                    },
                    "Custom Info (10s)"
                }
            }
        }
    }
}
