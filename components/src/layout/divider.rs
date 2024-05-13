use crate::style::{Color, Orientation};
use dioxus::prelude::*;
use std::fmt::Write as _;

const _: &str = manganis::mg!(file("./styles/layout/divider.css"));

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct DividerColorOverride(pub(crate) Signal<Color>);

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct DividerOrientationOverride(pub(crate) Signal<Orientation>);

#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    #[props(optional)]
    orientation: Orientation,
    
    #[props(optional, default = 0)]
    spacing: u32,

    #[props(optional, default = Color::hex("000000"))]
    color: Color,
}

pub fn Divider(props: DividerProps) -> Element {
    let spacing_half = props.spacing / 2;

    // Check for color override
    let color = match try_use_context::<DividerColorOverride>() {
        Some(color) => color.0(),
        None => props.color.clone(),
    };

    // Build styling
    let mut style = String::new();
    write!(style, "background-color:{};", color).ok();
    write!(style, "margin-left:{spacing_half}px;").ok();
    write!(style, "margin-right:{spacing_half}px;").ok();

    rsx! {
        div {
            class: "dxc-divider",
            style,
        }
    }
}
