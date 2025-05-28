use dioxus::prelude::*;
use dioxus_primitives::progress::{Progress, ProgressIndicator};

#[component]
pub(super) fn ProgressExample() -> Element {
    let mut progress = use_signal(|| 80.0);

    rsx! {
        Progress { class: "progress", value: Some(progress.into()),
            ProgressIndicator { class: "progress-indicator" }
        }
        button { onclick: move |_| progress.set(progress() + 10.0), "Increment" }
        button { onclick: move |_| progress.set(progress() - 10.0), "Decrement" }
        button { onclick: move |_| progress.set(0.0), "Reset" }
        button { onclick: move |_| progress.set(100.0), "Complete" }
    }
}
