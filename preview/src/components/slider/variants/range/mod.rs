use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::slider::SliderRangeValue;

#[component]
pub fn Demo() -> Element {
    let mut current_value = use_signal(|| SliderRangeValue::new(20.0, 80.0));
    let display = use_memo(move || {
        let v = current_value();
        format!("{:.0} – {:.0}", v.start(), v.end())
    });

    rsx! {
        div {
            style: "margin-bottom: 15px; font-size: 16px; font-weight: bold;",
            "{display}"
        }

        RangeSlider {
            label: "Range Slider",
            horizontal: true,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            default_value: SliderRangeValue::new(20.0, 80.0),
            on_value_change: move |value: SliderRangeValue| {
                current_value.set(value);
            },
            SliderTrack {
                SliderRange {}
                SliderThumb { index: 0 }
                SliderThumb { index: 1 }
            }
        }
    }
}
