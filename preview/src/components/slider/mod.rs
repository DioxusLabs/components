use dioxus::prelude::*;
use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack, SliderValue};

#[component]
pub(super) fn Demo() -> Element {
    let mut value = use_signal(|| SliderValue::Single(50.0));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/components/slider/style.css") }
        div { class: "slider-example",
            // Single value slider
            div {
                label { "Single Value Slider" }
                div { style: "display: flex; align-items: center; gap: 1rem;",
                    Slider {
                        class: "slider",
                        value,
                        horizontal: true,
                        on_value_change: move |v| {
                            value.set(v);
                        },

                        SliderTrack { class: "slider-track",
                            SliderRange { class: "slider-range" }
                            SliderThumb { class: "slider-thumb" }
                        }
                    }
                    input {
                        class: "slider-value",
                        r#type: "text",
                        readonly: true,
                        value: match value() {
                            SliderValue::Single(v) => format!("{v:.1}"),
                            _ => String::new(),
                        },
                    }
                }
            }
        }
    }
}
