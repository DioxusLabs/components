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

    children: Element,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let mut internal_pressed =
        use_signal(|| props.pressed.map(|x| x()).unwrap_or(props.default_pressed));

    let pressed = use_memo(move || props.pressed.unwrap_or(internal_pressed)());

    rsx! {
        button {
            type: "button",
            disabled: props.disabled,
            aria_pressed: pressed,
            "data-state": if pressed() { "on" } else { "off" },
            "data-disabled": props.disabled,

            onclick: move |_| {
                let new_pressed = !pressed();
                internal_pressed.set(new_pressed);
                props.on_pressed_change.call(new_pressed);
            },

            ..props.attributes,
            {props.children}
        }
    }
}
