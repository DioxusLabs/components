use dioxus::prelude::*;
use dioxus_primitives::toolbar::{Toolbar, ToolbarButton, ToolbarSeparator};
#[component]
pub(super) fn Demo() -> Element {
    let mut text_style = use_signal(Vec::new);
    let mut text_align = use_signal(|| String::from("left"));
    let mut toggle_style = move |style: &str| {
        let mut current_styles = text_style();
        if current_styles.contains(&style.to_string()) {
            current_styles.retain(|s| s != style);
        } else {
            current_styles.push(style.to_string());
        }
        text_style.set(current_styles);
    };
    let mut set_align = move |align: &str| {
        text_align.set(align.to_string());
    };
    let text_styles = use_memo(move || {
        let mut classes = Vec::new();
        for style in text_style() {
            match style.as_str() {
                "bold" => classes.push("font-weight: bold;"),
                "italic" => classes.push("font-style: italic;"),
                "underline" => classes.push("text-decoration: underline;"),
                _ => {}
            }
        }
        classes.join(" ")
    });
    let text_align_style =
        use_memo(move || format!("text-align: {}; {}", text_align(), text_styles()));
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/toolbar/style.css"),
        }
        Toolbar { class: "toolbar", aria_label: "Text formatting",
            div { class: "toolbar-group",
                ToolbarButton {
                    class: "toolbar-button",
                    index: 0usize,
                    on_click: move |_| toggle_style("bold"),
                    "Bold"
                }
                ToolbarButton {
                    class: "toolbar-button",
                    index: 1usize,
                    on_click: move |_| toggle_style("italic"),
                    "Italic"
                }
                ToolbarButton {
                    class: "toolbar-button",
                    index: 2usize,
                    on_click: move |_| toggle_style("underline"),
                    "Underline"
                }
            }
            ToolbarSeparator { class: "toolbar-separator" }
            div { class: "toolbar-group",
                ToolbarButton {
                    class: "toolbar-button",
                    index: 3usize,
                    on_click: move |_| set_align("left"),
                    "Align Left"
                }
                ToolbarButton {
                    class: "toolbar-button",
                    index: 4usize,
                    on_click: move |_| set_align("center"),
                    "Align Center"
                }
                ToolbarButton {
                    class: "toolbar-button",
                    index: 5usize,
                    on_click: move |_| set_align("right"),
                    "Align Right"
                }
            }
        }
        p { style: text_align_style, max_width: "30rem",
            "This is a sample text that will be formatted according to the toolbar buttons you click. Try clicking the buttons above to see how the text formatting changes."
        }
    }
}
