use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut avatar_state = use_signal(|| "No state yet".to_string());
    let basic_profile = AVATAR_PROFILE_OPTIONS[0];
    let rounded_profile = AVATAR_PROFILE_OPTIONS[1];
    let large_profile = AVATAR_PROFILE_OPTIONS[2];

    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            align_items: "center",
            justify_content: "between",
            gap: "1rem",
            div { class: "dx-avatar-item",
                p { class: "dx-avatar-label", "Basic Usage" }
                Avatar {
                    size: AvatarImageSize::Small,
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 1: {state:?}"));
                    },
                    aria_label: "Basic avatar",
                    AvatarImage {
                        class: "dx-avatar-image",
                        src: "{basic_profile.src}",
                        alt: "{basic_profile.name}",
                    }
                    AvatarFallback { class: "dx-avatar-fallback", "{basic_profile.initials}" }
                }
            }
            div { class: "dx-avatar-item",
                p { class: "dx-avatar-label", "Rounded" }
                Avatar {
                    size: AvatarImageSize::Small,
                    shape: AvatarShape::Rounded,
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 2: {state:?}"));
                    },
                    aria_label: "Basic avatar",
                    AvatarImage {
                        class: "dx-avatar-image",
                        src: "{rounded_profile.src}",
                        alt: "{rounded_profile.name}",
                    }
                    AvatarFallback { class: "dx-avatar-fallback", "{rounded_profile.initials}" }
                }
            }
            div { class: "dx-avatar-item",
                p { class: "dx-avatar-label", "Error State" }
                Avatar {
                    size: AvatarImageSize::Medium,
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 3: {state:?}"));
                    },
                    aria_label: "Error avatar",
                    AvatarImage {
                        class: "dx-avatar-image",
                        src: "https://invalid-url.example/image.jpg",
                        alt: "Invalid image",
                    }
                    AvatarFallback { class: "dx-avatar-fallback", "JK" }
                }
            }
            div { class: "dx-avatar-item",
                p { class: "dx-avatar-label", "Large Size" }
                Avatar {
                    size: AvatarImageSize::Large,
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 4: {state:?}"));
                    },
                    aria_label: "Large avatar",
                    AvatarImage {
                        class: "dx-avatar-image",
                        src: "{large_profile.src}",
                        alt: "{large_profile.name}",
                    }
                    AvatarFallback { class: "dx-avatar-fallback", "{large_profile.initials}" }
                }
            }
        }
    }
}
