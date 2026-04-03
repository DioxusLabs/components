use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::ContentSide;

/// Demonstrates automatic flip behavior. Each tooltip requests a specific side
/// but will flip to the opposite side when there is not enough room.
#[component]
pub fn Demo() -> Element {
    rsx! {
        p { margin: 0, margin_bottom: "0.5rem", font_size: "0.875rem", color: "var(--secondary-color-5)",
            "These tooltips request the bottom side. Scroll them near the bottom of the viewport and hover — they flip to the top automatically."
        }
        div {
            display: "flex",
            gap: "2rem",
            align_items: "center",
            justify_content: "center",
            padding: "2rem 0",
            Tooltip {
                TooltipTrigger { "Hover me (bottom)" }
                TooltipContent { side: ContentSide::Bottom,
                    "I flip to the top when there's no room below!"
                }
            }
            Tooltip {
                TooltipTrigger { "Hover me (left)" }
                TooltipContent { side: ContentSide::Left,
                    "I flip to the right when there's no room on the left!"
                }
            }
        }
    }
}
