use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::ContentSide;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { display: "flex", flex_wrap: "wrap", gap: "2rem", align_items: "center", justify_content: "center",
            Tooltip {
                TooltipTrigger { "Top" }
                TooltipContent { side: ContentSide::Top,
                    "Tooltip on top"
                }
            }
            Tooltip {
                TooltipTrigger { "Right" }
                TooltipContent { side: ContentSide::Right,
                    "Tooltip on right"
                }
            }
            Tooltip {
                TooltipTrigger { "Bottom" }
                TooltipContent { side: ContentSide::Bottom,
                    "Tooltip on bottom"
                }
            }
            Tooltip {
                TooltipTrigger { "Left" }
                TooltipContent { side: ContentSide::Left,
                    "Tooltip on left"
                }
            }
        }
    }
}
