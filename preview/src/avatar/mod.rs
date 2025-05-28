use dioxus::prelude::*;
use dioxus_primitives::avatar::{Avatar, AvatarFallback, AvatarImage};



#[component]
pub(super) fn AvatarExample() -> Element {
    let mut avatar_state = use_signal(|| "No state yet".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/src/avatar/style.css") }

        // Basic examples section
        div { class: "avatar-example-section",
            h4 { "Basic Examples" }
            div { class: "avatar-example",
                // Basic Avatar with image and fallback
                div { class: "avatar-item",
                    p { class: "avatar-label", "Basic Usage" }
                    Avatar {
                        class: "avatar",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 1: {:?}", state));
                        },

                        AvatarImage {
                            src: "https://github.com/DioxusLabs.png",
                            alt: "User avatar",
                        }

                        AvatarFallback { class: "avatar-fallback", "UA" }
                    }
                }

                // Avatar with error state (fallback shown)
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

                // Avatar with emoji fallback
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

                // Avatar with different size
                div { class: "avatar-item",
                    p { class: "avatar-label", "Large Size" }
                    Avatar {
                        class: "avatar avatar-lg",
                        on_state_change: move |state| {
                            avatar_state.set(format!("Avatar 4: {:?}", state));
                        },

                        AvatarImage {
                            src: "https://github.com/DioxusLabs.png",
                            alt: "Large avatar",
                        }

                        AvatarFallback { class: "avatar-fallback", "LG" }
                    }
                }
            }
        }
    }
}