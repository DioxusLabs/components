use dioxus_lib::prelude::*;

// TODO: Docs

#[derive(Props, Clone, PartialEq)]
pub struct AspectRatioProps {
    /// The desired ratio. E.g. 16.0 / 9.0
    #[props(default = 1.0)]
    ratio: f64,

    children: Element,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    let ratio = 100.0 / (props.ratio);

    rsx! {
        div {
            style: "position: relative; width: 100%; padding-bottom: {ratio}%;",
            div {
                style: "position: absolute; inset: 0;",
                ..props.attributes,

                {props.children}
            }
        }
    }
}
