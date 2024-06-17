use crate::{
    input::ButtonSpacing,
    layout::{DividerColor, DividerVisible},
    style::{Color, Orientation},
};
use dioxus::prelude::*;

const _: &str = manganis::mg!(file("./styles/input/button_group.css"));

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct ButtonGroupOrientation(pub(crate) Signal<Orientation>);

#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    /// The orientation of the button group.
    #[props(optional)]
    orientation: Orientation,

    /// The color of any children [`layout::Divider`].
    #[props(optional, default = Color::hex("1B85C0"))]
    divider_color: Color,

    /// The spacing between buttons.
    /// Any dividers between buttons will be hidden if `spacing` is greater than zero.
    #[props(optional, default = 0)]
    spacing: u8,

    children: Element,
}

pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    // Hide dividers when spacing > 0
    let should_hide_dividers = props.spacing > 0;
    let mut divider_spacing =
        use_context_provider(|| DividerVisible(Signal::new(should_hide_dividers)));

    if divider_spacing.0() != should_hide_dividers {
        divider_spacing.0.set(should_hide_dividers);
    }

    // Propogate spacing to buttons
    let mut btn_spacing = use_context_provider(|| ButtonSpacing(Signal::new(props.spacing)));
    if btn_spacing.0() != props.spacing {
        btn_spacing.0.set(props.spacing);
    }

    // Propogate orientation to children
    let mut orientation =
        use_context_provider(|| ButtonGroupOrientation(Signal::new(props.orientation)));

    if orientation.0() != props.orientation {
        orientation.0.set(props.orientation);
    }

    // Provide divider color (propogate up)
    let mut divider_color =
        use_context_provider(|| DividerColor(Signal::new(props.divider_color.clone())));

    if divider_color.0() != props.divider_color {
        divider_color.0.set(props.divider_color);
    }

    rsx! {
        div {
            class: "dxc-button-group {props.orientation.as_class()}",
            {props.children}
        }
    }
}
