use super::super::component::AspectRatio;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../../style.css") }
        div {
            class: "dx-aspect-ratio-container",
            width: "16rem",
            max_width: "100%",
            AspectRatio { ratio: 16.0 / 9.0,
                div {
                    width: "100%",
                    height: "100%",
                    border: "1px solid var(--primary-color-6)",
                    border_radius: "0.5rem",
                    background: "var(--primary-color-3)",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    color: "var(--secondary-color-4)",
                    font_weight: "600",
                    font_size: "0.95rem",
                    letter_spacing: "0.04em",
                    "16 : 9"
                }
            }
        }
    }
}
