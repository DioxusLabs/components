use dioxus::prelude::*;
use dioxus_primitives::tooltip::{Tooltip, TooltipContent, TooltipSide, TooltipTrigger};


#[component]
pub(super) fn TooltipExample() -> Element {
    rsx! {
        div {
            class: "tooltip-example",
            style: "padding: 50px; display: flex; gap: 20px;",
            // Basic tooltip
            Tooltip { class: "tooltip",
                TooltipTrigger { class: "tooltip-trigger",
                    button { "Hover me" }
                }
                TooltipContent { class: "tooltip-content", "This is a basic tooltip" }
            }
            // Tooltip with different position
            Tooltip { class: "tooltip",
                TooltipTrigger { class: "tooltip-trigger",
                    button { "Right tooltip" }
                }
                TooltipContent { class: "tooltip-content", side: TooltipSide::Right,
                    "This tooltip appears on the right"
                }
            }
            // Tooltip with HTML content
            Tooltip { class: "tooltip",
                TooltipTrigger { class: "tooltip-trigger",
                    button { "Rich content" }
                }
                TooltipContent { class: "tooltip-content", style: "width: 200px;",
                    h4 { style: "margin-top: 0; margin-bottom: 8px;", "Tooltip title" }
                    p { style: "margin: 0;", "This tooltip contains rich HTML content with styling." }
                }
            }
        }
    }
}
