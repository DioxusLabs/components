use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::ContentSide;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            gap: "0.5rem",
            div {
                color: "var(--secondary-color-5)",
                font_size: "0.85rem",
                "Hover the link"
            }
            HoverCard {
                HoverCardTrigger {
                    a {
                        href: "#",
                        color: "var(--highlight-color-tertiary)",
                        text_decoration: "underline",
                        font_weight: "500",
                        "@dioxuslabs"
                    }
                }
                HoverCardContent { side: ContentSide::Bottom,
                    div {
                        padding: "0.75rem",
                        max_width: "16rem",
                        font_size: "0.85rem",
                        b { "Dioxus" }
                        " · The Rust framework for building fullstack web, desktop, and mobile apps."
                    }
                }
            }
        }
    }
}
