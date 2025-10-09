use dioxus::prelude::*;
use dioxus_primitives::toast::{self, ToastProviderProps};

#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        toast::ToastProvider {
            default_duration: props.default_duration,
            max_toasts: props.max_toasts,
            render_toast: props.render_toast,
            {props.children}
        }
    }
}
