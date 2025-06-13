use dioxus::prelude::*;
use dioxus_primitives::progress::{Progress, ProgressIndicator};
#[component]
pub(super) fn Demo() -> Element {
    let mut progress = use_signal(|| 0);

    use_effect(move || {
        let mut timer = document::eval(
            "setInterval(() => {
                dioxus.send(Math.floor(Math.random() * 30));
            }, 1000);",
        );
        spawn(async move {
            while let Ok(new_progress) = timer.recv::<usize>().await {
                let mut progress = progress.write();
                *progress = (*progress + new_progress) % 101;
            }
        });
    });

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/progress/style.css"),
        }
        Progress {
            aria_label: "Progressbar Demo",
            class: "progress",
            value: progress() as f64,
            ProgressIndicator { class: "progress-indicator" }
        }
    }
}
