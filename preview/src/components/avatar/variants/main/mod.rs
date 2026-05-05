use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            align_items: "center",
            justify_content: "center",
            gap: "1rem",
            AvatarItem {
                name: "Evan A.",
                src: "https://avatars.githubusercontent.com/u/66571940?s=96&v=4",
                fallback: "EA",
                shape: AvatarShape::Circle,
            }
            AvatarItem {
                name: "JK",
                src: "https://invalid-url.example/image.jpg",
                fallback: "JK",
                shape: AvatarShape::Circle,
            }
            AvatarItem {
                name: "Dioxus",
                src: asset!("/assets/dioxus-logo.png", ImageAssetOptions::new().with_avif()).to_string(),
                fallback: "DX",
                shape: AvatarShape::Rounded,
            }
        }
    }
}

#[component]
fn AvatarItem(name: String, src: String, fallback: String, shape: AvatarShape) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            gap: "0.5rem",
            Avatar {
                size: AvatarImageSize::Large,
                shape,
                aria_label: name.clone(),
                AvatarImage {
                    class: "dx-avatar-image",
                    src,
                    alt: name.clone(),
                }
                AvatarFallback { class: "dx-avatar-fallback", "{fallback}" }
            }
            div {
                font_size: "0.85rem",
                color: "var(--secondary-color-4)",
                "{name}"
            }
        }
    }
}
