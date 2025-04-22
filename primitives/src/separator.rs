use dioxus_lib::prelude::*;

// TODO: Docs

#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    /// Horizontal if true, vertical if false.
    #[props(default = true)]
    horizontal: bool,

    /// If the separator is decorative and should not be classified
    /// as a separator to the ARIA standard.
    #[props(default = false)]
    decorative: bool,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let orientation = match props.horizontal {
        true => "horizontal",
        false => "vertical",
    };

    rsx! {
        div {
            role: if !props.decorative { "separator" } else { "none" },
            aria_orientation: if !props.decorative { orientation },
            "data-orientation": orientation,
            ..props.attributes,
        }
    }
}
