use dioxus::prelude::*;
use dioxus_primitives::otp::{
    self, OneTimePasswordGroupProps, OneTimePasswordInputProps, OneTimePasswordSeparatorProps,
    OneTimePasswordSlotProps,
};

#[component]
pub fn OneTimePasswordInput(props: OneTimePasswordInputProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        otp::OneTimePasswordInput {
            class: "dx-otp",
            value: props.value,
            default_value: props.default_value,
            maxlength: props.maxlength,
            pattern: props.pattern,
            inputmode: props.inputmode,
            autocomplete: props.autocomplete,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            on_value_change: props.on_value_change,
            on_complete: props.on_complete,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn OneTimePasswordGroup(props: OneTimePasswordGroupProps) -> Element {
    rsx! {
        otp::OneTimePasswordGroup {
            class: "dx-otp-group",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn OneTimePasswordSlot(props: OneTimePasswordSlotProps) -> Element {
    rsx! {
        otp::OneTimePasswordSlot {
            class: "dx-otp-slot",
            index: props.index,
            attributes: props.attributes,
            span { class: "dx-otp-caret", aria_hidden: "true" }
            {props.children}
        }
    }
}

#[component]
pub fn OneTimePasswordSeparator(props: OneTimePasswordSeparatorProps) -> Element {
    rsx! {
        otp::OneTimePasswordSeparator {
            class: "dx-otp-separator",
            attributes: props.attributes,
            svg {
                width: "10",
                height: "10",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                line { x1: "5", y1: "12", x2: "19", y2: "12" }
            }
            {props.children}
        }
    }
}
