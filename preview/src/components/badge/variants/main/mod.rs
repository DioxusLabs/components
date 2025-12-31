use dioxus::prelude::*;

use super::super::component::*;
use crate::components::avatar::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            class: "badge-example",

            div {
                class: "badge-item",
                p { class: "badge-label", "Basic" }
                Badge {
                    count: 5,
                    Avatar {
                        size: AvatarImageSize::Medium,
                        shape: AvatarShape::Rounded,
                        aria_label: "Space item",
                    }
                }
            }

            div {
                class: "badge-item",
                p { class: "badge-label", "Show Zero" }

                Badge {
                    count: 0,
                    show_zero: true,
                    Avatar {
                        size: AvatarImageSize::Medium,
                        shape: AvatarShape::Rounded,
                        aria_label: "Space item",
                    }
                }
            }

            div {
                class: "badge-item",
                p { class: "badge-label", "Overflow" }

                Badge {
                    count: 100,
                    overflow_count: 99,
                    Avatar {
                        size: AvatarImageSize::Medium,
                        shape: AvatarShape::Rounded,
                        aria_label: "Space item",
                    }
                }
            }

            div {
                class: "badge-item",
                p { class: "badge-label", "Colorful" }

                Badge {
                    count: 7,
                    color: String::from("52c41a"),
                    Avatar {
                        size: AvatarImageSize::Medium,
                        shape: AvatarShape::Rounded,
                        aria_label: "Space item",
                    }
                }
            }

            div {
                class: "badge-item",
                p { class: "badge-label", "As Dot" }

                Badge {
                    count: 5,
                    dot: true,
                    Avatar {
                        size: AvatarImageSize::Medium,
                        shape: AvatarShape::Rounded,
                        aria_label: "Space item",
                    }
                }
            }
        }
    }
}
