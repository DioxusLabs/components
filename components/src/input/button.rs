use crate::style::{Color, FontFamily, Size};
use dioxus::prelude::*;
use std::fmt::Display;

const _: &str = manganis::mg!(file("./styles/input/button.css"));

#[derive(Clone, PartialEq)]
pub struct ButtonStyling {
    pub background_color: Color,
    pub hover_background_color: Color,
    pub text_color: Color,
    pub font: FontFamily,
}

impl ButtonStyling {
    pub fn disabled() -> Self {
        Self {
            background_color: Color::hex("484848"),
            text_color: Color::hex("747474"),
            ..Default::default()
        }
    }
}

impl Default for ButtonStyling {
    fn default() -> Self {
        ButtonStyling {
            background_color: Color::hex("2B9FE1"),
            hover_background_color: Color::hex("166C9D"),
            text_color: Color::hex("FFFFFF"),
            font: FontFamily::default(),
        }
    }
}

impl Display for ButtonStyling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "background-color:{};", self.background_color)?;
        write!(f, "color:{};", self.text_color)?;
        write!(f, "{}", self.font)?;
        Ok(())
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(optional)]
    size: Size,

    #[props(optional)]
    style: ButtonStyling,

    #[props(optional, default = ButtonStyling::disabled())]
    disabled_style: ButtonStyling,

    #[props(optional, default = false)]
    disabled: bool,

    #[props(optional)]
    on_click: EventHandler<MouseEvent>,

    children: Element,
}

pub fn Button(props: ButtonProps) -> Element {
    let mut disabled = use_signal(|| props.disabled);

    // Subscribe to changes of `props.disabled`
    use_memo(use_reactive((&props.disabled,), move |(data,)| {
        disabled.set(data)
    }));

    // Determine styling if button is disabled.
    let styling = match disabled() {
        true => props.disabled_style,
        false => props.style,
    };

    let disabled_class = if disabled() { "disabled" } else { "" };

    // Handle color transition on mouse hover.
    let mut mouse_hover = use_signal(|| false);
    let hover_style = if !disabled() && mouse_hover() {
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
                if !disabled() {
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
