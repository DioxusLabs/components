use dioxus::prelude::*;

use super::super::component::*;
use dioxus_primitives::color_picker::Color;

#[component]
pub fn Demo() -> Element {
    let rgb = Color::random_rgb();
    let mut color = use_signal(|| rgb);

    rsx! {
        ColorPicker {
            label: "Pick",
            color: color(),
            on_color_change: move |c| {
                tracing::info!("Color changed: {:?}", c);
                color.set(c);
            },
            ColorPickerSelect { }
        }
    }
}
