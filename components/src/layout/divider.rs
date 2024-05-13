use crate::style::Color;
use dioxus::prelude::*;
use std::fmt::Write as _;

const _: &str = manganis::mg!(file("./styles/layout/divider.css"));

#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    #[props(optional, default = 0)]
    spacing: u32,

    #[props(optional, default = Color::hex("000000"))]
    color: Color,
}

pub fn Divider(props: DividerProps) -> Element {
    let spacing_half = props.spacing / 2;

    let mut style = String::new();
    write!(style, "background-color:{};", props.color).ok();
    write!(style, "margin-left:{spacing_half}px;").ok();
    write!(style, "margin-right:{spacing_half}px;").ok();

    rsx! {
        div {
            class: "dxc-divider",
            style,
        }
    }
}
