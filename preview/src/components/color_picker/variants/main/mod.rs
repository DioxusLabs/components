use dioxus::prelude::*;

use super::super::component::*;
use dioxus_primitives::color_picker::Color;

#[component]
pub fn Demo() -> Element {
    let mut color = use_signal(|| Color::new(155, 128, 255));

    rsx! {
        ColorPicker {
            label: "Pick",
            color: color(),
            on_color_change: move |c| {
                tracing::info!("Color changed: {:?}", c);
                color.set(c);
            },
        }
    }
}
