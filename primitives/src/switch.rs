use crate::use_controlled;
use dioxus_lib::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    checked: Option<Signal<bool>>,

    #[props(default = false)]
    default_checked: bool,

    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    required: ReadOnlySignal<bool>,

    #[props(default)]
    name: ReadOnlySignal<String>,

    #[props(default = ReadOnlySignal::new(Signal::new(String::from("on"))))]
    value: ReadOnlySignal<String>,

    #[props(default)]
    on_checked_change: Callback<bool>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let (checked, set_checked) = use_controlled(
        props.checked,
        props.default_checked,
        props.on_checked_change,
    );

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            value: props.value,
            aria_checked: checked,
            aria_required: props.required,
            disabled: props.disabled,
            "data-state": if checked() { "checked" } else { "unchecked" },
            // Only add data-disabled when actually disabled
            "data-disabled": if (props.disabled)() { "true" } else { "false" },

            onclick: move |_| {
                let new_checked = !checked();
                set_checked.call(new_checked);
            },

            // Switches should only toggle on Space, not Enter
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    e.prevent_default();
                }
            },

            ..props.attributes,
            {props.children}
        }

        // Hidden input for form submission
        input {
            r#type: "checkbox",
            "aria-hidden": true,
            tabindex: -1,
            name: props.name,
            value: props.value,
            checked,
            disabled: props.disabled,
            style: "transform: translateX(-100%); position: absolute; pointer-events: none; opacity: 0; margin: 0; width: 0; height: 0;",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SwitchThumbProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn SwitchThumb(props: SwitchThumbProps) -> Element {
    rsx! {
        span { ..props.attributes }
    }
}
