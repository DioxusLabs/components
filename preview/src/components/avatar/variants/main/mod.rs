use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut avatar_state = use_signal(|| "No state yet".to_string());
    rsx! {
        div { display: "flex", flex_direction: "row", align_items: "center", justify_content: "between", gap: "1rem",
            div { class: "avatar-item",
                p { class: "avatar-label", "Basic Usage" }
                Avatar {
                    size: AvatarImageSize::Small,
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 1: {state:?}"));
                    },
                    aria_label: "Basic avatar",
                    AvatarImage {
                        class: "avatar-image",
                        src: "https://avatars.githubusercontent.com/u/66571940?s=96&v=4",
                        alt: "User avatar",
                    }
                    AvatarFallback { class: "avatar-fallback", "EA" }
                }
            }
            div { class: "avatar-item",
                p { class: "avatar-label", "Error State" }
                Avatar {
                    size: AvatarImageSize::Medium,
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 2: {state:?}"));
                    },
                    aria_label: "Error avatar",
                    AvatarImage {
                        class: "avatar-image",
                        src: "https://invalid-url.example/image.jpg",
                        alt: "Invalid image",
                    }
                    AvatarFallback { class: "avatar-fallback", "JK" }
                }
            }
            div { class: "avatar-item",
                p { class: "avatar-label", "Large Size" }
                Avatar {
                    size: AvatarImageSize::Large,
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 4: {state:?}"));
                    },
                    aria_label: "Large avatar",
                    AvatarImage {
                        class: "avatar-image",
                        src: asset!("/assets/dioxus-logo.png", ImageAssetOptions::new().with_avif()),
                        alt: "Large avatar",
                    }
                    AvatarFallback { class: "avatar-fallback", "DX" }
                }
            }
        }
    }
}
