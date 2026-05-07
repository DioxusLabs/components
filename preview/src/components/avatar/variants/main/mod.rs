use super::super::component::*;
use dioxus::prelude::*;

#[css_module("/src/components/avatar/style.css")]
struct Styles;

#[component]
pub fn Demo() -> Element {
    let mut avatar_state = use_signal(|| "No state yet".to_string());
    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            align_items: "center",
            justify_content: "between",
            gap: "1rem",
            div { class: Styles::dx_avatar_item,
                p { class: Styles::dx_avatar_label, "Basic Usage" }
                Avatar {
                    size: AvatarImageSize::Small,
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 1: {state:?}"));
                    },
                    aria_label: "Basic avatar",
                    AvatarImage {
                        class: Styles::dx_avatar_image,
                        src: "https://avatars.githubusercontent.com/u/66571940?s=96&v=4",
                        alt: "User avatar",
                    }
                    AvatarFallback { class: Styles::dx_avatar_fallback, "EA" }
                }
            }
            div { class: Styles::dx_avatar_item,
                p { class: Styles::dx_avatar_label, "Rounded" }
                Avatar {
                    size: AvatarImageSize::Small,
                    shape: AvatarShape::Rounded,
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 2: {state:?}"));
                    },
                    aria_label: "Basic avatar",
                    AvatarImage {
                        class: Styles::dx_avatar_image,
                        src: "https://avatars.githubusercontent.com/u/66571940?s=96&v=4",
                        alt: "User avatar",
                    }
                    AvatarFallback { class: Styles::dx_avatar_fallback, "EA" }
                }
            }
            div { class: Styles::dx_avatar_item,
                p { class: Styles::dx_avatar_label, "Error State" }
                Avatar {
                    size: AvatarImageSize::Medium,
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 3: {state:?}"));
                    },
                    aria_label: "Error avatar",
                    AvatarImage {
                        class: Styles::dx_avatar_image,
                        src: "https://invalid-url.example/image.jpg",
                        alt: "Invalid image",
                    }
                    AvatarFallback { class: Styles::dx_avatar_fallback, "JK" }
                }
            }
            div { class: Styles::dx_avatar_item,
                p { class: Styles::dx_avatar_label, "Large Size" }
                Avatar {
                    size: AvatarImageSize::Large,
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 4: {state:?}"));
                    },
                    aria_label: "Large avatar",
                    AvatarImage {
                        class: Styles::dx_avatar_image,
                        src: asset!("/assets/dioxus-logo.png", ImageAssetOptions::new().with_avif()),
                        alt: "Large avatar",
                    }
                    AvatarFallback { class: Styles::dx_avatar_fallback, "DX" }
                }
            }
        }
    }
}
