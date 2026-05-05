use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: "0.75rem",
            label {
                display: "inline-flex",
                align_items: "center",
                gap: "0.625rem",
                cursor: "pointer",
                Checkbox { name: "agree-tos", aria_label: "Accept terms" }
                span { "Accept terms and conditions" }
            }
            label {
                display: "inline-flex",
                align_items: "center",
                gap: "0.625rem",
                cursor: "pointer",
                Checkbox { name: "agree-news", aria_label: "Subscribe to newsletter" }
                span { "Subscribe to newsletter" }
            }
            label {
                display: "inline-flex",
                align_items: "center",
                gap: "0.625rem",
                cursor: "pointer",
                Checkbox { name: "remember", aria_label: "Remember me" }
                span { "Remember this device" }
            }
        }
    }
}
