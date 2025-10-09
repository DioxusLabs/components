use dioxus::prelude::*;
use dioxus_primitives::toggle::{self, ToggleProps};

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        toggle::Toggle {
            class: "toggle",
            pressed: props.pressed,
            default_pressed: props.default_pressed,
            disabled: props.disabled,
            on_pressed_change: props.on_pressed_change,
            onmounted: props.onmounted,
            onfocus: props.onfocus,
            onkeydown: props.onkeydown,
            attributes: props.attributes,
            {props.children}
        }
    }
}
