use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut volume = use_signal(|| 65.0);
    let mut brightness = use_signal(|| 40.0);
    rsx! {
        div { display: "flex", flex_direction: "column", gap: "1rem", min_width: "16rem",
            SliderRow {
                label: "Volume",
                value: volume(),
                on_change: move |v: f64| volume.set(v),
                default: 65.0,
            }
            SliderRow {
                label: "Brightness",
                value: brightness(),
                on_change: move |v: f64| brightness.set(v),
                default: 40.0,
            }
        }
    }
}

#[component]
fn SliderRow(label: String, value: f64, default: f64, on_change: Callback<f64>) -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: "0.3rem",
            div {
                display: "flex",
                justify_content: "space-between",
                font_size: "0.85rem",
                color: "var(--secondary-color-4)",
                span { "{label}" }
                span { "{value:.0}%" }
            }
            Slider {
                label,
                horizontal: true,
                min: 0.0,
                max: 100.0,
                step: 1.0,
                default_value: default,
                on_value_change: move |v: f64| on_change.call(v),
                SliderTrack {
                    SliderRange {}
                    SliderThumb {}
                }
            }
        }
    }
}
