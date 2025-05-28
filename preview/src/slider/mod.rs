use dioxus::prelude::*;
use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack, SliderValue};

#[component]
pub(super) fn SliderExample() -> Element {
    let mut value = use_signal(|| SliderValue::Single(50.0));
    let mut range_value = use_signal(|| SliderValue::Range(25.0, 75.0));

    rsx! {
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
                        r#type: "text",
                        readonly: true,
                        value: match value() {
                            SliderValue::Single(v) => format!("{v:.1}"),
                            _ => String::new(),
                        },
                    }
                }
            }
            // Range slider
            div {
                label { "Range Slider" }
                div { style: "display: flex; align-items: center; gap: 1rem;",
                    Slider {
                        class: "slider",
                        value: range_value,
                        on_value_change: move |v| {
                            range_value.set(v);
                        },

                        SliderTrack { class: "slider-track",
                            SliderRange { class: "slider-range" }
                            SliderThumb { class: "slider-thumb", index: 0usize }
                            SliderThumb { class: "slider-thumb", index: 1usize }
                        }
                    }
                    input {
                        r#type: "text",
                        readonly: true,
                        value: match range_value() {
                            SliderValue::Range(start, end) => format!("{:.1}, {:.1}", start, end),
                            _ => String::new(),
                        },
                    }
                }
            }
        }
    }
}
