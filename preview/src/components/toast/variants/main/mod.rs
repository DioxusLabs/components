use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, ToastProvider, use_toast};
use std::time::Duration;

#[component]
pub fn Demo() -> Element {
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
            href: asset!("/src/components/toast/variants/main/style.css"),
        }
        // Additional styles just for the trigger button
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/button/variants/main/style.css"),
        }
        button {
            class: "button",
            "data-style": "outline",
            onclick: move |_| {
                toast_api
                    .info(
                        "Custom Toast".to_string(),
                        ToastOptions::new()
                            .description("Some info you need")
                            .duration(Duration::from_secs(60))
                            .permanent(false),
                    );
            },
            "Info (60s)"
        }
    }
}
