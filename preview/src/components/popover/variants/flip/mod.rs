use crate::components::button::component::Button;

use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::ContentSide;

/// Demonstrates automatic flip behavior. The popover is configured for the
/// bottom side, but when the trigger is scrolled near the bottom of the
/// viewport the popover flips to the top automatically.
#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        p { margin: 0, margin_bottom: "0.5rem", font_size: "0.875rem", color: "var(--secondary-color-5)",
            "This popover requests the bottom side. Scroll it near the bottom of the viewport and open it — it will flip to the top automatically."
        }
        div {
            display: "flex",
            gap: "1rem",
            align_items: "center",
            justify_content: "center",
            padding: "2rem 0",
            PopoverRoot { open: open(), on_open_change: move |v| open.set(v),
                PopoverTrigger { "Open popover (bottom)" }
                PopoverContent { side: ContentSide::Bottom,
                    p { margin: 0, padding: "0.25rem", "I flip to the top when there's no room below!" }
                    Button { r#type: "button", "data-style": "outline", onclick: move |_| open.set(false), "Close" }
                }
            }
        }
    }
}
