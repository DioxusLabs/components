use dioxus::prelude::*;
use dioxus_primitives::switch::{self, SwitchProps, SwitchThumbProps};

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        switch::Switch {
            class: "switch",
            checked: props.checked,
            default_checked: props.default_checked,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SwitchThumb(props: SwitchThumbProps) -> Element {
    rsx! {
        switch::SwitchThumb { class: "switch-thumb", attributes: props.attributes, {props.children} }
    }
}
