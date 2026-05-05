use dioxus::prelude::*;
use dioxus_primitives::slider::{
    self, RangeSliderProps, SliderProps, SliderRangeProps, SliderThumbProps, SliderTrackProps,
};

#[css_module("/src/components/slider/style.css")]
struct Styles;

#[component]
pub fn Slider(props: SliderProps) -> Element {
    rsx! {
        slider::Slider {
            class: Styles::dx_slider,
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
pub fn RangeSlider(props: RangeSliderProps) -> Element {
    rsx! {
        slider::RangeSlider {
            class: Styles::dx_slider,
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
        slider::SliderTrack { class: Styles::dx_slider_track, attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn SliderRange(props: SliderRangeProps) -> Element {
    rsx! {
        slider::SliderRange { class: Styles::dx_slider_range, attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn SliderThumb(props: SliderThumbProps) -> Element {
    rsx! {
        slider::SliderThumb {
            class: Styles::dx_slider_thumb,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}
