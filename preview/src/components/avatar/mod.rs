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
        div { class: "avatar-example-section",
            div { class: "avatar-example",
                div { class: "avatar-item",
                    p { class: "avatar-label", "Basic Usage" }
                    Avatar {
                        class: "avatar",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 1: {:?}", state));
                        },
                        AvatarImage {
                            src: asset!("/assets/dioxus-logo.png", ImageAssetOptions::new().with_avif()),
                            alt: "User avatar",
                        }
                        AvatarFallback { class: "avatar-fallback", "UA" }
                    }
                }
                div { class: "avatar-item",
                    p { class: "avatar-label", "Error State" }
                    Avatar {
                        class: "avatar",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 2: {:?}", state));
                        },
                        AvatarImage {
                            src: "https://invalid-url.example/image.jpg",
                            alt: "Invalid image",
                        }
                        AvatarFallback { class: "avatar-fallback", "JD" }
                    }
                }
                div { class: "avatar-item",
                    p { class: "avatar-label", "Emoji Fallback" }
                    Avatar {
                        class: "avatar",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 3: {:?}", state));
                        },
                        AvatarImage {
                            src: "https://invalid-url.example/image.jpg",
                            alt: "Invalid image",
                        }
                        AvatarFallback { class: "avatar-fallback", "👤" }
                    }
                }
                div { class: "avatar-item",
                    p { class: "avatar-label", "Large Size" }
                    Avatar {
                        class: "avatar avatar-lg",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 4: {:?}", state));
                        },
                        AvatarImage {
                            src: asset!("/assets/dioxus-logo.png", ImageAssetOptions::new().with_avif()),
                            alt: "Large avatar",
                        }
                        AvatarFallback { class: "avatar-fallback", "LG" }
                    }
                }
            }
        }
    }
}
