use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            min_width: "16rem",
            div {
                font_weight: "600",
                font_size: "1.05rem",
                "Dioxus Components"
            }
            div {
                color: "var(--secondary-color-5)",
                font_size: "0.85rem",
                "An accessible component library."
            }
            Separator {
                style: "margin: 0.875rem 0;",
                horizontal: true,
                decorative: true,
            }
            div {
                display: "flex",
                align_items: "center",
                gap: "0.875rem",
                font_size: "0.9rem",
                span { "Docs" }
                Separator { horizontal: false, decorative: true }
                span { "Source" }
                Separator { horizontal: false, decorative: true }
                span { "Examples" }
            }
        }
    }
}
