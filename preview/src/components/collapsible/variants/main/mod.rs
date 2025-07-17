use dioxus::prelude::*;
use dioxus_primitives::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/collapsible/variants/main/style.css"),
        }
        Collapsible {
            class: "collapsible",
            on_open_change: move |open| {
                tracing::info!("{open};");
            },
            CollapsibleTrigger { class: "collapsible-trigger",
                b { "Recent Activity" }
                svg {
                    class: "collapsible-expand-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    // shifted up by 6 polyline { points: "6 9 12 15 18 9" }
                    polyline { points: "6 15 12 21 18 15" }
                    // shifted down by 6 polyline { points: "6 15 12 9 18 15" }
                    polyline { points: "6 9 12 3 18 9" }
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                gap: "0.5rem",
                max_width: "20rem",
                color: "var(--secondary-color-3)",
                div {
                    border: "1px solid var(--primary-color-6)",
                    border_radius: "0.5rem",
                    padding: "1rem",
                    "Added a new feature to the collapsible component",
                }
                CollapsibleContent { class: "collapsible-content",
                    div {
                        border: "1px solid var(--primary-color-6)",
                        border_radius: "0.5rem",
                        padding: "1rem",
                        "Fixed a bug in the collapsible component",
                    }
                    div {
                        border: "1px solid var(--primary-color-6)",
                        border_radius: "0.5rem",
                        padding: "1rem",
                        "Updated the documentation for the collapsible component",
                    }
                }
            }
        }
    }
}
