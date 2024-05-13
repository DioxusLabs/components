use crate::style::{Color, FontFamily, Size};
use dioxus::prelude::*;
use std::fmt::Display;

const _: &str = manganis::mg!(file("./styles/input/button.css"));

#[derive(Clone, PartialEq)]
pub struct ButtonStyling {
    /// The background color of this button.
    pub background_color: Color,
    /// The background color of this button when hovered over.
    pub hover_background_color: Color,
    /// The text color of this button.
    pub text_color: Color,
    /// The font family of this button.
    pub text_font: FontFamily,
}

impl ButtonStyling {
    /// Default styling for disabled buttons.
    pub fn disabled() -> Self {
        Self {
            background_color: Color::hex("484848"),
            text_color: Color::hex("747474"),
            ..Default::default()
        }
    }
}

impl Default for ButtonStyling {
    /// Default styling for buttons.
    fn default() -> Self {
        ButtonStyling {
            background_color: Color::hex("2B9FE1"),
            hover_background_color: Color::hex("166C9D"),
            text_color: Color::hex("FFFFFF"),
            text_font: FontFamily::default(),
        }
    }
}

impl Display for ButtonStyling {
    /// Display [`ButtonStyling`] in valid css.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "background-color:{};", self.background_color)?;
        write!(f, "color:{};", self.text_color)?;
        write!(f, "{}", self.text_font)?;
        Ok(())
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Optional size for this  button.
    /// See [`style::Size`] for more info.
    #[props(optional)]
    size: Size,

    /// Optional styling for this button.
    #[props(optional)]
    style: ButtonStyling,

    /// Optional styling when this button is disabled.
    #[props(optional, default = ButtonStyling::disabled())]
    disabled_style: ButtonStyling,

    /// Optionally mark this button as disabled.
    /// Defaults to `false`.
    #[props(optional, default = false)]
    disabled: bool,

    /// Optionally listen to click events from this button.
    #[props(optional)]
    on_click: EventHandler<MouseEvent>,

    children: Element,
}

pub fn Button(props: ButtonProps) -> Element {
    // Determine styling if button is disabled.
    let styling = match props.disabled {
        true => props.disabled_style,
        false => props.style,
    };

    let disabled_class = if props.disabled { "disabled" } else { "" };

    // Handle color transition on mouse hover.
    let mut mouse_hover = use_signal(|| false);
    let hover_style = if !props.disabled && mouse_hover() {
        format!(
            "background-color:{};",
            styling.hover_background_color.to_string()
        )
    } else {
        "".to_string()
    };

    rsx! {
        button {
            class: "dxc-button {props.size} {disabled_class}",
            style: "{styling}{hover_style}",
            onclick: move |evt| {
                if !props.disabled {
                    props.on_click.call(evt);
                }
            },
            onmouseenter: move |_| {
                mouse_hover.set(true);
            },
            onmouseleave: move |_| {
                mouse_hover.set(false);
            },
            {props.children}
        }
    }
}
