use dioxus::prelude::*;
use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack};

#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/slider/style.css"),
        }
        Slider {
            class: "slider",
            horizontal: true,
            SliderTrack { class: "slider-track",
                SliderRange { class: "slider-range" }
                SliderThumb { class: "slider-thumb" }
            }
        }
    }
}
