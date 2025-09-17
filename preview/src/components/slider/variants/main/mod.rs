use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::slider::SliderValue;

#[component]
pub fn Demo() -> Element {
    let mut current_value = use_signal(|| 0.5);

    rsx! {
        // Display the current value
        div {
            style: "margin-bottom: 15px; font-size: 16px; font-weight: bold;",
            "{current_value:.0}%"
        }

        Slider {
            label: "Demo Slider",
            horizontal: true,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            default_value: SliderValue::Single(50.0),
            on_value_change: move |value: SliderValue| {
                // Extract the f64 value from SliderValue::Single
                let SliderValue::Single(v) = value;
                current_value.set(v);
            },
            SliderTrack {
                SliderRange {}
                SliderThumb {}
            }
        }
    }
}
