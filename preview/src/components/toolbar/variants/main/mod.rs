use super::super::component::*;
use dioxus::prelude::*;

#[component]
fn ToggleToolbarButton(
    index: usize,
    is_on: bool,
    on_click: Callback<()>,
    children: Element,
) -> Element {
    rsx! {
        ToolbarButton {
            index,
            on_click,
            "data-state": if is_on { "on" } else { "off" },
            background: if is_on { "var(--light, var(--primary-color-5)) var(--dark, var(--primary-color-6))" } else { "" },
            color: if is_on { "var(--secondary-color-1)" } else { "" },
            {children}
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let mut is_bold = use_signal(|| true);
    let mut is_italic = use_signal(|| false);
    let mut is_underline = use_signal(|| false);
    let mut text_align = use_signal(|| "left".to_string());

    rsx! {
        div { display: "flex", flex_direction: "column", gap: "0.75rem", align_items: "center",
            Toolbar { aria_label: "Text formatting",
                ToolbarGroup {
                    ToggleToolbarButton {
                        index: 0usize,
                        is_on: is_bold(),
                        on_click: move |_| is_bold.toggle(),
                        b { "B" }
                    }
                    ToggleToolbarButton {
                        index: 1usize,
                        is_on: is_italic(),
                        on_click: move |_| is_italic.toggle(),
                        i { "I" }
                    }
                    ToggleToolbarButton {
                        index: 2usize,
                        is_on: is_underline(),
                        on_click: move |_| is_underline.toggle(),
                        u { "U" }
                    }
                }
                ToolbarSeparator {}
                ToolbarGroup {
                    ToggleToolbarButton {
                        index: 3usize,
                        is_on: text_align() == "left",
                        on_click: move |_| text_align.set("left".to_string()),
                        "Left"
                    }
                    ToggleToolbarButton {
                        index: 4usize,
                        is_on: text_align() == "center",
                        on_click: move |_| text_align.set("center".to_string()),
                        "Center"
                    }
                    ToggleToolbarButton {
                        index: 5usize,
                        is_on: text_align() == "right",
                        on_click: move |_| text_align.set("right".to_string()),
                        "Right"
                    }
                }
            }
            p {
                margin: 0,
                max_width: "20rem",
                text_align: "{text_align}",
                font_weight: if is_bold() { "bold" } else { "normal" },
                font_style: if is_italic() { "italic" } else { "normal" },
                text_decoration: if is_underline() { "underline" } else { "none" },
                font_size: "0.85rem",
                "Sample text formatted by the toolbar."
            }
        }
    }
}
