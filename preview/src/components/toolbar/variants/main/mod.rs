use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut is_bold = use_signal(|| false);
    let mut is_italic = use_signal(|| false);
    let mut is_underline = use_signal(|| false);
    let mut text_align = use_signal(|| "left".to_string());

    rsx! {
        Toolbar { aria_label: "Text formatting",
            ToolbarGroup {
                ToolbarButton {
                    index: 0usize,
                    on_click: move |_| is_bold.toggle(),
                    "data-state": if is_bold() { "on" } else { "off" },
                    "Bold"
                }
                ToolbarButton {
                    index: 1usize,
                    on_click: move |_| is_italic.toggle(),
                    "data-state": if is_italic() { "on" } else { "off" },
                    "Italic"
                }
                ToolbarButton {
                    index: 2usize,
                    on_click: move |_| is_underline.toggle(),
                    "data-state": if is_underline() { "on" } else { "off" },
                    "Underline"
                }
            }
            ToolbarSeparator {}
            ToolbarGroup {
                ToolbarButton {
                    index: 3usize,
                    on_click: move |_| text_align.set("left".to_string()),
                    "data-state": if text_align() == "left" { "on" } else { "off" },
                    "Align Left"
                }
                ToolbarButton {
                    index: 4usize,
                    on_click: move |_| text_align.set("center".to_string()),
                    "data-state": if text_align() == "center" { "on" } else { "off" },
                    "Align Center"
                }
                ToolbarButton {
                    index: 5usize,
                    on_click: move |_| text_align.set("right".to_string()),
                    "data-state": if text_align() == "right" { "on" } else { "off" },
                    "Align Right"
                }
            }
        }
        p {
            max_width: "30rem",
            text_align: "{text_align}",
            font_weight: if is_bold() { "bold" } else { "normal" },
            font_style: if is_italic() { "italic" } else { "normal" },
            text_decoration: if is_underline() { "underline" } else { "none" },
            "This is a sample text that will be formatted according to the toolbar buttons you click. Try clicking the buttons above to see how the text formatting changes."
        }
    }
}
