use dioxus::prelude::*;
use dioxus_primitives::aspect_ratio::AspectRatio;
#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/aspect_ratio/variants/main/style.css"),
        }
        div {
            class: "aspect-ratio-container",
            width: "20rem",
            max_width: "30vw",
            AspectRatio { ratio: 4.0 / 3.0,
                div {
                    background: "linear-gradient(to bottom right, var(--primary-color-4), var(--primary-color-3))",
                    width: "100%",
                    height: "100%",
                }
            }
        }
    }
}
