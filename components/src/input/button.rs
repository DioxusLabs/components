use crate::{
    input::ButtonGroupOrientation,
    style::{AsCss, Color, FontFamily, Orientation, Size},
};
use dioxus::prelude::*;
use std::fmt::Write as _;

const _: &str = manganis::mg!(file("./styles/input/button.css"));

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct ButtonSpacing(pub(crate) Signal<u8>);

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

impl AsCss for ButtonStyling {
    fn as_css(&self) -> String {
        let mut css = String::new();

        write!(css, "background-color:{};", self.background_color.as_css()).ok();
        write!(css, "color:{};", self.text_color.as_css()).ok();
        write!(css, "{}", self.text_font.as_css()).ok();

        css
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

    // Handles color transition on mouse hover.
    let mut mouse_hover = use_signal(|| false);

    // Handle button spacing
    let mut independent_class = "";
    let btn_spacing_css = if let Some(spacing) = try_consume_context::<ButtonSpacing>() {
        let spacing = spacing.0();

        // If spacing is greater than zero we need to do two things:
        // 1. Apply `independent` class so that the button has full border-radius
        // 2. Apply margins according to the ButtonGroup orientation if applicable.
        match spacing > 0 {
            true => {
                let final_spacing = spacing * crate::SPACING_INTERVAL;

                // Start building css
                let mut css = String::new();
                write!(css, "margin-left:{final_spacing}px;").ok();
                write!(css, "margin-right:{final_spacing}px;").ok();

                // If the ButtonGroup orientation is vertical, we need to use margin top and botttom instead.
                if let Some(btn_group_orientation) = try_consume_context::<ButtonGroupOrientation>()
                {
                    if btn_group_orientation.0() == Orientation::Vertical {
                        let mut new_css = String::new();
                        write!(new_css, "margin-top:{final_spacing}px;").ok();
                        write!(new_css, "margin-bottom:{final_spacing}px;").ok();
                        css = new_css;
                    }
                }

                independent_class = "independent";
                css
            }
            false => "".to_string(),
        }
    } else {
        // No spacing needed.
        "".to_string()
    };

    rsx! {
        button {
            class: "dxc-button {props.size.as_class()} {disabled_class} {independent_class}",

            style: "{styling.as_css()}",
            style: if !props.disabled && mouse_hover() { "{styling.hover_background_color.as_bg_css()}" },
            style: "{btn_spacing_css}",

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
