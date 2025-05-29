use dioxus::prelude::*;
use dioxus_primitives::aspect_ratio::AspectRatio;
#[component]
pub(super) fn Demo() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/aspect_ratio/style.css"),
        }
        div {
            class: "aspect-ratio-container",
            width: "10em",
            min_width: "30vw",
            AspectRatio { ratio: 4.0 / 3.0,
                img {
                    class: "aspect-ratio-image",
                    src: asset!("/assets/dioxus-logo.png", ImageAssetOptions::new().with_avif()),
                }
            }
        }
    }
}
