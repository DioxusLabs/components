use dioxus::prelude::*;
use dioxus_primitives::slider::{
    self, SliderProps, SliderRangeProps, SliderThumbProps, SliderTrackProps,
};

#[component]
pub fn Slider(props: SliderProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        slider::Slider {
            class: "slider",
            value: props.value,
            default_value: props.default_value,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            horizontal: props.horizontal,
            inverted: props.inverted,
            on_value_change: props.on_value_change,
            label: props.label,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SliderTrack(props: SliderTrackProps) -> Element {
    rsx! {
        slider::SliderTrack { class: "slider-track", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn SliderRange(props: SliderRangeProps) -> Element {
    rsx! {
        slider::SliderRange { class: "slider-range", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn SliderThumb(props: SliderThumbProps) -> Element {
    rsx! {
        slider::SliderThumb {
            class: "slider-thumb",
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}
