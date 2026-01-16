use dioxus::prelude::*;

use super::super::component::*;
use crate::components::avatar::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "badge-example",

            div { class: "badge-item",
                p { class: "badge-label", "Basic" }
                NotifyBadge {
                    count: 5,
                    Avatar {
                        size: AvatarImageSize::Medium,
                        shape: AvatarShape::Rounded,
                        aria_label: "Space item",
                        CardIcon {}
                    }
                }
            }

            div { class: "badge-item",
                p { class: "badge-label", "Show Zero" }

                NotifyBadge {
                    count: 0,
                    show_zero: true,
                    Avatar {
                        size: AvatarImageSize::Medium,
                        shape: AvatarShape::Rounded,
                        aria_label: "Space item",
                        CardIcon {}
                    }
                }
            }

            div { class: "badge-item",
                p { class: "badge-label", "Overflow" }

                NotifyBadge {
                    count: 100,
                    overflow_count: 99,
                    Avatar {
                        size: AvatarImageSize::Medium,
                        shape: AvatarShape::Rounded,
                        aria_label: "Space item",
                        CardIcon {}
                    }
                }
            }

            div { class: "badge-item",
                p { class: "badge-label", "Colorful" }

                NotifyBadge {
                    count: 7,
                    style: "background-color: var(--highlight-color-main)",
                    Avatar {
                        size: AvatarImageSize::Medium,
                        shape: AvatarShape::Rounded,
                        aria_label: "Space item",
                        CardIcon {}
                    }
                }
            }

            div { class: "badge-item",
                p { class: "badge-label", "As Dot" }

                NotifyBadge {
                    count: 5,
                    dot: true,
                    Avatar {
                        size: AvatarImageSize::Medium,
                        shape: AvatarShape::Rounded,
                        aria_label: "Space item",
                        CardIcon {}
                    }
                }
            }
        }
    }
}
