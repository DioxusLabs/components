use dioxus::prelude::*;
use dioxus_primitives::aspect_ratio::AspectRatio;


#[component]
pub(super) fn AspectRatioExample() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/aspect_ratio/style.css") }
        div { class: "aspect-ratio-container",
            AspectRatio { ratio: 4.0 / 3.0,
                img {
                    class: "aspect-ratio-image",
                    src: "https://upload.wikimedia.org/wikipedia/commons/thumb/e/ea/Van_Gogh_-_Starry_Night_-_Google_Art_Project.jpg/1280px-Van_Gogh_-_Starry_Night_-_Google_Art_Project.jpg",
                }
            }
        }
    }
}