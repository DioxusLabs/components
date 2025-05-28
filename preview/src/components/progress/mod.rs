use dioxus::prelude::*;
use dioxus_primitives::progress::{Progress, ProgressIndicator};

#[component]
pub(super) fn Demo() -> Element {
    let mut progress = use_signal(|| 80.0);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/components/progress/style.css") }
        Progress { class: "progress", value: Some(progress.into()),
            ProgressIndicator { class: "progress-indicator" }
        }
        button { class: "progress-button", onclick: move |_| progress.set(progress() + 10.0), "Increment" }
        button { class: "progress-button", onclick: move |_| progress.set(progress() - 10.0), "Decrement" }
        button { class: "progress-button", onclick: move |_| progress.set(0.0), "Reset" }
        button { class: "progress-button", onclick: move |_| progress.set(100.0), "Complete" }
    }
}
