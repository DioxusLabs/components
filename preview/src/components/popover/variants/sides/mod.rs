use crate::components::button::component::Button;

use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};

#[component]
pub fn Demo() -> Element {
    let mut open_top = use_signal(|| false);
    let mut open_right = use_signal(|| false);
    let mut open_bottom = use_signal(|| false);
    let mut open_left = use_signal(|| false);

    rsx! {
        div { display: "flex", flex_wrap: "wrap", gap: "1rem", align_items: "center", justify_content: "center",
            PopoverRoot { open: open_top(), on_open_change: move |v| open_top.set(v),
                PopoverTrigger { "Top" }
                PopoverContent { side: ContentSide::Top, align: ContentAlign::Center,
                    p { margin: 0, padding: "0.25rem", "Popover on top" }
                    Button { r#type: "button", "data-style": "outline", onclick: move |_| open_top.set(false), "Close" }
                }
            }
            PopoverRoot { open: open_right(), on_open_change: move |v| open_right.set(v),
                PopoverTrigger { "Right" }
                PopoverContent { side: ContentSide::Right, align: ContentAlign::Center,
                    p { margin: 0, padding: "0.25rem", "Popover on right" }
                    Button { r#type: "button", "data-style": "outline", onclick: move |_| open_right.set(false), "Close" }
                }
            }
            PopoverRoot { open: open_bottom(), on_open_change: move |v| open_bottom.set(v),
                PopoverTrigger { "Bottom" }
                PopoverContent { side: ContentSide::Bottom, align: ContentAlign::Center,
                    p { margin: 0, padding: "0.25rem", "Popover on bottom" }
                    Button { r#type: "button", "data-style": "outline", onclick: move |_| open_bottom.set(false), "Close" }
                }
            }
            PopoverRoot { open: open_left(), on_open_change: move |v| open_left.set(v),
                PopoverTrigger { "Left" }
                PopoverContent { side: ContentSide::Left, align: ContentAlign::Center,
                    p { margin: 0, padding: "0.25rem", "Popover on left" }
                    Button { r#type: "button", "data-style": "outline", onclick: move |_| open_left.set(false), "Close" }
                }
            }
        }
    }
}
