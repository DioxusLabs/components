use crate::style::{Orientation, Size};
use dioxus::prelude::*;
use std::fmt::Write as _;

const _: &str = manganis::mg!(file("./styles/input/button_group.css"));

#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    #[props(optional)]
    size: Size,

    #[props(optional)]
    orientation: Orientation,

    children: Element,
}

pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let mut style = String::new();
    write!(
        style,
        "flex-direction:{};",
        props.orientation.as_flex_direction()
    )
    .ok();

    rsx! {
        div {
            class: "dxc-button-group",
            style,
            {props.children}
        }
    }
}
