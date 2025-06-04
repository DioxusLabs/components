use dioxus::prelude::*;
use dioxus_primitives::avatar::{Avatar, AvatarFallback, AvatarImage};
#[component]
pub(super) fn Demo() -> Element {
    let mut avatar_state = use_signal(|| "No state yet".to_string());
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/avatar/style.css"),
        }
        div { display: "flex", flex_direction: "row", align_items: "center", justify_content: "between", gap: "1rem",
            div { class: "avatar-item",
                p { class: "avatar-label", "Basic Usage" }
                Avatar {
                    class: "avatar avatar-sm",
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 1: {state:?}"));
                    },
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
                    class: "avatar avatar-md",
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 2: {state:?}"));
                    },
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
                    class: "avatar avatar-lg",
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 4: {state:?}"));
                    },
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
