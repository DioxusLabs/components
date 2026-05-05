use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut progress = use_signal(|| 28_usize);

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
        div {
            display: "flex",
            flex_direction: "column",
            gap: "0.75rem",
            min_width: "16rem",
            div {
                display: "flex",
                justify_content: "space-between",
                font_size: "0.9rem",
                span { color: "var(--secondary-color-4)", "Uploading…" }
                span { color: "var(--secondary-color-5)", "{progress()}%" }
            }
            Progress {
                aria_label: "Upload progress",
                value: progress() as f64,
                ProgressIndicator {}
            }
            div {
                color: "var(--secondary-color-5)",
                font_size: "0.8rem",
                "dioxus-components.zip · 2.4 MB"
            }
        }
    }
}
