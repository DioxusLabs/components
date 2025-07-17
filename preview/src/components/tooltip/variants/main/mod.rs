use dioxus::prelude::*;
use dioxus_primitives::{tooltip::{Tooltip, TooltipContent, TooltipTrigger}, ContentSide};

#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/tooltip/variants/main/style.css"),
        }
        Tooltip { class: "tooltip",
            TooltipTrigger { class: "tooltip-trigger",
                "Rich content"
            }
            TooltipContent {
                side: ContentSide::Left,
                class: "tooltip-content",
                style: "width: 200px;",
                h4 { style: "margin-top: 0; margin-bottom: 8px;", "Tooltip title" }
                p { style: "margin: 0;", "This tooltip contains rich HTML content with styling." }
            }
        }
    }
}
