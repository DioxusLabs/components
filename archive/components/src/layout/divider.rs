use crate::style::{Color, Orientation};
use dioxus::prelude::*;
use std::fmt::Write as _;

const _: &str = manganis::mg!(file("./styles/layout/divider.css"));

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct DividerColor(pub(crate) Signal<Color>);

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct DividerVisible(pub(crate) Signal<bool>);

#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    /// The orientation of the divider.
    #[props(optional)]
    orientation: Orientation,

    /// The color of the divider.
    color: Option<Color>,
}

pub fn Divider(props: DividerProps) -> Element {
    // Hide divider if context has divider visible override.
    if let Some(div_visible) = try_consume_context::<DividerVisible>() {
        if div_visible.0() {
            return None;
        }
    }

    // If the color prop is set, use that. Otherwise use default unless the DividerColorOverride is provided.
    let color = match props.color {
        Some(c) => c,
        None => match try_consume_context::<DividerColor>() {
            Some(color) => color.0(),
            None => Color::hex("000000"),
        },
    };

    // Build styling
    let mut style = String::new();
    write!(style, "background-color:{};", color).ok();

    rsx! {
        div {
            class: "dxc-divider",
            style,
        }
    }
}
