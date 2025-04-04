use crate::use_controlled;
use dioxus_lib::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    pressed: Option<Signal<bool>>,

    #[props(default)]
    default_pressed: bool,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    on_pressed_change: Callback<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    // https://github.com/DioxusLabs/dioxus/issues/2467
    #[props(default)]
    onmounted: Callback<Event<MountedData>>,
    #[props(default)]
    onfocus: Callback<Event<FocusData>>,
    #[props(default)]
    onkeydown: Callback<Event<KeyboardData>>,

    children: Element,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let (pressed, set_pressed) = use_controlled(
        props.pressed,
        props.default_pressed,
        props.on_pressed_change,
    );

    rsx! {
        button {
            onmounted: props.onmounted,
            onfocus: props.onfocus,
            onkeydown: props.onkeydown,

            type: "button",
            disabled: props.disabled,
            aria_pressed: pressed,
            "data-state": if pressed() { "on" } else { "off" },
            "data-disabled": props.disabled,

            onclick: move |_| {
                let new_pressed = !pressed();
                set_pressed.call(new_pressed);
            },

            ..props.attributes,
            {props.children}
        }
    }
}
