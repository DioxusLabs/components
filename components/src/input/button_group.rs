use crate::{
    layout::{DividerColorOverride, DividerOrientationOverride},
    style::{Color, Orientation, Size},
};
use dioxus::prelude::*;

const _: &str = manganis::mg!(file("./styles/input/button_group.css"));

#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    #[props(optional)]
    size: Size,

    #[props(optional)]
    orientation: Orientation,

    #[props(optional, default = Color::hex("1B85C0"))]
    divider_color: Color,

    children: Element,
}

pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    // Override divider color (propogate up)
    let mut divider_color =
        use_context_provider(|| DividerColorOverride(Signal::new(props.divider_color.clone())));

    if divider_color.0() != props.divider_color {
        divider_color.0.set(props.divider_color);
    }

    // Override divider orientation
    let mut divider_orientation =
        use_context_provider(|| DividerOrientationOverride(Signal::new(props.orientation.clone())));

    if divider_orientation.0() != props.orientation {
        divider_orientation.0.set(props.orientation);
    }

    rsx! {
        div {
            class: "dxc-button-group {props.orientation.as_class()}",
            {props.children}
        }
    }
}
